<script lang="ts">
  import { uiStore } from "$lib/stores/ui.svelte";
  import { i18n } from "$lib/stores/i18n.svelte";
  import { availableLocales, localeNames, type Locale } from "$lib/i18n";
  import { X, Globe } from "lucide-svelte";
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="fixed inset-0 z-50 flex items-center justify-center bg-black/50"
  onkeydown={(e) => e.key === "Escape" && (uiStore.settingsOpen = false)}
>
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div
    class="w-[420px] rounded-lg border border-border bg-bg-secondary shadow-2xl flex flex-col"
    onclick={(e) => e.stopPropagation()}
  >
    <div class="flex items-center justify-between border-b border-border px-4 py-2">
      <span class="text-sm font-semibold text-text-primary">{i18n.t.settings}</span>
      <button
        onclick={() => (uiStore.settingsOpen = false)}
        class="rounded p-1 text-text-secondary hover:bg-bg-hover hover:text-text-primary"
      >
        <X size={14} />
      </button>
    </div>

    <div class="p-4 space-y-4">
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
    </div>
  </div>
</div>
