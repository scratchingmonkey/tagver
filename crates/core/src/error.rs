//! Error types for TagVer operations.

use thiserror::Error;

/// Result type alias for TagVer operations.
pub type Result<T> = std::result::Result<T, TagVerError>;

/// Main error type for TagVer operations.
#[derive(Error, Debug)]
pub enum TagVerError {
    #[error("Git repository not found at path: {0}")]
    GitRepoNotFound(String),

    #[error("No commits found in repository")]
    NoCommits,

    #[error("Invalid tag prefix: {0}")]
    InvalidTagPrefix(String),

    #[error("Invalid version part: {0}")]
    InvalidVersionPart(String),

    #[error("Invalid major.minor: {0}")]
    InvalidMajorMinor(String),

    #[error("Invalid verbosity level: {0}")]
    InvalidVerbosity(String),

    #[error("Invalid semantic version: {0}")]
    InvalidSemver(String),

    #[error("No version tags found with prefix '{0}'")]
    NoVersionTags(String),

    #[error("Shallow repository detected - version calculation may be incorrect")]
    ShallowRepo,

    #[error("Git command failed: {0}")]
    GitCommand(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(#[from] std::num::ParseIntError),

    #[error("Semver error: {0}")]
    Semver(#[from] semver::Error),

    #[error("Other error: {0}")]
    Other(String),
}
