<script lang="ts">
  import { check } from "@tauri-apps/plugin-updater";
  import { relaunch } from "@tauri-apps/plugin-process";
  import { i18n } from "$lib/stores/i18n.svelte";
  import { toastStore } from "$lib/stores/toast.svelte";
  import { Download, X, RefreshCw } from "lucide-svelte";

  let updateInfo = $state<{ version: string; body: string } | null>(null);
  let isUpdating = $state(false);
  let showDialog = $state(false);

  // Check for updates on mount (silently)
  $effect(() => {
    checkForUpdates(false);
  });

  async function checkForUpdates(showNoUpdate: boolean) {
    try {
      const update = await check();
      if (update) {
        updateInfo = { version: update.version, body: update.body ?? "" };
        showDialog = true;
      } else if (showNoUpdate) {
        toastStore.success(i18n.t.upToDate);
      }
    } catch {
      // Silently ignore update check failures (offline, no endpoint, dev mode)
    }
  }

  async function handleUpdate() {
    if (!updateInfo) return;
    isUpdating = true;
    try {
      const update = await check();
      if (update) {
        await update.downloadAndInstall();
        toastStore.success(i18n.t.updateRestart);
        // Relaunch after a short delay
        setTimeout(() => relaunch(), 1500);
      }
    } catch (e) {
      toastStore.error(`${i18n.t.updateFailed}: ${e}`);
    } finally {
      isUpdating = false;
    }
  }

  function dismiss() {
    showDialog = false;
  }

  // Export for settings to trigger manual check
  export function manualCheck() {
    checkForUpdates(true);
  }
</script>

{#if showDialog && updateInfo}
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="fixed inset-0 z-[200] flex items-center justify-center bg-black/50">
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <div
      class="w-[420px] rounded-lg border border-border bg-bg-secondary shadow-2xl"
      onclick={(e) => e.stopPropagation()}
    >
      <div class="flex items-center gap-2 border-b border-border px-4 py-3">
        <Download size={18} class="text-accent" />
        <span class="text-sm font-semibold text-text-primary">{i18n.t.updateAvailable}</span>
        <div class="flex-1"></div>
        <button onclick={dismiss} class="rounded p-1 text-text-secondary hover:bg-bg-hover">
          <X size={14} />
        </button>
      </div>

      <div class="p-4">
        <p class="mb-3 text-sm text-text-primary">
          {i18n.t.updateMessage(updateInfo.version)}
        </p>
        {#if updateInfo.body}
          <div class="max-h-[150px] overflow-y-auto rounded border border-border bg-bg-primary p-3 text-xs text-text-secondary">
            {updateInfo.body}
          </div>
        {/if}
      </div>

      <div class="flex items-center justify-end gap-2 border-t border-border px-4 py-2">
        <button
          onclick={dismiss}
          disabled={isUpdating}
          class="rounded px-3 py-1.5 text-sm text-text-secondary hover:bg-bg-hover"
        >
          {i18n.t.updateLater}
        </button>
        <button
          onclick={handleUpdate}
          disabled={isUpdating}
          class="inline-flex items-center gap-1 rounded bg-accent px-3 py-1.5 text-sm font-medium text-white hover:bg-accent-hover disabled:opacity-50"
        >
          {#if isUpdating}
            <RefreshCw size={14} class="animate-spin" />
            {i18n.t.updating}
          {:else}
            <Download size={14} />
            {i18n.t.updateNow}
          {/if}
        </button>
      </div>
    </div>
  </div>
{/if}
