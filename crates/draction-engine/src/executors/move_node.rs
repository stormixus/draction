use crate::node_registry::{Artifact, NodeContext, NodeExecutor, NodeOutput};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;
use std::path::Path;

pub struct MoveNode;

fn expand_home(p: &str) -> std::path::PathBuf {
    if let Some(rest) = p.strip_prefix("~/")
        && let Some(home) = dirs::home_dir()
    {
        return home.join(rest);
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tempfile::tempdir;

    fn ctx(work_dir: &std::path::Path) -> NodeContext {
        NodeContext {
            run_id: "r".into(),
            event_id: "e".into(),
            work_dir: work_dir.to_string_lossy().into_owned(),
        }
    }

    #[test]
    fn kind_returns_move() {
        assert_eq!(MoveNode.kind(), "move");
    }

    #[tokio::test]
    async fn moves_file_into_existing_dest_dir() {
        let src_dir = tempdir().unwrap();
        let dest_dir = tempdir().unwrap();
        let src_path = src_dir.path().join("hello.txt");
        tokio::fs::write(&src_path, b"payload").await.unwrap();

        let params = json!({ "dest": dest_dir.path().to_string_lossy() });
        let out = MoveNode.execute(&ctx(&src_path), params).await.unwrap();

        let moved = dest_dir.path().join("hello.txt");
        assert!(moved.exists(), "file should be at the destination");
        assert!(!src_path.exists(), "source must be removed after move");
        assert_eq!(tokio::fs::read(&moved).await.unwrap(), b"payload");

        let path = out.artifacts[0].path.as_deref().unwrap();
        assert_eq!(path, moved.to_string_lossy());
    }

    #[tokio::test]
    async fn creates_dest_directory_when_missing() {
        let src_dir = tempdir().unwrap();
        let dest_root = tempdir().unwrap();
        let nested = dest_root.path().join("sub").join("deeper");
        let src_path = src_dir.path().join("a.bin");
        tokio::fs::write(&src_path, b"x").await.unwrap();

        let params = json!({ "dest": nested.to_string_lossy() });
        MoveNode.execute(&ctx(&src_path), params).await.unwrap();

        assert!(nested.join("a.bin").exists());
    }

    #[tokio::test]
    async fn missing_dest_param_returns_error() {
        let dir = tempdir().unwrap();
        let src = dir.path().join("a.txt");
        tokio::fs::write(&src, b"x").await.unwrap();
        let err = MoveNode.execute(&ctx(&src), json!({})).await.unwrap_err();
        assert!(err.to_string().contains("'dest'"));
    }

    #[tokio::test]
    async fn missing_source_returns_error() {
        let dir = tempdir().unwrap();
        let dest = tempdir().unwrap();
        let nonexistent = dir.path().join("ghost.txt");
        let params = json!({ "dest": dest.path().to_string_lossy() });
        let err = MoveNode.execute(&ctx(&nonexistent), params).await.unwrap_err();
        assert!(err.to_string().contains("source file not found"));
    }

    #[test]
    fn expand_home_passes_through_when_not_tilde_prefixed() {
        let p = expand_home("/absolute/path");
        assert_eq!(p, std::path::PathBuf::from("/absolute/path"));
    }

    #[test]
    fn expand_home_replaces_tilde_with_home_dir() {
        if let Some(home) = dirs::home_dir() {
            let p = expand_home("~/Draction/Photos");
            assert_eq!(p, home.join("Draction/Photos"));
        }
    }
}
