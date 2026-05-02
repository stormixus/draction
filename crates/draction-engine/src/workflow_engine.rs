use crate::node_registry::{NodeContext, NodeRegistry};
use draction_domain::workflow::Workflow;
use anyhow::Result;
use tracing::{info, error};

pub struct WorkflowEngine {
    pub registry: NodeRegistry,
}

impl WorkflowEngine {
    pub fn new(registry: NodeRegistry) -> Self {
        Self { registry }
    }

    /// Execute a workflow serially (v0.1: fail-fast).
    pub async fn execute(&self, run_id: &str, event_id: &str, workflow: &Workflow, work_dir: &str) -> Result<()> {
        let ctx = NodeContext {
            run_id: run_id.to_string(),
            event_id: event_id.to_string(),
            work_dir: work_dir.to_string(),
        };

        // Topological order: follow edges linearly (v0.1: serial)
        let ordered = topo_sort(workflow)?;

        for node in &ordered {
            info!(run_id, node_id = %node.id, node_type = %node.node_type, "executing node");
            let executor = self.registry.get(&node.node_type)?;
            match executor.execute(&ctx, node.params.clone()).await {
                Ok(_output) => {
                    info!(node_id = %node.id, "node succeeded");
                }
                Err(err) => {
                    error!(node_id = %node.id, %err, "node failed — aborting run (fail-fast)");
                    return Err(err);
                }
            }
        }

        Ok(())
    }
}

