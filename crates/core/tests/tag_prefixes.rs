//! Tag prefixes tests - ported from MinVerTests.Lib/TagPrefixes.cs

use tempfile::TempDir;
use test_case::test_case;

mod common;

#[test_case("2.3.4", "", "2.3.4")]
#[test_case("v3.4.5", "v", "3.4.5")]
#[test_case("version5.6.7", "version", "5.6.7")]
#[tokio::test]
async fn test_tag_prefix(tag_name: &str, prefix: &str, expected_version: &str) {
    use minver_rs::{calculate_version, Config};

    // Create test directory
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let path = temp_dir.path();

    // Create repository with tag
    common::git::ensure_empty_repository_and_commit(path)
        .await
        .expect("Failed to create repo");
    common::git::tag(path, tag_name)
        .await
        .expect("Failed to create tag");

    // Configure version calculation with prefix
    let mut config = Config::default();
    config.tag_prefix = prefix.to_string();

    // Calculate version
    let result = calculate_version(path, &config).expect("Failed to calculate version");

    // Verify the version
    assert_eq!(result.to_string(), expected_version);
}
