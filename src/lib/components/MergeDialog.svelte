<script lang="ts">
  import { mergeBranch } from "$lib/services/git";
  import { refreshRepo } from "$lib/services/repo-actions";
  import { repoStore } from "$lib/stores/repo.svelte";
  import { uiStore } from "$lib/stores/ui.svelte";
  import { toastStore } from "$lib/stores/toast.svelte";
  import { i18n } from "$lib/stores/i18n.svelte";
  import { X, GitMerge } from "lucide-svelte";

  let selectedSource = $state(uiStore.mergeDialogOpen?.preselectedBranch ?? "");
  let isMerging = $state(false);

  let currentBranch = $derived(repoStore.currentBranch ?? "");
  let localBranches = $derived(
    repoStore.branches
      .filter((b) => b.kind === "localbranch" && !b.is_head)
      .map((b) => b.name),
  );
  let remoteBranches = $derived(
    repoStore.branches
      .filter((b) => b.kind === "remotebranch")
      .map((b) => b.name),
  );
  let allSourceBranches = $derived([...localBranches, ...remoteBranches]);

  function close() {
    uiStore.mergeDialogOpen = null;
    selectedSource = "";
  }

  async function handleMerge() {
    const repo = repoStore.activeRepo;
    if (!repo || !selectedSource) return;

    isMerging = true;
    try {
      const result = await mergeBranch(repo.repoPath, selectedSource);
      await refreshRepo();

      if (result.has_conflicts) {
        uiStore.mergeConflict = { files: result.conflict_files, repoPath: repo.repoPath };
      } else {
        toastStore.success(i18n.t.mergeStarted(selectedSource));
      }
      close();
    } catch (e) {
      toastStore.error(String(e));
    } finally {
      isMerging = false;
    }
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="fixed inset-0 z-50 flex items-center justify-center bg-black/50"
  onkeydown={(e) => e.key === "Escape" && close()}
>
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div
    class="w-[480px] rounded-lg border border-border bg-bg-secondary shadow-2xl flex flex-col"
    onclick={(e) => e.stopPropagation()}
  >
    <!-- Header -->
    <div class="flex items-center justify-between border-b border-border px-4 py-2">
      <div class="flex items-center gap-2">
        <GitMerge size={16} class="text-accent" />
        <span class="text-sm font-semibold text-text-primary">{i18n.t.mergeBranches}</span>
      </div>
      <button onclick={close} class="rounded p-1 text-text-secondary hover:bg-bg-hover hover:text-text-primary">
        <X size={14} />
      </button>
    </div>

    <!-- Body -->
    <div class="p-4 space-y-4">
      <!-- Source branch selector -->
      <div>
        <label class="block text-xs font-medium text-text-secondary mb-1.5">{i18n.t.mergeFrom}</label>
        <select
          bind:value={selectedSource}
          class="w-full rounded border border-border bg-bg-primary px-3 py-2 text-sm text-text-primary focus:border-accent focus:outline-none appearance-none cursor-pointer"
        >
          <option value="" disabled>{i18n.t.selectBranch}</option>
          {#if localBranches.length > 0}
            <optgroup label="Local">
              {#each localBranches as branch}
                <option value={branch}>{branch}</option>
              {/each}
            </optgroup>
          {/if}
          {#if remoteBranches.length > 0}
            <optgroup label="Remote">
              {#each remoteBranches as branch}
                <option value={branch}>{branch}</option>
              {/each}
            </optgroup>
          {/if}
        </select>
      </div>

      <!-- Target (current branch, read-only) -->
      <div>
        <label class="block text-xs font-medium text-text-secondary mb-1.5">{i18n.t.mergeInto}</label>
        <div class="flex items-center gap-2 w-full rounded border border-border bg-bg-tertiary px-3 py-2 text-sm text-text-primary">
          <span class="inline-block h-2 w-2 rounded-full bg-accent"></span>
          <span class="font-medium">{currentBranch}</span>
        </div>
      </div>

      <!-- Description -->
      {#if selectedSource}
        <div class="rounded border border-border bg-bg-primary px-3 py-2">
          <p class="text-xs text-text-secondary">{i18n.t.mergeDescription(selectedSource, currentBranch)}</p>
        </div>
      {/if}
    </div>

    <!-- Footer -->
    <div class="flex items-center justify-end gap-2 border-t border-border px-4 py-2">
      <button onclick={close} class="rounded px-3 py-1.5 text-sm text-text-secondary hover:bg-bg-hover">
        {i18n.t.cancel}
      </button>
      <button
        onclick={handleMerge}
        disabled={!selectedSource || isMerging}
        class="inline-flex items-center gap-1.5 rounded bg-accent px-3 py-1.5 text-sm font-medium text-white hover:bg-accent-hover disabled:opacity-50"
      >
        <GitMerge size={14} />
        {isMerging ? i18n.t.merging : i18n.t.mergeAction}
      </button>
    </div>
  </div>
</div>
