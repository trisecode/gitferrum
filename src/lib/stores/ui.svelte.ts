class UIStore {
  sidebarCollapsed = $state(false);
  detailPaneOpen = $state(false);
  changesPanelOpen = $state(true);
  commitDialogOpen = $state(false);
  cloneDialogOpen = $state(false);
  pushConfirm = $state<{ branch: string; remote: string } | null>(null);
  settingsOpen = $state(false);
  mergeConflict = $state<{ files: string[]; repoPath: string } | null>(null);
  createTag = $state<{ commit: string; shortOid: string } | null>(null);
  resetConfirm = $state<{ commit: string; shortOid: string; mode: string } | null>(null);
  mergeDialogOpen = $state<{ preselectedBranch?: string } | null>(null);
  deleteBranchConfirm = $state<{ name: string; force: boolean } | null>(null);
  deleteRemoteBranchConfirm = $state<{ fullName: string; remote: string; branch: string } | null>(null);

  toggleSidebar() {
    this.sidebarCollapsed = !this.sidebarCollapsed;
  }

  openDetailPane() {
    this.detailPaneOpen = true;
  }

  closeDetailPane() {
    this.detailPaneOpen = false;
  }

  toggleCommitDialog() {
    this.commitDialogOpen = !this.commitDialogOpen;
  }
}

export const uiStore = new UIStore();
