use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppState {
    pub pid: u32,
    pub port: u16,
    pub last_seen: String,
}

pub fn write_state(path: &Path, state: &AppState) -> Result<()> {
    let json = serde_json::to_string_pretty(state)?;
    std::fs::write(path, json)?;
    Ok(())
}

pub fn read_state(path: &Path) -> Result<AppState> {
    let json = std::fs::read_to_string(path)?;
    Ok(serde_json::from_str(&json)?)
}
