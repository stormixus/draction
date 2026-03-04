use anyhow::Result;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

pub fn acquire_lock(lock_path: &Path) -> Result<File> {
    if lock_path.exists() {
        let pid = fs::read_to_string(lock_path)?;
        tracing::warn!(pid = %pid.trim(), "stale lock file found, overwriting");
    }
    let mut f = File::create(lock_path)?;
    write!(f, "{}", std::process::id())?;
    Ok(f)
}

pub fn release_lock(lock_path: &Path) {
    let _ = fs::remove_file(lock_path);
}
