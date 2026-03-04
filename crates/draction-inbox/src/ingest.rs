use anyhow::Result;
use std::path::{Path, PathBuf};
use chrono::Utc;

/// Returns the inbox directory for today: ~/Draction/Inbox/YYYY-MM-DD/
pub fn inbox_dir(base: &Path) -> PathBuf {
    let date = Utc::now().format("%Y-%m-%d").to_string();
    base.join("Inbox").join(date)
}

/// Move or copy a file into the inbox directory.
pub async fn ingest_file(src: &Path, inbox: &Path, copy_mode: bool) -> Result<PathBuf> {
    tokio::fs::create_dir_all(inbox).await?;
    let dest = resolve_dest(inbox, src)?;
    if copy_mode {
        tokio::fs::copy(src, &dest).await?;
    } else {
        tokio::fs::rename(src, &dest).await?;
    }
    Ok(dest)
}

/// Resolve destination path, adding numeric suffix on collision.
fn resolve_dest(inbox: &Path, src: &Path) -> Result<PathBuf> {
    let name = src.file_stem().unwrap_or_default().to_string_lossy();
    let ext = src.extension().map(|e| e.to_string_lossy().to_string());
    let mut dest = inbox.join(src.file_name().unwrap_or_default());
    let mut i = 1u32;
    while dest.exists() {
        let new_name = match &ext {
            Some(e) => format!("{name}_{i}.{e}"),
            None => format!("{name}_{i}"),
        };
        dest = inbox.join(new_name);
        i += 1;
    }
    Ok(dest)
}
