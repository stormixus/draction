use anyhow::Result;
use notify::{Event, EventKind, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::mpsc;

use crate::DractionRuntime;

pub struct FolderWatcher {
    stop_flag: Arc<AtomicBool>,
}

impl FolderWatcher {
    pub async fn start(runtime: DractionRuntime, watch_paths: Vec<PathBuf>) -> Result<Self> {
        let stop_flag = Arc::new(AtomicBool::new(false));
        let flag = stop_flag.clone();
        let (tx, mut rx) = mpsc::channel::<PathBuf>(256);

        // Debounce: collect files arriving within 500ms windows
        let runtime_clone = runtime.clone();
        tokio::spawn(async move {
            let mut pending: Vec<PathBuf> = Vec::new();
            loop {
                tokio::select! {
                    path = rx.recv() => {
                        match path {
                            Some(p) => pending.push(p),
                            None => break,
                        }
                        // Debounce: wait 500ms for more events
                        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                        let files = std::mem::take(&mut pending);
                        if !files.is_empty() {
                            let paths = files.into_iter().collect::<Vec<_>>();
                            tracing::info!("Watch folder detected {} file(s)", paths.len());
                            if let Err(e) = runtime_clone.ingest_paths(paths, None).await {
                                tracing::error!("Watch folder ingest failed: {}", e);
                            }
                        }
                    }
                }
            }
        });

        // Set up filesystem watcher
        let mut watcher = notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
            if let Ok(event) = res {
                if matches!(event.kind, EventKind::Create(_)) {
                    for path in event.paths {
                        let _ = tx.blocking_send(path);
                    }
                }
            }
        })?;

        for path in &watch_paths {
            watcher.watch(path, RecursiveMode::NonRecursive)?;
            tracing::info!("Watching folder: {}", path.display());
        }

        // Keep watcher alive
        let keep_flag = flag.clone();
        tokio::spawn(async move {
            let _watcher = watcher;
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                if keep_flag.load(Ordering::Relaxed) {
                    break;
                }
            }
        });

        Ok(Self { stop_flag })
    }

    pub fn stop_flag(&self) -> &Arc<AtomicBool> {
        &self.stop_flag
    }

    pub fn stop(&self) {
        self.stop_flag.store(true, Ordering::Relaxed);
    }
}
