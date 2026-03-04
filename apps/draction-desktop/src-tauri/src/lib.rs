mod commands;
mod platform;

use commands::undo::AppUndoStack;
use draction_events::EventBus;
use draction_inbox::undo::UndoStack;
use std::sync::{Arc, Mutex};
use tauri::Manager;
use tracing_subscriber::EnvFilter;
use dirs;

pub struct ApiPort(pub u16);

/// Tauri managed state wrapping the shared EventBus.
pub struct AppEventBus(pub Arc<EventBus>);

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to Draction.", name)
}

pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    tracing::info!("Draction starting...");

    let undo_stack = AppUndoStack(Arc::new(Mutex::new(UndoStack::new())));

    tauri::Builder::default()
        .manage(undo_stack)
        .setup(|app| {
            // Acquire process lock
            let home = dirs::home_dir().ok_or("Cannot find home directory")?;
            let base = home.join("Draction");
            std::fs::create_dir_all(&base)?;
            let lock_path = base.join(".lock");
            draction_lifecycle::lock::acquire_lock(&lock_path)
                .map_err(|e| format!("Failed to acquire lock: {e}"))?;
            tracing::info!("Process lock acquired at {}", lock_path.display());

            // Open database
            let db_path = base.join("draction.db");
            let db = draction_db::DractionDb::open(&db_path)
                .map_err(|e| format!("Failed to open database: {e}"))?;

            // Load or create auth token
            let auth_token = draction_api::auth::load_or_create_token(&base)
                .map_err(|e| format!("Failed to load auth token: {e}"))?;

            // Create shared EventBus
            let event_bus = Arc::new(EventBus::new(256));
            app.manage(AppEventBus(event_bus.clone()));

            let app_state = draction_api::state::AppState {
                db: Arc::new(db),
                base_dir: base.clone(),
                auth_token,
                event_bus: event_bus.clone(),
            };

            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                match draction_api::start_server(9400, app_state).await {
                    Ok(port) => {
                        tracing::info!("API server started on port {}", port);
                        handle.manage(ApiPort(port));
                    }
                    Err(e) => {
                        tracing::error!("Failed to start API server: {}", e);
                        handle.manage(ApiPort(0));
                    }
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            commands::ingest::ingest_files,
            commands::overlay::set_overlay_visible,
            commands::settings::get_api_port,
            commands::undo::undo_last_ingest,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Draction");
}
