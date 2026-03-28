use crate::error::AppError;
use crate::types::{DiffHunk, DiffLine, DiffResult, DiffStats, FileDiff, FileStatus, LineKind};
use std::path::Path;
use std::process::Command;

/// Get diff between two commits, or between a commit and its parent.
///
/// For the MVP, we shell out to `git diff` for reliability.
/// Pure gix diff can be implemented later for better performance.
pub fn get_diff(
    repo_path: &Path,
    from_hash: Option<&str>,
    to_hash: Option<&str>,
) -> Result<DiffResult, AppError> {
    let mut cmd = Command::new("git");
    cmd.current_dir(repo_path);

    match (from_hash, to_hash) {
        (Some(from), Some(to)) => {
            cmd.args(["diff", from, to]);
        }
        (Some(hash), None) => {
            // Diff commit vs its parent
            cmd.args(["diff", &format!("{}^", hash), hash]);
        }
        (None, Some(hash)) => {
            // Diff working directory vs commit
            cmd.args(["diff", hash]);
        }
        (None, None) => {
            // Diff working directory vs HEAD
            cmd.arg("diff");
        }
    }

    cmd.args(["--unified=3", "--no-color"]);

    let output = cmd.output()?;
    let diff_text = String::from_utf8_lossy(&output.stdout);

    parse_unified_diff(&diff_text)
}

/// Get diff of staged changes
pub fn get_staged_diff(repo_path: &Path) -> Result<DiffResult, AppError> {
    let output = Command::new("git")
        .current_dir(repo_path)
        .args(["diff", "--cached", "--unified=3", "--no-color"])
        .output()?;

    let diff_text = String::from_utf8_lossy(&output.stdout);
    parse_unified_diff(&diff_text)
}

fn parse_unified_diff(diff_text: &str) -> Result<DiffResult, AppError> {
    let mut files: Vec<FileDiff> = Vec::new();
    let mut current_file: Option<FileDiff> = None;
    let mut current_hunk: Option<DiffHunk> = None;
    let mut old_line: u32 = 0;
    let mut new_line: u32 = 0;
    let mut total_insertions: usize = 0;
    let mut total_deletions: usize = 0;

    for line in diff_text.lines() {
        if line.starts_with("diff --git") {
            // Save previous hunk and file
            if let Some(ref mut file) = current_file {
                if let Some(hunk) = current_hunk.take() {
                    file.hunks.push(hunk);
                }
                files.push(file.clone());
            }

            // Start new file
            let path = extract_path_from_diff_header(line);
            current_file = Some(FileDiff {
                path,
                status: FileStatus::Modified,
                hunks: Vec::new(),
                is_binary: false,
            });
            current_hunk = None;
        } else if line.starts_with("new file") {
            if let Some(ref mut file) = current_file {
                file.status = FileStatus::Added;
            }
        } else if line.starts_with("deleted file") {
            if let Some(ref mut file) = current_file {
                file.status = FileStatus::Deleted;
            }
        } else if line.starts_with("rename from") {
            if let Some(ref mut file) = current_file {
                file.status = FileStatus::Renamed;
            }
        } else if line.starts_with("Binary files") {
            if let Some(ref mut file) = current_file {
                file.is_binary = true;
            }
        } else if line.starts_with("@@") {
            // Save previous hunk
            if let Some(ref mut file) = current_file {
                if let Some(hunk) = current_hunk.take() {
                    file.hunks.push(hunk);
                }
            }

            // Parse hunk header: @@ -old_start,old_count +new_start,new_count @@
            let (old_start, new_start) = parse_hunk_header(line);
            old_line = old_start;
            new_line = new_start;

            current_hunk = Some(DiffHunk {
                header: line.to_string(),
                lines: Vec::new(),
            });
        } else if let Some(ref mut hunk) = current_hunk {
            if let Some(stripped) = line.strip_prefix('+') {
                total_insertions += 1;
                hunk.lines.push(DiffLine {
                    kind: LineKind::Add,
                    content: stripped.to_string(),
                    old_line_no: None,
                    new_line_no: Some(new_line),
                });
                new_line += 1;
            } else if let Some(stripped) = line.strip_prefix('-') {
                total_deletions += 1;
                hunk.lines.push(DiffLine {
                    kind: LineKind::Remove,
                    content: stripped.to_string(),
                    old_line_no: Some(old_line),
                    new_line_no: None,
                });
                old_line += 1;
            } else {
                let content = line.strip_prefix(' ').unwrap_or(line);
                hunk.lines.push(DiffLine {
                    kind: LineKind::Context,
                    content: content.to_string(),
                    old_line_no: Some(old_line),
                    new_line_no: Some(new_line),
                });
                old_line += 1;
                new_line += 1;
            }
        }
    }

    // Save last hunk and file
    if let Some(ref mut file) = current_file {
        if let Some(hunk) = current_hunk.take() {
            file.hunks.push(hunk);
        }
        files.push(file.clone());
    }

    let files_changed = files.len();

    Ok(DiffResult {
        files,
        stats: DiffStats {
            files_changed,
            insertions: total_insertions,
            deletions: total_deletions,
        },
    })
}

