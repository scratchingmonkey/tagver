//! Build metadata tests - ported from MinVerTests.Lib/BuildMetadata.cs

use tempfile::TempDir;
use test_case::test_case;

mod common;

// No commits scenarios
#[test_case("", "0.0.0-alpha.0")]
#[test_case("a", "0.0.0-alpha.0+a")]
#[tokio::test]
async fn test_no_commits(build_metadata: &str, expected_version: &str) {
    use minver_rs::{calculate_version_with_fallback, Config};

    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let path = temp_dir.path();

    // Create empty repository
    common::git::ensure_empty_repository(path)
        .await
        .expect("Failed to create repo");

    // Configure version calculation
    let config = Config {
        build_metadata: if build_metadata.is_empty() {
            None
        } else {
            Some(build_metadata.to_string())
        },
        ..Default::default()
    };

    // Calculate version
    let result =
        calculate_version_with_fallback(path, &config).expect("Failed to calculate version");

    // Verify the version
    assert_eq!(result.to_string(), expected_version);
}

// No tag scenarios
#[test_case("", "0.0.0-alpha.0")]
#[test_case("a", "0.0.0-alpha.0+a")]
#[tokio::test]
async fn test_no_tag(build_metadata: &str, expected_version: &str) {
    use minver_rs::{calculate_version_with_fallback, Config};

    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let path = temp_dir.path();

    // Create repository with commit but no tag
    common::git::ensure_empty_repository_and_commit(path)
        .await
        .expect("Failed to create repo");

    // Configure version calculation
    let config = Config {
        build_metadata: if build_metadata.is_empty() {
            None
        } else {
            Some(build_metadata.to_string())
        },
        ..Default::default()
    };

    // Calculate version
    let result =
        calculate_version_with_fallback(path, &config).expect("Failed to calculate version");

    // Verify the version
    assert_eq!(result.to_string(), expected_version);
}

// Current tag scenarios
#[test_case("1.2.3+a", "", "1.2.3+a")]
#[test_case("1.2.3", "b", "1.2.3+b")]
#[test_case("1.2.3+a", "b", "1.2.3+a.b")]
#[test_case("1.2.3-pre+a", "", "1.2.3-pre+a")]
#[test_case("1.2.3-pre", "b", "1.2.3-pre+b")]
#[test_case("1.2.3-pre+a", "b", "1.2.3-pre+a.b")]
#[tokio::test]
async fn test_current_tag(tag_name: &str, build_metadata: &str, expected_version: &str) {
    use minver_rs::{calculate_version_with_fallback, Config};

    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let path = temp_dir.path();

    // Create repository with tag
    common::git::ensure_empty_repository_and_commit(path)
        .await
        .expect("Failed to create repo");
    common::git::tag(path, tag_name)
        .await
        .expect("Failed to create tag");

    // Configure version calculation
    let config = Config {
        build_metadata: if build_metadata.is_empty() {
            None
        } else {
            Some(build_metadata.to_string())
        },
        ..Default::default()
    };

    // Calculate version
    let result =
        calculate_version_with_fallback(path, &config).expect("Failed to calculate version");

    // Verify the version
    assert_eq!(result.to_string(), expected_version);
}

// Previous tag scenarios
#[test_case("1.2.3+a", "", "1.2.4-alpha.0.1")]
#[test_case("1.2.3", "b", "1.2.4-alpha.0.1+b")]
#[test_case("1.2.3+a", "b", "1.2.4-alpha.0.1+b")]
#[test_case("1.2.3-pre+a", "", "1.2.3-pre.1")]
#[test_case("1.2.3-pre", "b", "1.2.3-pre.1+b")]
#[test_case("1.2.3-pre+a", "b", "1.2.3-pre.1+b")]
#[tokio::test]
async fn test_previous_tag(tag_name: &str, build_metadata: &str, expected_version: &str) {
    use minver_rs::{calculate_version_with_fallback, Config};

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
        build_metadata: if build_metadata.is_empty() {
            None
        } else {
            Some(build_metadata.to_string())
        },
        ..Default::default()
    };

    // Calculate version
    let result =
        calculate_version_with_fallback(path, &config).expect("Failed to calculate version");

    // Verify the version
    assert_eq!(result.to_string(), expected_version);
}
