//! Git repository discovery and traversal functionality.

use std::path::{Path, PathBuf};

use crate::config::Config;
use crate::error::{Result, TagVerError};
use crate::tags::{parse_tags, TagMap, VersionTag};
use crate::version::Version;

/// Git repository wrapper with tagver-specific operations.
pub struct Repository {
    inner: gix::Repository,
    is_shallow: bool,
}

impl Repository {
    /// Discover and open a Git repository in the given directory.
    pub fn discover(path: impl Into<PathBuf>) -> Result<Self> {
        let path = path.into();

        let repo = gix::discover(&path)
            .map_err(|e| TagVerError::GitRepoNotFound(format!("{}: {}", path.display(), e)))?;

        let is_shallow = repo.is_shallow();

        Ok(Self {
            inner: repo,
            is_shallow,
        })
    }

    /// Check if the repository is shallow.
    pub fn is_shallow(&self) -> bool {
        self.is_shallow
    }

    /// Get the repository's work directory.
    pub fn work_dir(&self) -> Option<&Path> {
        self.inner.workdir()
    }

    /// Get the inner gix repository.
    pub fn inner(&self) -> &gix::Repository {
        &self.inner
    }
}

/// Calculate version by traversing the commit graph.
///
/// Algorithm:
/// 1. Parse all tags matching the prefix into a commit->version map
/// 2. Walk from HEAD towards root, counting height
/// 3. When a tagged commit is found, synthesize version based on:
///    - If at tag (height=0): use exact version
///    - If past pre-release tag: append height to prerelease
///    - If past RTM tag: increment + default prerelease + height
/// 4. Apply minimum major.minor constraint if configured
/// 5. Merge build metadata
pub fn calculate_version(repo: &Repository, config: &Config) -> Result<(Version, u32, bool)> {
    // Step 1: Parse all version tags
    let (tag_map, _invalid_tags) = parse_tags(repo.inner(), config)?;

    // Step 2: Get HEAD commit
    let mut head = repo
        .inner()
        .head()
        .map_err(|e| TagVerError::Other(format!("Failed to get HEAD: {}", e)))?;

    let head_commit = match head.try_peel_to_id() {
        Ok(Some(id)) => id.detach(),
        Ok(None) | Err(_) => {
            // No commits - return default version
            let version = Version::default(&config.default_prerelease_identifiers);
            let version = apply_config(version, config, None, 0);
            return Ok((version, 0, false));
        }
    };

    // Step 3: Walk the commit graph
    let (base_tag, height) = walk_to_tag(repo.inner(), head_commit, &tag_map)?;

    // Respect ignore_height by zeroing the height used for version synthesis
    let effective_height = if config.ignore_height { 0 } else { height };

    // Step 4: Synthesize version based on tag type and height
    let (version, is_from_tag) = match base_tag {
        Some(ref tag) => {
            let synthesized = synthesize_version(&tag.version, effective_height, config);
            (synthesized, height == 0)
        }
        None => {
            // No tag found - use default version
            let default = Version::default(&config.default_prerelease_identifiers);
            let version = if effective_height > 0 {
                let mut v = default;
                v.prerelease.push(effective_height.to_string());
                v
            } else {
                default
            };
            (version, false)
        }
    };

    // Step 5: Apply config (minimum, build metadata)
    let final_version = apply_config(version, config, base_tag.as_ref(), height);

    Ok((final_version, height, is_from_tag))
}

/// Walk from a commit towards ancestors, looking for a tagged commit.
/// Returns the found tag (if any) and the height (number of commits walked).
fn walk_to_tag(
    repo: &gix::Repository,
    start: gix::ObjectId,
    tag_map: &TagMap,
) -> Result<(Option<VersionTag>, u32)> {
    let mut height: u32 = 0;
    let mut current = start;

    loop {
        // Check if current commit has a tag
        if let Some(tags) = tag_map.get(&current) {
            // Tags are sorted highest first, use the first one
            if let Some(tag) = tags.first() {
                return Ok((Some(tag.clone()), height));
            }
        }

        // Get the commit object
        let commit = match repo.find_object(current) {
            Ok(obj) => match obj.try_into_commit() {
                Ok(c) => c,
                Err(_) => break,
            },
            Err(_) => break,
        };

        // Get first parent (for first-parent traversal)
        let parents: Vec<_> = commit.parent_ids().collect();

        // If no parents, we've reached the root
        if parents.is_empty() {
            break;
        }

        // Move to first parent and increment height
        current = parents[0].detach();
        height += 1;
    }

    Ok((None, height))
}

/// Synthesize version based on base tag, height, and config.
fn synthesize_version(base: &Version, height: u32, config: &Config) -> Version {
    if height == 0 {
        // Exactly on tag - use as-is (build metadata handled later)
        return base.clone();
    }

    if base.is_prerelease() {
        // Pre-release: append height
        // 1.0.0-beta.1 + height=3 -> 1.0.0-beta.1.3
        base.with_prerelease_height(height)
    } else {
        // RTM: increment + default prerelease + height
        // 1.0.0 + Patch + height=5 -> 1.0.1-alpha.0.5
        base.with_rtm_height(
            height,
            &config.auto_increment,
            &config.default_prerelease_identifiers,
        )
    }
}

/// Apply configuration constraints and metadata.
fn apply_config(
    mut version: Version,
    config: &Config,
    tag: Option<&VersionTag>,
    height: u32,
) -> Version {
    // Apply minimum major.minor
    // Only apply if we are not exactly on a tag, or if there is no tag
    if let Some(ref min) = config.minimum_major_minor {
        if height > 0 || tag.is_none() {
            version = version.apply_minimum(min, &config.default_prerelease_identifiers);
        }
    }

    // Merge build metadata
    let tag_metadata = tag.and_then(|t| t.version.build_metadata.as_deref());
    let config_metadata = config.build_metadata.as_deref();

    // Only merge build metadata if we're on a tag or config provides it
    if tag_metadata.is_some() || config_metadata.is_some() {
        // For height > 0, tag metadata is NOT carried forward
        // Only config metadata is used
        let effective_tag_metadata = if height == 0 { tag_metadata } else { None };

        version = version.with_merged_build_metadata(effective_tag_metadata, config_metadata);
    }

    version
}

/// Calculate version, handling the case where no repository is found.
/// This allows the version calculation to work even in non-git directories.
pub fn calculate_version_fallback(
    work_dir: impl Into<PathBuf>,
    config: &Config,
) -> Result<(Version, u32, bool)> {
    let work_dir = work_dir.into();

    match Repository::discover(&work_dir) {
        Ok(repo) => calculate_version(&repo, config),
        Err(TagVerError::GitRepoNotFound(_)) => {
            let version = Version::default(&config.default_prerelease_identifiers);
            let version = apply_config(version, config, None, 0);
            Ok((version, 0, false))
        }
        Err(e) => Err(e),
    }
}

/// Check if the given directory is a valid Git working directory.
pub fn is_git_directory(path: impl Into<PathBuf>) -> bool {
    let path = path.into();

    gix::discover(path).is_ok()
}