fn extract_path_from_diff_header(line: &str) -> String {
    // "diff --git a/path/to/file b/path/to/file"
    if let Some(b_part) = line.split(" b/").last() {
        b_part.to_string()
    } else {
        line.to_string()
    }
}

fn parse_hunk_header(line: &str) -> (u32, u32) {
    // @@ -old_start,old_count +new_start,new_count @@ optional context
    let parts: Vec<&str> = line.split_whitespace().collect();
    let old_start = parts
        .get(1)
        .and_then(|s| s.strip_prefix('-'))
        .and_then(|s| s.split(',').next())
        .and_then(|s| s.parse().ok())
        .unwrap_or(1);
    let new_start = parts
        .get(2)
        .and_then(|s| s.strip_prefix('+'))
        .and_then(|s| s.split(',').next())
        .and_then(|s| s.parse().ok())
        .unwrap_or(1);
    (old_start, new_start)
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
    fn test_parse_hunk_header() {
        let (old, new) = parse_hunk_header("@@ -10,5 +20,8 @@ fn example()");
        assert_eq!(old, 10);
        assert_eq!(new, 20);
    }

    #[test]
    fn test_parse_hunk_header_no_count() {
        let (old, new) = parse_hunk_header("@@ -1 +1 @@");
        assert_eq!(old, 1);
        assert_eq!(new, 1);
    }

    #[test]
    fn test_extract_path_from_diff_header() {
        let path = extract_path_from_diff_header("diff --git a/src/main.rs b/src/main.rs");
        assert_eq!(path, "src/main.rs");
    }

    #[test]
    fn test_parse_unified_diff() {
        let diff_text = r#"diff --git a/file.txt b/file.txt
new file mode 100644
--- /dev/null
+++ b/file.txt
@@ -0,0 +1,3 @@
+line 1
+line 2
+line 3
"#;
        let result = parse_unified_diff(diff_text).unwrap();
        assert_eq!(result.files.len(), 1);
        assert_eq!(result.files[0].path, "file.txt");
        assert!(matches!(result.files[0].status, FileStatus::Added));
        assert_eq!(result.stats.insertions, 3);
        assert_eq!(result.stats.deletions, 0);
    }

    #[test]
    fn test_parse_unified_diff_modification() {
        let diff_text = r#"diff --git a/file.txt b/file.txt
--- a/file.txt
+++ b/file.txt
@@ -1,3 +1,3 @@
 line 1
-old line 2
+new line 2
 line 3
"#;
        let result = parse_unified_diff(diff_text).unwrap();
        assert_eq!(result.files.len(), 1);
        assert!(matches!(result.files[0].status, FileStatus::Modified));
        assert_eq!(result.stats.insertions, 1);
        assert_eq!(result.stats.deletions, 1);
        assert_eq!(result.files[0].hunks.len(), 1);
        assert_eq!(result.files[0].hunks[0].lines.len(), 4); // 1 context + 1 remove + 1 add + 1 context
    }

    #[test]
    fn test_get_diff_on_fixture() {
        let path = fixture_path();
        // Diff working directory vs HEAD (should have unstaged changes)
        let result = get_diff(&path, None, None).unwrap();
        // We have unstaged modifications to file1.txt
        assert!(
            result.files.iter().any(|f| f.path == "file1.txt"),
            "Should find modified file1.txt"
        );
    }

    #[test]
    fn test_get_diff_between_commits() {
        let path = fixture_path();
        // Get two commit hashes from git log
        let output = std::process::Command::new("git")
            .current_dir(&path)
            .args(["log", "--oneline", "-2", "--format=%H"])
            .output()
            .unwrap();
        let hashes: Vec<&str> = std::str::from_utf8(&output.stdout)
            .unwrap()
            .trim()
            .lines()
            .collect();

        if hashes.len() == 2 {
            let result = get_diff(&path, Some(hashes[1]), Some(hashes[0])).unwrap();
            assert!(!result.files.is_empty(), "Diff between commits should have files");
        }
    }
}
