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
