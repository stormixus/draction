use crate::node_registry::{Artifact, NodeContext, NodeExecutor, NodeOutput};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;
use std::path::Path;

pub struct MoveNode;

fn expand_home(p: &str) -> std::path::PathBuf {
    if p.starts_with("~/") {
        if let Some(home) = dirs::home_dir() {
            return home.join(&p[2..]);
        }
    }
    std::path::PathBuf::from(p)
}

#[async_trait]
impl NodeExecutor for MoveNode {
    fn kind(&self) -> &'static str { "move" }

    async fn execute(&self, ctx: &NodeContext, params: Value) -> Result<NodeOutput> {
        let dest_dir_str = params["dest"].as_str()
            .ok_or_else(|| anyhow::anyhow!("move node: missing 'dest' param"))?;

        let dest_dir = expand_home(dest_dir_str);
        let src = Path::new(&ctx.work_dir);
        if !src.exists() {
            return Err(anyhow::anyhow!("source file not found: {}", ctx.work_dir));
        }

        tokio::fs::create_dir_all(&dest_dir).await?;

        let file_name = src.file_name().unwrap_or_default();
        let dest_file = dest_dir.join(file_name);

        // rename fails across filesystems, fall back to copy+delete
        if tokio::fs::rename(src, &dest_file).await.is_err() {
            tokio::fs::copy(src, &dest_file).await?;
            tokio::fs::remove_file(src).await?;
        }

        tracing::info!(src = %ctx.work_dir, dest = %dest_file.display(), "file moved");

        Ok(NodeOutput {
            artifacts: vec![Artifact {
                kind: "file".into(),
                path: Some(dest_file.to_string_lossy().into()),
                url: None,
            }],
        })
    }
}
