//! Configuration for minver-rs operations.

use std::path::PathBuf;
use std::str::FromStr;

/// Verbosity levels for logging.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verbosity {
    Quiet,
    Normal,
    Verbose,
    Debug,
    Trace,
}

impl FromStr for Verbosity {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "quiet" => Ok(Verbosity::Quiet),
            "normal" => Ok(Verbosity::Normal),
            "verbose" | "info" => Ok(Verbosity::Verbose),
            "debug" => Ok(Verbosity::Debug),
            "trace" => Ok(Verbosity::Trace),
            _ => Err(format!("Invalid verbosity level: {}", s)),
        }
    }
}

/// Version parts that can be auto-incremented.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VersionPart {
    Major,
    Minor,
    Patch,
}

impl FromStr for VersionPart {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "major" => Ok(VersionPart::Major),
            "minor" => Ok(VersionPart::Minor),
            "patch" => Ok(VersionPart::Patch),
            _ => Err(format!("Invalid version part: {}", s)),
        }
    }
}

/// Major.minor constraint.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MajorMinor {
    pub major: u32,
    pub minor: u32,
}

impl MajorMinor {
    /// Parse a MajorMinor from a string in the format "major.minor"
    pub fn parse(s: &str) -> Result<Self, String> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 2 {
            return Err(format!("Expected format 'major.minor', got: {}", s));
        }

        let major = parts[0]
            .parse::<u32>()
            .map_err(|_| format!("Invalid major version: {}", parts[0]))?;
        let minor = parts[1]
            .parse::<u32>()
            .map_err(|_| format!("Invalid minor version: {}", parts[1]))?;

        Ok(MajorMinor { major, minor })
    }
}

/// Runtime configuration for minver-rs operations.
///
/// # Defaults
/// - `work_dir`: current directory (`.`)
/// - `tag_prefix`: empty (accept all tags)
/// - `auto_increment`: [`VersionPart::Patch`](crate::config::VersionPart)
/// - `default_prerelease_identifiers`: `"alpha.0"`
/// - `ignore_height`: `false`
/// - `verbosity`: [`Verbosity::Normal`](crate::config::Verbosity)
///
/// # Examples
/// ```rust
/// use minver_rs::{Config, VersionPart};
///
/// let mut config = Config::default();
/// config.tag_prefix = "v".into();
/// config.auto_increment = VersionPart::Minor;
/// config.ignore_height = true;
///
/// assert_eq!(config.tag_prefix, "v");
/// assert_eq!(config.auto_increment, VersionPart::Minor);
/// assert!(config.ignore_height);
/// ```
#[derive(Debug, Clone)]
pub struct Config {
    pub work_dir: PathBuf,
    pub tag_prefix: String,
    pub auto_increment: VersionPart,
    pub minimum_major_minor: Option<MajorMinor>,
    pub default_prerelease_identifiers: Vec<String>,
    pub build_metadata: Option<String>,
    pub ignore_height: bool,
    pub verbosity: Verbosity,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            work_dir: ".".into(),
            tag_prefix: "".into(),
            auto_increment: VersionPart::Patch,
            minimum_major_minor: None,
            default_prerelease_identifiers: vec!["alpha".into(), "0".into()],
            build_metadata: None,
            ignore_height: false,
            verbosity: Verbosity::Normal,
        }
    }
}
