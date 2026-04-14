use crate::error::AppError;
use crate::state::AppState;
use crate::types::{CommitGraph, DiffResult, RefLabel, RepoStatus};
use std::path::PathBuf;
use tauri::State;

#[tauri::command]
pub async fn open_repo(
    path: String,
    state: State<'_, AppState>,
) -> Result<RepoStatus, AppError> {
    let repo_path = PathBuf::from(&path);

    if !repo_path.exists() {
        return Err(AppError::PathNotFound(path));
    }

    // Verify it's a valid git repo
    crate::git::open_repo(&repo_path)?;

    state.add_repo(repo_path);

    crate::git::status::get_status(&PathBuf::from(&path))
}

#[tauri::command]
pub async fn close_repo(
    path: String,
    state: State<'_, AppState>,
) -> Result<Option<String>, AppError> {
    let repo_path = PathBuf::from(&path);
    let new_active = state.remove_repo(&repo_path);
    Ok(new_active.map(|p| p.to_string_lossy().to_string()))
}

#[tauri::command]
pub async fn switch_repo(
    path: String,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    state.set_active_repo(PathBuf::from(&path))
}

#[tauri::command]
pub async fn get_open_repos(
    state: State<'_, AppState>,
) -> Result<Vec<String>, AppError> {
    Ok(state
        .get_open_repos()
        .into_iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect())
}

#[tauri::command]
pub async fn clone_repo(
    url: String,
    dest: String,
    state: State<'_, AppState>,
) -> Result<RepoStatus, AppError> {
    let dest_path = PathBuf::from(&dest);

    crate::git::actions::clone_repo(&url, &dest_path).await?;

    // Auto-open the cloned repo
    crate::git::open_repo(&dest_path)?;
    state.add_repo(dest_path);

    crate::git::status::get_status(&PathBuf::from(&dest))
}

#[tauri::command]
pub async fn get_commit_graph(
    repo_path: String,
    skip: usize,
    limit: usize,
) -> Result<CommitGraph, AppError> {
    crate::git::graph::build_commit_graph(&PathBuf::from(&repo_path), skip, limit)
}

#[tauri::command]
pub async fn get_diff(
    repo_path: String,
    from_hash: Option<String>,
    to_hash: Option<String>,
) -> Result<DiffResult, AppError> {
    crate::git::diff::get_diff(
        &PathBuf::from(&repo_path),
        from_hash.as_deref(),
        to_hash.as_deref(),
    )
}

#[tauri::command]
pub async fn get_staged_diff(repo_path: String) -> Result<DiffResult, AppError> {
    crate::git::diff::get_staged_diff(&PathBuf::from(&repo_path))
}

#[tauri::command]
pub async fn get_branches(repo_path: String) -> Result<Vec<RefLabel>, AppError> {
    crate::git::refs::list_refs(&PathBuf::from(&repo_path))
}

#[tauri::command]
pub async fn get_status(repo_path: String) -> Result<RepoStatus, AppError> {
    crate::git::status::get_status(&PathBuf::from(&repo_path))
}

#[tauri::command]
pub async fn check_push_status(
    repo_path: String,
) -> Result<crate::types::PushStatus, AppError> {
    let (has_upstream, branch_name, remote_name, has_remote) =
        crate::git::actions::check_push_status(&PathBuf::from(&repo_path))?;
    Ok(crate::types::PushStatus {
        has_upstream,
        has_remote,
        branch_name,
        remote_name,
    })
}

#[tauri::command]
pub async fn fetch_repo(repo_path: String) -> Result<String, AppError> {
    crate::git::actions::fetch(&PathBuf::from(&repo_path)).await
}

#[tauri::command]
pub async fn pull_repo(repo_path: String) -> Result<crate::types::PullResult, AppError> {
    crate::git::actions::pull(&PathBuf::from(&repo_path)).await
}

#[tauri::command]
pub async fn get_repo_state(repo_path: String) -> Result<String, AppError> {
    Ok(crate::git::actions::get_repo_state(&PathBuf::from(&repo_path)))
}

#[tauri::command]
pub async fn abort_merge(repo_path: String) -> Result<(), AppError> {
    crate::git::actions::abort_merge(&PathBuf::from(&repo_path))
}

