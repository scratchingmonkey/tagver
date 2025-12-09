//! Tag parsing and management functionality.

use crate::config::Config;
use crate::error::{Result, TagVerError};
use crate::version::Version;
use std::collections::HashMap;

/// A map from commit IDs to their version tags.
pub type TagMap = HashMap<gix::ObjectId, Vec<VersionTag>>;

/// A version tag with its parsed information.
#[derive(Debug, Clone)]
pub struct VersionTag {
    pub version: Version,
    pub tag_name: String,
}

/// Parse all tags in the repository that match the configured prefix.
///
/// Returns:
/// - TagMap: mapping from commit ObjectId to list of version tags
/// - `Vec<String>`: list of invalid/unparseable tags (for warnings)
pub fn parse_tags(repo: &gix::Repository, config: &Config) -> Result<(TagMap, Vec<String>)> {
    let mut tag_map: TagMap = HashMap::new();
    let mut invalid_tags: Vec<String> = Vec::new();

    // Get all references
    let refs = repo
        .references()
        .map_err(|e| TagVerError::Other(format!("Failed to get references: {}", e)))?;

    // Filter for tags
    let tag_refs = refs
        .tags()
        .map_err(|e| TagVerError::Other(format!("Failed to get tags: {}", e)))?;

    for mut tag_ref in tag_refs.flatten() {
        let tag_name = tag_ref.name().shorten().to_string();

        // Apply prefix filter
        let version_str = if config.tag_prefix.is_empty() {
            tag_name.clone()
        } else if tag_name.starts_with(&config.tag_prefix) {
            tag_name[config.tag_prefix.len()..].to_string()
        } else {
            continue; // Skip tags that don't match prefix
        };

        // Parse as semver
        match version_str.parse::<semver::Version>() {
            Ok(semver) => {
                // Resolve the tag to its target commit
                let target_id = match tag_ref.peel_to_id() {
                    Ok(id) => id.detach(),
                    Err(_) => continue, // Skip if we can't resolve
                };

                let version = Version::from_semver_full(&semver);
                let version_tag = VersionTag {
                    version,
                    tag_name: tag_name.clone(),
                };

                tag_map.entry(target_id).or_default().push(version_tag);
            }
            Err(_) => {
                invalid_tags.push(tag_name);
            }
        }
    }

    // Sort tags at each commit by version (highest first)
    for tags in tag_map.values_mut() {
        tags.sort_by(|a, b| b.version.cmp(&a.version));
    }

    Ok((tag_map, invalid_tags))
}
