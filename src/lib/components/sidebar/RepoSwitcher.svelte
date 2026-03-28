<script lang="ts">
  import { repoStore } from "$lib/stores/repo.svelte";
  import { closeAndRemoveRepo } from "$lib/services/repo-actions";
  import { open } from "@tauri-apps/plugin-dialog";
  import { openAndLoadRepo } from "$lib/services/repo-actions";
  import { uiStore } from "$lib/stores/ui.svelte";
  import { i18n } from "$lib/stores/i18n.svelte";
  import { X, FolderGit2, Plus, Download } from "lucide-svelte";

  async function handleOpenNew() {
    const selected = await open({ directory: true, multiple: false });
    if (!selected) return;
    const path = typeof selected === "string" ? selected : selected[0];
    if (!path) return;
    await openAndLoadRepo(path);
  }

  async function handleClose(e: Event, path: string) {
    e.stopPropagation();
    await closeAndRemoveRepo(path);
  }
</script>

<div class="border-b border-border">
  <div class="flex items-center justify-between px-3 py-1.5">
    <span class="text-xs font-semibold uppercase tracking-wider text-text-muted">{i18n.t.repositories}</span>
    <div class="flex items-center gap-0.5">
      <button
        onclick={() => (uiStore.cloneDialogOpen = true)}
        class="rounded p-1 text-text-secondary transition-colors hover:bg-bg-hover hover:text-accent"
        title={i18n.t.cloneRepoTooltip}
      >
        <Download size={14} />
      </button>
      <button
        onclick={handleOpenNew}
        class="rounded p-1 text-text-secondary transition-colors hover:bg-bg-hover hover:text-accent"
        title={i18n.t.openRepoTooltip}
      >
        <Plus size={14} />
      </button>
    </div>
  </div>

  <div class="pb-1">
    {#each repoStore.openRepos as repo (repo.repoPath)}
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <div
        onclick={() => repoStore.switchTo(repo.repoPath)}
        class="group flex w-full cursor-pointer items-center gap-2 px-3 py-1.5 text-sm transition-colors hover:bg-bg-hover"
        class:bg-bg-tertiary={repoStore.activeRepoId === repo.repoPath}
        title={repo.repoPath}
      >
        <FolderGit2 size={14} class={repoStore.activeRepoId === repo.repoPath ? "text-accent" : "text-text-muted"} />
        <span
          class="flex-1 truncate text-left"
          class:text-accent={repoStore.activeRepoId === repo.repoPath}
          class:font-medium={repoStore.activeRepoId === repo.repoPath}
          class:text-text-primary={repoStore.activeRepoId !== repo.repoPath}
        >
          {repo.displayName}
        </span>
        {#if repo.isLoading}
          <span class="text-[10px] text-text-muted">...</span>
        {/if}
        <button
          onclick={(e) => handleClose(e, repo.repoPath)}
          class="hidden rounded p-0.5 text-text-muted hover:bg-bg-hover hover:text-diff-remove group-hover:block"
          title={i18n.t.closeRepoTooltip}
        >
          <X size={12} />
        </button>
      </div>
    {:else}
      <p class="px-3 py-2 text-xs text-text-muted">{i18n.t.noReposOpen}</p>
    {/each}
  </div>
</div>
