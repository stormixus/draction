use tauri::WebviewWindow;

/// Set the overlay window to ignore mouse events on macOS.
/// Uses NSWindow.ignoresMouseEvents via raw window handle.
pub fn set_click_through(window: &WebviewWindow, click_through: bool) {
    // TODO: Implement via objc2 crate or raw NSWindow pointer
    // For now, log the intent
    tracing::debug!(
        label = window.label(),
        click_through,
        "set_click_through (macOS stub)"
    );
}
