use draction_app_core::DractionRuntime;
use tauri::{AppHandle, Manager};

#[tauri::command]
pub fn get_auth_token(app_handle: AppHandle) -> String {
    app_handle
        .state::<DractionRuntime>()
        .auth_token_cell
        .read()
        .map(|token| token.clone())
        .unwrap_or_else(|_| app_handle.state::<DractionRuntime>().auth_token.clone())
}