#[tauri::command]
pub async fn push_branch(
    repo_path: String,
    remote: String,
    branch: String,
    set_upstream: bool,
) -> Result<String, AppError> {
    crate::git::actions::push(&PathBuf::from(&repo_path), &remote, &branch, set_upstream).await
}

// ─── v0.2.0: Branch/Tag CRUD, Reset, Amend, Stash, Merge, Cherry-pick, Revert ───

#[tauri::command]
pub async fn delete_branch(repo_path: String, name: String, force: bool) -> Result<(), AppError> {
    crate::git::actions::delete_branch(&PathBuf::from(&repo_path), &name, force)
}

#[tauri::command]
pub async fn rename_branch(repo_path: String, old_name: String, new_name: String) -> Result<(), AppError> {
    crate::git::actions::rename_branch(&PathBuf::from(&repo_path), &old_name, &new_name)
}

#[tauri::command]
pub async fn delete_remote_branch(repo_path: String, remote: String, branch: String) -> Result<(), AppError> {
    crate::git::actions::delete_remote_branch(&PathBuf::from(&repo_path), &remote, &branch).await
}

#[tauri::command]
pub async fn create_tag(repo_path: String, name: String, commit: String, message: Option<String>) -> Result<(), AppError> {
    crate::git::actions::create_tag(&PathBuf::from(&repo_path), &name, &commit, message.as_deref())
}

#[tauri::command]
pub async fn delete_tag(repo_path: String, name: String) -> Result<(), AppError> {
    crate::git::actions::delete_tag(&PathBuf::from(&repo_path), &name)
}

#[tauri::command]
pub async fn delete_remote_tag(repo_path: String, remote: String, tag: String) -> Result<(), AppError> {
    crate::git::actions::delete_remote_tag(&PathBuf::from(&repo_path), &remote, &tag).await
}

#[tauri::command]
pub async fn amend_commit(repo_path: String, message: String) -> Result<String, AppError> {
    crate::git::actions::amend_commit(&PathBuf::from(&repo_path), &message)
}

#[tauri::command]
pub async fn reset_to_commit(repo_path: String, commit: String, mode: String) -> Result<(), AppError> {
    crate::git::actions::reset(&PathBuf::from(&repo_path), &commit, &mode)
}

#[tauri::command]
pub async fn stash_list(repo_path: String) -> Result<Vec<(usize, String)>, AppError> {
    crate::git::actions::stash_list(&PathBuf::from(&repo_path))
}

#[tauri::command]
pub async fn stash_apply(repo_path: String, index: usize) -> Result<(), AppError> {
    crate::git::actions::stash_apply(&PathBuf::from(&repo_path), index)
}

#[tauri::command]
pub async fn stash_drop(repo_path: String, index: usize) -> Result<(), AppError> {
    crate::git::actions::stash_drop(&PathBuf::from(&repo_path), index)
}

#[tauri::command]
pub async fn merge_branch(repo_path: String, branch: String) -> Result<crate::types::PullResult, AppError> {
    crate::git::actions::merge_branch(&PathBuf::from(&repo_path), &branch)
}

#[tauri::command]
pub async fn cherry_pick(repo_path: String, commit: String) -> Result<crate::types::PullResult, AppError> {
    crate::git::actions::cherry_pick(&PathBuf::from(&repo_path), &commit)
}

#[tauri::command]
pub async fn revert_commit(repo_path: String, commit: String) -> Result<crate::types::PullResult, AppError> {
    crate::git::actions::revert_commit(&PathBuf::from(&repo_path), &commit)
}

