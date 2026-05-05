mod commands;
mod platform;

use commands::undo::AppUndoStack;
use draction_app_core::DractionRuntime;
use draction_events::EventBus;
use std::sync::Arc;
use tauri::Manager;
use tracing_subscriber::EnvFilter;

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

    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle().clone();

            tauri::async_runtime::block_on(async move {
                let runtime = DractionRuntime::bootstrap()
                    .await
                    .map_err(|e| format!("Bootstrap failed: {e}"))?;

                tracing::info!("Draction bootstrapped, API on port {}", runtime.api_port);

                handle.manage(ApiPort(runtime.api_port));
                handle.manage(AppEventBus(runtime.event_bus.clone()));
                handle.manage(AppUndoStack(runtime.undo_stack.clone()));
                handle.manage(runtime);

                Ok::<_, String>(())
            })?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            commands::auth::get_auth_token,
            commands::ingest::ingest_files,
            commands::overlay::set_overlay_visible,
            commands::settings::get_api_port,
            commands::undo::undo_last_ingest,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Draction");
}
