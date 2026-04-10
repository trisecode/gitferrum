<script lang="ts">
  import type { RefLabel } from "$lib/types";
  import { gitAction } from "$lib/services/git";
  import { refreshRepo } from "$lib/services/repo-actions";
  import { repoStore } from "$lib/stores/repo.svelte";
  import { uiStore } from "$lib/stores/ui.svelte";
  import { toastStore } from "$lib/stores/toast.svelte";
  import { i18n } from "$lib/stores/i18n.svelte";
  import { Globe, GitBranchPlus, GitFork, Eye, Trash2 } from "lucide-svelte";
  import ContextMenu from "../ContextMenu.svelte";
  import ContextMenuItem from "../ContextMenuItem.svelte";

  let { branches }: { branches: RefLabel[] } = $props();

  let menu = $state<{ x: number; y: number; branch: string } | null>(null);
  let newBranchName = $state("");
  let showNameInput = $state(false);

  function openMenu(e: MouseEvent, branchName: string) {
    e.preventDefault();
    menu = { x: e.clientX, y: e.clientY, branch: branchName };
    showNameInput = false;
    newBranchName = branchName.split("/").slice(1).join("/");
  }

  function closeMenu() {
    menu = null;
    showNameInput = false;
  }

  async function handleCheckoutLocal() {
    if (!menu) return;
    const repo = repoStore.activeRepo;
    if (!repo) return;
    const remoteBranch = menu.branch;
    closeMenu();
    try {
      await gitAction(repo.repoPath, "checkout_remote", { remoteBranch });
      await refreshRepo();
      const localName = remoteBranch.split("/").slice(1).join("/");
      toastStore.success(i18n.t.checkedOut(localName));
    } catch (e) {
      toastStore.error(String(e));
    }
  }

  function handleNewBranch() {
    showNameInput = true;
  }

  async function handleCreateNewBranch() {
    if (!menu || !newBranchName.trim()) return;
    const repo = repoStore.activeRepo;
    if (!repo) return;
    const remoteBranch = menu.branch;
    const name = newBranchName.trim();
    closeMenu();
    try {
      await gitAction(repo.repoPath, "create_branch_from_remote", { newName: name, remoteBranch });
      await refreshRepo();
      toastStore.success(i18n.t.branchCreated(name));
    } catch (e) {
      toastStore.error(String(e));
    }
  }

  function handleDeleteRemoteBranch() {
    if (!menu) return;
    const fullName = menu.branch;
    const parts = fullName.split("/");
    const remote = parts[0];
    const branch = parts.slice(1).join("/");
    closeMenu();
    uiStore.deleteRemoteBranchConfirm = { fullName, remote, branch };
  }

  async function handleBrowse() {
    if (!menu) return;
    const repo = repoStore.activeRepo;
    if (!repo) return;
    const refName = menu.branch;
    closeMenu();
    try {
      await gitAction(repo.repoPath, "checkout_detached", { refName });
      await refreshRepo();
      toastStore.success(i18n.t.browsingDetached(refName));
    } catch (e) {
      toastStore.error(String(e));
    }
  }
</script>

{#each branches as branch}
  <button
    onclick={(e) => openMenu(e, branch.name)}
    oncontextmenu={(e) => openMenu(e, branch.name)}
    class="flex w-full items-center gap-2 px-4 py-1 text-sm text-text-primary transition-colors hover:bg-bg-hover"
  >
    <Globe size={14} class="text-text-muted" />
    <span class="truncate">{branch.name}</span>
  </button>
{:else}
  <p class="px-4 py-2 text-xs text-text-muted">{i18n.t.noRemotes}</p>
{/each}

{#if menu}
  <ContextMenu x={menu.x} y={menu.y} onclose={closeMenu}>
    {#if !showNameInput}
      <div class="px-3 py-1 text-[10px] font-semibold uppercase tracking-wider text-text-muted">{menu.branch}</div>
      <div class="my-1 border-t border-border"></div>
      <ContextMenuItem label={i18n.t.createLocalBranch} icon={GitBranchPlus} onclick={handleCheckoutLocal} />
      <ContextMenuItem label={i18n.t.createCustomName} icon={GitFork} onclick={handleNewBranch} />
      <ContextMenuItem label={i18n.t.browseDetached} icon={Eye} onclick={handleBrowse} />
      <div class="my-1 border-t border-border"></div>
      <ContextMenuItem label={i18n.t.deleteRemoteBranch} icon={Trash2} onclick={handleDeleteRemoteBranch} danger />
    {:else}
      <div class="px-3 py-2">
        <label class="block text-xs text-text-secondary mb-1">{i18n.t.newBranchName}</label>
        <input
          bind:value={newBranchName}
          onkeydown={(e) => e.key === "Enter" && handleCreateNewBranch()}
          class="w-full rounded border border-border bg-bg-primary px-2 py-1 text-sm text-text-primary focus:border-accent focus:outline-none"
          autofocus
        />
        <div class="mt-2 flex justify-end gap-1">
          <button onclick={closeMenu} class="rounded px-2 py-1 text-xs text-text-secondary hover:bg-bg-hover">{i18n.t.cancel}</button>
          <button
            onclick={handleCreateNewBranch}
            disabled={!newBranchName.trim()}
            class="rounded bg-accent px-2 py-1 text-xs font-medium text-white hover:bg-accent-hover disabled:opacity-50"
          >{i18n.t.create}</button>
        </div>
      </div>
    {/if}
  </ContextMenu>
{/if}
