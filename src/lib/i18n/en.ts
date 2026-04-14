export default {
  // App
  appName: "GitFerrum",
  appSubtitle: "High-performance Git client",

  // Actions
  openRepository: "Open Repository",
  cloneRepository: "Clone Repository",
  cancel: "Cancel",
  commit: "Commit",
  create: "Create",
  fetch: "Fetch",
  pull: "Pull",
  push: "Push",
  close: "Close",
  browse: "Browse",
  selectAll: "Select all",
  deselectAll: "Deselect all",
  unstageAll: "Unstage all",

  // Loading states
  opening: "Opening...",
  cloning: "Cloning...",
  committing: "Committing...",
  loadingCommits: "Loading commits...",
  loadingMore: "Loading more...",
  loading: "Loading...",

  // Sections
  repositories: "Repositories",
  branches: "Branches",
  remotes: "Remotes",
  tags: "Tags",
  stashes: "Stashes",
  commits: "Commits",
  details: "Details",
  changes: "Changes",
  staged: "Staged",
  modified: "Modified",
  untracked: "Untracked",

  // Empty states
  noChanges: "No changes",
  noBranches: "No branches",
  noRemotes: "No remotes",
  noTags: "No tags",
  noStashes: "No stashes",
  noReposOpen: "No repositories open",
  noCommitsFound: "No commits found",

  // Commit dialog
  commitChanges: "Commit Changes",
  commitMessage: "Commit message...",
  files: "Files",

  // Clone dialog
  repositoryUrl: "Repository URL",
  urlPlaceholder: "https://github.com/user/repo.git",
  cloneTo: "Clone to",
  destPlaceholder: "/path/to/destination",

  // Changes panel
  stageNFiles: (n: number) => `Stage ${n} file${n !== 1 ? "s" : ""}`,
  noStagedChanges: "No staged changes to commit",
  commitCreated: "Commit created",
  nChanges: (n: number) => `${n} changes`,

  // Context menu - remotes
  createLocalBranch: "Create local branch",
  createCustomName: "Create with custom name...",
  browseDetached: "Browse (detached)",
  newBranchName: "New branch name",

  // Tooltips
  openRepoTooltip: "Open repository",
  cloneRepoTooltip: "Clone repository",
  closeRepoTooltip: "Close repository",
  fetchTooltip: "Fetch",
  pullTooltip: (n: number) => `Pull — ${n} commit${n !== 1 ? "s" : ""} behind`,
  pushTooltip: (n: number) => `Push — ${n} commit${n !== 1 ? "s" : ""} ahead`,

  // Toasts
  repoOpened: (name: string) => `Opened ${name}`,
  repoCloned: (name: string) => `Cloned ${name}`,
  fetchedFromRemote: "Fetched from remote",
  pulledFromRemote: "Pulled from remote",
  pushedToRemote: "Pushed to remote",
  pushNewBranchTitle: "New remote branch",
  pushNewBranchMessage: (branch: string, remote: string) =>
    `Branch "${branch}" does not exist on "${remote}". It will be created and pushed.`,
  pushNewBranchConfirm: "Create & Push",
  cannotPushDetached: "Cannot push from detached HEAD",
  noRemoteConfigured: "No remote configured. Add a remote first with:\ngit remote add origin <url>",
  pushTimeout: "Push timed out. Make sure your SSH key is loaded:\nssh-add ~/.ssh/your_key",
  pushRejected: "Push rejected: the remote has new commits. Pull first to merge changes.",
  mergeConflictsTitle: "Merge Conflicts",
  mergeConflictsMessage: (n: number) => `${n} file${n !== 1 ? "s have" : " has"} merge conflicts. Resolve them manually, then stage and commit.`,
  abortMerge: "Abort Merge",
  resolveManually: "Resolve Manually",
  conflictFiles: "Conflicting files",
  repoStateMerging: "MERGING",
  repoStateRebasing: "REBASING",
  repoStateCherryPick: "CHERRY-PICKING",
  repoStateReverting: "REVERTING",
  checkedOut: (name: string) => `Checked out ${name}`,
  branchCreated: (name: string) => `Created branch ${name}`,
  browsingDetached: (ref: string) => `Browsing ${ref} (detached HEAD)`,

  // Relative time
  justNow: "just now",
  minutesAgo: (n: number) => `${n}m ago`,
  hoursAgo: (n: number) => `${n}h ago`,
  daysAgo: (n: number) => `${n}d ago`,
  weeksAgo: (n: number) => `${n}w ago`,
  monthsAgo: (n: number) => `${n}mo ago`,
  yearsAgo: (n: number) => `${n}y ago`,

  // Updates
  updateAvailable: "Update Available",
  updateMessage: (version: string) => `Version ${version} is available. Would you like to update now?`,
  updateNow: "Update Now",
  updating: "Updating...",
  updateLater: "Later",
  upToDate: "You're up to date",
  checkingUpdates: "Checking for updates...",
  updateFailed: "Update failed",
  updateRestart: "Update installed. Restart to apply.",

  // Settings
  settings: "Settings",
  language: "Language",
  settingsTooltip: "Settings",
  general: "General",
  about: "About",
  version: "Version",
  checkForUpdates: "Check for updates",
  gitConfiguration: "Git Configuration",
  gitConfigDescription: "Configure your global Git identity. This is used for all repositories that don't have a local configuration.",
  gitUserName: "Name",
  gitUserEmail: "Email",
  gitUserNamePlaceholder: "Your name",
  gitUserEmailPlaceholder: "your@email.com",
  gitConfigSaved: "Git configuration saved",
  save: "Save",
  website: "Website",

  // File status
  binary: "(binary)",
  head: "HEAD",
  stagedBadge: "S",

  // Detail pane
  filesChanged: (n: number) => `${n} file${n !== 1 ? "s" : ""} changed`,
  parents: "Parents",

  // Branch operations
  deleteBranch: "Delete branch",
  forceDeleteBranch: "Force delete branch",
  renameBranch: "Rename branch...",
  mergeBranchInto: "Merge into current",
  branchDeleted: (name: string) => `Deleted branch ${name}`,
  branchRenamed: (oldName: string, newName: string) => `Renamed ${oldName} to ${newName}`,
  cannotDeleteHead: "Cannot delete the current branch",
  newBranchNameLabel: "New branch name",

  // Remote branch operations
  deleteRemoteBranch: "Delete remote branch",
  deleteRemoteBranchConfirm: (name: string) => `Are you sure you want to delete the remote branch "${name}"? This cannot be undone.`,
  remoteBranchDeleted: (name: string) => `Deleted remote branch ${name}`,

  // Tag operations
  createTagLabel: "Create tag...",
  deleteTagLabel: "Delete tag",
  deleteRemoteTagLabel: "Delete tag from remote",
  tagName: "Tag name",
  tagMessage: "Message (optional, for annotated tag)",
  tagMessagePlaceholder: "Tag message...",
  tagCreated: (name: string) => `Created tag ${name}`,
  tagDeleted: (name: string) => `Deleted tag ${name}`,
  remoteTagDeleted: (name: string) => `Deleted remote tag ${name}`,

  // Amend
  amendLastCommit: "Amend last commit",
  amend: "Amend",
  commitAmended: "Commit amended",

  // Reset
  resetSoft: "Reset --soft to here",
  resetMixed: "Reset --mixed to here",
  resetHard: "Reset --hard to here",
  resetHardWarning: "This will discard all changes in your working directory. This cannot be undone.",
  resetComplete: (mode: string) => `Reset (${mode}) complete`,

  // Stash operations
  applyStash: "Apply",
  dropStash: "Drop",
  stashApplied: (index: number) => `Applied stash@{${index}}`,
  stashDropped: (index: number) => `Dropped stash@{${index}}`,
  confirmDropStash: "Are you sure you want to drop this stash? This cannot be undone.",

  // Cherry-pick & Revert
  cherryPickCommit: "Cherry-pick",
  cherryPickApplied: "Cherry-pick applied",
  revertCommitLabel: "Revert commit",
  commitReverted: "Commit reverted",

  // Copy
  copySha: "Copy SHA",
  shaCopied: "SHA copied to clipboard",

  // Merge
  mergeStarted: (name: string) => `Merged ${name}`,
  confirm: "Confirm",
  deleteConfirm: "Delete",

  // Merge dialog
  mergeBranches: "Merge Branches",
  mergeFrom: "Merge from",
  mergeInto: "Into (current branch)",
  mergeAction: "Merge",
  merging: "Merging...",
  selectBranch: "Select a branch...",
  mergeDescription: (source: string, target: string) => `This will merge "${source}" into "${target}". If there are conflicts, you will need to resolve them manually.`,

  // Delete branch confirmation
  deleteBranchTitle: "Delete Branch",
  deleteBranchConfirm: (name: string) => `Are you sure you want to delete the local branch "${name}"?`,
  forceDeleteBranchTitle: "Force Delete Branch",
  forceDeleteBranchConfirm: (name: string) => `Are you sure you want to force delete the local branch "${name}"? Unmerged changes will be lost.`,

  // Discard changes
  discardChanges: "Discard changes",
  discardAllChanges: "Discard all changes",
  discardFileTitle: "Discard Changes",
  discardFileMessage: "Discard all changes to this file? This cannot be undone.",
  discardAllTitle: "Discard All Changes",
  discardAllMessage: "Discard all unstaged changes and remove all untracked files? This cannot be undone.",
  changesDiscarded: "Changes discarded",
  allChangesDiscarded: "All changes discarded",
};
