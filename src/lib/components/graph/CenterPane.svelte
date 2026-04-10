<script lang="ts">
  import { repoStore } from "$lib/stores/repo.svelte";
  import { uiStore } from "$lib/stores/ui.svelte";
  import { getDiff, getCommitGraph, checkPushStatus, fetchRepo, pullRepo, cherryPick, revertCommit, resetToCommit } from "$lib/services/git";
  import { executePush, refreshRepo } from "$lib/services/repo-actions";
  import { toastStore } from "$lib/stores/toast.svelte";
  import CommitRow from "./CommitRow.svelte";
  import GraphLanes from "./GraphLanes.svelte";
  import LoadingState from "../LoadingState.svelte";
  import ContextMenu from "../ContextMenu.svelte";
  import ContextMenuItem from "../ContextMenuItem.svelte";
  import type { CommitNode } from "$lib/types";
  import { ROW_HEIGHT } from "$lib/utils/graph-layout";
  import { ArrowDownToLine, ArrowUpFromLine, RefreshCw, Tag, Cherry, Undo2, RotateCcw, Copy, GitMerge } from "lucide-svelte";
  import { i18n } from "$lib/stores/i18n.svelte";

  let scrollContainer: HTMLDivElement | undefined = $state();
  let isLoadingMore = $state(false);
  let actionInProgress = $state<string | null>(null);
  let commitMenu = $state<{ x: number; y: number; commit: CommitNode } | null>(null);

  let behind = $derived(repoStore.status?.behind ?? 0);
  let ahead = $derived(repoStore.status?.ahead ?? 0);

  function openCommitMenu(e: MouseEvent, commit: CommitNode) {
    e.preventDefault();
    commitMenu = { x: e.clientX, y: e.clientY, commit };
  }

  function closeCommitMenu() {
    commitMenu = null;
  }

  async function handleCherryPick() {
    if (!commitMenu) return;
    const repo = repoStore.activeRepo;
    if (!repo) return;
    const oid = commitMenu.commit.oid;
    closeCommitMenu();
    try {
      const result = await cherryPick(repo.repoPath, oid);
      await refreshRepo();
      if (result.has_conflicts) {
        uiStore.mergeConflict = { files: result.conflict_files, repoPath: repo.repoPath };
      } else {
        toastStore.success(i18n.t.cherryPickApplied);
      }
    } catch (e) {
      toastStore.error(String(e));
    }
  }

  async function handleRevert() {
    if (!commitMenu) return;
    const repo = repoStore.activeRepo;
    if (!repo) return;
    const oid = commitMenu.commit.oid;
    closeCommitMenu();
    try {
      const result = await revertCommit(repo.repoPath, oid);
      await refreshRepo();
      if (result.has_conflicts) {
        uiStore.mergeConflict = { files: result.conflict_files, repoPath: repo.repoPath };
      } else {
        toastStore.success(i18n.t.commitReverted);
      }
    } catch (e) {
      toastStore.error(String(e));
    }
  }

  async function handleReset(mode: string) {
    if (!commitMenu) return;
    const commit = commitMenu.commit;
    if (mode === "hard") {
      uiStore.resetConfirm = { commit: commit.oid, shortOid: commit.short_oid, mode };
      closeCommitMenu();
      return;
    }
    const repo = repoStore.activeRepo;
    if (!repo) return;
    closeCommitMenu();
    try {
      await resetToCommit(repo.repoPath, commit.oid, mode);
      await refreshRepo();
      toastStore.success(i18n.t.resetComplete(mode));
    } catch (e) {
      toastStore.error(String(e));
    }
  }

  function handleCreateTag() {
    if (!commitMenu) return;
    uiStore.createTag = { commit: commitMenu.commit.oid, shortOid: commitMenu.commit.short_oid };
    closeCommitMenu();
  }

  async function handleCopySha() {
    if (!commitMenu) return;
    await navigator.clipboard.writeText(commitMenu.commit.oid);
    toastStore.success(i18n.t.shaCopied);
    closeCommitMenu();
  }

  async function handleFetch() {
    const repo = repoStore.activeRepo;
    if (!repo || actionInProgress) return;
    actionInProgress = "fetch";
    try {
      await fetchRepo(repo.repoPath);
      await refreshRepo();
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
      await refreshRepo();

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
      <button onclick={() => (uiStore.mergeDialogOpen = {})} class="flex items-center gap-1 rounded px-2 py-0.5 text-xs text-text-secondary transition-colors hover:bg-bg-hover hover:text-text-primary" title={i18n.t.mergeBranches}>
        <GitMerge size={13} />
        {i18n.t.mergeAction}
      </button>
      <div class="mx-0.5 h-4 w-px bg-border"></div>
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
            oncontextmenu={(e) => openCommitMenu(e, commit)}
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

  {#if commitMenu}
    <ContextMenu x={commitMenu.x} y={commitMenu.y} onclose={closeCommitMenu}>
      <div class="px-3 py-1 text-[10px] font-semibold uppercase tracking-wider text-text-muted">{commitMenu.commit.short_oid}</div>
      <div class="my-1 border-t border-border"></div>
      <ContextMenuItem label={i18n.t.createTagLabel} icon={Tag} onclick={handleCreateTag} />
      <ContextMenuItem label={i18n.t.cherryPickCommit} icon={Cherry} onclick={handleCherryPick} />
      <ContextMenuItem label={i18n.t.revertCommitLabel} icon={Undo2} onclick={handleRevert} />
      <div class="my-1 border-t border-border"></div>
      <ContextMenuItem label={i18n.t.resetSoft} icon={RotateCcw} onclick={() => handleReset("soft")} />
      <ContextMenuItem label={i18n.t.resetMixed} icon={RotateCcw} onclick={() => handleReset("mixed")} />
      <ContextMenuItem label={i18n.t.resetHard} icon={RotateCcw} onclick={() => handleReset("hard")} danger />
      <div class="my-1 border-t border-border"></div>
      <ContextMenuItem label={i18n.t.copySha} icon={Copy} onclick={handleCopySha} />
    </ContextMenu>
  {/if}
</main>

