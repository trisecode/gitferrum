use crate::error::AppError;
use std::path::Path;
use std::process::{Command, Stdio};


/// Run a git network command in a dedicated OS thread (completely outside Tokio).
/// Reads stdout/stderr in sub-threads to prevent pipe buffer deadlock.
pub async fn run_network_command(args: &[&str], repo_path: Option<&Path>) -> Result<(bool, String, String), AppError> {
    let args: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    let repo_path: Option<std::path::PathBuf> = repo_path.map(|p| p.to_owned());
    let ssh_sock = std::env::var("SSH_AUTH_SOCK").unwrap_or_default();

    let (tx, rx) = tokio::sync::oneshot::channel();

    std::thread::spawn(move || {
        use std::io::Read;


        let mut cmd = Command::new("git");
        cmd.args(&args);
        cmd.stdin(Stdio::null());
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());
        cmd.env("GIT_TERMINAL_PROMPT", "0");

        if !ssh_sock.is_empty() {
            cmd.env("SSH_AUTH_SOCK", &ssh_sock);
        }
        if let Some(ref path) = repo_path {
            cmd.current_dir(path);
        }

        let mut child = match cmd.spawn() {
            Ok(c) => c,
            Err(e) => {
                let _ = tx.send(Err(format!("Failed to start git: {}", e)));
                return;
            }
        };

        // Read stdout and stderr in separate threads to avoid pipe deadlock
        let stdout_pipe = child.stdout.take();
        let stderr_pipe = child.stderr.take();

        let out_thread = std::thread::spawn(move || {
            let mut buf = String::new();
            if let Some(mut pipe) = stdout_pipe {
                let _ = pipe.read_to_string(&mut buf);
            }
            buf
        });
        let err_thread = std::thread::spawn(move || {
            let mut buf = String::new();
            if let Some(mut pipe) = stderr_pipe {
                let _ = pipe.read_to_string(&mut buf);
            }
            buf
        });

        let status = child.wait();
        let stdout = out_thread.join().unwrap_or_default();
        let stderr = err_thread.join().unwrap_or_default();

        match status {
            Ok(s) => {
                let _ = tx.send(Ok((s.success(), stdout, stderr)));
            }
            Err(e) => {
                let _ = tx.send(Err(format!("Git process error: {}", e)));
            }
        }
    });

    match tokio::time::timeout(std::time::Duration::from_secs(60), rx).await {
        Ok(Ok(Ok((success, stdout, stderr)))) => {
            Ok((success, stdout.trim().to_string(), stderr.trim().to_string()))
        }
        Ok(Ok(Err(e))) => Err(AppError::Git(e)),
        Ok(Err(_)) => Err(AppError::Git("Git process failed unexpectedly".into())),
        Err(_) => Err(AppError::Git(
            "Operation timed out (60s). SSH credentials may not be configured. Try: ssh-add".into(),
        )),
    }
}

/// Clone a remote repository.
pub async fn clone_repo(url: &str, dest: &Path) -> Result<String, AppError> {
    let dest_str = dest.to_string_lossy().to_string();
    let (success, _stdout, stderr) = run_network_command(
        &["clone", url, &dest_str],
        None,
    ).await?;
    if !success {
        return Err(AppError::Git(if stderr.is_empty() { "Clone failed".into() } else { stderr }));
    }
    Ok(dest_str)
}

/// Create a local branch tracking a remote branch and check it out.
/// e.g. remote_branch = "origin/feature" → creates local "feature" and checks it out.
pub fn checkout_remote_branch(repo_path: &Path, remote_branch: &str) -> Result<(), AppError> {
    // Extract local name: "origin/feature" → "feature"
    let local_name = remote_branch
        .split('/')
        .skip(1)
        .collect::<Vec<_>>()
        .join("/");

    if local_name.is_empty() {
        return Err(AppError::Other(format!(
            "Invalid remote branch: {}",
            remote_branch
        )));
    }

    let output = Command::new("git")
        .current_dir(repo_path)
        .args(["checkout", "-b", &local_name, "--track", &format!("refs/remotes/{}", remote_branch)])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        // If local branch already exists, just checkout
        if stderr.contains("already exists") {
            let output2 = Command::new("git")
                .current_dir(repo_path)
                .args(["checkout", &local_name])
                .output()?;
            if !output2.status.success() {
                return Err(AppError::Git(
                    String::from_utf8_lossy(&output2.stderr).trim().to_string(),
                ));
            }
            return Ok(());
        }
        return Err(AppError::Git(stderr));
    }

    Ok(())
}

