/// Tauri commands for settings management
use crate::ApiPort;
use draction_app_core::DractionRuntime;
use draction_domain::settings::Settings;
use std::path::{Path, PathBuf};
use std::process::Command;
use tauri::{AppHandle, Manager, State};

#[tauri::command]
pub fn get_api_port(port: State<ApiPort>) -> u16 {
    port.0
}

#[tauri::command]
pub async fn rotate_auth_token(app_handle: AppHandle) -> Result<String, String> {
    let runtime = app_handle.state::<DractionRuntime>();
    let token = draction_api::auth::generate_token();
    draction_api::auth::save_token(&runtime.base_dir, &token)
        .await
        .map_err(|e| e.to_string())?;
    *runtime
        .auth_token_cell
        .write()
        .map_err(|e| format!("auth token lock: {e}"))? = token.clone();
    Ok(token)
}

#[tauri::command]
pub async fn reset_settings_section(
    section: String,
    app_handle: AppHandle,
) -> Result<Settings, String> {
    let runtime = app_handle.state::<DractionRuntime>();
    let mut settings = Settings::load(&runtime.base_dir)
        .await
        .map_err(|e| e.to_string())?;
    let defaults = Settings::default();

    match section.as_str() {
        "general" => {
            settings.launch_at_login = defaults.launch_at_login;
            settings.show_draky_on_launch = defaults.show_draky_on_launch;
            settings.run_minimized = defaults.run_minimized;
            settings.theme = defaults.theme;
            settings.accent_color = defaults.accent_color;
            settings.reduce_motion = defaults.reduce_motion;
            settings.language = defaults.language;
            settings.date_format = defaults.date_format;
        }
        "inbox" => {
            settings.inbox_location = defaults.inbox_location;
            settings.conflict_resolution = defaults.conflict_resolution;
            settings.delete_source_after_ingest = defaults.delete_source_after_ingest;
            settings.date_subfolders = defaults.date_subfolders;
            settings.inbox_size_limit_gb = defaults.inbox_size_limit_gb;
            settings.auto_archive_days = defaults.auto_archive_days;
            settings.undo_window_seconds = defaults.undo_window_seconds;
            settings.undo_history_depth = defaults.undo_history_depth;
        }
        "draky" => {
            settings.draky_size = defaults.draky_size;
            settings.draky_personality = defaults.draky_personality;
            settings.draky_always_on_top = defaults.draky_always_on_top;
            settings.draky_overlay_visible = defaults.draky_overlay_visible;
            settings.draky_snap_to_corner = defaults.draky_snap_to_corner;
            settings.draky_burp_on_success = defaults.draky_burp_on_success;
            settings.draky_idle_behaviors = defaults.draky_idle_behaviors;
            settings.draky_file_type_munch = defaults.draky_file_type_munch;
        }
        _ => return Err(format!("Unknown settings section: {section}")),
    }

    settings
        .save(&runtime.base_dir)
        .await
        .map_err(|e| e.to_string())?;
    Ok(settings)
}

#[tauri::command]
pub fn reveal_path(path: String, app_handle: AppHandle) -> Result<(), String> {
    let runtime = app_handle.state::<DractionRuntime>();
    let path = expand_runtime_path(&path, &runtime.base_dir);
    if cfg!(target_os = "macos") {
        Command::new("open")
            .arg("-R")
            .arg(&path)
            .status()
            .map_err(|e| e.to_string())?;
    } else {
        open_path(path.to_string_lossy().to_string(), app_handle)?;
    }
    Ok(())
}

#[tauri::command]
pub fn open_path(path: String, app_handle: AppHandle) -> Result<(), String> {
    let runtime = app_handle.state::<DractionRuntime>();
    let path = expand_runtime_path(&path, &runtime.base_dir);
    let target = if path.is_file() {
        path.parent().unwrap_or(&path).to_path_buf()
    } else {
        path
    };

    let mut cmd = if cfg!(target_os = "macos") {
        let mut c = Command::new("open");
        c.arg(&target);
        c
    } else if cfg!(target_os = "windows") {
        let mut c = Command::new("explorer");
        c.arg(&target);
        c
    } else {
        let mut c = Command::new("xdg-open");
        c.arg(&target);
        c
    };
    cmd.status().map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn clear_database(app_handle: AppHandle) -> Result<(), String> {
    let runtime = app_handle.state::<DractionRuntime>();
    runtime.db.clear_history().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn erase_runtime_data(app_handle: AppHandle) -> Result<(), String> {
    let runtime = app_handle.state::<DractionRuntime>();
    runtime.db.clear_history().map_err(|e| e.to_string())?;

    for name in [
        "settings.json",
        "rules.json",
        "workflows.json",
        "config.json",
        "Inbox",
        "Archive",
        "Work",
        "logs",
    ] {
        let path = runtime.base_dir.join(name);
        if path.is_dir() {
            tokio::fs::remove_dir_all(&path)
                .await
                .map_err(|e| e.to_string())?;
        } else if path.exists() {
            tokio::fs::remove_file(&path)
                .await
                .map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}

fn expand_runtime_path(path: &str, base_dir: &Path) -> PathBuf {
    if path == "~/Draction" {
        return base_dir.to_path_buf();
    }
    if let Some(rest) = path.strip_prefix("~/Draction/") {
        return base_dir.join(rest);
    }
    if let Some(rest) = path.strip_prefix("~/") {
        if let Some(home) = dirs::home_dir() {
            return home.join(rest);
        }
    }
    PathBuf::from(path)
}
