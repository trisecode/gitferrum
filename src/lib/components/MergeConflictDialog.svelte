<script lang="ts">
  import { uiStore } from "$lib/stores/ui.svelte";
  import { i18n } from "$lib/stores/i18n.svelte";
  import { abortMerge, getStatus, getCommitGraph, getBranches } from "$lib/services/git";
  import { repoStore } from "$lib/stores/repo.svelte";
  import { toastStore } from "$lib/stores/toast.svelte";
  import { X, AlertTriangle, FileWarning } from "lucide-svelte";

  let conflict = $derived(uiStore.mergeConflict);

  async function handleAbort() {
    if (!conflict) return;
    try {
      await abortMerge(conflict.repoPath);
      uiStore.mergeConflict = null;
      // Refresh repo state
      const repo = repoStore.activeRepo;
      if (repo) {
        const [status, graph, branches] = await Promise.all([
          getStatus(repo.repoPath),
          getCommitGraph(repo.repoPath, 0, 200),
          getBranches(repo.repoPath),
        ]);
        repo.status = status;
        repo.commitGraph = graph;
        repo.branches = branches;
      }
      toastStore.success(i18n.t.abortMerge + " — OK");
    } catch (e) {
      toastStore.error(String(e));
    }
  }

  function handleResolve() {
    uiStore.mergeConflict = null;
    // User resolves manually, stages files, and commits
  }
</script>

{#if conflict}
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <div
      class="w-[480px] max-h-[70vh] rounded-lg border border-border bg-bg-secondary shadow-2xl flex flex-col"
      onclick={(e) => e.stopPropagation()}
    >
      <div class="flex items-center gap-2 border-b border-border px-4 py-3">
        <AlertTriangle size={18} class="text-yellow-400" />
        <span class="text-sm font-semibold text-text-primary">{i18n.t.mergeConflictsTitle}</span>
        <div class="flex-1"></div>
        <button
          onclick={handleResolve}
          class="rounded p-1 text-text-secondary hover:bg-bg-hover hover:text-text-primary"
        >
          <X size={14} />
        </button>
      </div>

      <div class="p-4">
        <p class="mb-3 text-xs text-text-secondary">
          {i18n.t.mergeConflictsMessage(conflict.files.length)}
        </p>

        <div class="mb-3">
          <span class="text-xs font-medium text-text-muted">{i18n.t.conflictFiles}:</span>
          <div class="mt-1 max-h-[200px] overflow-y-auto rounded border border-border bg-bg-primary">
            {#each conflict.files as file}
              <div class="flex items-center gap-2 px-3 py-1.5 text-xs border-b border-border last:border-0">
                <FileWarning size={12} class="text-yellow-400 shrink-0" />
                <span class="truncate text-text-primary font-mono">{file}</span>
              </div>
            {/each}
          </div>
        </div>
      </div>

      <div class="flex items-center justify-end gap-2 border-t border-border px-4 py-2">
        <button
          onclick={handleAbort}
          class="rounded border border-diff-remove px-3 py-1.5 text-xs font-medium text-diff-remove hover:bg-diff-remove-bg"
        >
          {i18n.t.abortMerge}
        </button>
        <button
          onclick={handleResolve}
          class="rounded bg-accent px-3 py-1.5 text-xs font-medium text-white hover:bg-accent-hover"
        >
          {i18n.t.resolveManually}
        </button>
      </div>
    </div>
  </div>
{/if}
