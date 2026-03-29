import type { CommitGraph, CommitNode, DiffResult, RefLabel, RepoStatus } from "$lib/types";

export class RepoState {
  repoPath = $state("");
  branches = $state<RefLabel[]>([]);
  commitGraph = $state<CommitGraph | null>(null);
  selectedCommit = $state<CommitNode | null>(null);
  diff = $state<DiffResult | null>(null);
  status = $state<RepoStatus | null>(null);
  isLoading = $state(false);
  error = $state<string | null>(null);

  currentBranch = $derived(this.status?.branch ?? null);
  hasUnstagedChanges = $derived(
    (this.status?.unstaged.length ?? 0) > 0 || (this.status?.untracked.length ?? 0) > 0,
  );
  hasStagedChanges = $derived((this.status?.staged.length ?? 0) > 0);
  displayName = $derived(this.repoPath.split("/").pop() ?? this.repoPath);

  constructor(path: string) {
    this.repoPath = path;
  }
}

export class MultiRepoStore {
  repos = $state<Map<string, RepoState>>(new Map());
  activeRepoId = $state<string | null>(null);

  // Active repo reference
  activeRepo = $derived(
    this.activeRepoId ? this.repos.get(this.activeRepoId) ?? null : null,
  );

  // List for the switcher UI
  openRepos = $derived(Array.from(this.repos.values()));
  isAnyRepoOpen = $derived(this.repos.size > 0);

  // Compatibility shims — read-only delegates to activeRepo
  // This allows existing components to keep reading repoStore.branches etc.
  repoPath = $derived(this.activeRepo?.repoPath ?? null);
  branches = $derived(this.activeRepo?.branches ?? []);
  commitGraph = $derived(this.activeRepo?.commitGraph ?? null);
  selectedCommit = $derived(this.activeRepo?.selectedCommit ?? null);
  diff = $derived(this.activeRepo?.diff ?? null);
  status = $derived(this.activeRepo?.status ?? null);
  isLoading = $derived(this.activeRepo?.isLoading ?? false);
  error = $derived(this.activeRepo?.error ?? null);
  currentBranch = $derived(this.activeRepo?.currentBranch ?? null);
  isRepoOpen = $derived(this.activeRepoId !== null);
  hasUnstagedChanges = $derived(this.activeRepo?.hasUnstagedChanges ?? false);
  hasStagedChanges = $derived(this.activeRepo?.hasStagedChanges ?? false);

  addRepo(path: string): RepoState {
    const existing = this.repos.get(path);
    if (existing) {
      this.activeRepoId = path;
      return existing;
    }
    const state = new RepoState(path);
    this.repos = new Map(this.repos).set(path, state);
    this.activeRepoId = path;
    this.persist();
    return state;
  }

  removeRepo(path: string) {
    const newMap = new Map(this.repos);
    newMap.delete(path);
    this.repos = newMap;

    if (this.activeRepoId === path) {
      const remaining = Array.from(newMap.keys());
      this.activeRepoId = remaining.length > 0 ? remaining[remaining.length - 1] : null;
    }
    this.persist();
  }

  switchTo(path: string) {
    if (this.repos.has(path)) {
      this.activeRepoId = path;
      this.persist();
    }
  }

  /** Save open repo paths + active to localStorage */
  persist() {
    const data = {
      paths: Array.from(this.repos.keys()),
      active: this.activeRepoId,
    };
    localStorage.setItem("gitferrum-repos", JSON.stringify(data));
  }

  /** Get saved repo paths from previous session */
  static getSavedSession(): { paths: string[]; active: string | null } {
    try {
      const raw = localStorage.getItem("gitferrum-repos");
      if (!raw) return { paths: [], active: null };
      const data = JSON.parse(raw);
      return {
        paths: Array.isArray(data.paths) ? data.paths : [],
        active: typeof data.active === "string" ? data.active : null,
      };
    } catch {
      return { paths: [], active: null };
    }
  }
}

export const repoStore = new MultiRepoStore();
