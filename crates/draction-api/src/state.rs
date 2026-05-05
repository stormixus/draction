use draction_db::DractionDb;
use draction_events::EventBus;
use draction_inbox::undo::UndoStack;
use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DractionDb>,
    pub base_dir: PathBuf,
    pub auth_token: String,
    pub event_bus: Arc<EventBus>,
    pub undo_stack: Arc<Mutex<UndoStack>>,
    pub watcher_flag: Arc<Mutex<Option<Arc<AtomicBool>>>>,
    pub watcher_tx: Arc<Mutex<Option<mpsc::UnboundedSender<Vec<PathBuf>>>>>,
}
