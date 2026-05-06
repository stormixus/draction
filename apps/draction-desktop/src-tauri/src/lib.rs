mod commands;
mod platform;

use commands::undo::AppUndoStack;
use draction_app_core::DractionRuntime;
use draction_events::EventBus;
use std::sync::Arc;
use tauri::{Emitter, Manager, WindowEvent};
use tracing_subscriber::EnvFilter;

pub struct ApiPort(pub u16);

/// Tauri managed state wrapping the shared EventBus.
pub struct AppEventBus(pub Arc<EventBus>);

const SETTINGS_MENU_ID: &str = "draction-settings";
const OPEN_SETTINGS_EVENT: &str = "open-settings";

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to Draction.", name)
}

#[cfg(target_os = "macos")]
fn build_app_menu(app: &tauri::App<tauri::Wry>) -> tauri::Result<tauri::menu::Menu<tauri::Wry>> {
    use tauri::menu::{
        AboutMetadata, HELP_SUBMENU_ID, Menu, MenuItem, PredefinedMenuItem, Submenu,
        WINDOW_SUBMENU_ID,
    };

    let handle = app.handle();
    let pkg_info = handle.package_info();
    let config = handle.config();
    let about_metadata = AboutMetadata {
        name: Some("Draction".to_string()),
        version: Some(pkg_info.version.to_string()),
        copyright: config.bundle.copyright.clone(),
        authors: config.bundle.publisher.clone().map(|p| vec![p]),
        ..Default::default()
    };
    let settings_item = MenuItem::with_id(
        handle,
        SETTINGS_MENU_ID,
        "Settings...",
        true,
        Some("CmdOrCtrl+,"),
    )?;

    let app_menu = Submenu::with_items(
        handle,
        "Draction",
        true,
        &[
            &PredefinedMenuItem::about(handle, None, Some(about_metadata))?,
            &PredefinedMenuItem::separator(handle)?,
            &settings_item,
            &PredefinedMenuItem::separator(handle)?,
            &PredefinedMenuItem::services(handle, None)?,
            &PredefinedMenuItem::separator(handle)?,
            &PredefinedMenuItem::hide(handle, None)?,
            &PredefinedMenuItem::hide_others(handle, None)?,
            &PredefinedMenuItem::separator(handle)?,
            &PredefinedMenuItem::quit(handle, None)?,
        ],
    )?;
    let file_menu = Submenu::with_items(
        handle,
        "File",
        true,
        &[&PredefinedMenuItem::close_window(handle, None)?],
    )?;
    let edit_menu = Submenu::with_items(
        handle,
        "Edit",
        true,
        &[
            &PredefinedMenuItem::undo(handle, None)?,
            &PredefinedMenuItem::redo(handle, None)?,
            &PredefinedMenuItem::separator(handle)?,
            &PredefinedMenuItem::cut(handle, None)?,
            &PredefinedMenuItem::copy(handle, None)?,
            &PredefinedMenuItem::paste(handle, None)?,
            &PredefinedMenuItem::select_all(handle, None)?,
        ],
    )?;
    let view_menu = Submenu::with_items(
        handle,
        "View",
        true,
        &[&PredefinedMenuItem::fullscreen(handle, None)?],
    )?;
    let window_menu = Submenu::with_id_and_items(
        handle,
        WINDOW_SUBMENU_ID,
        "Window",
        true,
        &[
            &PredefinedMenuItem::minimize(handle, None)?,
            &PredefinedMenuItem::maximize(handle, None)?,
            &PredefinedMenuItem::separator(handle)?,
            &PredefinedMenuItem::close_window(handle, None)?,
        ],
    )?;
    let help_menu = Submenu::with_id_and_items(handle, HELP_SUBMENU_ID, "Help", true, &[])?;

    Menu::with_items(
        handle,
        &[
            &app_menu,
            &file_menu,
            &edit_menu,
            &view_menu,
            &window_menu,
            &help_menu,
        ],
    )
}

#[cfg(not(target_os = "macos"))]
fn build_app_menu(app: &tauri::App<tauri::Wry>) -> tauri::Result<tauri::menu::Menu<tauri::Wry>> {
    tauri::menu::Menu::default(app.handle())
}

fn open_settings_window(app: &tauri::AppHandle<tauri::Wry>) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
        let _ = window.emit(OPEN_SETTINGS_EVENT, ());
        let _ = window.eval("window.__dractionOpenSettings?.()");
    } else {
        let _ = app.emit(OPEN_SETTINGS_EVENT, ());
    }
}

fn handle_menu_event(app: &tauri::AppHandle<tauri::Wry>, event: tauri::menu::MenuEvent) {
    tracing::debug!(menu_id = %event.id().as_ref(), "menu event");
    if event.id() == SETTINGS_MENU_ID {
        open_settings_window(app);
    }
}

fn handle_window_event(window: &tauri::Window<tauri::Wry>, event: &WindowEvent) {
    if window.label() != "main" {
        return;
    }

    if let WindowEvent::CloseRequested { api, .. } = event {
        api.prevent_close();
        let _ = window.hide();
    }
}

pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    tracing::info!("Draction starting...");

    tauri::Builder::default()
        .on_menu_event(handle_menu_event)
        .on_window_event(handle_window_event)
        .setup(|app| {
            app.set_menu(build_app_menu(app)?)?;

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
            commands::settings::clear_database,
            commands::settings::erase_runtime_data,
            commands::settings::get_api_port,
            commands::settings::open_path,
            commands::settings::reset_settings_section,
            commands::settings::reveal_path,
            commands::settings::rotate_auth_token,
            commands::undo::undo_last_ingest,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Draction");
}
