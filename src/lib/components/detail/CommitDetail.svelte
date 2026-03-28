<script lang="ts">
  import type { CommitNode } from "$lib/types";
  import { relativeTime } from "$lib/utils/time";
  import { i18n } from "$lib/stores/i18n.svelte";
  import { GitCommit, User, Clock } from "lucide-svelte";

  let { commit }: { commit: CommitNode } = $props();
</script>

<div class="border-b border-border p-3">
  <div class="mb-2 flex items-center gap-2">
    <GitCommit size={14} class="text-accent" />
    <span class="font-mono text-xs text-accent">{commit.oid}</span>
  </div>

  <p class="mb-3 text-sm font-medium text-text-primary">{commit.message}</p>

  <div class="space-y-1 text-xs text-text-secondary">
    <div class="flex items-center gap-2">
      <User size={12} />
      <span>{commit.author_name} &lt;{commit.author_email}&gt;</span>
    </div>
    <div class="flex items-center gap-2">
      <Clock size={12} />
      <span>{relativeTime(commit.author_timestamp, i18n.t)}</span>
    </div>
  </div>

  {#if commit.parents.length > 0}
    <div class="mt-2 text-xs text-text-muted">
      {i18n.t.parents}: {commit.parents.map((p) => p.slice(0, 7)).join(", ")}
    </div>
  {/if}
</div>
