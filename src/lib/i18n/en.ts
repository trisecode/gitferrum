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

  // File status
  binary: "(binary)",
  head: "HEAD",
  stagedBadge: "S",

  // Detail pane
  filesChanged: (n: number) => `${n} file${n !== 1 ? "s" : ""} changed`,
  parents: "Parents",
};
