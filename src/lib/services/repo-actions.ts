import { repoStore, MultiRepoStore } from "$lib/stores/repo.svelte";
import { toastStore } from "$lib/stores/toast.svelte";
import { i18n } from "$lib/stores/i18n.svelte";
import { openRepo, cloneRepo, getCommitGraph, getBranches, getStatus, watchRepo, closeRepo as closeRepoCmd, pushBranch } from "$lib/services/git";

export async function openAndLoadRepo(path: string): Promise<void> {
  const repo = repoStore.addRepo(path);
  repo.isLoading = true;

  try {
    const status = await openRepo(path);
    repo.status = status;

    const [graph, branches] = await Promise.all([
      getCommitGraph(path, 0, 200),
      getBranches(path),
    ]);

    repo.commitGraph = graph;
    repo.branches = branches;

    await watchRepo(path);
    toastStore.success(i18n.t.repoOpened(repo.displayName));
  } catch (err) {
    repoStore.removeRepo(path);
    toastStore.error(String(err));
  } finally {
    repo.isLoading = false;
  }
}

export async function cloneAndOpenRepo(url: string, dest: string): Promise<void> {
  const repo = repoStore.addRepo(dest);
  repo.isLoading = true;

  try {
    const status = await cloneRepo(url, dest);
    repo.status = status;

    const [graph, branches] = await Promise.all([
      getCommitGraph(dest, 0, 200),
      getBranches(dest),
    ]);

    repo.commitGraph = graph;
    repo.branches = branches;

    await watchRepo(dest);
    toastStore.success(i18n.t.repoCloned(repo.displayName));
  } catch (err) {
    repoStore.removeRepo(dest);
    toastStore.error(String(err));
  } finally {
    repo.isLoading = false;
  }
}

export async function executePush(remote: string, branch: string, setUpstream: boolean): Promise<void> {
  const repo = repoStore.activeRepo;
  if (!repo) return;
  try {
    await pushBranch(repo.repoPath, remote, branch, setUpstream);
    // Refresh status after push
    const [status, graph, branches] = await Promise.all([
      getStatus(repo.repoPath),
      getCommitGraph(repo.repoPath, 0, 200),
      getBranches(repo.repoPath),
    ]);
    repo.status = status;
    repo.commitGraph = graph;
    repo.branches = branches;
    toastStore.success(i18n.t.pushedToRemote);
  } catch (e) {
    toastStore.error(String(e));
  }
}

/** Restore repos from previous session on app startup */
export async function restoreSession(): Promise<void> {
  const { paths, active } = MultiRepoStore.getSavedSession();
  if (paths.length === 0) return;

  // Open all saved repos (skip ones that fail — they may have been deleted)
  for (const path of paths) {
    try {
      const repo = repoStore.addRepo(path);
      repo.isLoading = true;
      const status = await openRepo(path);
      repo.status = status;
      const [graph, branches] = await Promise.all([
        getCommitGraph(path, 0, 200),
        getBranches(path),
      ]);
      repo.commitGraph = graph;
      repo.branches = branches;
      await watchRepo(path);
      repo.isLoading = false;
    } catch {
      // Repo no longer exists or is invalid — remove silently
      repoStore.removeRepo(path);
    }
  }

  // Restore the active repo
  if (active && repoStore.repos.has(active)) {
    repoStore.switchTo(active);
  }
}

export async function closeAndRemoveRepo(path: string): Promise<void> {
  try {
    await closeRepoCmd(path);
  } catch {
    // Backend might fail if already removed, that's ok
  }
  repoStore.removeRepo(path);
}
