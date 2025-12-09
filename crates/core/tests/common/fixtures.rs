//! Test fixtures for TagVer tests

use std::path::Path;

/// Test fixture for complex repository with history
/// This mimics the test from Versions.cs in the .NET tests
#[allow(dead_code)]
pub const REPO_WITH_HISTORY_COMMANDS: &[&str] = &[
    "git commit --allow-empty -m '.'",
    "git commit --allow-empty -m '.'",
    "git commit --allow-empty -m '.'",
    "git tag 0.0.0-alpha.1",
    "git commit --allow-empty -m '.'",
    "git commit --allow-empty -m '.'",
    "git tag 0.0.0",
    "git commit --allow-empty -m '.'",
    "git commit --allow-empty -m '.'",
    "git tag 0.1.0-beta.1",
    "git commit --allow-empty -m '.'",
    "git commit --allow-empty -m '.'",
    "git tag 0.1.0",
    "git commit --allow-empty -m '.'",
    "git commit --allow-empty -m '.'",
    "git tag 1.0.0-alpha.1",
    "git commit --allow-empty -m '.'",
    "git commit --allow-empty -m '.'",
    "git tag 1.0.0-rc.1",
    "git tag 1.0.0",
    "git checkout -b foo",
    "git commit --allow-empty -m '.'",
    "git commit --allow-empty -m '.'",
    "git commit --allow-empty -m '.'",
    "git tag 1.0.1-alpha.1",
    "git commit --allow-empty -m '.'",
    "git commit --allow-empty -m '.'",
    "git tag 1.0.1",
    "git commit --allow-empty -m '.'",
    "git checkout main",
    "git commit --allow-empty -m '.'",
    "git commit --allow-empty -m '.'",
    "git commit --allow-empty -m '.'",
    "git tag 1.1.0-alpha.1",
    "git commit --allow-empty -m '.'",
    "git merge foo --no-edit",
    "git commit --allow-empty -m '.'",
    "git tag 1.1.0-beta.2",
    "git tag 1.1.0-beta.10",
    "git commit --allow-empty -m '.'",
    "git commit --allow-empty -m '.'",
    "git tag 1.1.0-rc.1",
    "git tag 1.1.0 -a -m '.'",
];

/// Expected graph output for the complex history test
#[allow(dead_code)]
pub const EXPECTED_GRAPH: &str = r#"* ' (HEAD -> main, tag: v/1.1.0, tag: 1.1.0-rc.1, tag: 1.1.0)'
* ' (tag: v/1.1.0-beta.10.1)'
* ' (tag: v/1.1.0-beta.10, tag: 1.1.0-beta.2, tag: 1.1.0-beta.10)'
*   ' (tag: v/1.1.0-alpha.1.2)'
|\  
| * ' (tag: v/1.0.2-alpha.0.1, foo)'
| * ' (tag: v/1.0.1, tag: 1.0.1)'
| * ' (tag: v/1.0.1-alpha.1.1)'
| * ' (tag: v/1.0.1-alpha.1, tag: 1.0.1-alpha.1)'
| * ' (tag: v(2)/1.0.1-alpha.0.2)'
| * ' (tag: v(2)/1.0.1-alpha.0.1)'
* | ' (tag: v/1.1.0-alpha.1.1)'
* | ' (tag: v/1.1.0-alpha.1, tag: 1.1.0-alpha.1)'
* | ' (tag: v/1.0.1-alpha.0.2)'
* | ' (tag: v/1.0.1-alpha.0.1)'
|/  
* ' (tag: v/1.0.0, tag: 1.0.0-rc.1, tag: 1.0.0)'
* ' (tag: v/1.0.0-alpha.1.1)'
* ' (tag: v/1.0.0-alpha.1, tag: 1.0.0-alpha.1)'
* ' (tag: v/0.1.1-alpha.0.1)'
* ' (tag: v/0.1.0, tag: 0.1.0)'
* ' (tag: v/0.1.0-beta.1.1)'
* ' (tag: v/0.1.0-beta.1, tag: 0.1.0-beta.1)'
* ' (tag: v/0.0.1-alpha.0.1)'
* ' (tag: v/0.0.0, tag: 0.0.0)'
* ' (tag: v/0.0.0-alpha.1.1)'
* ' (tag: v/0.0.0-alpha.1, tag: 0.0.0-alpha.1)'
* ' (tag: v/0.0.0-alpha.0.2)'
* ' (tag: v/0.0.0-alpha.0.1)'
* ' (tag: v/0.0.0-alpha.0)'
"#;

/// Test fixture for minimum major minor scenarios
/// This mimics the test from LogMessages.cs in the .NET tests
#[allow(dead_code)]
pub const MIN_MAJOR_MINOR_COMMANDS_AFTER_TAG: &[&str] = &[
    "git commit --allow-empty -m '.'",
    "git tag not-a-version",
    "git checkout -b foo",
    "git commit --allow-empty -m '.'",
    "git tag 1.0.0-foo.1",
    "git checkout main",
    "git merge foo --no-edit --no-ff",
    "git checkout -b bar",
    "git commit --allow-empty -m '.'",
    "git checkout main",
    "git checkout -b baz",
    "git commit --allow-empty -m '.'",
    "git checkout main",
    "git merge bar baz --no-edit --no-ff --strategy=octopus",
];

/// Test fixture for minimum major minor on tag scenario
#[allow(dead_code)]
pub const MIN_MAJOR_MINOR_COMMANDS_ON_TAG: &[&str] = &[
    "git commit --allow-empty -m '.'",
    "git tag not-a-version",
    "git checkout -b foo",
    "git commit --allow-empty -m '.'",
    "git tag 1.0.0-foo.1",
];

/// Helper to create a file before commits (to avoid git issues with empty commits)
#[allow(dead_code)]
pub fn create_test_file(path: &Path, index: &str) -> std::io::Result<()> {
    let file_path = path.join(index);
    std::fs::write(&file_path, index)?;
    Ok(())
}
