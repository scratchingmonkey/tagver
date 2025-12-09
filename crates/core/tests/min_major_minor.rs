//! Minimum major.minor tests - ported from MinVerTests.Lib/MinMajorMinor.cs

use tempfile::TempDir;
use test_case::test_case;

mod common;

#[tokio::test]
async fn test_no_commits() {
    use tagver::config::MajorMinor;
    use tagver::{calculate_version_with_fallback, Config};

    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let path = temp_dir.path();

    // Create empty repository
    common::git::ensure_empty_repository(path)
        .await
        .expect("Failed to create repo");

    // Configure version calculation with minimum major.minor
    let config = Config {
        minimum_major_minor: Some(MajorMinor::parse("1.2").unwrap()),
        ..Default::default()
    };

    // Calculate version
    let result =
        calculate_version_with_fallback(path, &config).expect("Failed to calculate version");

    // Verify the version - should be bumped to minimum
    assert_eq!(result.to_string(), "1.2.0-alpha.0");
}

#[test_case("4.0.0", 3, 2, "4.0.0")]
#[test_case("4.3.0", 4, 3, "4.3.0")]
#[test_case("4.3.0", 5, 4, "4.3.0")]
#[tokio::test]
async fn test_tagged(tag_name: &str, major: u32, minor: u32, expected_version: &str) {
    use tagver::config::MajorMinor;
    use tagver::{calculate_version_with_fallback, Config};

    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let path = temp_dir.path();

    // Create repository with tag
    common::git::ensure_empty_repository_and_commit(path)
        .await
        .expect("Failed to create repo");
    common::git::tag(path, tag_name)
        .await
        .expect("Failed to create tag");

    // Configure version calculation with minimum major.minor
    let config = Config {
        minimum_major_minor: Some(MajorMinor::parse(&format!("{}.{}", major, minor)).unwrap()),
        ..Default::default()
    };

    // Calculate version
    let result =
        calculate_version_with_fallback(path, &config).expect("Failed to calculate version");

    // Verify the version
    assert_eq!(result.to_string(), expected_version);
}

#[tokio::test]
async fn test_not_tagged() {
    use tagver::config::MajorMinor;
    use tagver::{calculate_version_with_fallback, Config};

    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let path = temp_dir.path();

    // Create repository with commit but no tag
    common::git::ensure_empty_repository_and_commit(path)
        .await
        .expect("Failed to create repo");

    // Configure version calculation with minimum major.minor
    let config = Config {
        minimum_major_minor: Some(MajorMinor::parse("1.0").unwrap()),
        ..Default::default()
    };

    // Calculate version
    let result =
        calculate_version_with_fallback(path, &config).expect("Failed to calculate version");

    // Verify the version - should be bumped to minimum
    assert_eq!(result.to_string(), "1.0.0-alpha.0");
}
