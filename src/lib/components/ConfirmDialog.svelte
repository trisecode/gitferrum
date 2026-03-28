<script lang="ts">
  import { i18n } from "$lib/stores/i18n.svelte";
  import { AlertCircle } from "lucide-svelte";

  let {
    title,
    message,
    confirmLabel,
    onconfirm,
    oncancel,
  }: {
    title: string;
    message: string;
    confirmLabel?: string;
    onconfirm: () => void;
    oncancel: () => void;
  } = $props();
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="fixed inset-0 z-50 flex items-center justify-center bg-black/50"
  onkeydown={(e) => e.key === "Escape" && oncancel()}
>
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div
    class="w-[420px] rounded-lg border border-border bg-bg-secondary shadow-2xl"
    onclick={(e) => e.stopPropagation()}
  >
    <div class="flex items-start gap-3 p-4">
      <AlertCircle size={20} class="shrink-0 mt-0.5 text-accent" />
      <div>
        <h3 class="text-sm font-semibold text-text-primary">{title}</h3>
        <p class="mt-1 text-xs text-text-secondary">{message}</p>
      </div>
    </div>

    <div class="flex items-center justify-end gap-2 border-t border-border px-4 py-2">
      <button
        onclick={oncancel}
        class="rounded px-3 py-1.5 text-sm text-text-secondary hover:bg-bg-hover"
      >
        {i18n.t.cancel}
      </button>
      <button
        onclick={onconfirm}
        class="rounded bg-accent px-3 py-1.5 text-sm font-medium text-white hover:bg-accent-hover"
      >
        {confirmLabel ?? "OK"}
      </button>
    </div>
  </div>
</div>
