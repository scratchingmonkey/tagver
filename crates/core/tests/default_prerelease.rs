//! Default pre-release identifiers tests - ported from MinVerTests.Lib/DefaultPreReleaseIdentifiers.cs

use tempfile::TempDir;
use test_case::test_case;

mod common;

#[test_case("alpha.0", "0.0.0-alpha.0")]
#[test_case("preview.x", "0.0.0-preview.x")]
#[tokio::test]
async fn test_various_identifiers(identifiers: &str, expected_version: &str) {
    use minver_rs::{calculate_version_with_fallback, Config};

    // Create test directory
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let path = temp_dir.path();

    // Create repository with commit
    common::git::ensure_empty_repository_and_commit(path)
        .await
        .expect("Failed to create repo");

    // Parse identifiers
    let identifier_list: Vec<String> = identifiers.split('.').map(|s| s.to_string()).collect();

    // Configure version calculation
    let mut config = Config::default();
    config.default_prerelease_identifiers = identifier_list;

    // Calculate version
    let result =
        calculate_version_with_fallback(path, &config).expect("Failed to calculate version");

    // Verify the version
    assert_eq!(result.to_string(), expected_version);
}
