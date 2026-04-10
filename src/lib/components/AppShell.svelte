<script lang="ts">
  import { repoStore } from "$lib/stores/repo.svelte";
  import { uiStore } from "$lib/stores/ui.svelte";
  import { i18n } from "$lib/stores/i18n.svelte";
  import { resetToCommit, deleteBranch, deleteRemoteBranch } from "$lib/services/git";
  import { executePush, refreshRepo } from "$lib/services/repo-actions";
  import { toastStore } from "$lib/stores/toast.svelte";
  import Sidebar from "./sidebar/Sidebar.svelte";
  import CenterPane from "./graph/CenterPane.svelte";
  import DetailPane from "./detail/DetailPane.svelte";
  import ChangesPanel from "./ChangesPanel.svelte";
  import StatusBar from "./StatusBar.svelte";
  import WelcomeScreen from "./WelcomeScreen.svelte";
  import CommitDialog from "./CommitDialog.svelte";
  import CloneDialog from "./CloneDialog.svelte";
  import ConfirmDialog from "./ConfirmDialog.svelte";
  import SettingsDialog from "./SettingsDialog.svelte";
  import MergeConflictDialog from "./MergeConflictDialog.svelte";
  import CreateTagDialog from "./CreateTagDialog.svelte";
  import MergeDialog from "./MergeDialog.svelte";

  let rightPanelOpen = $derived(uiStore.detailPaneOpen || uiStore.changesPanelOpen);

  async function handlePushConfirm() {
    const confirm = uiStore.pushConfirm;
    if (!confirm) return;
    uiStore.pushConfirm = null;
    await executePush(confirm.remote, confirm.branch, true);
  }

  async function handleDeleteBranchConfirm() {
    const info = uiStore.deleteBranchConfirm;
    if (!info) return;
    const repo = repoStore.activeRepo;
    if (!repo) return;
    uiStore.deleteBranchConfirm = null;
    try {
      await deleteBranch(repo.repoPath, info.name, info.force);
      await refreshRepo();
      toastStore.success(i18n.t.branchDeleted(info.name));
    } catch (e) {
      toastStore.error(String(e));
    }
  }

  async function handleDeleteRemoteBranchConfirm() {
    const info = uiStore.deleteRemoteBranchConfirm;
    if (!info) return;
    const repo = repoStore.activeRepo;
    if (!repo) return;
    uiStore.deleteRemoteBranchConfirm = null;
    try {
      await deleteRemoteBranch(repo.repoPath, info.remote, info.branch);
      await refreshRepo();
      toastStore.success(i18n.t.remoteBranchDeleted(info.fullName));
    } catch (e) {
      toastStore.error(String(e));
    }
  }

  async function handleResetHardConfirm() {
    const info = uiStore.resetConfirm;
    if (!info) return;
    const repo = repoStore.activeRepo;
    if (!repo) return;
    uiStore.resetConfirm = null;
    try {
      await resetToCommit(repo.repoPath, info.commit, info.mode);
      await refreshRepo();
      toastStore.success(i18n.t.resetComplete(info.mode));
    } catch (e) {
      toastStore.error(String(e));
    }
  }
</script>

{#if !repoStore.isRepoOpen}
  <WelcomeScreen />
{:else}
  <div
    class="app-shell"
    class:sidebar-collapsed={uiStore.sidebarCollapsed}
    class:right-open={rightPanelOpen}
  >
    <Sidebar />
    <CenterPane />

    {#if uiStore.detailPaneOpen}
      <DetailPane />
    {:else if uiStore.changesPanelOpen}
      <ChangesPanel />
    {/if}

    <StatusBar />
  </div>

  {#if uiStore.commitDialogOpen}
    <CommitDialog />
  {/if}
{/if}

{#if uiStore.cloneDialogOpen}
  <CloneDialog />
{/if}

{#if uiStore.pushConfirm}
  <ConfirmDialog
    title={i18n.t.pushNewBranchTitle}
    message={i18n.t.pushNewBranchMessage(uiStore.pushConfirm.branch, uiStore.pushConfirm.remote)}
    confirmLabel={i18n.t.pushNewBranchConfirm}
    onconfirm={handlePushConfirm}
    oncancel={() => (uiStore.pushConfirm = null)}
  />
{/if}

{#if uiStore.settingsOpen}
  <SettingsDialog />
{/if}

{#if uiStore.mergeConflict}
  <MergeConflictDialog />
{/if}

{#if uiStore.createTag}
  <CreateTagDialog />
{/if}

{#if uiStore.resetConfirm}
  <ConfirmDialog
    title={i18n.t.resetHard}
    message={i18n.t.resetHardWarning}
    confirmLabel={i18n.t.confirm}
    onconfirm={handleResetHardConfirm}
    oncancel={() => (uiStore.resetConfirm = null)}
  />
{/if}

{#if uiStore.mergeDialogOpen}
  <MergeDialog />
{/if}

{#if uiStore.deleteBranchConfirm}
  <ConfirmDialog
    title={uiStore.deleteBranchConfirm.force ? i18n.t.forceDeleteBranchTitle : i18n.t.deleteBranchTitle}
    message={uiStore.deleteBranchConfirm.force
      ? i18n.t.forceDeleteBranchConfirm(uiStore.deleteBranchConfirm.name)
      : i18n.t.deleteBranchConfirm(uiStore.deleteBranchConfirm.name)}
    confirmLabel={i18n.t.deleteConfirm}
    onconfirm={handleDeleteBranchConfirm}
    oncancel={() => (uiStore.deleteBranchConfirm = null)}
  />
{/if}

{#if uiStore.deleteRemoteBranchConfirm}
  <ConfirmDialog
    title={i18n.t.deleteRemoteBranch}
    message={i18n.t.deleteRemoteBranchConfirm(uiStore.deleteRemoteBranchConfirm.fullName)}
    confirmLabel={i18n.t.deleteConfirm}
    onconfirm={handleDeleteRemoteBranchConfirm}
    oncancel={() => (uiStore.deleteRemoteBranchConfirm = null)}
  />
{/if}

<style>
  .app-shell {
    display: grid;
    grid-template-columns: 240px 1fr 0px;
    grid-template-rows: 1fr auto;
    height: 100vh;
    overflow: hidden;
    transition: grid-template-columns 0.2s ease;
  }

  .app-shell.sidebar-collapsed {
    grid-template-columns: 48px 1fr 0px;
  }

  .app-shell.right-open {
    grid-template-columns: 240px 1fr 320px;
  }

  .app-shell.sidebar-collapsed.right-open {
    grid-template-columns: 48px 1fr 320px;
  }
</style>
