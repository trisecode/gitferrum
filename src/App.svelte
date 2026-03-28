<script lang="ts">
  import AppShell from "$lib/components/AppShell.svelte";
  import Toast from "$lib/components/Toast.svelte";
  import UpdateChecker from "$lib/components/UpdateChecker.svelte";
  import { repoStore } from "$lib/stores/repo.svelte";
  import { uiStore } from "$lib/stores/ui.svelte";
  import { toastStore } from "$lib/stores/toast.svelte";
  import { setupWatcher } from "$lib/services/watcher";
  import { getStatus } from "$lib/services/git";
  import { open } from "@tauri-apps/plugin-dialog";
  import { openAndLoadRepo, restoreSession } from "$lib/services/repo-actions";

  // Restore previous session on startup (once)
  let sessionRestored = false;
  $effect(() => {
    if (!sessionRestored) {
      sessionRestored = true;
      restoreSession();
    }
  });

  // Global watcher: listens for repo-changed events with repo path payload
  $effect(() => {
    let unlisten: (() => void) | undefined;

    setupWatcher(async (changedPath: string) => {
      const repo = repoStore.repos.get(changedPath);
      if (!repo) return;
      try {
        repo.status = await getStatus(changedPath);
      } catch {
        // Silently ignore watcher refresh errors
      }
    }).then((fn) => (unlisten = fn));

    return () => unlisten?.();
  });

  // Show errors as toasts
  $effect(() => {
    const repo = repoStore.activeRepo;
    if (repo?.error) {
      toastStore.error(repo.error);
      repo.error = null;
    }
  });

  // Global keyboard shortcuts
  async function handleKeydown(e: KeyboardEvent) {
    const mod = e.metaKey || e.ctrlKey;

    // Cmd+O: Open repository (adds, doesn't replace)
    if (mod && e.key === "o") {
      e.preventDefault();
      const selected = await open({ directory: true, multiple: false });
      if (!selected) return;
      const path = typeof selected === "string" ? selected : selected[0];
      if (!path) return;
      await openAndLoadRepo(path);
      return;
    }

    // Cmd+Enter: Open commit dialog
    if (mod && e.key === "Enter" && repoStore.isRepoOpen) {
      e.preventDefault();
      uiStore.commitDialogOpen = true;
      return;
    }

    // Escape: Close detail pane or commit dialog
    if (e.key === "Escape") {
      if (uiStore.commitDialogOpen) {
        uiStore.commitDialogOpen = false;
      } else if (uiStore.detailPaneOpen) {
        uiStore.closeDetailPane();
        if (repoStore.activeRepo) {
          repoStore.activeRepo.selectedCommit = null;
          repoStore.activeRepo.diff = null;
        }
      }
      return;
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<AppShell />
<Toast />
<UpdateChecker />
