import { listen } from "@tauri-apps/api/event";

export function setupWatcher(
  onRepoChanged: (repoPath: string) => void,
): Promise<() => void> {
  return listen<string>("repo-changed", (event) => {
    onRepoChanged(event.payload);
  }).then((unlisten) => unlisten);
}
