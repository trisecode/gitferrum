<script lang="ts">
  import { repoStore } from "$lib/stores/repo.svelte";
  import { uiStore } from "$lib/stores/ui.svelte";
  import { getDiff, getCommitGraph, getStatus, getBranches, checkPushStatus, fetchRepo, pullRepo } from "$lib/services/git";
  import { executePush } from "$lib/services/repo-actions";
  import { toastStore } from "$lib/stores/toast.svelte";
  import CommitRow from "./CommitRow.svelte";
  import GraphLanes from "./GraphLanes.svelte";
  import LoadingState from "../LoadingState.svelte";
  // ConfirmDialog is rendered in AppShell to avoid overflow:hidden clipping
  import type { CommitNode } from "$lib/types";
  import { ROW_HEIGHT } from "$lib/utils/graph-layout";
  import { ArrowDownToLine, ArrowUpFromLine, GitPullRequest, RefreshCw } from "lucide-svelte";
  import { i18n } from "$lib/stores/i18n.svelte";

  let scrollContainer: HTMLDivElement | undefined = $state();
  let isLoadingMore = $state(false);
  let actionInProgress = $state<string | null>(null);

  let behind = $derived(repoStore.status?.behind ?? 0);
  let ahead = $derived(repoStore.status?.ahead ?? 0);

  async function refreshAfterAction() {
    const repo = repoStore.activeRepo;
    if (!repo) return;
    const path = repo.repoPath;
    const [status, graph, branches] = await Promise.all([
      getStatus(path),
      getCommitGraph(path, 0, 200),
      getBranches(path),
    ]);
    repo.status = status;
    repo.commitGraph = graph;
    repo.branches = branches;
  }

  async function handleFetch() {
    const repo = repoStore.activeRepo;
    if (!repo || actionInProgress) return;
    actionInProgress = "fetch";
    try {
      await fetchRepo(repo.repoPath);
      await refreshAfterAction();
      toastStore.success(i18n.t.fetchedFromRemote);
    } catch (e) {
      toastStore.error(String(e));
    } finally {
      actionInProgress = null;
    }
  }

  async function handlePull() {
    const repo = repoStore.activeRepo;
    if (!repo || actionInProgress) return;
    actionInProgress = "pull";
    try {
      const result = await pullRepo(repo.repoPath);
      await refreshAfterAction();

      if (result.has_conflicts) {
        uiStore.mergeConflict = {
          files: result.conflict_files,
          repoPath: repo.repoPath,
        };
      } else {
        toastStore.success(i18n.t.pulledFromRemote);
      }
    } catch (e) {
      toastStore.error(String(e));
    } finally {
      actionInProgress = null;
    }
  }

  async function handlePush() {
    const repo = repoStore.activeRepo;
    if (!repo || actionInProgress) return;

    actionInProgress = "push";
    try {
      const status = await checkPushStatus(repo.repoPath);

      if (!status.has_remote) {
        toastStore.error(i18n.t.noRemoteConfigured);
        return;
      }

      if (!status.has_upstream) {
        // Show confirm dialog (rendered in AppShell, outside grid overflow)
        uiStore.pushConfirm = { branch: status.branch_name, remote: status.remote_name };
        return;
      }

      // Normal push
      await executePush(status.remote_name, status.branch_name, false);
    } catch (e) {
      toastStore.error(String(e));
    } finally {
      actionInProgress = null;
    }
  }

  let nodes = $derived(repoStore.commitGraph?.nodes ?? []);
  let maxColumn = $derived(
    nodes.length > 0 ? Math.max(...nodes.map((n) => n.column)) + 1 : 1,
  );

  async function handleSelectCommit(commit: CommitNode) {
    const repo = repoStore.activeRepo;
    if (!repo) return;

    if (repoStore.selectedCommit?.oid === commit.oid) {
      repo.selectedCommit = null;
      repo.diff = null;
      uiStore.detailPaneOpen = false;
      return;
    }

    repo.selectedCommit = commit;
    uiStore.openDetailPane();

    try {
      const diff = await getDiff(repo.repoPath, commit.oid);
      repo.diff = diff;
    } catch (e) {
      repo.error = String(e);
    }
  }

  async function handleScroll() {
    if (!scrollContainer || isLoadingMore) return;
    const repo = repoStore.activeRepo;
    if (!repo) return;

    const { scrollTop, scrollHeight, clientHeight } = scrollContainer;
    const nearBottom = scrollHeight - scrollTop - clientHeight < 200;

    if (nearBottom && repo.commitGraph) {
      const loaded = repo.commitGraph.nodes.length;
      if (loaded < repo.commitGraph.total_count) {
        isLoadingMore = true;
        try {
          const more = await getCommitGraph(repo.repoPath, loaded, 200);
          if (repo.commitGraph && more.nodes.length > 0) {
            repo.commitGraph = {
              nodes: [...repo.commitGraph.nodes, ...more.nodes],
              edges: [...repo.commitGraph.edges, ...more.edges],
              total_count: more.total_count,
            };
          }
        } finally {
          isLoadingMore = false;
        }
      }
    }
  }
