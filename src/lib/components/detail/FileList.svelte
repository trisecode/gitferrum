<script lang="ts">
  import type { DiffStats, FileDiff } from "$lib/types";
  import { FilePlus, FileEdit, FileX, FileSymlink, Copy } from "lucide-svelte";
  import type { Component } from "svelte";

  import { i18n } from "$lib/stores/i18n.svelte";

  let { files, stats }: { files: FileDiff[]; stats: DiffStats } = $props();

  function statusIcon(status: string): Component<{ size?: number; class?: string }> {
    switch (status) {
      case "added": return FilePlus;
      case "deleted": return FileX;
      case "renamed": return FileSymlink;
      case "copied": return Copy;
      default: return FileEdit;
    }
  }

  function statusColor(status: string): string {
    switch (status) {
      case "added": return "text-diff-add";
      case "deleted": return "text-diff-remove";
      case "renamed": return "text-blue-400";
      default: return "text-yellow-400";
    }
  }
</script>

<div class="border-b border-border p-3">
  <div class="mb-2 text-xs text-text-muted">
    {i18n.t.filesChanged(stats.files_changed)},
    <span class="text-diff-add">+{stats.insertions}</span>
    <span class="text-diff-remove">-{stats.deletions}</span>
  </div>

  <div class="space-y-0.5">
    {#each files as file}
      {@const Icon = statusIcon(file.status)}
      <div class="flex items-center gap-2 rounded px-1 py-0.5 text-xs hover:bg-bg-hover">
        <Icon size={12} class={statusColor(file.status)} />
        <span class="truncate text-text-primary">{file.path}</span>
        {#if file.is_binary}
          <span class="text-text-muted">{i18n.t.binary}</span>
        {/if}
      </div>
    {/each}
  </div>
</div>
