<script lang="ts">
  import type { Snippet } from "svelte";

  let {
    x,
    y,
    onclose,
    children,
  }: {
    x: number;
    y: number;
    onclose: () => void;
    children: Snippet;
  } = $props();

  function handleClickOutside() {
    onclose();
  }

  function handleContextMenu(e: MouseEvent) {
    e.preventDefault();
    onclose();
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="fixed inset-0 z-50" onclick={handleClickOutside} oncontextmenu={handleContextMenu}>
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div
    class="absolute z-50 min-w-[200px] rounded-lg border border-border bg-bg-secondary py-1 shadow-xl"
    style="left: {x}px; top: {y}px;"
    onclick={(e) => e.stopPropagation()}
  >
    {@render children()}
  </div>
</div>
