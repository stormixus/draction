use anyhow::Result;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct Config {
    token: String,
}

pub fn generate_token() -> String {
    let mut bytes = [0u8; 32];
    rand::rng().fill_bytes(&mut bytes);
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

pub fn load_or_create_token(base: &Path) -> Result<String> {
    let config_path = base.join("config.json");
    if config_path.exists() {
        let content = std::fs::read_to_string(&config_path)?;
        let config: Config = serde_json::from_str(&content)?;
        return Ok(config.token);
    }

    let token = generate_token();
    let config = Config { token: token.clone() };
    std::fs::create_dir_all(base)?;
    std::fs::write(&config_path, serde_json::to_string_pretty(&config)?)?;
    Ok(token)
}