#[tauri::command]
pub async fn git_action(
    repo_path: String,
    action: String,
    args: serde_json::Value,
) -> Result<serde_json::Value, AppError> {
    let repo = PathBuf::from(&repo_path);

    match action.as_str() {
        "stage" => {
            let files: Vec<String> = serde_json::from_value(
                args.get("files")
                    .cloned()
                    .unwrap_or(serde_json::Value::Array(vec![])),
            )
            .map_err(|e| AppError::Other(e.to_string()))?;
            crate::git::actions::stage_files(&repo, &files)?;
            Ok(serde_json::json!({"success": true}))
        }
        "unstage" => {
            let files: Vec<String> = serde_json::from_value(
                args.get("files")
                    .cloned()
                    .unwrap_or(serde_json::Value::Array(vec![])),
            )
            .map_err(|e| AppError::Other(e.to_string()))?;
            crate::git::actions::unstage_files(&repo, &files)?;
            Ok(serde_json::json!({"success": true}))
        }
        "commit" => {
            let message = args
                .get("message")
                .and_then(|v| v.as_str())
                .ok_or_else(|| AppError::Other("Missing commit message".into()))?;
            let hash = crate::git::actions::commit(&repo, message)?;
            Ok(serde_json::json!({"success": true, "hash": hash}))
        }
        "checkout" => {
            let branch = args
                .get("branch")
                .and_then(|v| v.as_str())
                .ok_or_else(|| AppError::Other("Missing branch name".into()))?;
            crate::git::actions::checkout(&repo, branch)?;
            Ok(serde_json::json!({"success": true}))
        }
        "stash_push" => {
            let message = args.get("message").and_then(|v| v.as_str());
            crate::git::actions::stash_push(&repo, message)?;
            Ok(serde_json::json!({"success": true}))
        }
        "stash_pop" => {
            crate::git::actions::stash_pop(&repo)?;
            Ok(serde_json::json!({"success": true}))
        }
        "checkout_remote" => {
            let remote_branch = args
                .get("remoteBranch")
                .and_then(|v| v.as_str())
                .ok_or_else(|| AppError::Other("Missing remote branch name".into()))?;
            crate::git::actions::checkout_remote_branch(&repo, remote_branch)?;
            Ok(serde_json::json!({"success": true}))
        }
        "checkout_detached" => {
            let ref_name = args
                .get("refName")
                .and_then(|v| v.as_str())
                .ok_or_else(|| AppError::Other("Missing ref name".into()))?;
            crate::git::actions::checkout_detached(&repo, ref_name)?;
            Ok(serde_json::json!({"success": true}))
        }
        "create_branch_from_remote" => {
            let new_name = args
                .get("newName")
                .and_then(|v| v.as_str())
                .ok_or_else(|| AppError::Other("Missing new branch name".into()))?;
            let remote_branch = args
                .get("remoteBranch")
                .and_then(|v| v.as_str())
                .ok_or_else(|| AppError::Other("Missing remote branch name".into()))?;
            crate::git::actions::create_branch_from_remote(&repo, new_name, remote_branch)?;
            Ok(serde_json::json!({"success": true}))
        }
        "discard_files" => {
            let files: Vec<String> = serde_json::from_value(
                args.get("files")
                    .cloned()
                    .unwrap_or(serde_json::Value::Array(vec![])),
            )
            .map_err(|e| AppError::Other(e.to_string()))?;
            let untracked_files: Vec<String> = serde_json::from_value(
                args.get("untracked_files")
                    .cloned()
                    .unwrap_or(serde_json::Value::Array(vec![])),
            )
            .map_err(|e| AppError::Other(e.to_string()))?;
            crate::git::actions::discard_files(&repo, &files, &untracked_files)
        }
        "discard_all" => {
            crate::git::actions::discard_all(&repo)
        }
        _ => Err(AppError::Other(format!("Unknown action: {}", action))),
    }
}

// ─── Git global config (not repo-specific) ───

#[tauri::command]
pub async fn get_git_config() -> Result<serde_json::Value, AppError> {
    let (name, email) = crate::git::actions::get_git_config()?;
    Ok(serde_json::json!({"name": name, "email": email}))
}

#[tauri::command]
pub async fn set_git_config(name: String, email: String) -> Result<(), AppError> {
    crate::git::actions::set_git_config(&name, &email)
}

#[tauri::command]
pub async fn watch_repo(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    repo_path: String,
) -> Result<(), AppError> {
    let path = PathBuf::from(&repo_path);
    let handle = crate::watcher::start_watcher(app, path.clone());
    state.set_watcher_handle(&path, handle);
    Ok(())
}