/// Checkout a remote branch in detached HEAD mode (browse without creating a local branch).
pub fn checkout_detached(repo_path: &Path, ref_name: &str) -> Result<(), AppError> {
    let output = Command::new("git")
        .current_dir(repo_path)
        .args(["checkout", "--detach", &format!("refs/remotes/{}", ref_name)])
        .output()?;

    if !output.status.success() {
        return Err(AppError::Git(
            String::from_utf8_lossy(&output.stderr).trim().to_string(),
        ));
    }

    Ok(())
}

/// Create a new local branch from a remote branch with a custom name.
pub fn create_branch_from_remote(
    repo_path: &Path,
    new_name: &str,
    remote_branch: &str,
) -> Result<(), AppError> {
    let output = Command::new("git")
        .current_dir(repo_path)
        .args([
            "checkout",
            "-b",
            new_name,
            "--track",
            &format!("refs/remotes/{}", remote_branch),
        ])
        .output()?;

    if !output.status.success() {
        return Err(AppError::Git(
            String::from_utf8_lossy(&output.stderr).trim().to_string(),
        ));
    }

    Ok(())
}

/// Get the remote name for the current branch's upstream, or "origin" as fallback.
fn get_current_remote(repo_path: &Path) -> String {
    Command::new("git")
        .current_dir(repo_path)
        .args(["rev-parse", "--abbrev-ref", "--symbolic-full-name", "@{u}"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| {
            let upstream = String::from_utf8_lossy(&o.stdout).trim().to_string();
            upstream.split('/').next().unwrap_or("origin").to_string()
        })
        .unwrap_or_else(|| "origin".to_string())
}

/// Get the current branch name.
fn get_current_branch(repo_path: &Path) -> Result<String, AppError> {
    let output = Command::new("git")
        .current_dir(repo_path)
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()?;
    let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if branch.is_empty() || branch == "HEAD" {
        Err(AppError::Git("Detached HEAD".into()))
    } else {
        Ok(branch)
    }
}

/// Get list of files with merge conflicts.
fn get_conflict_files(repo_path: &Path) -> Vec<String> {
    Command::new("git")
        .current_dir(repo_path)
        .args(["diff", "--name-only", "--diff-filter=U"])
        .output()
        .ok()
        .map(|o| {
            String::from_utf8_lossy(&o.stdout)
                .trim()
                .lines()
                .filter(|l| !l.is_empty())
                .map(|l| l.to_string())
                .collect()
        })
        .unwrap_or_default()
}

/// Detect repository state (normal, merge, rebase, cherry-pick, revert).
pub fn get_repo_state(repo_path: &Path) -> String {
    let git_dir = repo_path.join(".git");
    if git_dir.join("MERGE_HEAD").exists() { return "merge".into(); }
    if git_dir.join("rebase-merge").exists() || git_dir.join("rebase-apply").exists() { return "rebase".into(); }
    if git_dir.join("CHERRY_PICK_HEAD").exists() { return "cherry-pick".into(); }
    if git_dir.join("REVERT_HEAD").exists() { return "revert".into(); }
    "normal".into()
}

/// Abort an in-progress merge.
pub fn abort_merge(repo_path: &Path) -> Result<(), AppError> {
    let output = Command::new("git")
        .current_dir(repo_path)
        .args(["merge", "--abort"])
        .output()?;
    if !output.status.success() {
        return Err(AppError::Git(String::from_utf8_lossy(&output.stderr).trim().to_string()));
    }
    Ok(())
}

/// Smart fetch: fetches from the current branch's remote with prune and tags.
pub async fn fetch(repo_path: &Path) -> Result<String, AppError> {
    let remote = get_current_remote(repo_path);
    let (success, _stdout, stderr) = run_network_command(
        &["fetch", &remote, "--prune", "--tags"],
        Some(repo_path),
    ).await?;
    if !success {
        return Err(AppError::Git(if stderr.is_empty() { "Fetch failed".into() } else { stderr }));
    }
    Ok(stderr)
}

/// Stage files for commit.
pub fn stage_files(repo_path: &Path, files: &[String]) -> Result<(), AppError> {
    if files.is_empty() {
        return Ok(());
    }

    let mut cmd = Command::new("git");
    cmd.current_dir(repo_path).arg("add");
    for f in files {
        cmd.arg(f);
    }

    let output = cmd.output()?;
    if !output.status.success() {
        return Err(AppError::Git(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    Ok(())
}

/// Unstage files.
pub fn unstage_files(repo_path: &Path, files: &[String]) -> Result<(), AppError> {
    if files.is_empty() {
        return Ok(());
    }

    let mut cmd = Command::new("git");
    cmd.current_dir(repo_path).args(["reset", "HEAD", "--"]);
    for f in files {
        cmd.arg(f);
    }

    let output = cmd.output()?;
    if !output.status.success() {
        return Err(AppError::Git(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    Ok(())
}

/// Create a commit with the given message.
pub fn commit(repo_path: &Path, message: &str) -> Result<String, AppError> {
    let output = Command::new("git")
        .current_dir(repo_path)
        .args(["commit", "-m", message])
        .output()?;

    if !output.status.success() {
        return Err(AppError::Git(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    // Get the new commit hash
    let hash_output = Command::new("git")
        .current_dir(repo_path)
        .args(["rev-parse", "HEAD"])
        .output()?;

    Ok(String::from_utf8_lossy(&hash_output.stdout)
        .trim()
        .to_string())
}

/// Check if the current branch has an upstream tracking branch.
/// Returns (has_upstream, current_branch_name, remote_name, has_remote).
pub fn check_push_status(repo_path: &Path) -> Result<(bool, String, String, bool), AppError> {
    // Get current branch name
    let branch_output = Command::new("git")
        .current_dir(repo_path)
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()?;

    if !branch_output.status.success() {
        return Err(AppError::Git("No commits yet — nothing to push".into()));
    }

    let branch_name = String::from_utf8_lossy(&branch_output.stdout)
        .trim()
        .to_string();

    if branch_name.is_empty() || branch_name == "HEAD" {
        return Err(AppError::Git("Cannot push from detached HEAD".into()));
    }

    // Get default remote
    let remote_output = Command::new("git")
        .current_dir(repo_path)
        .args(["remote"])
        .output()?;

    let remote_list = String::from_utf8_lossy(&remote_output.stdout)
        .trim()
        .to_string();

    if remote_list.is_empty() {
        return Ok((false, branch_name, String::new(), false));
    }

    let remote_name = remote_list
        .lines()
        .next()
        .unwrap_or("origin")
        .to_string();

    // Check if upstream exists
    let upstream_output = Command::new("git")
        .current_dir(repo_path)
        .args(["rev-parse", "--abbrev-ref", &format!("{}@{{u}}", branch_name)])
        .output()?;

    let has_upstream = upstream_output.status.success();
    Ok((has_upstream, branch_name, remote_name, true))
}

/// Push to remote. If no upstream, uses --set-upstream.
/// Detects common push rejections and returns clear error messages.
pub async fn push(repo_path: &Path, remote: &str, branch: &str, set_upstream: bool) -> Result<String, AppError> {
    if remote.is_empty() {
        return Err(AppError::Git("No remote configured. Add a remote first with: git remote add origin <url>".into()));
    }

    let mut args = vec!["push"];
    if set_upstream {
        args.push("--set-upstream");
    }
    args.push(remote);
    args.push(branch);

    let (success, stdout, stderr) = run_network_command(&args, Some(repo_path)).await?;

    if !success {
        let err = stderr.to_lowercase();
        if err.contains("non-fast-forward") || err.contains("fetch first") || err.contains("rejected") {
            return Err(AppError::Git(
                "Push rejected: the remote has commits you don't have. Pull first to merge changes.".into()
            ));
        }
        if err.contains("permission denied") || err.contains("authentication") {
            return Err(AppError::Git(
                "Permission denied. Check your SSH key or credentials.".into()
            ));
        }
        return Err(AppError::Git(if stderr.is_empty() { "Push failed".into() } else { stderr }));
    }

    Ok(if stderr.is_empty() { stdout } else { stderr })
}

/// Smart pull: fetch from upstream remote, then merge.
/// Returns PullResult with conflict info if merge fails.
pub async fn pull(repo_path: &Path) -> Result<crate::types::PullResult, AppError> {
    let remote = get_current_remote(repo_path);
    let branch = get_current_branch(repo_path)?;

    // Step 1: Fetch from remote
    let (success, _, stderr) = run_network_command(
        &["fetch", &remote],
        Some(repo_path),
    ).await?;
    if !success {
        return Err(AppError::Git(if stderr.is_empty() { "Fetch failed".into() } else { stderr }));
    }

    // Step 2: Merge remote/branch into current
    let merge_ref = format!("{}/{}", remote, branch);
    let merge_output = Command::new("git")
        .current_dir(repo_path)
        .args(["merge", &merge_ref])
        .output()?;

    let merge_stdout = String::from_utf8_lossy(&merge_output.stdout).trim().to_string();
    let merge_stderr = String::from_utf8_lossy(&merge_output.stderr).trim().to_string();

    if merge_output.status.success() {
        return Ok(crate::types::PullResult {
            success: true,
            message: if merge_stdout.is_empty() { merge_stderr } else { merge_stdout },
            has_conflicts: false,
            conflict_files: vec![],
        });
    }

    // Step 3: Check for merge conflicts
    let conflicts = get_conflict_files(repo_path);
    if !conflicts.is_empty() {
        return Ok(crate::types::PullResult {
            success: false,
            message: "Merge conflicts detected. Resolve conflicts and commit.".into(),
            has_conflicts: true,
            conflict_files: conflicts,
        });
    }

    // Other merge error (not conflicts)
    Err(AppError::Git(if merge_stderr.is_empty() { "Merge failed".into() } else { merge_stderr }))
}

/// Checkout a branch.
pub fn checkout(repo_path: &Path, branch: &str) -> Result<(), AppError> {
    let output = Command::new("git")
        .current_dir(repo_path)
        .args(["checkout", branch])
        .output()?;

    if !output.status.success() {
        return Err(AppError::Git(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    Ok(())
}

/// Stash changes.
pub fn stash_push(repo_path: &Path, message: Option<&str>) -> Result<(), AppError> {
    let mut cmd = Command::new("git");
    cmd.current_dir(repo_path).args(["stash", "push"]);

    if let Some(msg) = message {
        cmd.args(["-m", msg]);
    }

    let output = cmd.output()?;
    if !output.status.success() {
        return Err(AppError::Git(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    Ok(())
}

/// Pop stash.
pub fn stash_pop(repo_path: &Path) -> Result<(), AppError> {
    let output = Command::new("git")
        .current_dir(repo_path)
        .args(["stash", "pop"])
        .output()?;

    if !output.status.success() {
        return Err(AppError::Git(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    Ok(())
}

// ─── v0.2.0: Branch/Tag CRUD, Reset, Amend, Stash management ───

/// Delete a local branch. Use force=true for unmerged branches.
pub fn delete_branch(repo_path: &Path, name: &str, force: bool) -> Result<(), AppError> {
    let flag = if force { "-D" } else { "-d" };
    let output = Command::new("git")
        .current_dir(repo_path)
        .args(["branch", flag, name])
        .output()?;
    if !output.status.success() {
        return Err(AppError::Git(String::from_utf8_lossy(&output.stderr).trim().to_string()));
    }
    Ok(())
}

/// Rename a branch.
pub fn rename_branch(repo_path: &Path, old_name: &str, new_name: &str) -> Result<(), AppError> {
    let output = Command::new("git")
        .current_dir(repo_path)
        .args(["branch", "-m", old_name, new_name])
        .output()?;
    if !output.status.success() {
        return Err(AppError::Git(String::from_utf8_lossy(&output.stderr).trim().to_string()));
    }
    Ok(())
}

/// Delete a remote branch.
pub async fn delete_remote_branch(repo_path: &Path, remote: &str, branch: &str) -> Result<(), AppError> {
    let (success, _, stderr) = run_network_command(
        &["push", remote, "--delete", branch],
        Some(repo_path),
    ).await?;
    if !success {
        return Err(AppError::Git(if stderr.is_empty() { "Failed to delete remote branch".into() } else { stderr }));
    }
    Ok(())
}

/// Create a tag (annotated if message is provided, lightweight otherwise).
pub fn create_tag(repo_path: &Path, name: &str, commit: &str, message: Option<&str>) -> Result<(), AppError> {
    let mut cmd = Command::new("git");
    cmd.current_dir(repo_path);
    if let Some(msg) = message {
        cmd.args(["tag", "-a", name, commit, "-m", msg]);
    } else {
        cmd.args(["tag", name, commit]);
    }
    let output = cmd.output()?;
    if !output.status.success() {
        return Err(AppError::Git(String::from_utf8_lossy(&output.stderr).trim().to_string()));
    }
    Ok(())
}

/// Delete a local tag.
pub fn delete_tag(repo_path: &Path, name: &str) -> Result<(), AppError> {
    let output = Command::new("git")
        .current_dir(repo_path)
        .args(["tag", "-d", name])
        .output()?;
    if !output.status.success() {
        return Err(AppError::Git(String::from_utf8_lossy(&output.stderr).trim().to_string()));
    }
    Ok(())
}

/// Delete a remote tag.
pub async fn delete_remote_tag(repo_path: &Path, remote: &str, tag: &str) -> Result<(), AppError> {
    let (success, _, stderr) = run_network_command(
        &["push", remote, "--delete", &format!("refs/tags/{}", tag)],
        Some(repo_path),
    ).await?;
    if !success {
        return Err(AppError::Git(if stderr.is_empty() { "Failed to delete remote tag".into() } else { stderr }));
    }
    Ok(())
}

/// Amend the last commit with a new message.
pub fn amend_commit(repo_path: &Path, message: &str) -> Result<String, AppError> {
    let output = Command::new("git")
        .current_dir(repo_path)
        .args(["commit", "--amend", "-m", message])
        .output()?;
    if !output.status.success() {
        return Err(AppError::Git(String::from_utf8_lossy(&output.stderr).trim().to_string()));
    }
    let hash = Command::new("git")
        .current_dir(repo_path)
        .args(["rev-parse", "HEAD"])
        .output()?;
    Ok(String::from_utf8_lossy(&hash.stdout).trim().to_string())
}

/// Reset to a specific commit.
pub fn reset(repo_path: &Path, commit: &str, mode: &str) -> Result<(), AppError> {
    let flag = match mode {
        "soft" => "--soft",
        "hard" => "--hard",
        _ => "--mixed",
    };
    let output = Command::new("git")
        .current_dir(repo_path)
        .args(["reset", flag, commit])
        .output()?;
    if !output.status.success() {
        return Err(AppError::Git(String::from_utf8_lossy(&output.stderr).trim().to_string()));
    }
    Ok(())
}

/// List stashes.
pub fn stash_list(repo_path: &Path) -> Result<Vec<(usize, String)>, AppError> {
    let output = Command::new("git")
        .current_dir(repo_path)
        .args(["stash", "list", "--format=%gd||%s"])
        .output()?;
    let text = String::from_utf8_lossy(&output.stdout);
    let stashes: Vec<(usize, String)> = text
        .trim()
        .lines()
        .enumerate()
        .filter(|(_, l)| !l.is_empty())
        .map(|(i, line)| {
            let msg = line.split("||").nth(1).unwrap_or(line).to_string();
            (i, msg)
        })
        .collect();
    Ok(stashes)
}

/// Apply a stash by index.
pub fn stash_apply(repo_path: &Path, index: usize) -> Result<(), AppError> {
    let output = Command::new("git")
        .current_dir(repo_path)
        .args(["stash", "apply", &format!("stash@{{{}}}", index)])
        .output()?;
    if !output.status.success() {
        return Err(AppError::Git(String::from_utf8_lossy(&output.stderr).trim().to_string()));
    }
    Ok(())
}

/// Drop a stash by index.
pub fn stash_drop(repo_path: &Path, index: usize) -> Result<(), AppError> {
    let output = Command::new("git")
        .current_dir(repo_path)
        .args(["stash", "drop", &format!("stash@{{{}}}", index)])
        .output()?;
    if !output.status.success() {
        return Err(AppError::Git(String::from_utf8_lossy(&output.stderr).trim().to_string()));
    }
    Ok(())
}

/// Merge a branch into the current branch.
pub fn merge_branch(repo_path: &Path, branch: &str) -> Result<crate::types::PullResult, AppError> {
    let output = Command::new("git")
        .current_dir(repo_path)
        .args(["merge", branch])
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();

    if output.status.success() {
        return Ok(crate::types::PullResult {
            success: true,
            message: if stdout.is_empty() { stderr } else { stdout },
            has_conflicts: false,
            conflict_files: vec![],
        });
    }

    let conflicts = get_conflict_files(repo_path);
    if !conflicts.is_empty() {
        return Ok(crate::types::PullResult {
            success: false,
            message: "Merge conflicts detected".into(),
            has_conflicts: true,
            conflict_files: conflicts,
        });
    }

    Err(AppError::Git(if stderr.is_empty() { "Merge failed".into() } else { stderr }))
}

/// Cherry-pick a commit.
pub fn cherry_pick(repo_path: &Path, commit: &str) -> Result<crate::types::PullResult, AppError> {
    let output = Command::new("git")
        .current_dir(repo_path)
        .args(["cherry-pick", commit])
        .output()?;

    if output.status.success() {
        return Ok(crate::types::PullResult {
            success: true,
            message: "Cherry-pick applied".into(),
            has_conflicts: false,
            conflict_files: vec![],
        });
    }

    let conflicts = get_conflict_files(repo_path);
    if !conflicts.is_empty() {
        return Ok(crate::types::PullResult {
            success: false,
            message: "Cherry-pick conflicts detected".into(),
            has_conflicts: true,
            conflict_files: conflicts,
        });
    }

    Err(AppError::Git(String::from_utf8_lossy(&output.stderr).trim().to_string()))
}

/// Discard changes for specific files.
/// tracked modified/deleted → git checkout -- <file>
/// untracked → git clean -f -- <file>
pub fn discard_files(repo_path: &Path, files: &[String], untracked_files: &[String]) -> Result<serde_json::Value, AppError> {
    if !files.is_empty() {
        let mut cmd = Command::new("git");
        cmd.current_dir(repo_path).args(["checkout", "--", "--"]);
        for f in files {
            cmd.arg(f);
        }
        let output = cmd.output()?;
        if !output.status.success() {
            return Err(AppError::Git(
                String::from_utf8_lossy(&output.stderr).trim().to_string(),
            ));
        }
    }

    if !untracked_files.is_empty() {
        let mut cmd = Command::new("git");
        cmd.current_dir(repo_path).args(["clean", "-f", "--"]);
        for f in untracked_files {
            cmd.arg(f);
        }
        let output = cmd.output()?;
        if !output.status.success() {
            return Err(AppError::Git(
                String::from_utf8_lossy(&output.stderr).trim().to_string(),
            ));
        }
    }

    Ok(serde_json::json!({"success": true}))
}

/// Discard ALL unstaged changes + untracked files.
/// git checkout -- .  (tracked)
/// git clean -fd      (untracked)
pub fn discard_all(repo_path: &Path) -> Result<serde_json::Value, AppError> {
    let output = Command::new("git")
        .current_dir(repo_path)
        .args(["checkout", "--", "."])
        .output()?;
    if !output.status.success() {
        return Err(AppError::Git(
            String::from_utf8_lossy(&output.stderr).trim().to_string(),
        ));
    }

    let output = Command::new("git")
        .current_dir(repo_path)
        .args(["clean", "-fd"])
        .output()?;
    if !output.status.success() {
        return Err(AppError::Git(
            String::from_utf8_lossy(&output.stderr).trim().to_string(),
        ));
    }

    Ok(serde_json::json!({"success": true}))
}

/// Get global git config values for user.name and user.email.
pub fn get_git_config() -> Result<(String, String), AppError> {
    let name = Command::new("git")
        .args(["config", "--global", "user.name"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default();

    let email = Command::new("git")
        .args(["config", "--global", "user.email"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default();

    Ok((name, email))
}

/// Set global git config values for user.name and user.email.
pub fn set_git_config(name: &str, email: &str) -> Result<(), AppError> {
    let output = Command::new("git")
        .args(["config", "--global", "user.name", name])
        .output()?;
    if !output.status.success() {
        return Err(AppError::Git(String::from_utf8_lossy(&output.stderr).trim().to_string()));
    }

    let output = Command::new("git")
        .args(["config", "--global", "user.email", email])
        .output()?;
    if !output.status.success() {
        return Err(AppError::Git(String::from_utf8_lossy(&output.stderr).trim().to_string()));
    }

    Ok(())
}

/// Revert a commit.
pub fn revert_commit(repo_path: &Path, commit: &str) -> Result<crate::types::PullResult, AppError> {
    let output = Command::new("git")
        .current_dir(repo_path)
        .args(["revert", "--no-edit", commit])
        .output()?;

    if output.status.success() {
        return Ok(crate::types::PullResult {
            success: true,
            message: "Commit reverted".into(),
            has_conflicts: false,
            conflict_files: vec![],
        });
    }

    let conflicts = get_conflict_files(repo_path);
    if !conflicts.is_empty() {
        return Ok(crate::types::PullResult {
            success: false,
            message: "Revert conflicts detected".into(),
            has_conflicts: true,
            conflict_files: conflicts,
        });
    }

    Err(AppError::Git(String::from_utf8_lossy(&output.stderr).trim().to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    /// Create a temporary clone of the fixture repo for destructive tests.
    fn create_temp_repo() -> tempfile::TempDir {
        let tmp = tempfile::tempdir().unwrap();
        let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("tests")
            .join("fixtures")
            .join("test-repo");

        // Clone the fixture to a temp dir
        let output = Command::new("git")
            .args(["clone", fixture.to_str().unwrap(), tmp.path().to_str().unwrap()])
            .output()
            .unwrap();
        assert!(output.status.success(), "Failed to clone fixture: {}", String::from_utf8_lossy(&output.stderr));

        // Configure git user
        Command::new("git")
            .current_dir(tmp.path())
            .args(["config", "user.email", "test@test.com"])
            .output()
            .unwrap();
        Command::new("git")
            .current_dir(tmp.path())
            .args(["config", "user.name", "Test User"])
            .output()
            .unwrap();

        tmp
    }

    #[test]
    fn test_stage_and_commit() {
        let tmp = create_temp_repo();
        let repo_path = tmp.path();

        // Create a new file
        fs::write(repo_path.join("new_file.txt"), "test content").unwrap();

        // Stage it
        stage_files(repo_path, &["new_file.txt".to_string()]).unwrap();

        // Verify it's staged
        let output = Command::new("git")
            .current_dir(repo_path)
            .args(["diff", "--cached", "--name-only"])
            .output()
            .unwrap();
        let staged = String::from_utf8_lossy(&output.stdout);
        assert!(staged.contains("new_file.txt"));

        // Commit it
        let hash = commit(repo_path, "Test commit from Rust tests").unwrap();
        assert_eq!(hash.len(), 40, "Commit hash should be 40 chars");

        // Verify the commit exists
        let output = Command::new("git")
            .current_dir(repo_path)
            .args(["log", "-1", "--format=%s"])
            .output()
            .unwrap();
        let msg = String::from_utf8_lossy(&output.stdout).trim().to_string();
        assert_eq!(msg, "Test commit from Rust tests");
    }

    #[test]
    fn test_unstage_files() {
        let tmp = create_temp_repo();
        let repo_path = tmp.path();

        // Create and stage a file
        fs::write(repo_path.join("to_unstage.txt"), "content").unwrap();
        stage_files(repo_path, &["to_unstage.txt".to_string()]).unwrap();

        // Unstage it
        unstage_files(repo_path, &["to_unstage.txt".to_string()]).unwrap();

        // Verify it's no longer staged
        let output = Command::new("git")
            .current_dir(repo_path)
            .args(["diff", "--cached", "--name-only"])
            .output()
            .unwrap();
        let staged = String::from_utf8_lossy(&output.stdout);
        assert!(!staged.contains("to_unstage.txt"));
    }

    #[test]
    fn test_checkout_branch() {
        let tmp = create_temp_repo();
        let repo_path = tmp.path();

        // We should be on main initially
        let output = Command::new("git")
            .current_dir(repo_path)
            .args(["rev-parse", "--abbrev-ref", "HEAD"])
            .output()
            .unwrap();
        let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
        assert_eq!(branch, "main");

        // Checkout feature-branch
        checkout(repo_path, "feature-branch").unwrap();

        let output = Command::new("git")
            .current_dir(repo_path)
            .args(["rev-parse", "--abbrev-ref", "HEAD"])
            .output()
            .unwrap();
        let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
        assert_eq!(branch, "feature-branch");
    }

    #[test]
    fn test_stash_push_and_pop() {
        let tmp = create_temp_repo();
        let repo_path = tmp.path();

        // Create a modification
        fs::write(repo_path.join("stash_test.txt"), "stash me").unwrap();
        stage_files(repo_path, &["stash_test.txt".to_string()]).unwrap();

        // Stash
        stash_push(repo_path, Some("test stash")).unwrap();

        // File should be gone
        assert!(!repo_path.join("stash_test.txt").exists());

        // Pop stash
        stash_pop(repo_path).unwrap();

        // File should be back
        assert!(repo_path.join("stash_test.txt").exists());
    }
}
