use draction_db::DractionDb;
use draction_events::EventBus;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DractionDb>,
    pub base_dir: PathBuf,
    pub auth_token: String,
    pub event_bus: Arc<EventBus>,
}
