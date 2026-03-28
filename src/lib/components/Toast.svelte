<script lang="ts">
  import { toastStore } from "$lib/stores/toast.svelte";
  import { X, AlertCircle, CheckCircle, Info } from "lucide-svelte";
</script>

{#if toastStore.items.length > 0}
  <div class="fixed bottom-12 right-4 z-[100] flex flex-col gap-2">
    {#each toastStore.items as toast (toast.id)}
      <div
        class="flex items-start gap-2 rounded-lg border px-4 py-3 shadow-lg max-w-[400px] animate-slide-in"
        class:border-diff-remove={toast.type === "error"}
        class:bg-diff-remove-bg={toast.type === "error"}
        class:border-diff-add={toast.type === "success"}
        class:bg-diff-add-bg={toast.type === "success"}
        class:border-border={toast.type === "info"}
        class:bg-bg-secondary={toast.type === "info"}
      >
        {#if toast.type === "error"}
          <AlertCircle size={16} class="shrink-0 mt-0.5 text-diff-remove" />
        {:else if toast.type === "success"}
          <CheckCircle size={16} class="shrink-0 mt-0.5 text-diff-add" />
        {:else}
          <Info size={16} class="shrink-0 mt-0.5 text-text-secondary" />
        {/if}
        <span class="flex-1 text-sm text-text-primary">{toast.message}</span>
        <button
          onclick={() => toastStore.dismiss(toast.id)}
          class="shrink-0 rounded p-0.5 text-text-muted hover:text-text-primary"
        >
          <X size={12} />
        </button>
      </div>
    {/each}
  </div>
{/if}

<style>
  @keyframes slide-in {
    from {
      transform: translateX(100%);
      opacity: 0;
    }
    to {
      transform: translateX(0);
      opacity: 1;
    }
  }
  .animate-slide-in {
    animation: slide-in 0.2s ease-out;
  }
</style>
