<script lang="ts">
  import { getVersion } from "@tauri-apps/api/app";
  import { check } from "@tauri-apps/plugin-updater";
  import { relaunch } from "@tauri-apps/plugin-process";
  import { uiStore } from "$lib/stores/ui.svelte";
  import { i18n } from "$lib/stores/i18n.svelte";
  import { toastStore } from "$lib/stores/toast.svelte";
  import { availableLocales, localeNames, type Locale } from "$lib/i18n";
  import { X, Globe, Info, RefreshCw, Download } from "lucide-svelte";
  import logoSvg from "$lib/assets/logo.svg";

  let activeTab = $state<"general" | "about">("general");
  let appVersion = $state("...");
  let checking = $state(false);
  let updateInfo = $state<{ version: string; body: string } | null>(null);
  let isUpdating = $state(false);

  $effect(() => {
    getVersion().then((v) => (appVersion = v));
  });

  async function checkForUpdates() {
    checking = true;
    updateInfo = null;
    try {
      const update = await check();
      if (update) {
        updateInfo = { version: update.version, body: update.body ?? "" };
      } else {
        toastStore.success(i18n.t.upToDate);
      }
    } catch {
      toastStore.error(i18n.t.updateFailed);
    } finally {
      checking = false;
    }
  }

  async function handleUpdate() {
    isUpdating = true;
    try {
      const update = await check();
      if (update) {
        await update.downloadAndInstall();
        toastStore.success(i18n.t.updateRestart);
        setTimeout(() => relaunch(), 1500);
      }
    } catch (e) {
      toastStore.error(`${i18n.t.updateFailed}: ${e}`);
    } finally {
      isUpdating = false;
    }
  }

  const tabs = [
    { id: "general" as const, label: () => i18n.t.general, icon: Globe },
    { id: "about" as const, label: () => i18n.t.about, icon: Info },
  ];
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="fixed inset-0 z-50 flex items-center justify-center bg-black/50"
  onkeydown={(e) => e.key === "Escape" && (uiStore.settingsOpen = false)}
>
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div
    class="w-[480px] rounded-lg border border-border bg-bg-secondary shadow-2xl flex flex-col"
    onclick={(e) => e.stopPropagation()}
  >
    <!-- Header -->
    <div class="flex items-center justify-between border-b border-border px-4 py-2">
      <span class="text-sm font-semibold text-text-primary">{i18n.t.settings}</span>
      <button
        onclick={() => (uiStore.settingsOpen = false)}
        class="rounded p-1 text-text-secondary hover:bg-bg-hover hover:text-text-primary"
      >
        <X size={14} />
      </button>
    </div>

    <!-- Tabs -->
    <div class="flex border-b border-border px-4 gap-1">
      {#each tabs as tab}
        {@const Icon = tab.icon}
        <button
          onclick={() => (activeTab = tab.id)}
          class="flex items-center gap-1.5 px-3 py-2 text-xs font-medium transition-colors border-b-2 -mb-px"
          class:border-accent={activeTab === tab.id}
          class:text-accent={activeTab === tab.id}
          class:border-transparent={activeTab !== tab.id}
          class:text-text-secondary={activeTab !== tab.id}
          class:hover:text-text-primary={activeTab !== tab.id}
        >
          <Icon size={13} />
          {tab.label()}
        </button>
      {/each}
    </div>

    <!-- Content -->
    <div class="p-4 space-y-4 min-h-[180px]">
      {#if activeTab === "general"}
        <!-- Language -->
        <div>
          <label class="flex items-center gap-2 text-xs font-medium text-text-secondary mb-2">
            <Globe size={14} />
            {i18n.t.language}
          </label>
          <div class="grid grid-cols-3 gap-2">
            {#each availableLocales as locale}
              <button
                onclick={() => i18n.setLocale(locale)}
                class="rounded border px-3 py-2 text-sm transition-colors"
                class:border-accent={i18n.locale === locale}
                class:bg-accent={i18n.locale === locale}
                class:text-white={i18n.locale === locale}
                class:border-border={i18n.locale !== locale}
                class:bg-bg-primary={i18n.locale !== locale}
                class:text-text-primary={i18n.locale !== locale}
                class:hover:bg-bg-hover={i18n.locale !== locale}
              >
                {localeNames[locale]}
              </button>
            {/each}
          </div>
        </div>
      {:else if activeTab === "about"}
        <!-- Version info -->
        <div class="flex flex-col items-center gap-4 py-2">
          <div class="flex items-center gap-3">
            <img src={logoSvg} alt="GitFerrum" class="h-12 w-12" />
            <div>
              <h3 class="text-sm font-semibold text-text-primary">{i18n.t.appName}</h3>
              <p class="text-xs text-text-secondary">{i18n.t.appSubtitle}</p>
            </div>
          </div>

          <div class="flex items-center gap-2 rounded-md border border-border bg-bg-primary px-4 py-2">
            <span class="text-xs text-text-secondary">{i18n.t.version}</span>
            <span class="text-sm font-mono font-semibold text-text-primary">{appVersion}</span>
          </div>

          {#if updateInfo}
            <div class="w-full rounded-md border border-accent/30 bg-accent/5 p-3">
              <p class="text-sm text-text-primary mb-2">
                {i18n.t.updateMessage(updateInfo.version)}
              </p>
              {#if updateInfo.body}
                <div class="max-h-[100px] overflow-y-auto rounded border border-border bg-bg-primary p-2 text-xs text-text-secondary mb-3">
                  {updateInfo.body}
                </div>
              {/if}
              <button
                onclick={handleUpdate}
                disabled={isUpdating}
                class="inline-flex items-center gap-1.5 rounded bg-accent px-3 py-1.5 text-sm font-medium text-white hover:bg-accent-hover disabled:opacity-50"
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
          {:else}
            <button
              onclick={checkForUpdates}
              disabled={checking}
              class="inline-flex items-center gap-1.5 rounded border border-border bg-bg-primary px-3 py-1.5 text-sm text-text-primary hover:bg-bg-hover disabled:opacity-50"
            >
              {#if checking}
                <RefreshCw size={14} class="animate-spin" />
                {i18n.t.checkingUpdates}
              {:else}
                <RefreshCw size={14} />
                {i18n.t.checkForUpdates}
              {/if}
            </button>
          {/if}
        </div>
      {/if}
    </div>
  </div>
</div>
