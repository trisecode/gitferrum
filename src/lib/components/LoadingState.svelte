<script lang="ts">
  import { i18n } from "$lib/stores/i18n.svelte";

  let { message }: { message?: string } = $props();
  let displayMessage = $derived(message ?? i18n.t.loading);
</script>

<div class="flex flex-col items-center justify-center gap-3 p-8">
  <div class="loading-spinner"></div>
  <span class="text-sm text-text-muted">{displayMessage}</span>
</div>

<style>
  .loading-spinner {
    width: 24px;
    height: 24px;
    border: 2px solid var(--color-border);
    border-top-color: var(--color-accent);
    border-radius: 50%;
    animation: spin 0.6s linear infinite;
  }
  @keyframes spin {
    to { transform: rotate(360deg); }
  }
</style>
