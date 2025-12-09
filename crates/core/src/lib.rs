//! # TagVer core library
//! Minimalistic version calculation from Git tags, mirroring the original .NET MinVer behavior.
//!
//! ## Quick Start
//!
//! Get the version for the current Git repository.
//!
//! ```rust,no_run
//! # use tagver::TagVerError;
//! use tagver::{calculate_version, Config};
//!
//! // Use default configuration
//! let config = Config::default();
//!
//! // Calculate version from the current directory
//! let result = calculate_version(".", &config)?;
//!
//! println!("Calculated version: {}", result);
//! # Ok::<_, TagVerError>(())
//! ```
//!
//! ## Strict vs. fallback entry points
//! - [`calculate_version`] — requires a real Git repository and errors otherwise.
//! - [`calculate_version_with_fallback`] — returns the default version when no repository is found.

pub mod config;
pub mod error;
pub mod git;
pub mod tags;
pub mod version;

pub use config::{Config, Verbosity, VersionPart};
pub use error::{Result, TagVerError};
pub use git::Repository;
pub use version::Version;

/// Calculate the version for the given repository using the TagVer algorithm.
///
/// # Examples
/// Returning an error when the target is not a Git repository:
/// ```rust
/// use tagver::{calculate_version, Config, TagVerError};
///
/// let config = Config::default();
/// let err = calculate_version("/tmp/not-a-repo-tagver", &config).unwrap_err();
/// match err {
///     TagVerError::GitRepoNotFound(_) => {},
///     other => panic!("unexpected error: {other}"),
/// }
/// ```
///
/// # Errors
/// - [`TagVerError::GitRepoNotFound`] if the path is not inside a Git repository.
/// - [`TagVerError::GitCommand`] or [`TagVerError::Other`] for underlying Git failures.
/// - [`TagVerError::InvalidSemver`] if tags contain invalid SemVer.
pub fn calculate_version(
    work_dir: impl Into<std::path::PathBuf>,
    config: &Config,
) -> Result<CalculationResult> {
    let work_dir = work_dir.into();

    // Try to discover the repository
    let repo = Repository::discover(&work_dir)?;

    // Check for shallow clone warning
    if repo.is_shallow() {
        tracing::warn!("Shallow repository detected. Version calculation may be incorrect. Fetch full history with 'git fetch --unshallow'.");
    }

    // Calculate the version
    let (version, height, is_from_tag) = git::calculate_version(&repo, config)?;

    Ok(CalculationResult {
        version,
        height,
        is_from_tag,
        work_dir,
    })
}

/// Calculate the version, falling back to the default version when no repository is found.
///
/// # Examples
/// Using a non-repository directory will return the default version instead of erroring:
/// ```rust
/// use tagver::{calculate_version_with_fallback, Config, TagVerError};
///
/// let config = Config::default();
/// let result = calculate_version_with_fallback("/tmp/not-a-repo-tagver", &config)?;
/// assert_eq!(result.to_string(), "0.0.0-alpha.0");
/// assert!(!result.is_from_tag);
/// # Ok::<_, TagVerError>(())
/// ```
///
/// # Errors
/// - [`TagVerError::GitCommand`] or [`TagVerError::Other`] for unexpected Git errors during fallback discovery.
pub fn calculate_version_with_fallback(
    work_dir: impl Into<std::path::PathBuf>,
    config: &Config,
) -> Result<CalculationResult> {
    let work_dir = work_dir.into();

    // Try to discover and calculate version
    let (version, height, is_from_tag) = git::calculate_version_fallback(&work_dir, config)?;

    Ok(CalculationResult {
        version,
        height,
        is_from_tag,
        work_dir,
    })
}

/// Result of a version calculation.
///
/// # Examples
/// ```rust
/// use tagver::{calculate_version_with_fallback, Config, TagVerError};
///
/// let result = calculate_version_with_fallback("/tmp/not-a-repo-tagver", &Config::default())?;
/// assert_eq!(result.version.to_string(), "0.0.0-alpha.0");
/// assert_eq!(result.height, 0);
/// assert!(!result.is_from_tag);
/// # Ok::<_, TagVerError>(())
/// ```
#[derive(Debug, Clone, serde::Serialize)]
pub struct CalculationResult {
    pub version: Version,
    pub height: u32,
    pub is_from_tag: bool,
    pub work_dir: std::path::PathBuf,
}

impl std::fmt::Display for CalculationResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.version)
    }
}
