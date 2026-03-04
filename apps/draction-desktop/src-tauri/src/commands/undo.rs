use draction_inbox::undo::{UndoEntry, UndoStack};
use std::sync::{Arc, Mutex};

pub struct AppUndoStack(pub Arc<Mutex<UndoStack>>);

#[tauri::command]
pub async fn undo_last_ingest(
    event_id: String,
    state: tauri::State<'_, AppUndoStack>,
) -> Result<String, String> {
    let entry = {
        let mut stack = state.0.lock().map_err(|e| format!("Lock poisoned: {e}"))?;
        stack
            .try_undo(&event_id)
            .map_err(|e| format!("Undo error: {e}"))?
    };

    match entry {
        None => Err("Nothing to undo or expired".into()),
        Some(UndoEntry {
            src_path,
            dst_path,
            is_copy,
            ..
        }) => {
            if is_copy {
                // File was copied to inbox — delete the copy
                tokio::fs::remove_file(&dst_path)
                    .await
                    .map_err(|e| format!("Failed to remove inbox copy: {e}"))?;
                tracing::info!(path = %dst_path, "Undo: removed inbox copy");
            } else {
                // File was moved — move it back
                tokio::fs::rename(&dst_path, &src_path)
                    .await
                    .map_err(|e| format!("Failed to restore file: {e}"))?;
                tracing::info!(from = %dst_path, to = %src_path, "Undo: restored file");
            }
            Ok(src_path)
        }
    }
}
