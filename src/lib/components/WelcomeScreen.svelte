<script lang="ts">
  import { open } from "@tauri-apps/plugin-dialog";
  import { openAndLoadRepo } from "$lib/services/repo-actions";
  import { uiStore } from "$lib/stores/ui.svelte";
  import { i18n } from "$lib/stores/i18n.svelte";
  import { FolderOpen, Download } from "lucide-svelte";
  import logoSvg from "$lib/assets/logo.svg?url";

  let isOpening = $state(false);

  async function handleOpenRepo() {
    const selected = await open({ directory: true, multiple: false });
    if (!selected) return;
    const path = typeof selected === "string" ? selected : selected[0];
    if (!path) return;
    isOpening = true;
    await openAndLoadRepo(path);
    isOpening = false;
  }
</script>

<div class="flex h-full w-full flex-col items-center justify-center bg-bg-primary">
  <div class="text-center">
    <img src={logoSvg} alt={i18n.t.appName} class="mx-auto mb-4 h-28 w-28" />
    <h1 class="mb-2 text-5xl font-bold text-accent">{i18n.t.appName}</h1>
    <p class="mb-8 text-text-secondary">{i18n.t.appSubtitle}</p>

    <div class="flex gap-3 justify-center">
      <button
        onclick={handleOpenRepo}
        disabled={isOpening}
        class="inline-flex items-center gap-2 rounded-lg bg-accent px-6 py-3 font-medium text-white transition-colors hover:bg-accent-hover disabled:opacity-50"
      >
        <FolderOpen size={20} />
        {isOpening ? i18n.t.opening : i18n.t.openRepository}
      </button>

      <button
        onclick={() => (uiStore.cloneDialogOpen = true)}
        class="inline-flex items-center gap-2 rounded-lg border border-border bg-bg-secondary px-6 py-3 font-medium text-text-primary transition-colors hover:bg-bg-hover"
      >
        <Download size={20} />
        {i18n.t.cloneRepository}
      </button>
    </div>
  </div>
</div>
