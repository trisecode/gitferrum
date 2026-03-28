use crate::error::AppError;
use crate::types::{FileStatus, RepoStatus, StatusEntry};
use std::path::Path;
use std::process::Command;

/// Get the current repository status using git CLI.
///
/// Uses `git status --porcelain=v1` for reliable parsing.
/// Pure gix status can be implemented later.
pub fn get_status(repo_path: &Path) -> Result<RepoStatus, AppError> {
    let branch = get_current_branch(repo_path)?;
    let (ahead, behind) = get_ahead_behind(repo_path)?;

    let output = Command::new("git")
        .current_dir(repo_path)
        .args(["status", "--porcelain=v1"])
        .output()?;

    let text = String::from_utf8_lossy(&output.stdout);

    let mut staged = Vec::new();
    let mut unstaged = Vec::new();
    let mut untracked = Vec::new();

    for line in text.lines() {
        if line.len() < 3 {
            continue;
        }

        let index_status = line.as_bytes()[0];
        let worktree_status = line.as_bytes()[1];
        let path = line[3..].to_string();

        // Untracked
        if index_status == b'?' {
            untracked.push(path);
            continue;
        }

        // Staged changes (index)
        if index_status != b' ' && index_status != b'?' {
            let status = match index_status {
                b'A' => FileStatus::Added,
                b'M' => FileStatus::Modified,
                b'D' => FileStatus::Deleted,
                b'R' => FileStatus::Renamed,
                b'C' => FileStatus::Copied,
                _ => FileStatus::Modified,
            };
            staged.push(StatusEntry {
                path: path.clone(),
                status,
            });
        }

        // Unstaged changes (worktree)
        if worktree_status != b' ' && worktree_status != b'?' {
            let status = match worktree_status {
                b'M' => FileStatus::Modified,
                b'D' => FileStatus::Deleted,
                _ => FileStatus::Modified,
            };
            unstaged.push(StatusEntry { path, status });
        }
    }

    Ok(RepoStatus {
        staged,
        unstaged,
        untracked,
        branch,
        ahead,
        behind,
    })
}

fn get_current_branch(repo_path: &Path) -> Result<Option<String>, AppError> {
    let output = Command::new("git")
        .current_dir(repo_path)
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()?;

    if output.status.success() {
        let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if branch == "HEAD" {
            Ok(None) // Detached HEAD
        } else {
            Ok(Some(branch))
        }
    } else {
        Ok(None)
    }
}

fn get_ahead_behind(repo_path: &Path) -> Result<(u32, u32), AppError> {
    let output = Command::new("git")
        .current_dir(repo_path)
        .args(["rev-list", "--left-right", "--count", "HEAD...@{u}"])
        .output()?;

    if output.status.success() {
        let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let parts: Vec<&str> = text.split_whitespace().collect();
        if parts.len() == 2 {
            let ahead = parts[0].parse().unwrap_or(0);
            let behind = parts[1].parse().unwrap_or(0);
            return Ok((ahead, behind));
        }
    }

    Ok((0, 0))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn fixture_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("tests")
            .join("fixtures")
            .join("test-repo")
    }

    #[test]
    fn test_get_status_returns_current_branch() {
        let path = fixture_path();
        let status = get_status(&path).unwrap();
        assert_eq!(status.branch, Some("main".to_string()));
    }

    #[test]
    fn test_get_status_detects_unstaged_changes() {
        let path = fixture_path();
        let status = get_status(&path).unwrap();

        // file1.txt was modified in the fixture
        assert!(
            status
                .unstaged
                .iter()
                .any(|e| e.path == "file1.txt"),
            "Should detect modified file1.txt as unstaged"
        );
    }

    #[test]
    fn test_get_status_detects_untracked_files() {
        let path = fixture_path();
        let status = get_status(&path).unwrap();

        // unstaged.txt was created but not added
        assert!(
            status.untracked.contains(&"unstaged.txt".to_string()),
            "Should detect unstaged.txt as untracked"
        );
    }

    #[test]
    fn test_get_current_branch() {
        let path = fixture_path();
        let branch = get_current_branch(&path).unwrap();
        assert_eq!(branch, Some("main".to_string()));
    }
}