</script>

<main class="relative flex flex-col overflow-hidden bg-bg-primary">
  <div class="flex items-center justify-between border-b border-border px-3 py-1.5">
    <span class="text-xs font-semibold uppercase tracking-wider text-text-muted">
      {i18n.t.commits}
      {#if repoStore.commitGraph}
        <span class="font-normal">({repoStore.commitGraph.total_count})</span>
      {/if}
    </span>

    <div class="flex items-center gap-1">
      <button onclick={handleFetch} disabled={!!actionInProgress} class="relative flex items-center gap-1 rounded px-2 py-0.5 text-xs text-text-secondary transition-colors hover:bg-bg-hover hover:text-text-primary disabled:opacity-40" title={i18n.t.fetchTooltip}>
        <RefreshCw size={13} class={actionInProgress === "fetch" ? "animate-spin" : ""} />
        <span class="hidden sm:inline">{i18n.t.fetch}</span>
      </button>
      <button onclick={handlePull} disabled={!!actionInProgress} class="relative flex items-center gap-1 rounded px-2 py-0.5 text-xs text-text-secondary transition-colors hover:bg-bg-hover hover:text-text-primary disabled:opacity-40" title={i18n.t.pullTooltip(behind)}>
        <ArrowDownToLine size={13} class={actionInProgress === "pull" ? "animate-pulse" : ""} />
        <span class="hidden sm:inline">{i18n.t.pull}</span>
        {#if behind > 0}
          <span class="absolute -right-0.5 -top-0.5 flex h-4 min-w-4 items-center justify-center rounded-full bg-diff-remove px-1 text-[9px] font-bold text-white">{behind}</span>
        {/if}
      </button>
      <button onclick={handlePush} disabled={!!actionInProgress} class="relative flex items-center gap-1 rounded px-2 py-0.5 text-xs text-text-secondary transition-colors hover:bg-bg-hover hover:text-text-primary disabled:opacity-40" title={i18n.t.pushTooltip(ahead)}>
        <ArrowUpFromLine size={13} class={actionInProgress === "push" ? "animate-pulse" : ""} />
        <span class="hidden sm:inline">{i18n.t.push}</span>
        {#if ahead > 0}
          <span class="absolute -right-0.5 -top-0.5 flex h-4 min-w-4 items-center justify-center rounded-full bg-accent px-1 text-[9px] font-bold text-white">{ahead}</span>
        {/if}
      </button>
    </div>
  </div>

  <div
    bind:this={scrollContainer}
    onscroll={handleScroll}
    class="flex-1 overflow-y-auto"
  >
    {#if nodes.length > 0}
      <div class="relative" style="min-height: {nodes.length * ROW_HEIGHT}px">
        <GraphLanes {nodes} edges={repoStore.commitGraph?.edges ?? []} {maxColumn} />

        {#each nodes as commit (commit.oid)}
          <CommitRow
            {commit}
            selected={repoStore.selectedCommit?.oid === commit.oid}
            graphWidth={maxColumn * 20 + 32}
            onclick={() => handleSelectCommit(commit)}
          />
        {/each}
      </div>
    {:else if repoStore.isLoading}
      <LoadingState message={i18n.t.loadingCommits} />
    {:else}
      <div class="flex items-center justify-center p-8 text-text-muted">
        {i18n.t.noCommitsFound}
      </div>
    {/if}

    {#if isLoadingMore}
      <div class="py-2 text-center text-xs text-text-muted">{i18n.t.loadingMore}</div>
    {/if}
  </div>
</main>

