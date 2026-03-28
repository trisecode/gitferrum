<script lang="ts">
  import type { Component } from "svelte";

  let {
    label,
    icon,
    onclick,
    danger = false,
  }: {
    label: string;
    icon?: Component<{ size?: number; class?: string }>;
    onclick: () => void;
    danger?: boolean;
  } = $props();
</script>

<button
  {onclick}
  class="flex w-full items-center gap-2 px-3 py-1.5 text-sm transition-colors hover:bg-bg-hover"
  class:text-text-primary={!danger}
  class:text-diff-remove={danger}
>
  {#if icon}
    {@const Icon = icon}
    <Icon size={14} class={danger ? "text-diff-remove" : "text-text-muted"} />
  {/if}
  <span>{label}</span>
</button>
