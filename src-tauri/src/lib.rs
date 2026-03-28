mod commands;
mod error;
mod git;
mod state;
mod types;
mod watcher;

use state::AppState;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            commands::open_repo,
            commands::clone_repo,
            commands::close_repo,
            commands::switch_repo,
            commands::get_open_repos,
            commands::get_commit_graph,
            commands::get_diff,
            commands::get_staged_diff,
            commands::get_branches,
            commands::get_status,
            commands::git_action,
            commands::fetch_repo,
            commands::pull_repo,
            commands::check_push_status,
            commands::push_branch,
            commands::get_repo_state,
            commands::abort_merge,
            commands::delete_branch,
            commands::rename_branch,
            commands::delete_remote_branch,
            commands::create_tag,
            commands::delete_tag,
            commands::delete_remote_tag,
            commands::amend_commit,
            commands::reset_to_commit,
            commands::stash_list,
            commands::stash_apply,
            commands::stash_drop,
            commands::merge_branch,
            commands::cherry_pick,
            commands::revert_commit,
            commands::watch_repo,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
