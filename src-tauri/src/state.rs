use crate::error::AppError;
use crate::watcher::WatcherHandle;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct RepoEntry {
    pub watcher_handle: Option<WatcherHandle>,
}

pub struct AppState {
    pub repos: Mutex<HashMap<PathBuf, RepoEntry>>,
    pub active_repo: Mutex<Option<PathBuf>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            repos: Mutex::new(HashMap::new()),
            active_repo: Mutex::new(None),
        }
    }

    pub fn add_repo(&self, path: PathBuf) {
        self.repos
            .lock()
            .entry(path.clone())
            .or_insert(RepoEntry {
                watcher_handle: None,
            });
        *self.active_repo.lock() = Some(path);
    }

    pub fn remove_repo(&self, path: &PathBuf) -> Option<PathBuf> {
        let mut repos = self.repos.lock();
        repos.remove(path); // Drop triggers WatcherHandle::drop → stops watcher

        let mut active = self.active_repo.lock();
        if active.as_ref() == Some(path) {
            *active = repos.keys().next().cloned();
        }
        active.clone()
    }

    pub fn set_active_repo(&self, path: PathBuf) -> Result<(), AppError> {
        if !self.repos.lock().contains_key(&path) {
            return Err(AppError::PathNotFound(path.to_string_lossy().to_string()));
        }
        *self.active_repo.lock() = Some(path);
        Ok(())
    }

    pub fn get_open_repos(&self) -> Vec<PathBuf> {
        self.repos.lock().keys().cloned().collect()
    }

    pub fn set_watcher_handle(&self, path: &PathBuf, handle: WatcherHandle) {
        if let Some(entry) = self.repos.lock().get_mut(path) {
            entry.watcher_handle = Some(handle);
        }
    }
}
