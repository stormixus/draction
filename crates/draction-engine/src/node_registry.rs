use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;
use std::collections::HashMap;

pub struct NodeContext {
    pub run_id: String,
    pub event_id: String,
    pub work_dir: String,
}

#[derive(Debug)]
pub struct NodeOutput {
    pub artifacts: Vec<Artifact>,
}

#[derive(Debug)]
pub struct Artifact {
    pub kind: String, // "file" | "link"
    pub path: Option<String>,
    pub url: Option<String>,
}

#[async_trait]
pub trait NodeExecutor: Send + Sync {
    fn kind(&self) -> &'static str;
    async fn execute(&self, ctx: &NodeContext, params: Value) -> Result<NodeOutput>;
}

pub struct NodeRegistry {
    executors: HashMap<String, Box<dyn NodeExecutor>>,
}

impl Default for NodeRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeRegistry {
    pub fn new() -> Self {
        Self { executors: HashMap::new() }
    }

    pub fn register(&mut self, executor: Box<dyn NodeExecutor>) {
        let kind = executor.kind().to_string();
        self.executors.insert(kind, executor);
    }

    pub fn get(&self, kind: &str) -> Result<&dyn NodeExecutor> {
        self.executors
            .get(kind)
            .map(|e| e.as_ref())
            .ok_or_else(|| anyhow::anyhow!("unknown node type: {kind}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    struct StubNode {
        kind: &'static str,
    }

    #[async_trait]
    impl NodeExecutor for StubNode {
        fn kind(&self) -> &'static str {
            self.kind
        }
        async fn execute(&self, _ctx: &NodeContext, _params: Value) -> Result<NodeOutput> {
            Ok(NodeOutput { artifacts: vec![] })
        }
    }

    #[test]
    fn register_then_get_returns_the_same_executor_kind() {
        let mut reg = NodeRegistry::new();
        reg.register(Box::new(StubNode { kind: "stub" }));
        let exec = reg.get("stub").expect("registered kind must resolve");
        assert_eq!(exec.kind(), "stub");
    }

    #[test]
    fn get_unknown_kind_returns_descriptive_error() {
        let reg = NodeRegistry::new();
        let err = match reg.get("missing") {
            Ok(_) => panic!("expected unknown-kind error"),
            Err(e) => e,
        };
        assert!(err.to_string().contains("unknown node type"));
        assert!(err.to_string().contains("missing"));
    }

    #[test]
    fn re_registering_same_kind_overwrites_previous() {
        let mut reg = NodeRegistry::new();
        reg.register(Box::new(StubNode { kind: "dup" }));
        reg.register(Box::new(StubNode { kind: "dup" }));
        assert!(reg.get("dup").is_ok());
    }

    #[tokio::test]
    async fn stub_executor_runs_through_registry() {
        let mut reg = NodeRegistry::new();
        reg.register(Box::new(StubNode { kind: "stub" }));
        let ctx = NodeContext {
            run_id: "r".into(),
            event_id: "e".into(),
            work_dir: "/tmp/x".into(),
        };
        let out = reg.get("stub").unwrap().execute(&ctx, json!({})).await.unwrap();
        assert!(out.artifacts.is_empty());
    }
}
