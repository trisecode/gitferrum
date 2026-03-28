<script lang="ts">
  import { repoStore } from "$lib/stores/repo.svelte";
  import { uiStore } from "$lib/stores/ui.svelte";
  import { toastStore } from "$lib/stores/toast.svelte";
  import { gitAction, getStatus, getCommitGraph, getBranches, getDiff } from "$lib/services/git";
  import { FilePlus, FileEdit, FileX, Check, X, ChevronDown, ChevronRight, PanelRightClose } from "lucide-svelte";
  import { i18n } from "$lib/stores/i18n.svelte";

  let commitMessage = $state("");
  let isCommitting = $state(false);
  let selectedFiles = $state(new Set<string>());
  let stagedExpanded = $state(true);
  let unstagedExpanded = $state(true);
  let untrackedExpanded = $state(true);
  let selectedFilePath = $state<string | null>(null);

  let staged = $derived(repoStore.status?.staged ?? []);
  let unstaged = $derived(repoStore.status?.unstaged ?? []);
  let untracked = $derived(repoStore.status?.untracked ?? []);
  let totalChanges = $derived(staged.length + unstaged.length + untracked.length);

  function statusIcon(status: string) {
    switch (status) {
      case "added": return FilePlus;
      case "deleted": return FileX;
      default: return FileEdit;
    }
  }

  function statusColor(status: string): string {
    switch (status) {
      case "added": return "text-diff-add";
      case "deleted": return "text-diff-remove";
      default: return "text-yellow-400";
    }
  }

  function toggleFile(path: string) {
    if (selectedFiles.has(path)) {
      selectedFiles.delete(path);
    } else {
      selectedFiles.add(path);
    }
    selectedFiles = new Set(selectedFiles);
  }

  function selectAllUnstaged() {
    const allPaths = [
      ...unstaged.map((f) => f.path),
      ...untracked,
    ];
    const allSelected = allPaths.every((p) => selectedFiles.has(p));
    if (allSelected) {
      allPaths.forEach((p) => selectedFiles.delete(p));
    } else {
      allPaths.forEach((p) => selectedFiles.add(p));
    }
    selectedFiles = new Set(selectedFiles);
  }

  async function handleStageSelected() {
    const repo = repoStore.activeRepo;
    if (!repo || selectedFiles.size === 0) return;
    try {
      await gitAction(repo.repoPath, "stage", { files: Array.from(selectedFiles) });
      repo.status = await getStatus(repo.repoPath);
      selectedFiles = new Set();
    } catch (e) {
      toastStore.error(String(e));
    }
  }

  async function handleUnstageAll() {
    const repo = repoStore.activeRepo;
    if (!repo || staged.length === 0) return;
    try {
      await gitAction(repo.repoPath, "unstage", { files: staged.map((f) => f.path) });
      repo.status = await getStatus(repo.repoPath);
    } catch (e) {
      toastStore.error(String(e));
    }
  }

  async function handleCommit() {
    const repo = repoStore.activeRepo;
    if (!repo || !commitMessage.trim()) return;

    // If there are selected unstaged files, stage them first
    if (selectedFiles.size > 0) {
      await gitAction(repo.repoPath, "stage", { files: Array.from(selectedFiles) });
    }

    // Need at least staged files to commit
    const freshStatus = await getStatus(repo.repoPath);
    if (freshStatus.staged.length === 0) {
      toastStore.error(i18n.t.noStagedChanges);
      return;
    }

    isCommitting = true;
    try {
      const path = repo.repoPath;
      await gitAction(path, "commit", { message: commitMessage.trim() });

      commitMessage = "";
      selectedFiles = new Set();

      const [status, graph, branches] = await Promise.all([
        getStatus(path),
        getCommitGraph(path, 0, 200),
        getBranches(path),
      ]);
      repo.status = status;
      repo.commitGraph = graph;
      repo.branches = branches;
      toastStore.success(i18n.t.commitCreated);
    } catch (e) {
      toastStore.error(String(e));
    } finally {
      isCommitting = false;
    }
  }

  async function handleFileClick(path: string) {
    const repo = repoStore.activeRepo;
    if (!repo) return;
    selectedFilePath = path;
    // Show diff for this file in the detail pane
    try {
      const diff = await getDiff(repo.repoPath);
      // Filter to only the clicked file
      if (diff.files.length > 0) {
        const fileDiff = diff.files.find((f) => f.path === path);
        if (fileDiff) {
          repo.diff = { files: [fileDiff], stats: { files_changed: 1, insertions: diff.stats.insertions, deletions: diff.stats.deletions } };
          repo.selectedCommit = null;
          uiStore.openDetailPane();
        }
      }
    } catch {
      // ignore
    }
  }
</script>

