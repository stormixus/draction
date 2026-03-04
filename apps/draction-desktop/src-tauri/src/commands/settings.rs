/// Tauri commands for settings management

use tauri::State;
use crate::ApiPort;

#[tauri::command]
pub fn get_api_port(port: State<ApiPort>) -> u16 {
    port.0
}
