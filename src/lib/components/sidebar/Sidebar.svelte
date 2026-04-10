<script lang="ts">
  import { repoStore } from "$lib/stores/repo.svelte";
  import { uiStore } from "$lib/stores/ui.svelte";
  import { i18n } from "$lib/stores/i18n.svelte";
  import { refreshStashes } from "$lib/services/repo-actions";
  import SidebarSection from "./SidebarSection.svelte";
  import BranchList from "./BranchList.svelte";
  import RemoteList from "./RemoteList.svelte";
  import TagList from "./TagList.svelte";
  import StashList from "./StashList.svelte";
  import RepoSwitcher from "./RepoSwitcher.svelte";
  import { PanelLeftClose, PanelLeftOpen, Settings } from "lucide-svelte";

  let localBranches = $derived(
    repoStore.branches.filter((b) => b.kind === "localbranch"),
  );
  let remoteBranches = $derived(
    repoStore.branches.filter((b) => b.kind === "remotebranch"),
  );
  let tags = $derived(
    repoStore.branches.filter((b) => b.kind === "tag"),
  );

  let stashes = $state<[number, string][]>([]);

  async function loadStashes() {
    stashes = await refreshStashes();
  }

  $effect(() => {
    // Reload stashes when active repo changes
    const _id = repoStore.activeRepoId;
    if (_id) loadStashes();
  });
</script>

<aside class="flex flex-col overflow-hidden border-r border-border bg-bg-secondary">
  <!-- Header -->
  <div class="flex items-center justify-between p-2">
    {#if !uiStore.sidebarCollapsed}
      <span class="text-xs font-semibold uppercase tracking-wider text-text-muted">{i18n.t.appName}</span>
    {/if}
    <button
      onclick={() => uiStore.toggleSidebar()}
      class="rounded p-1 text-text-secondary transition-colors hover:bg-bg-hover hover:text-text-primary"
    >
      {#if uiStore.sidebarCollapsed}
        <PanelLeftOpen size={16} />
      {:else}
        <PanelLeftClose size={16} />
      {/if}
    </button>
  </div>

  <!-- Content -->
  {#if !uiStore.sidebarCollapsed}
    <div class="flex-1 overflow-y-auto">
      <RepoSwitcher />

      <SidebarSection title={i18n.t.branches} count={localBranches.length}>
        <BranchList branches={localBranches} />
      </SidebarSection>

      <SidebarSection title={i18n.t.remotes} count={remoteBranches.length}>
        <RemoteList branches={remoteBranches} />
      </SidebarSection>

      <SidebarSection title={i18n.t.tags} count={tags.length}>
        <TagList {tags} />
      </SidebarSection>

      <SidebarSection title={i18n.t.stashes} count={stashes.length}>
        <StashList {stashes} onrefresh={loadStashes} />
      </SidebarSection>
    </div>
  {/if}

  <!-- Settings button at bottom -->
  <div class="border-t border-border p-2">
    <button
      onclick={() => (uiStore.settingsOpen = true)}
      class="flex w-full items-center gap-2 rounded px-2 py-1.5 text-xs text-text-secondary transition-colors hover:bg-bg-hover hover:text-text-primary"
      title={i18n.t.settingsTooltip}
    >
      <Settings size={14} />
      {#if !uiStore.sidebarCollapsed}
        <span>{i18n.t.settings}</span>
      {/if}
    </button>
  </div>
</aside>