<aside class="flex flex-col overflow-hidden border-l border-border bg-bg-secondary">
  <!-- Header -->
  <div class="flex items-center justify-between border-b border-border px-3 py-1.5">
    <span class="text-xs font-semibold uppercase tracking-wider text-text-muted">
      {i18n.t.changes}
      {#if totalChanges > 0}
        <span class="font-normal">({totalChanges})</span>
      {/if}
    </span>
    <button
      onclick={() => (uiStore.changesPanelOpen = false)}
      class="rounded p-1 text-text-secondary transition-colors hover:bg-bg-hover hover:text-text-primary"
    >
      <PanelRightClose size={14} />
    </button>
  </div>

  <!-- File lists -->
  <div class="flex-1 overflow-y-auto">
    <!-- Staged -->
    {#if staged.length > 0}
      <div class="border-b border-border">
        <div class="flex w-full items-center gap-1 px-3 py-1.5 text-xs font-semibold uppercase tracking-wider text-diff-add">
          <button onclick={() => (stagedExpanded = !stagedExpanded)} class="flex flex-1 items-center gap-1 hover:opacity-80">
            {#if stagedExpanded}<ChevronDown size={12} />{:else}<ChevronRight size={12} />{/if}
            <span class="flex-1 text-left">{i18n.t.staged} ({staged.length})</span>
          </button>
          <button
            onclick={handleUnstageAll}
            class="text-[10px] font-normal normal-case tracking-normal text-text-muted hover:text-text-primary"
          >
            {i18n.t.unstageAll}
          </button>
        </div>
        {#if stagedExpanded}
          {#each staged as file}
            {@const Icon = statusIcon(file.status)}
            <button
              onclick={() => handleFileClick(file.path)}
              class="flex w-full items-center gap-2 px-4 py-1 text-xs transition-colors hover:bg-bg-hover"
              class:bg-bg-tertiary={selectedFilePath === file.path}
            >
              <Icon size={12} class={statusColor(file.status)} />
              <span class="flex-1 truncate text-left text-text-primary">{file.path}</span>
              <span class="text-[10px] text-diff-add">{i18n.t.stagedBadge}</span>
            </button>
          {/each}
        {/if}
      </div>
    {/if}

    <!-- Unstaged -->
    {#if unstaged.length > 0}
      <div class="border-b border-border">
        <div class="flex w-full items-center gap-1 px-3 py-1.5 text-xs font-semibold uppercase tracking-wider text-yellow-400">
          <button onclick={() => (unstagedExpanded = !unstagedExpanded)} class="flex flex-1 items-center gap-1 hover:opacity-80">
            {#if unstagedExpanded}<ChevronDown size={12} />{:else}<ChevronRight size={12} />{/if}
            <span class="flex-1 text-left">{i18n.t.modified} ({unstaged.length})</span>
          </button>
          <button
            onclick={selectAllUnstaged}
            class="text-[10px] font-normal normal-case tracking-normal text-text-muted hover:text-text-primary"
          >
            {i18n.t.selectAll}
          </button>
        </div>
        {#if unstagedExpanded}
          {#each unstaged as file}
            {@const Icon = statusIcon(file.status)}
            <div class="flex w-full items-center gap-2 px-4 py-1 text-xs transition-colors hover:bg-bg-hover">
              <input
                type="checkbox"
                checked={selectedFiles.has(file.path)}
                onchange={() => toggleFile(file.path)}
                class="accent-accent h-3 w-3"
              />
              <Icon size={12} class={statusColor(file.status)} />
              <button
                onclick={() => handleFileClick(file.path)}
                class="flex-1 truncate text-left text-text-primary hover:underline"
              >
                {file.path}
              </button>
            </div>
          {/each}
        {/if}
      </div>
    {/if}

    <!-- Untracked -->
    {#if untracked.length > 0}
      <div class="border-b border-border">
        <button
          onclick={() => (untrackedExpanded = !untrackedExpanded)}
          class="flex w-full items-center gap-1 px-3 py-1.5 text-xs font-semibold uppercase tracking-wider text-text-muted hover:bg-bg-hover"
        >
          {#if untrackedExpanded}<ChevronDown size={12} />{:else}<ChevronRight size={12} />{/if}
          <span class="flex-1 text-left">{i18n.t.untracked} ({untracked.length})</span>
        </button>
        {#if untrackedExpanded}
          {#each untracked as path}
            <label class="flex w-full cursor-pointer items-center gap-2 px-4 py-1 text-xs transition-colors hover:bg-bg-hover">
              <input
                type="checkbox"
                checked={selectedFiles.has(path)}
                onchange={() => toggleFile(path)}
                class="accent-accent h-3 w-3"
              />
              <FilePlus size={12} class="text-text-muted" />
              <span class="flex-1 truncate text-text-primary">{path}</span>
            </label>
          {/each}
        {/if}
      </div>
    {/if}

    {#if totalChanges === 0}
      <div class="flex items-center justify-center p-6 text-xs text-text-muted">
        {i18n.t.noChanges}
      </div>
    {/if}
  </div>

  <!-- Commit area -->
  <div class="border-t border-border p-3">
    {#if selectedFiles.size > 0}
      <button
        onclick={handleStageSelected}
        class="mb-2 w-full rounded bg-bg-tertiary px-3 py-1.5 text-xs text-text-secondary transition-colors hover:bg-bg-hover hover:text-text-primary"
      >
        {i18n.t.stageNFiles(selectedFiles.size)}
      </button>
    {/if}

    <textarea
      bind:value={commitMessage}
      placeholder={i18n.t.commitMessage}
      rows={3}
      class="w-full resize-none rounded border border-border bg-bg-primary px-3 py-2 text-xs text-text-primary placeholder:text-text-muted focus:border-accent focus:outline-none"
    ></textarea>

    <button
      onclick={handleCommit}
      disabled={!commitMessage.trim() || (staged.length === 0 && selectedFiles.size === 0) || isCommitting}
      class="mt-2 flex w-full items-center justify-center gap-1 rounded bg-accent px-3 py-1.5 text-xs font-medium text-white transition-colors hover:bg-accent-hover disabled:opacity-40"
    >
      <Check size={13} />
      {isCommitting ? i18n.t.committing : i18n.t.commit}
    </button>
  </div>
</aside>
