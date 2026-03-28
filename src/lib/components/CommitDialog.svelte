<script lang="ts">
  import { repoStore } from "$lib/stores/repo.svelte";
  import { uiStore } from "$lib/stores/ui.svelte";
  import { gitAction, getStatus, getCommitGraph, getBranches } from "$lib/services/git";
  import { X, Check } from "lucide-svelte";
  import { i18n } from "$lib/stores/i18n.svelte";

  let message = $state("");
  let isCommitting = $state(false);

  let allFiles = $derived([
    ...(repoStore.status?.unstaged.map((f) => ({ ...f, staged: false })) ?? []),
    ...(repoStore.status?.untracked.map((p) => ({ path: p, status: "added" as const, staged: false })) ?? []),
    ...(repoStore.status?.staged.map((f) => ({ ...f, staged: true })) ?? []),
  ]);

  let selectedFiles = $state(new Set<string>());

  function toggleFile(path: string) {
    if (selectedFiles.has(path)) {
      selectedFiles.delete(path);
    } else {
      selectedFiles.add(path);
    }
    selectedFiles = new Set(selectedFiles);
  }

  function selectAll() {
    if (selectedFiles.size === allFiles.length) {
      selectedFiles = new Set();
    } else {
      selectedFiles = new Set(allFiles.map((f) => f.path));
    }
  }

  async function handleCommit() {
    if (!message.trim() || selectedFiles.size === 0) return;
    const repo = repoStore.activeRepo;
    if (!repo) return;

    isCommitting = true;
    try {
      const path = repo.repoPath;
      const files = Array.from(selectedFiles);
      await gitAction(path, "stage", { files });
      await gitAction(path, "commit", { message: message.trim() });

      message = "";
      selectedFiles = new Set();
      uiStore.commitDialogOpen = false;

      const [status, graph, branches] = await Promise.all([
        getStatus(path),
        getCommitGraph(path, 0, 200),
        getBranches(path),
      ]);
      repo.status = status;
      repo.commitGraph = graph;
      repo.branches = branches;
    } catch (e) {
      repo.error = String(e);
    } finally {
      isCommitting = false;
    }
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="fixed inset-0 z-50 flex items-center justify-center bg-black/50"
  onkeydown={(e) => e.key === "Escape" && (uiStore.commitDialogOpen = false)}
>
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div
    class="w-[520px] max-h-[80vh] rounded-lg border border-border bg-bg-secondary shadow-2xl flex flex-col"
    onclick={(e) => e.stopPropagation()}
  >
    <div class="flex items-center justify-between border-b border-border px-4 py-2">
      <span class="text-sm font-semibold text-text-primary">{i18n.t.commitChanges}</span>
      <button
        onclick={() => (uiStore.commitDialogOpen = false)}
        class="rounded p-1 text-text-secondary hover:bg-bg-hover hover:text-text-primary"
      >
        <X size={14} />
      </button>
    </div>

    <div class="flex-1 overflow-y-auto p-4">
      <div class="mb-3">
        <div class="flex items-center justify-between mb-1">
          <span class="text-xs font-medium text-text-secondary">{i18n.t.files} ({allFiles.length})</span>
          <button onclick={selectAll} class="text-xs text-accent hover:underline">
            {selectedFiles.size === allFiles.length ? i18n.t.deselectAll : i18n.t.selectAll}
          </button>
        </div>

        <div class="max-h-[200px] overflow-y-auto rounded border border-border bg-bg-primary">
          {#each allFiles as file}
            <label class="flex cursor-pointer items-center gap-2 px-3 py-1 text-xs hover:bg-bg-hover">
              <input
                type="checkbox"
                checked={selectedFiles.has(file.path)}
                onchange={() => toggleFile(file.path)}
                class="accent-accent"
              />
              <span
                class="truncate"
                class:text-diff-add={file.status === "added"}
                class:text-yellow-400={file.status === "modified"}
                class:text-diff-remove={file.status === "deleted"}
              >
                {file.path}
              </span>
            </label>
          {/each}
        </div>
      </div>

      <textarea
        bind:value={message}
        placeholder={i18n.t.commitMessage}
        rows={3}
        class="w-full resize-none rounded border border-border bg-bg-primary px-3 py-2 text-sm text-text-primary placeholder:text-text-muted focus:border-accent focus:outline-none"
      ></textarea>
    </div>

    <div class="flex items-center justify-end gap-2 border-t border-border px-4 py-2">
      <button
        onclick={() => (uiStore.commitDialogOpen = false)}
        class="rounded px-3 py-1.5 text-sm text-text-secondary hover:bg-bg-hover"
      >
        {i18n.t.cancel}
      </button>
      <button
        onclick={handleCommit}
        disabled={!message.trim() || selectedFiles.size === 0 || isCommitting}
        class="inline-flex items-center gap-1 rounded bg-accent px-3 py-1.5 text-sm font-medium text-white hover:bg-accent-hover disabled:opacity-50"
      >
        <Check size={14} />
        {isCommitting ? i18n.t.committing : i18n.t.commit}
      </button>
    </div>
  </div>
</div>
