<script lang="ts">
  import { repoStore } from "$lib/stores/repo.svelte";
  import { uiStore } from "$lib/stores/ui.svelte";
  import CommitDetail from "./CommitDetail.svelte";
  import FileList from "./FileList.svelte";
  import DiffView from "./DiffView.svelte";
  import { X } from "lucide-svelte";
  import { i18n } from "$lib/stores/i18n.svelte";
</script>

<aside class="flex flex-col overflow-hidden border-l border-border bg-bg-secondary">
  <div class="flex items-center justify-between border-b border-border px-3 py-1.5">
    <span class="text-xs font-semibold uppercase tracking-wider text-text-muted">{i18n.t.details}</span>
    <button
      onclick={() => {
        uiStore.detailPaneOpen = false;
        if (repoStore.activeRepo) {
          repoStore.activeRepo.selectedCommit = null;
          repoStore.activeRepo.diff = null;
        }
      }}
      class="rounded p-1 text-text-secondary transition-colors hover:bg-bg-hover hover:text-text-primary"
    >
      <X size={14} />
    </button>
  </div>

  <div class="flex-1 overflow-y-auto">
    {#if repoStore.selectedCommit}
      <CommitDetail commit={repoStore.selectedCommit} />
    {/if}

    {#if repoStore.diff}
      <FileList files={repoStore.diff.files} stats={repoStore.diff.stats} />
      <DiffView files={repoStore.diff.files} />
    {/if}
  </div>
</aside>
