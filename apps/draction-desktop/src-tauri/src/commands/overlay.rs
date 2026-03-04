/// Tauri commands for the overlay window

use tauri::WebviewWindow;

#[tauri::command]
pub fn set_overlay_visible(window: WebviewWindow, visible: bool) -> Result<(), String> {
    if visible {
        window.show().map_err(|e| e.to_string())
    } else {
        window.hide().map_err(|e| e.to_string())
    }
}
