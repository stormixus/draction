#[cfg(target_os = "macos")]
pub mod macos;

/// Make the overlay window click-through (mouse events pass to windows below).
pub fn set_overlay_click_through(_window: &tauri::WebviewWindow, _click_through: bool) {
    #[cfg(target_os = "macos")]
    macos::set_click_through(_window, _click_through);
}
