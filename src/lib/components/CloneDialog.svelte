<script lang="ts">
  import { open } from "@tauri-apps/plugin-dialog";
  import { cloneAndOpenRepo } from "$lib/services/repo-actions";
  import { uiStore } from "$lib/stores/ui.svelte";
  import { i18n } from "$lib/stores/i18n.svelte";
  import { X, Download, FolderOpen } from "lucide-svelte";

  let url = $state("");
  let dest = $state("");
  let isCloning = $state(false);

  async function handlePickDest() {
    const selected = await open({ directory: true, multiple: false });
    if (!selected) return;
    const path = typeof selected === "string" ? selected : selected[0];
    if (!path) return;
    const repoName = url.replace(/\.git$/, "").split("/").pop()?.trim();
    dest = repoName ? `${path}/${repoName}` : path;
  }

  async function handleClone() {
    if (!url.trim() || !dest.trim()) return;
    isCloning = true;
    await cloneAndOpenRepo(url.trim(), dest.trim());
    isCloning = false;
    if (!isCloning) {
      url = "";
      dest = "";
      uiStore.cloneDialogOpen = false;
    }
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="fixed inset-0 z-50 flex items-center justify-center bg-black/50"
  onkeydown={(e) => e.key === "Escape" && (uiStore.cloneDialogOpen = false)}
>
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div
    class="w-[520px] rounded-lg border border-border bg-bg-secondary shadow-2xl flex flex-col"
    onclick={(e) => e.stopPropagation()}
  >
    <div class="flex items-center justify-between border-b border-border px-4 py-2">
      <span class="text-sm font-semibold text-text-primary">{i18n.t.cloneRepository}</span>
      <button onclick={() => (uiStore.cloneDialogOpen = false)} class="rounded p-1 text-text-secondary hover:bg-bg-hover hover:text-text-primary">
        <X size={14} />
      </button>
    </div>

    <div class="p-4 space-y-3">
      <div>
        <label class="block text-xs font-medium text-text-secondary mb-1">{i18n.t.repositoryUrl}</label>
        <input bind:value={url} placeholder={i18n.t.urlPlaceholder} class="w-full rounded border border-border bg-bg-primary px-3 py-2 text-sm text-text-primary placeholder:text-text-muted focus:border-accent focus:outline-none" />
      </div>
      <div>
        <label class="block text-xs font-medium text-text-secondary mb-1">{i18n.t.cloneTo}</label>
        <div class="flex gap-2">
          <input bind:value={dest} placeholder={i18n.t.destPlaceholder} class="flex-1 rounded border border-border bg-bg-primary px-3 py-2 text-sm text-text-primary placeholder:text-text-muted focus:border-accent focus:outline-none" />
          <button onclick={handlePickDest} class="rounded border border-border bg-bg-primary px-3 py-2 text-text-secondary hover:bg-bg-hover hover:text-text-primary" title={i18n.t.browse}>
            <FolderOpen size={16} />
          </button>
        </div>
      </div>
    </div>

    <div class="flex items-center justify-end gap-2 border-t border-border px-4 py-2">
      <button onclick={() => (uiStore.cloneDialogOpen = false)} class="rounded px-3 py-1.5 text-sm text-text-secondary hover:bg-bg-hover">{i18n.t.cancel}</button>
      <button
        onclick={handleClone}
        disabled={!url.trim() || !dest.trim() || isCloning}
        class="inline-flex items-center gap-1 rounded bg-accent px-3 py-1.5 text-sm font-medium text-white hover:bg-accent-hover disabled:opacity-50"
      >
        <Download size={14} />
        {isCloning ? i18n.t.cloning : i18n.t.cloneRepository}
      </button>
    </div>
  </div>
</div>
