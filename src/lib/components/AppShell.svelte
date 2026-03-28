<script lang="ts">
  import { repoStore } from "$lib/stores/repo.svelte";
  import { uiStore } from "$lib/stores/ui.svelte";
  import { i18n } from "$lib/stores/i18n.svelte";
  import { executePush } from "$lib/services/repo-actions";
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

  let rightPanelOpen = $derived(uiStore.detailPaneOpen || uiStore.changesPanelOpen);

  async function handlePushConfirm() {
    const confirm = uiStore.pushConfirm;
    if (!confirm) return;
    uiStore.pushConfirm = null;
    await executePush(confirm.remote, confirm.branch, true);
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
