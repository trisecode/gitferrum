<script lang="ts">
  import type { RefLabel } from "$lib/types";
  import { gitAction, getCommitGraph, getBranches, getStatus } from "$lib/services/git";
  import { repoStore } from "$lib/stores/repo.svelte";
  import { i18n } from "$lib/stores/i18n.svelte";
  import { GitBranch } from "lucide-svelte";

  let { branches }: { branches: RefLabel[] } = $props();

  async function handleCheckout(branch: string) {
    const repo = repoStore.activeRepo;
    if (!repo) return;
    try {
      const path = repo.repoPath;
      await gitAction(path, "checkout", { branch });
      const [status, graph, refs] = await Promise.all([
        getStatus(path),
        getCommitGraph(path, 0, 200),
        getBranches(path),
      ]);
      repo.status = status;
      repo.commitGraph = graph;
      repo.branches = refs;
    } catch (e) {
      repo.error = String(e);
    }
  }
</script>

{#each branches as branch}
  <button
    ondblclick={() => handleCheckout(branch.name)}
    class="flex w-full items-center gap-2 px-4 py-1 text-sm transition-colors hover:bg-bg-hover"
    class:text-accent={branch.is_head}
    class:font-medium={branch.is_head}
    class:text-text-primary={!branch.is_head}
  >
    <GitBranch size={14} class={branch.is_head ? "text-accent" : "text-text-muted"} />
    <span class="truncate">{branch.name}</span>
    {#if branch.is_head}
      <span class="ml-auto text-[10px] text-accent">{i18n.t.head}</span>
    {/if}
  </button>
{:else}
  <p class="px-4 py-2 text-xs text-text-muted">{i18n.t.noBranches}</p>
{/each}
