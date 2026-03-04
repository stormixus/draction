use crate::node_registry::{Artifact, NodeContext, NodeExecutor, NodeOutput};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;
use std::path::Path;

pub struct CopyNode;

#[async_trait]
impl NodeExecutor for CopyNode {
    fn kind(&self) -> &'static str { "copy" }

    async fn execute(&self, ctx: &NodeContext, params: Value) -> Result<NodeOutput> {
        let dest_dir = params["dest"].as_str()
            .ok_or_else(|| anyhow::anyhow!("copy node: missing 'dest' param"))?;

        // Expand ~ to home dir
        let dest_dir = if dest_dir.starts_with("~/") {
            dirs::home_dir()
                .ok_or_else(|| anyhow::anyhow!("cannot resolve home dir"))?
                .join(&dest_dir[2..])
        } else {
            std::path::PathBuf::from(dest_dir)
        };

        let src = Path::new(&ctx.work_dir);
        if !src.exists() {
            return Err(anyhow::anyhow!("source file not found: {}", ctx.work_dir));
        }

        tokio::fs::create_dir_all(&dest_dir).await?;

        let file_name = src.file_name().unwrap_or_default();
        let dest_file = dest_dir.join(file_name);

        tokio::fs::copy(src, &dest_file).await?;

        tracing::info!(src = %ctx.work_dir, dest = %dest_file.display(), "file copied");

        Ok(NodeOutput {
            artifacts: vec![Artifact {
                kind: "file".into(),
                path: Some(dest_file.to_string_lossy().into()),
                url: None,
            }],
        })
    }
}
