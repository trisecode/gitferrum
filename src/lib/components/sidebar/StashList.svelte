<script lang="ts">
  import { stashApply, stashDrop } from "$lib/services/git";
  import { refreshRepo } from "$lib/services/repo-actions";
  import { repoStore } from "$lib/stores/repo.svelte";
  import { toastStore } from "$lib/stores/toast.svelte";
  import { i18n } from "$lib/stores/i18n.svelte";
  import { Archive, Play, Trash2 } from "lucide-svelte";
  import ContextMenu from "../ContextMenu.svelte";
  import ContextMenuItem from "../ContextMenuItem.svelte";

  let {
    stashes,
    onrefresh,
  }: {
    stashes: [number, string][];
    onrefresh: () => void;
  } = $props();

  let menu = $state<{ x: number; y: number; index: number; message: string } | null>(null);

  function openMenu(e: MouseEvent, index: number, message: string) {
    e.preventDefault();
    menu = { x: e.clientX, y: e.clientY, index, message };
  }

  function closeMenu() {
    menu = null;
  }

  async function handleApply(index: number) {
    closeMenu();
    const repo = repoStore.activeRepo;
    if (!repo) return;
    try {
      await stashApply(repo.repoPath, index);
      await refreshRepo();
      onrefresh();
      toastStore.success(i18n.t.stashApplied(index));
    } catch (e) {
      toastStore.error(String(e));
    }
  }

  async function handleDrop(index: number) {
    closeMenu();
    const repo = repoStore.activeRepo;
    if (!repo) return;
    try {
      await stashDrop(repo.repoPath, index);
      onrefresh();
      toastStore.success(i18n.t.stashDropped(index));
    } catch (e) {
      toastStore.error(String(e));
    }
  }
</script>

{#if stashes.length > 0}
  {#each stashes as [index, message]}
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
      oncontextmenu={(e) => openMenu(e, index, message)}
      class="group flex w-full items-center gap-2 px-4 py-1 text-sm text-text-primary transition-colors hover:bg-bg-hover"
    >
      <Archive size={14} class="shrink-0 text-text-muted" />
      <span class="flex-1 truncate text-left">{message}</span>
      <span class="shrink-0 hidden group-hover:flex items-center gap-0.5">
        <button
          onclick={() => handleApply(index)}
          class="rounded p-0.5 text-text-muted hover:text-diff-add"
          title={i18n.t.applyStash}
        >
          <Play size={12} />
        </button>
        <button
          onclick={() => handleDrop(index)}
          class="rounded p-0.5 text-text-muted hover:text-diff-remove"
          title={i18n.t.dropStash}
        >
          <Trash2 size={12} />
        </button>
      </span>
    </div>
  {/each}
{:else}
  <p class="px-4 py-2 text-xs text-text-muted">{i18n.t.noStashes}</p>
{/if}

{#if menu}
  <ContextMenu x={menu.x} y={menu.y} onclose={closeMenu}>
    <div class="px-3 py-1 text-[10px] font-semibold uppercase tracking-wider text-text-muted">stash@&lbrace;{menu.index}&rbrace;</div>
    <div class="my-1 border-t border-border"></div>
    <ContextMenuItem label={i18n.t.applyStash} icon={Play} onclick={() => menu && handleApply(menu.index)} />
    <ContextMenuItem label={i18n.t.dropStash} icon={Trash2} onclick={() => menu && handleDrop(menu.index)} danger />
  </ContextMenu>
{/if}
