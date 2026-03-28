use notify_debouncer_mini::DebouncedEventKind;
use std::path::PathBuf;
use std::sync::mpsc;
use std::time::Duration;
use tauri::Emitter;

pub struct WatcherHandle {
    stop_tx: mpsc::Sender<()>,
}

impl WatcherHandle {
    pub fn stop(&self) {
        let _ = self.stop_tx.send(());
    }
}

impl Drop for WatcherHandle {
    fn drop(&mut self) {
        self.stop();
    }
}

pub fn start_watcher(app_handle: tauri::AppHandle, repo_path: PathBuf) -> WatcherHandle {
    let (stop_tx, stop_rx) = mpsc::channel();
    let path_payload = repo_path.to_string_lossy().to_string();

    std::thread::spawn(move || {
        let (tx, rx) = mpsc::channel();

        let mut debouncer =
            match notify_debouncer_mini::new_debouncer(Duration::from_millis(500), tx) {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("Failed to create file watcher: {}", e);
                    return;
                }
            };

        if let Err(e) = debouncer
            .watcher()
            .watch(&repo_path, notify::RecursiveMode::Recursive)
        {
            eprintln!("Failed to watch path: {}", e);
            return;
        }

        loop {
            if stop_rx.try_recv().is_ok() {
                break;
            }

            match rx.recv_timeout(Duration::from_secs(1)) {
                Ok(Ok(events)) => {
                    let meaningful = events.iter().any(|e| {
                        e.kind == DebouncedEventKind::Any
                            && !e.path.to_string_lossy().contains(".git/objects")
                            && !e.path.to_string_lossy().contains(".git/logs")
                    });
                    if meaningful {
                        let _ = app_handle.emit("repo-changed", &path_payload);
                    }
                }
                Ok(Err(_)) => {}
                Err(mpsc::RecvTimeoutError::Timeout) => {}
                Err(mpsc::RecvTimeoutError::Disconnected) => break,
            }
        }
    });

    WatcherHandle { stop_tx }
}
