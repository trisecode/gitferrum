<script lang="ts">
  import { createTag } from "$lib/services/git";
  import { refreshRepo } from "$lib/services/repo-actions";
  import { repoStore } from "$lib/stores/repo.svelte";
  import { uiStore } from "$lib/stores/ui.svelte";
  import { toastStore } from "$lib/stores/toast.svelte";
  import { i18n } from "$lib/stores/i18n.svelte";
  import { X, Tag } from "lucide-svelte";

  let tagName = $state("");
  let tagMessage = $state("");
  let isCreating = $state(false);

  function close() {
    uiStore.createTag = null;
  }

  async function handleCreate() {
    const info = uiStore.createTag;
    const repo = repoStore.activeRepo;
    if (!info || !repo || !tagName.trim()) return;

    isCreating = true;
    try {
      await createTag(repo.repoPath, tagName.trim(), info.commit, tagMessage.trim() || undefined);
      await refreshRepo();
      toastStore.success(i18n.t.tagCreated(tagName.trim()));
      close();
    } catch (e) {
      toastStore.error(String(e));
    } finally {
      isCreating = false;
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
    class="w-[420px] rounded-lg border border-border bg-bg-secondary shadow-2xl flex flex-col"
    onclick={(e) => e.stopPropagation()}
  >
    <div class="flex items-center justify-between border-b border-border px-4 py-2">
      <span class="text-sm font-semibold text-text-primary">{i18n.t.createTagLabel}</span>
      <button onclick={close} class="rounded p-1 text-text-secondary hover:bg-bg-hover hover:text-text-primary">
        <X size={14} />
      </button>
    </div>

    <div class="p-4 space-y-3">
      {#if uiStore.createTag}
        <p class="text-xs text-text-muted">Commit: <span class="font-mono text-accent">{uiStore.createTag.shortOid}</span></p>
      {/if}
      <div>
        <label class="block text-xs font-medium text-text-secondary mb-1">{i18n.t.tagName}</label>
        <input
          bind:value={tagName}
          onkeydown={(e) => e.key === "Enter" && handleCreate()}
          placeholder="v1.0.0"
          class="w-full rounded border border-border bg-bg-primary px-3 py-2 text-sm text-text-primary placeholder:text-text-muted focus:border-accent focus:outline-none"
          autofocus
        />
      </div>
      <div>
        <label class="block text-xs font-medium text-text-secondary mb-1">{i18n.t.tagMessage}</label>
        <textarea
          bind:value={tagMessage}
          placeholder={i18n.t.tagMessagePlaceholder}
          rows={2}
          class="w-full resize-none rounded border border-border bg-bg-primary px-3 py-2 text-sm text-text-primary placeholder:text-text-muted focus:border-accent focus:outline-none"
        ></textarea>
      </div>
    </div>

    <div class="flex items-center justify-end gap-2 border-t border-border px-4 py-2">
      <button onclick={close} class="rounded px-3 py-1.5 text-sm text-text-secondary hover:bg-bg-hover">{i18n.t.cancel}</button>
      <button
        onclick={handleCreate}
        disabled={!tagName.trim() || isCreating}
        class="inline-flex items-center gap-1 rounded bg-accent px-3 py-1.5 text-sm font-medium text-white hover:bg-accent-hover disabled:opacity-50"
      >
        <Tag size={14} />
        {i18n.t.create}
      </button>
    </div>
  </div>
</div>
