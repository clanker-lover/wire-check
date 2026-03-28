//! Git operations: base ref detection, new/modified file listing.

use std::path::{Path, PathBuf};
use std::process::Command;

/// Errors from git operations.
#[derive(Debug, thiserror::Error)]
pub(crate) enum GitError {
    #[error("not a git repository: {0}")]
    NotARepo(PathBuf),
    #[error("git command failed: {0}")]
    CommandFailed(String),
}

/// Detect default branch: tries main, then master, falls back to HEAD~1.
pub(crate) fn detect_base_ref(project_root: &Path) -> Result<String, GitError> {
    if !project_root.join(".git").exists() {
        return Err(GitError::NotARepo(project_root.to_path_buf()));
    }

    for branch in &["main", "master"] {
        let output = Command::new("git")
            .args(["rev-parse", "--verify", branch])
            .current_dir(project_root)
            .output()
            .map_err(|e| GitError::CommandFailed(e.to_string()))?;

        if output.status.success() {
            return Ok(branch.to_string());
        }
    }

    Ok("HEAD~1".to_string())
}

/// List .rs files added since base_ref. Excludes main.rs and configured test paths.
pub(crate) fn new_files(
    project_root: &Path,
    base_ref: &str,
    test_modules: &[String],
    test_files: &[PathBuf],
) -> Result<Vec<PathBuf>, GitError> {
    let output = Command::new("git")
        .args([
            "diff",
            base_ref,
            "--name-only",
            "--diff-filter=A",
            "--",
            "src/**/*.rs",
        ])
        .current_dir(project_root)
        .output()
        .map_err(|e| GitError::CommandFailed(e.to_string()))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(GitError::CommandFailed(stderr.to_string()));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let files: Vec<PathBuf> = stdout
        .lines()
        .filter(|line| !line.is_empty())
        .filter(|line| line.ends_with(".rs"))
        .filter(|line| !line.ends_with("main.rs"))
        .filter(|line| !is_test_path(line, test_modules, test_files))
        .map(PathBuf::from)
        .collect();

    Ok(files)
}

/// Check if a path matches configured test modules or test file patterns.
fn is_test_path(path: &str, test_modules: &[String], test_files: &[PathBuf]) -> bool {
    for module in test_modules {
        // Match src/connection_tests.rs or src/connection_tests/anything.rs
        if path.contains(&format!("/{module}.rs")) || path.contains(&format!("/{module}/")) {
            return true;
        }
    }
    for test_path in test_files {
        let tp = test_path.to_string_lossy();
        if path.starts_with(tp.as_ref()) || path.ends_with(tp.as_ref()) {
            return true;
        }
    }
    false
}

/// List all files modified since base_ref.
pub(crate) fn modified_files(
    project_root: &Path,
    base_ref: &str,
) -> Result<Vec<PathBuf>, GitError> {
    let output = Command::new("git")
        .args(["diff", base_ref, "--name-only"])
        .current_dir(project_root)
        .output()
        .map_err(|e| GitError::CommandFailed(e.to_string()))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(GitError::CommandFailed(stderr.to_string()));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let files: Vec<PathBuf> = stdout
        .lines()
        .filter(|line| !line.is_empty())
        .map(PathBuf::from)
        .collect();

    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_git_repo() -> tempfile::TempDir {
        let dir = tempfile::tempdir().expect("tempdir");
        Command::new("git")
            .args(["init"])
            .current_dir(dir.path())
            .output()
            .expect("git init");
        Command::new("git")
            .args(["config", "user.email", "test@test.com"])
            .current_dir(dir.path())
            .output()
            .expect("git config email");
        Command::new("git")
            .args(["config", "user.name", "Test"])
            .current_dir(dir.path())
            .output()
            .expect("git config name");

        let src = dir.path().join("src");
        std::fs::create_dir(&src).expect("mkdir src");
        std::fs::write(src.join("main.rs"), "fn main() {}").expect("write main");

        Command::new("git")
            .args(["add", "."])
            .current_dir(dir.path())
            .output()
            .expect("git add");
        Command::new("git")
            .args(["commit", "-m", "initial"])
            .current_dir(dir.path())
            .output()
            .expect("git commit");

        dir
    }

    #[test]
    fn detect_base_ref_finds_master() {
        let dir = setup_git_repo();
        let base = detect_base_ref(dir.path()).expect("detect");
        assert!(base == "main" || base == "master" || base == "HEAD~1");
    }

    #[test]
    fn detect_base_ref_fails_on_non_repo() {
        let dir = tempfile::tempdir().expect("tempdir");
        let result = detect_base_ref(dir.path());
        assert!(matches!(result, Err(GitError::NotARepo(_))));
    }

    #[test]
    fn new_files_returns_empty_when_no_changes() {
        let dir = setup_git_repo();
        let files = new_files(dir.path(), "HEAD", &[], &[]).expect("new_files");
        assert!(files.is_empty());
    }

    #[test]
    fn modified_files_returns_empty_when_no_changes() {
        let dir = setup_git_repo();
        let files = modified_files(dir.path(), "HEAD").expect("modified_files");
        assert!(files.is_empty());
    }

    #[test]
    fn is_test_path_matches_module_name() {
        assert!(is_test_path(
            "src/connection_tests.rs",
            &["connection_tests".to_string()],
            &[]
        ));
        assert!(is_test_path(
            "src/connection_tests/helpers.rs",
            &["connection_tests".to_string()],
            &[]
        ));
    }

    #[test]
    fn is_test_path_does_not_match_substrings() {
        // "attestation.rs" should NOT match test_modules ["tests"]
        assert!(!is_test_path(
            "src/attestation.rs",
            &["tests".to_string()],
            &[]
        ));
        assert!(!is_test_path("src/contest.rs", &["tests".to_string()], &[]));
        assert!(!is_test_path(
            "src/latest_results.rs",
            &["tests".to_string()],
            &[]
        ));
    }

    #[test]
    fn is_test_path_matches_test_files() {
        assert!(is_test_path(
            "tests/integration.rs",
            &[],
            &[PathBuf::from("tests/")]
        ));
    }
}
