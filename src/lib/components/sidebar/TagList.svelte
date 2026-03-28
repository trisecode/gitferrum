<script lang="ts">
  import type { RefLabel } from "$lib/types";
  import { i18n } from "$lib/stores/i18n.svelte";
  import { Tag } from "lucide-svelte";

  let { tags }: { tags: RefLabel[] } = $props();
</script>

{#each tags as tag}
  <div class="flex items-center gap-2 px-4 py-1 text-sm text-text-primary">
    <Tag size={14} class="text-yellow-400" />
    <span class="truncate">{tag.name}</span>
  </div>
{:else}
  <p class="px-4 py-2 text-xs text-text-muted">{i18n.t.noTags}</p>
{/each}
