<script lang="ts">
  import type { CommitNode } from "$lib/types";
  import { relativeTime } from "$lib/utils/time";
  import { ROW_HEIGHT } from "$lib/utils/graph-layout";
  import { i18n } from "$lib/stores/i18n.svelte";

  let {
    commit,
    selected = false,
    graphWidth,
    onclick,
    oncontextmenu,
  }: {
    commit: CommitNode;
    selected?: boolean;
    graphWidth: number;
    onclick: () => void;
    oncontextmenu?: (e: MouseEvent) => void;
  } = $props();
</script>

<button
  {onclick}
  {oncontextmenu}
  class="flex w-full items-center border-b border-border text-left transition-colors hover:bg-bg-hover"
  class:bg-bg-tertiary={selected}
  style="height: {ROW_HEIGHT}px; padding-left: {graphWidth}px"
>
  <span class="shrink-0 w-[72px] font-mono text-xs text-accent">{commit.short_oid}</span>

  <span class="min-w-0 flex-1 truncate px-2 text-sm text-text-primary">
    {commit.message}

    {#each commit.refs as ref}
      <span
        class="ml-1 inline-block rounded px-1 py-0 text-[10px] font-medium leading-4 {ref.kind === 'localbranch' ? 'ref-local' : ref.kind === 'remotebranch' ? 'ref-remote' : 'ref-tag'}"
      >
        {ref.name}
      </span>
    {/each}
  </span>

  <span class="shrink-0 w-[120px] truncate px-2 text-xs text-text-secondary">{commit.author_name}</span>
  <span class="shrink-0 w-[80px] text-right pr-3 text-xs text-text-muted">{relativeTime(commit.author_timestamp, i18n.t)}</span>
</button>

<style>
  :global(.ref-local) {
    background-color: rgba(249, 115, 22, 0.2);
    color: #f97316;
  }
  :global(.ref-remote) {
    background-color: rgba(59, 130, 246, 0.2);
    color: #60a5fa;
  }
  :global(.ref-tag) {
    background-color: rgba(234, 179, 8, 0.2);
    color: #facc15;
  }
</style>
