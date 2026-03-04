use anyhow::Result;
use sha2::{Digest, Sha256};
use std::path::Path;

pub async fn compute_sha256(path: &Path) -> Result<String> {
    let bytes = tokio::fs::read(path).await?;
    let hash = Sha256::digest(&bytes);
    Ok(format!("{hash:x}"))
}

pub async fn file_size(path: &Path) -> Result<u64> {
    let meta = tokio::fs::metadata(path).await?;
    Ok(meta.len())
}
