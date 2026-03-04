use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;
use std::collections::HashMap;

pub struct NodeContext {
    pub run_id: String,
    pub event_id: String,
    pub work_dir: String,
}

pub struct NodeOutput {
    pub artifacts: Vec<Artifact>,
}

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