fn topo_sort(wf: &Workflow) -> Result<Vec<&draction_domain::workflow::WorkflowNode>> {
    // v0.1: assume edges define linear chain, return nodes in edge order
    if wf.nodes.is_empty() {
        return Ok(vec![]);
    }
    let mut ordered = Vec::with_capacity(wf.nodes.len());
    let mut visited = std::collections::HashSet::new();

    // Find root (node that is never a 'to')
    let to_set: std::collections::HashSet<&str> = wf.edges.iter().map(|e| e.to.as_str()).collect();
    let root = wf.nodes.iter().find(|n| !to_set.contains(n.id.as_str()));
    if let Some(root) = root {
        let mut current = root;
        loop {
            if !visited.insert(&current.id) { break; }
            ordered.push(current);
            let next_id = wf.edges.iter().find(|e| e.from == current.id).map(|e| &e.to);
            match next_id {
                Some(id) => {
                    current = wf.nodes.iter().find(|n| n.id == *id)
                        .ok_or_else(|| anyhow::anyhow!("node {id} not found"))?;
                }
                None => break,
            }
        }
    }
    Ok(ordered)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node_registry::{NodeContext, NodeExecutor, NodeOutput};
    use async_trait::async_trait;
    use draction_domain::workflow::{Edge, WorkflowNode};
    use serde_json::{json, Value};
    use std::sync::{Arc, Mutex};

    struct RecordingNode {
        kind: &'static str,
        log: Arc<Mutex<Vec<String>>>,
        fail: bool,
    }

    #[async_trait]
    impl NodeExecutor for RecordingNode {
        fn kind(&self) -> &'static str {
            self.kind
        }
        async fn execute(&self, ctx: &NodeContext, _params: Value) -> Result<NodeOutput> {
            self.log.lock().unwrap().push(format!("{}:{}", self.kind, ctx.run_id));
            if self.fail {
                anyhow::bail!("RecordingNode '{}' configured to fail", self.kind);
            }
            Ok(NodeOutput { artifacts: vec![] })
        }
    }

    fn node(id: &str, kind: &str) -> WorkflowNode {
        WorkflowNode {
            id: id.into(),
            node_type: kind.into(),
            params: json!({}),
        }
    }

    fn edge(from: &str, to: &str) -> Edge {
        Edge { from: from.into(), to: to.into() }
    }

    #[test]
    fn topo_sort_returns_empty_for_workflow_with_no_nodes() {
        let wf = Workflow {
            id: "w".into(),
            name: "w".into(),
            nodes: vec![],
            edges: vec![],
        };
        assert!(topo_sort(&wf).unwrap().is_empty());
    }

    #[test]
    fn topo_sort_orders_linear_chain_by_edges() {
        // n3 -> n1 -> n2, with nodes listed in scrambled order
        let wf = Workflow {
            id: "w".into(),
            name: "w".into(),
            nodes: vec![node("n2", "k"), node("n1", "k"), node("n3", "k")],
            edges: vec![edge("n3", "n1"), edge("n1", "n2")],
        };
        let order: Vec<&str> = topo_sort(&wf).unwrap().iter().map(|n| n.id.as_str()).collect();
        assert_eq!(order, vec!["n3", "n1", "n2"]);
    }

    #[test]
    fn topo_sort_returns_single_node_when_no_edges() {
        let wf = Workflow {
            id: "w".into(),
            name: "w".into(),
            nodes: vec![node("only", "k")],
            edges: vec![],
        };
        let order: Vec<&str> = topo_sort(&wf).unwrap().iter().map(|n| n.id.as_str()).collect();
        assert_eq!(order, vec!["only"]);
    }

    #[test]
    fn topo_sort_errors_when_edge_points_to_unknown_node() {
        let wf = Workflow {
            id: "w".into(),
            name: "w".into(),
            nodes: vec![node("n1", "k")],
            edges: vec![edge("n1", "ghost")],
        };
        let err = topo_sort(&wf).unwrap_err();
        assert!(err.to_string().contains("ghost"));
    }

    #[tokio::test]
    async fn execute_runs_nodes_in_topological_order() {
        let log = Arc::new(Mutex::new(Vec::<String>::new()));
        let mut reg = NodeRegistry::new();
        reg.register(Box::new(RecordingNode { kind: "first", log: log.clone(), fail: false }));
        reg.register(Box::new(RecordingNode { kind: "second", log: log.clone(), fail: false }));
        let engine = WorkflowEngine::new(reg);

        let wf = Workflow {
            id: "wf".into(),
            name: "wf".into(),
            nodes: vec![node("n1", "first"), node("n2", "second")],
            edges: vec![edge("n1", "n2")],
        };

        engine.execute("run-1", "evt-1", &wf, "/tmp/work").await.unwrap();
        let log = log.lock().unwrap().clone();
        assert_eq!(log, vec!["first:run-1", "second:run-1"]);
    }

    #[tokio::test]
    async fn execute_fails_fast_and_skips_remaining_nodes() {
        let log = Arc::new(Mutex::new(Vec::<String>::new()));
        let mut reg = NodeRegistry::new();
        reg.register(Box::new(RecordingNode { kind: "ok", log: log.clone(), fail: false }));
        reg.register(Box::new(RecordingNode { kind: "boom", log: log.clone(), fail: true }));
        reg.register(Box::new(RecordingNode { kind: "after", log: log.clone(), fail: false }));
        let engine = WorkflowEngine::new(reg);

        let wf = Workflow {
            id: "wf".into(),
            name: "wf".into(),
            nodes: vec![node("a", "ok"), node("b", "boom"), node("c", "after")],
            edges: vec![edge("a", "b"), edge("b", "c")],
        };

        let err = engine.execute("r", "e", &wf, "/tmp").await.unwrap_err();
        assert!(err.to_string().contains("boom"));

        // The third node must NOT have been invoked
        let log = log.lock().unwrap().clone();
        assert_eq!(log, vec!["ok:r", "boom:r"]);
    }

    #[tokio::test]
    async fn execute_unknown_node_type_returns_err() {
        let reg = NodeRegistry::new();
        let engine = WorkflowEngine::new(reg);
        let wf = Workflow {
            id: "w".into(),
            name: "w".into(),
            nodes: vec![node("n1", "missing_type")],
            edges: vec![],
        };
        let err = engine.execute("r", "e", &wf, "/tmp").await.unwrap_err();
        assert!(err.to_string().contains("unknown node type"));
    }
}
