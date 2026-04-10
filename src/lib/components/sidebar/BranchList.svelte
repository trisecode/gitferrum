<script lang="ts">
  import type { RefLabel } from "$lib/types";
  import { gitAction, renameBranch } from "$lib/services/git";
  import { refreshRepo } from "$lib/services/repo-actions";
  import { repoStore } from "$lib/stores/repo.svelte";
  import { uiStore } from "$lib/stores/ui.svelte";
  import { toastStore } from "$lib/stores/toast.svelte";
  import { i18n } from "$lib/stores/i18n.svelte";
  import { GitBranch, Pencil, GitMerge, Trash2 } from "lucide-svelte";
  import ContextMenu from "../ContextMenu.svelte";
  import ContextMenuItem from "../ContextMenuItem.svelte";

  let { branches }: { branches: RefLabel[] } = $props();

  let menu = $state<{ x: number; y: number; branch: RefLabel } | null>(null);
  let showRenameInput = $state(false);
  let renameValue = $state("");

  function openMenu(e: MouseEvent, branch: RefLabel) {
    e.preventDefault();
    menu = { x: e.clientX, y: e.clientY, branch };
    showRenameInput = false;
  }

  function closeMenu() {
    menu = null;
    showRenameInput = false;
  }

  async function handleCheckout(branch: string) {
    const repo = repoStore.activeRepo;
    if (!repo) return;
    try {
      await gitAction(repo.repoPath, "checkout", { branch });
      await refreshRepo();
    } catch (e) {
      repo.error = String(e);
    }
  }

  function handleDelete(force: boolean) {
    if (!menu) return;
    const branch = menu.branch;
    if (branch.is_head) {
      toastStore.error(i18n.t.cannotDeleteHead);
      closeMenu();
      return;
    }
    const name = branch.name;
    closeMenu();
    // Show confirmation dialog
    uiStore.deleteBranchConfirm = { name, force };
  }

  function handleStartRename() {
    if (!menu) return;
    renameValue = menu.branch.name;
    showRenameInput = true;
  }

  async function handleRename() {
    if (!menu || !renameValue.trim()) return;
    const oldName = menu.branch.name;
    const newName = renameValue.trim();
    if (oldName === newName) { closeMenu(); return; }
    closeMenu();
    const repo = repoStore.activeRepo;
    if (!repo) return;
    try {
      await renameBranch(repo.repoPath, oldName, newName);
      await refreshRepo();
      toastStore.success(i18n.t.branchRenamed(oldName, newName));
    } catch (e) {
      toastStore.error(String(e));
    }
  }

  function handleMerge() {
    if (!menu) return;
    const name = menu.branch.name;
    closeMenu();
    uiStore.mergeDialogOpen = { preselectedBranch: name };
  }
</script>

{#each branches as branch}
  <button
    ondblclick={() => handleCheckout(branch.name)}
    oncontextmenu={(e) => openMenu(e, branch)}
    class="flex w-full items-center gap-2 px-4 py-1 text-sm transition-colors hover:bg-bg-hover"
    class:text-accent={branch.is_head}
    class:font-medium={branch.is_head}
    class:text-text-primary={!branch.is_head}
  >
    <GitBranch size={14} class={branch.is_head ? "text-accent" : "text-text-muted"} />
    <span class="truncate">{branch.name}</span>
    {#if branch.is_head}
      <span class="ml-auto text-[10px] text-accent">{i18n.t.head}</span>
    {/if}
  </button>
{:else}
  <p class="px-4 py-2 text-xs text-text-muted">{i18n.t.noBranches}</p>
{/each}

{#if menu}
  <ContextMenu x={menu.x} y={menu.y} onclose={closeMenu}>
    {#if !showRenameInput}
      <div class="px-3 py-1 text-[10px] font-semibold uppercase tracking-wider text-text-muted">{menu.branch.name}</div>
      <div class="my-1 border-t border-border"></div>
      <ContextMenuItem label={i18n.t.renameBranch} icon={Pencil} onclick={handleStartRename} />
      {#if !menu.branch.is_head}
        <ContextMenuItem label={i18n.t.mergeBranchInto} icon={GitMerge} onclick={handleMerge} />
        <div class="my-1 border-t border-border"></div>
        <ContextMenuItem label={i18n.t.deleteBranch} icon={Trash2} onclick={() => handleDelete(false)} danger />
        <ContextMenuItem label={i18n.t.forceDeleteBranch} icon={Trash2} onclick={() => handleDelete(true)} danger />
      {/if}
    {:else}
      <div class="px-3 py-2">
        <label class="block text-xs text-text-secondary mb-1">{i18n.t.newBranchNameLabel}</label>
        <input
          bind:value={renameValue}
          onkeydown={(e) => e.key === "Enter" && handleRename()}
          class="w-full rounded border border-border bg-bg-primary px-2 py-1 text-sm text-text-primary focus:border-accent focus:outline-none"
          autofocus
        />
        <div class="mt-2 flex justify-end gap-1">
          <button onclick={closeMenu} class="rounded px-2 py-1 text-xs text-text-secondary hover:bg-bg-hover">{i18n.t.cancel}</button>
          <button
            onclick={handleRename}
            disabled={!renameValue.trim()}
            class="rounded bg-accent px-2 py-1 text-xs font-medium text-white hover:bg-accent-hover disabled:opacity-50"
          >{i18n.t.confirm}</button>
        </div>
      </div>
    {/if}
  </ContextMenu>
{/if}
