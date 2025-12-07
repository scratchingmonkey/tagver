//! Auto-increment tests - ported from MinVerTests.Lib/AutoIncrement.cs

use minver_rs::{Config, VersionPart};
use tempfile::TempDir;
use test_case::test_case;

mod common;

#[test_case("1.2.3", VersionPart::Major, "2.0.0-alpha.0.1")]
#[test_case("1.2.3", VersionPart::Minor, "1.3.0-alpha.0.1")]
#[test_case("1.2.3", VersionPart::Patch, "1.2.4-alpha.0.1")]
#[tokio::test]
async fn test_rtm_version_increment(
    tag_name: &str,
    auto_increment: VersionPart,
    expected_version: &str,
) {
    use minver_rs::calculate_version_with_fallback;

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
    common::git::commit(path)
        .await
        .expect("Failed to create commit");

    // Configure version calculation
    let config = Config {
        auto_increment,
        ..Default::default()
    };

    // Calculate version
    let result =
        calculate_version_with_fallback(path, &config).expect("Failed to calculate version");

    // Verify the version
    assert_eq!(result.to_string(), expected_version);
}
