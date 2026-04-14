import { invoke } from "@tauri-apps/api/core";
import type {
  CommitGraph,
  DiffResult,
  PullResult,
  PushStatus,
  RefLabel,
  RepoStatus,
} from "$lib/types";

export async function openRepo(path: string): Promise<RepoStatus> {
  return invoke<RepoStatus>("open_repo", { path });
}

export async function cloneRepo(url: string, dest: string): Promise<RepoStatus> {
  return invoke<RepoStatus>("clone_repo", { url, dest });
}

export async function closeRepo(path: string): Promise<string | null> {
  return invoke<string | null>("close_repo", { path });
}

export async function switchRepo(path: string): Promise<void> {
  return invoke("switch_repo", { path });
}

export async function getOpenRepos(): Promise<string[]> {
  return invoke<string[]>("get_open_repos");
}

export async function getCommitGraph(
  repoPath: string,
  skip: number,
  limit: number,
): Promise<CommitGraph> {
  return invoke<CommitGraph>("get_commit_graph", { repoPath, skip, limit });
}

export async function getDiff(
  repoPath: string,
  fromHash?: string,
  toHash?: string,
): Promise<DiffResult> {
  return invoke<DiffResult>("get_diff", { repoPath, fromHash, toHash });
}

export async function getStagedDiff(repoPath: string): Promise<DiffResult> {
  return invoke<DiffResult>("get_staged_diff", { repoPath });
}

export async function getBranches(repoPath: string): Promise<RefLabel[]> {
  return invoke<RefLabel[]>("get_branches", { repoPath });
}

export async function getStatus(repoPath: string): Promise<RepoStatus> {
  return invoke<RepoStatus>("get_status", { repoPath });
}

export async function gitAction(
  repoPath: string,
  action: string,
  args: Record<string, unknown>,
): Promise<unknown> {
  return invoke("git_action", { repoPath, action, args });
}

export async function checkPushStatus(repoPath: string): Promise<PushStatus> {
  return invoke<PushStatus>("check_push_status", { repoPath });
}

export async function fetchRepo(repoPath: string): Promise<void> {
  return invoke("fetch_repo", { repoPath });
}

export async function pullRepo(repoPath: string): Promise<PullResult> {
  return invoke<PullResult>("pull_repo", { repoPath });
}

export async function getRepoState(repoPath: string): Promise<string> {
  return invoke<string>("get_repo_state", { repoPath });
}

export async function abortMerge(repoPath: string): Promise<void> {
  return invoke("abort_merge", { repoPath });
}

// ─── v0.2.0: Branch/Tag CRUD, Reset, Amend, Stash, Merge, Cherry-pick, Revert ───

export async function deleteBranch(repoPath: string, name: string, force: boolean): Promise<void> {
  return invoke("delete_branch", { repoPath, name, force });
}

export async function renameBranch(repoPath: string, oldName: string, newName: string): Promise<void> {
  return invoke("rename_branch", { repoPath, oldName, newName });
}

export async function deleteRemoteBranch(repoPath: string, remote: string, branch: string): Promise<void> {
  return invoke("delete_remote_branch", { repoPath, remote, branch });
}

export async function createTag(repoPath: string, name: string, commit: string, message?: string): Promise<void> {
  return invoke("create_tag", { repoPath, name, commit, message: message ?? null });
}

export async function deleteTag(repoPath: string, name: string): Promise<void> {
  return invoke("delete_tag", { repoPath, name });
}

export async function deleteRemoteTag(repoPath: string, remote: string, tag: string): Promise<void> {
  return invoke("delete_remote_tag", { repoPath, remote, tag });
}

export async function amendCommit(repoPath: string, message: string): Promise<string> {
  return invoke<string>("amend_commit", { repoPath, message });
}

export async function resetToCommit(repoPath: string, commit: string, mode: string): Promise<void> {
  return invoke("reset_to_commit", { repoPath, commit, mode });
}

export async function stashList(repoPath: string): Promise<[number, string][]> {
  return invoke<[number, string][]>("stash_list", { repoPath });
}

export async function stashApply(repoPath: string, index: number): Promise<void> {
  return invoke("stash_apply", { repoPath, index });
}

export async function stashDrop(repoPath: string, index: number): Promise<void> {
  return invoke("stash_drop", { repoPath, index });
}

export async function mergeBranch(repoPath: string, branch: string): Promise<PullResult> {
  return invoke<PullResult>("merge_branch", { repoPath, branch });
}

export async function cherryPick(repoPath: string, commit: string): Promise<PullResult> {
  return invoke<PullResult>("cherry_pick", { repoPath, commit });
}

export async function revertCommit(repoPath: string, commit: string): Promise<PullResult> {
  return invoke<PullResult>("revert_commit", { repoPath, commit });
}

export async function pushBranch(
  repoPath: string,
  remote: string,
  branch: string,
  setUpstream: boolean,
): Promise<string> {
  return invoke<string>("push_branch", { repoPath, remote, branch, setUpstream });
}

export async function watchRepo(repoPath: string): Promise<void> {
  return invoke("watch_repo", { repoPath });
}

// ─── Git global config ───

export async function getGitConfig(): Promise<{ name: string; email: string }> {
  return invoke<{ name: string; email: string }>("get_git_config");
}

export async function setGitConfig(name: string, email: string): Promise<void> {
  return invoke("set_git_config", { name, email });
}
