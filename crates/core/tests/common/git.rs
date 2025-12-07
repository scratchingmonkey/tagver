//! Git test helpers for creating and manipulating test repositories

use std::path::Path;
use std::process::Command;
use tempfile::TempDir;

/// Result type for test operations
pub type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

/// Run a git command and check for success
pub fn run_git_command(args: &[&str], cwd: &Path) -> TestResult {
    let output = Command::new("git")
        .args(args)
        .current_dir(cwd)
        .output()
        .map_err(|e| format!("Failed to run git command: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Git command failed: {}", stderr).into());
    }

    Ok(())
}

/// Create an empty git repository and make an initial commit
#[allow(dead_code)]
pub async fn ensure_empty_repository_and_commit(path: &Path) -> TestResult {
    ensure_empty_repository(path).await?;
    commit(path).await
}

/// Create an empty git repository
pub async fn ensure_empty_repository(path: &Path) -> TestResult {
    // Create directory
    std::fs::create_dir_all(path).map_err(|e| format!("Failed to create directory: {}", e))?;

    // Initialize repository
    run_git_command(&["init", "--initial-branch=main"], path)?;

    // Configure git user
    run_git_command(&["config", "user.email", "test@example.com"], path)?;
    run_git_command(&["config", "user.name", "Test User"], path)?;
    // Disable GPG signing
    run_git_command(&["config", "commit.gpgsign", "false"], path)?;

    Ok(())
}

/// Create a commit
#[allow(dead_code)]
pub async fn commit(path: &Path) -> TestResult {
    run_git_command(&["commit", "--allow-empty", "-m", "."], path)
}

/// Create a tag
#[allow(dead_code)]
pub async fn tag(path: &Path, tag_name: &str) -> TestResult {
    run_git_command(&["tag", tag_name], path)
}

/// Create an annotated tag
#[allow(dead_code)]
pub async fn annotated_tag(path: &Path, tag_name: &str, message: &str) -> TestResult {
    run_git_command(&["tag", "-a", tag_name, "-m", message], path)
}

/// Checkout a specific commit or branch
#[allow(dead_code)]
pub async fn checkout(path: &Path, ref_name: &str) -> TestResult {
    run_git_command(&["checkout", ref_name], path)
}

/// Get all commit SHAs in the repository
#[allow(dead_code)]
pub async fn get_commit_shas(path: &Path) -> TestResult<Vec<String>> {
    let output = Command::new("git")
        .args(["log", "--pretty=format:%H"])
        .current_dir(path)
        .output()
        .map_err(|e| format!("Failed to run git log: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let shas: Vec<String> = stdout
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect();

    Ok(shas)
}

/// Get the git graph as a string
#[allow(dead_code)]
pub async fn get_graph(path: &Path) -> TestResult<String> {
    let output = Command::new("git")
        .args(["log", "--graph", "--pretty=format:'%d'"])
        .current_dir(path)
        .output()
        .map_err(|e| format!("Failed to run git log: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.to_string())
}

/// Create a temporary test directory with a specific name
#[allow(dead_code)]
pub fn get_test_directory(_test_name: &str) -> TempDir {
    let _run_id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    tempfile::tempdir().expect("Failed to create temp directory")
}
