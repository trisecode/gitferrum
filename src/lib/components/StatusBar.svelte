<script lang="ts">
  import { repoStore } from "$lib/stores/repo.svelte";
  import { uiStore } from "$lib/stores/ui.svelte";
  import { i18n } from "$lib/stores/i18n.svelte";
  import { getRepoState, abortMerge, getStatus, getCommitGraph, getBranches } from "$lib/services/git";
  import { toastStore } from "$lib/stores/toast.svelte";
  import { GitBranch, PanelRight, X } from "lucide-svelte";

  let repoState = $state("normal");

  // Poll repo state when status changes
  $effect(() => {
    const repo = repoStore.activeRepo;
    if (!repo?.repoPath) {
      repoState = "normal";
      return;
    }
    getRepoState(repo.repoPath).then((s) => (repoState = s)).catch(() => {});
  });

  const stateLabels: Record<string, string> = {
    merge: "repoStateMerging",
    rebase: "repoStateRebasing",
    "cherry-pick": "repoStateCherryPick",
    revert: "repoStateReverting",
  };

  async function handleAbortMerge() {
    const repo = repoStore.activeRepo;
    if (!repo) return;
    try {
      await abortMerge(repo.repoPath);
      repoState = "normal";
      const [status, graph, branches] = await Promise.all([
        getStatus(repo.repoPath),
        getCommitGraph(repo.repoPath, 0, 200),
        getBranches(repo.repoPath),
      ]);
      repo.status = status;
      repo.commitGraph = graph;
      repo.branches = branches;
    } catch (e) {
      toastStore.error(String(e));
    }
  }
</script>

<footer class="col-span-full flex items-center justify-between border-t border-border bg-bg-secondary px-3 py-1 text-xs text-text-secondary">
  <div class="flex items-center gap-3">
    {#if repoStore.currentBranch}
      <span class="flex items-center gap-1">
        <GitBranch size={12} />
        <span class="font-medium text-text-primary">{repoStore.currentBranch}</span>
      </span>
    {/if}

    {#if repoState !== "normal"}
      <span class="flex items-center gap-1 rounded bg-yellow-400/20 px-2 py-0.5 text-[10px] font-bold text-yellow-400">
        {i18n.t[stateLabels[repoState] as keyof typeof i18n.t] ?? repoState.toUpperCase()}
        <button onclick={handleAbortMerge} class="ml-1 rounded hover:bg-yellow-400/30" title="Abort">
          <X size={10} />
        </button>
      </span>
    {/if}

    {#if repoStore.status}
      {#if repoStore.status.ahead > 0}
        <span>↑{repoStore.status.ahead}</span>
      {/if}
      {#if repoStore.status.behind > 0}
        <span>↓{repoStore.status.behind}</span>
      {/if}
    {/if}
  </div>

  <div class="flex items-center gap-3">
    {#if repoStore.status}
      <button
        onclick={() => { uiStore.detailPaneOpen = false; uiStore.changesPanelOpen = !uiStore.changesPanelOpen; }}
        class="relative flex items-center gap-1 rounded px-2 py-0.5 transition-colors hover:bg-bg-hover"
        class:text-accent={uiStore.changesPanelOpen}
      >
        <PanelRight size={12} />
        <span>{i18n.t.nChanges(repoStore.status.staged.length + repoStore.status.unstaged.length + repoStore.status.untracked.length)}</span>
        {#if repoStore.status.staged.length + repoStore.status.unstaged.length + repoStore.status.untracked.length > 0}
          <span class="h-1.5 w-1.5 rounded-full bg-accent"></span>
        {/if}
      </button>
    {/if}
  </div>
</footer>
