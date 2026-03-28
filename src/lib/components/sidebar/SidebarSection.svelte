<script lang="ts">
  import { ChevronDown, ChevronRight } from "lucide-svelte";
  import type { Snippet } from "svelte";

  let { title, count = 0, children }: { title: string; count?: number; children: Snippet } = $props();
  let expanded = $state(true);
</script>

<div class="border-b border-border">
  <button
    onclick={() => (expanded = !expanded)}
    class="flex w-full items-center gap-1 px-3 py-1.5 text-xs font-semibold uppercase tracking-wider text-text-muted transition-colors hover:bg-bg-hover"
  >
    {#if expanded}
      <ChevronDown size={14} />
    {:else}
      <ChevronRight size={14} />
    {/if}
    <span class="flex-1 text-left">{title}</span>
    {#if count > 0}
      <span class="text-text-muted">{count}</span>
    {/if}
  </button>

  {#if expanded}
    <div class="pb-1">
      {@render children()}
    </div>
  {/if}
</div>
