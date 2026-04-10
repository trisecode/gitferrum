<script lang="ts">
  import type { RefLabel } from "$lib/types";
  import { deleteTag, deleteRemoteTag } from "$lib/services/git";
  import { refreshRepo } from "$lib/services/repo-actions";
  import { repoStore } from "$lib/stores/repo.svelte";
  import { toastStore } from "$lib/stores/toast.svelte";
  import { i18n } from "$lib/stores/i18n.svelte";
  import { Tag, Trash2, Globe } from "lucide-svelte";
  import ContextMenu from "../ContextMenu.svelte";
  import ContextMenuItem from "../ContextMenuItem.svelte";

  let { tags }: { tags: RefLabel[] } = $props();

  let menu = $state<{ x: number; y: number; tag: RefLabel } | null>(null);

  function openMenu(e: MouseEvent, tag: RefLabel) {
    e.preventDefault();
    menu = { x: e.clientX, y: e.clientY, tag };
  }

  function closeMenu() {
    menu = null;
  }

  async function handleDeleteTag() {
    if (!menu) return;
    const name = menu.tag.name;
    closeMenu();
    const repo = repoStore.activeRepo;
    if (!repo) return;
    try {
      await deleteTag(repo.repoPath, name);
      await refreshRepo();
      toastStore.success(i18n.t.tagDeleted(name));
    } catch (e) {
      toastStore.error(String(e));
    }
  }

  async function handleDeleteRemoteTag() {
    if (!menu) return;
    const name = menu.tag.name;
    closeMenu();
    const repo = repoStore.activeRepo;
    if (!repo) return;
    try {
      await deleteRemoteTag(repo.repoPath, "origin", name);
      await refreshRepo();
      toastStore.success(i18n.t.remoteTagDeleted(name));
    } catch (e) {
      toastStore.error(String(e));
    }
  }
</script>

{#each tags as tag}
  <button
    oncontextmenu={(e) => openMenu(e, tag)}
    class="flex w-full items-center gap-2 px-4 py-1 text-sm text-text-primary transition-colors hover:bg-bg-hover"
  >
    <Tag size={14} class="text-yellow-400" />
    <span class="truncate">{tag.name}</span>
  </button>
{:else}
  <p class="px-4 py-2 text-xs text-text-muted">{i18n.t.noTags}</p>
{/each}

{#if menu}
  <ContextMenu x={menu.x} y={menu.y} onclose={closeMenu}>
    <div class="px-3 py-1 text-[10px] font-semibold uppercase tracking-wider text-text-muted">{menu.tag.name}</div>
    <div class="my-1 border-t border-border"></div>
    <ContextMenuItem label={i18n.t.deleteTagLabel} icon={Trash2} onclick={handleDeleteTag} danger />
    <ContextMenuItem label={i18n.t.deleteRemoteTagLabel} icon={Globe} onclick={handleDeleteRemoteTag} danger />
  </ContextMenu>
{/if}
