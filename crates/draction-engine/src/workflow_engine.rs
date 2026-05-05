use crate::node_registry::{NodeContext, NodeRegistry};
use draction_domain::workflow::Workflow;
use anyhow::Result;
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use std::collections::HashMap;
use tracing::{info, error};

pub struct WorkflowEngine {
    pub registry: NodeRegistry,
}

impl WorkflowEngine {
    pub fn new(registry: NodeRegistry) -> Self {
        Self { registry }
    }

    /// Execute a workflow with level-based parallelism.
    ///
    /// First, nodes are topologically sorted. Then they are grouped into
    /// levels (waves) where all preceding dependencies have completed.
    /// Within a level, nodes execute concurrently via `FuturesUnordered`.
    /// Fail-fast: if any node in a level fails, the entire run is aborted
    /// and remaining levels are skipped.
    pub async fn execute(
        &self,
        run_id: &str,
        event_id: &str,
        workflow: &Workflow,
        work_dir: &str,
    ) -> Result<()> {
        let ctx = NodeContext {
            run_id: run_id.to_string(),
            event_id: event_id.to_string(),
            work_dir: work_dir.to_string(),
        };

        if workflow.nodes.is_empty() {
            return Ok(());
        }

        // 1. Topological sort.
        let ordered = topo_sort(workflow)?;

        // 2. Build adjacency (reverse: predecessors) for depth computation.
        let mut predecessors: HashMap<&str, Vec<&str>> = HashMap::new();
        for node in &workflow.nodes {
            predecessors.entry(node.id.as_str()).or_default();
        }
        for edge in &workflow.edges {
            predecessors
                .entry(edge.to.as_str())
                .or_default()
                .push(edge.from.as_str());
        }

        // 3. Compute node depths (longest path from any source).
        //    Iterate in topological order so predecessors are already computed.
        let mut depth: HashMap<&str, usize> = HashMap::new();
        let mut max_depth = 0;
        for node in &ordered {
            let d = predecessors
                .get(node.id.as_str())
                .map(|preds| {
                    preds
                        .iter()
                        .map(|p| depth.get(p).copied().unwrap_or(0))
                        .max()
                        .unwrap_or(0)
                        + 1
                })
                .unwrap_or(0);
            depth.insert(node.id.as_str(), d);
            max_depth = max_depth.max(d);
        }

        // 4. Group nodes by depth level.
        let mut levels: Vec<Vec<&&draction_domain::workflow::WorkflowNode>> =
            vec![Vec::new(); max_depth + 1];
        for node in &ordered {
            let d = depth[node.id.as_str()];
            levels[d].push(node);
        }

        // 5. Execute level by level.
        for level_nodes in &levels {
            // Pre-resolve executor references.
            let exec_results: Vec<Result<&dyn crate::node_registry::NodeExecutor>> =
                level_nodes
                    .iter()
                    .map(|node| {
                        self.registry
                            .get(&node.node_type)
                            .map_err(|e| anyhow::anyhow!("{}", e))
                    })
                    .collect();

            // Build parallel futures for this level.
            let mut futures: FuturesUnordered<_> = level_nodes
                .iter()
                .zip(exec_results.into_iter())
                .map(|(node, exec_result)| {
                    let ctx = &ctx;
                    let node = *node; // &&WorkflowNode -> &WorkflowNode
                    let params = node.params.clone();
                    async move {
                        let executor = match exec_result {
                            Ok(e) => e,
                            Err(e) => return (node.id.as_str(), Err(e)),
                        };
                        info!(
                            run_id = %ctx.run_id,
                            node_id = %node.id,
                            node_type = %node.node_type,
                            "executing node"
                        );
                        let result = executor.execute(ctx, params).await;
                        (node.id.as_str(), result)
                    }
                })
                .collect();

            // Wait for all nodes in this level, fail-fast.
            while let Some((node_id, result)) = futures.next().await {
                match result {
                    Ok(_output) => {
                        info!(node_id = %node_id, "node succeeded");
                    }
                    Err(err) => {
                        error!(
                            node_id = %node_id,
                            %err,
                            "node failed — aborting run (fail-fast)"
                        );
                        return Err(err);
                    }
                }
            }
        }

        Ok(())
    }
}

/// Topological sort using Kahn's algorithm.
///
/// - Returns nodes in topological order.
/// - If edges are empty, returns nodes in their natural order (backward compat).
/// - Detects cycles: if ordered.len() < nodes.len() -> error.
/// - Handles missing node references gracefully with a clear error.
fn topo_sort(wf: &Workflow) -> Result<Vec<&draction_domain::workflow::WorkflowNode>> {
    use std::collections::{HashMap, VecDeque};

    if wf.nodes.is_empty() {
        return Ok(vec![]);
    }

    // Build node map for O(1) lookups
    let node_map: HashMap<&str, &draction_domain::workflow::WorkflowNode> = wf
        .nodes
        .iter()
        .map(|n| (n.id.as_str(), n))
        .collect();

    // Backward compat: no edges -> return nodes in natural order
    if wf.edges.is_empty() {
        return Ok(wf.nodes.iter().collect());
    }

    // Validate all edge endpoints exist in nodes
    for edge in &wf.edges {
        if !node_map.contains_key(edge.from.as_str()) {
            anyhow::bail!(
                "edge references unknown source node: {}",
                edge.from
            );
        }
        if !node_map.contains_key(edge.to.as_str()) {
            anyhow::bail!(
                "edge references unknown target node: {}",
                edge.to
            );
        }
    }

    // Build adjacency list and in-degree map
    let mut adj: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut in_degree: HashMap<&str, usize> = HashMap::new();

    for node in &wf.nodes {
        adj.entry(node.id.as_str()).or_default();
        in_degree.entry(node.id.as_str()).or_insert(0);
    }

    for edge in &wf.edges {
        adj.entry(edge.from.as_str())
            .or_default()
            .push(edge.to.as_str());
        *in_degree.entry(edge.to.as_str()).or_insert(0) += 1;
    }

    // Kahn's algorithm: start with nodes that have no incoming edges
    let mut queue: VecDeque<&str> = VecDeque::new();
    for (id, &deg) in &in_degree {
        if deg == 0 {
            queue.push_back(id);
        }
    }

    let mut ordered: Vec<&draction_domain::workflow::WorkflowNode> = Vec::new();

    while let Some(id) = queue.pop_front() {
        ordered.push(node_map[id]);

        if let Some(neighbors) = adj.get(id) {
            for &neighbor in neighbors {
                if let Some(deg) = in_degree.get_mut(neighbor) {
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }

    // Cycle detection: Kahn's algorithm should produce all nodes
    if ordered.len() != wf.nodes.len() {
        anyhow::bail!(
            "Cycle detected in workflow: topological sort returned {} nodes but workflow has {} nodes",
            ordered.len(),
            wf.nodes.len()
        );
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
        Edge {
            from: from.into(),
            to: to.into(),
            edge_type: draction_domain::workflow::EdgeType::default(),
        }
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

    // --- New tests for Workflow Engine v2.0 ---

    #[test]
    fn topo_sort_detects_cycle() {
        // a -> b -> c -> a forms a cycle
        let wf = Workflow {
            id: "w".into(),
            name: "w".into(),
            nodes: vec![
                node("a", "k"),
                node("b", "k"),
                node("c", "k"),
            ],
            edges: vec![
                edge("a", "b"),
                edge("b", "c"),
                edge("c", "a"),
            ],
        };
        let err = topo_sort(&wf).unwrap_err();
        assert!(
            err.to_string().contains("Cycle detected"),
            "expected cycle error, got: {err}"
        );
    }

    #[test]
    fn topo_sort_handles_diamond() {
        // a -> b, a -> c, b -> d, c -> d (diamond shape)
        let wf = Workflow {
            id: "w".into(),
            name: "w".into(),
            nodes: vec![
                node("a", "k"),
                node("b", "k"),
                node("c", "k"),
                node("d", "k"),
            ],
            edges: vec![
                edge("a", "b"),
                edge("a", "c"),
                edge("b", "d"),
                edge("c", "d"),
            ],
        };
        let order: Vec<&str> =
            topo_sort(&wf).unwrap().iter().map(|n| n.id.as_str()).collect();
        // `a` must come first, `d` must come last.
        // `b` and `c` can be in either order (they are independent).
        assert_eq!(order[0], "a");
        assert_eq!(order[3], "d");
        assert!(order.contains(&"b"));
        assert!(order.contains(&"c"));
    }

    #[tokio::test]
    async fn execute_runs_independent_nodes_in_parallel() {
        use std::time::Instant;

        // Two nodes with no edges between them should execute concurrently.
        // We use a small delay to verify they overlap in time.
        let log = Arc::new(Mutex::new(Vec::<String>::new()));
        let start = Arc::new(Instant::now());
        let timestamps = Arc::new(Mutex::new(Vec::<(String, u64)>::new()));

        struct DelayedNode {
            kind: &'static str,
            log: Arc<Mutex<Vec<String>>>,
            start: Arc<Instant>,
            timestamps: Arc<Mutex<Vec<(String, u64)>>>,
            delay_ms: u64,
        }

        #[async_trait]
        impl NodeExecutor for DelayedNode {
            fn kind(&self) -> &'static str {
                self.kind
            }
            async fn execute(
                &self,
                ctx: &NodeContext,
                _params: Value,
            ) -> Result<NodeOutput> {
                self.log
                    .lock()
                    .unwrap()
                    .push(format!("{}:{}", self.kind, ctx.run_id));
                tokio::time::sleep(std::time::Duration::from_millis(self.delay_ms)).await;
                let elapsed = self.start.elapsed().as_millis() as u64;
                self.timestamps
                    .lock()
                    .unwrap()
                    .push((self.kind.to_string(), elapsed));
                Ok(NodeOutput { artifacts: vec![] })
            }
        }

        let timestamps_clone = timestamps.clone();
        let start_clone = start.clone();

        let mut reg = NodeRegistry::new();
        reg.register(Box::new(DelayedNode {
            kind: "slow_a",
            log: log.clone(),
            start: start_clone.clone(),
            timestamps: timestamps_clone.clone(),
            delay_ms: 200,
        }));
        reg.register(Box::new(DelayedNode {
            kind: "slow_b",
            log: log.clone(),
            start: start_clone,
            timestamps: timestamps_clone,
            delay_ms: 200,
        }));

        let engine = WorkflowEngine::new(reg);

        let wf = Workflow {
            id: "wf".into(),
            name: "wf".into(),
            nodes: vec![node("n1", "slow_a"), node("n2", "slow_b")],
            edges: vec![], // no edges = independent, should run in parallel
        };

        let exec_start = Instant::now();
        engine
            .execute("run-p", "evt-p", &wf, "/tmp/work")
            .await
            .unwrap();
        let total_elapsed = exec_start.elapsed().as_millis();

        // If nodes ran sequentially, total time >= 400ms.
        // If parallel, total time is ~200ms (+ overhead).
        assert!(
            total_elapsed < 350,
            "total_elapsed={total_elapsed}ms; nodes should have run concurrently (expected < 350ms)"
        );

        // Both nodes should have logged.
        let log = log.lock().unwrap().clone();
        assert_eq!(log.len(), 2);
        assert!(log.contains(&"slow_a:run-p".to_string()));
        assert!(log.contains(&"slow_b:run-p".to_string()));

        // Both timestamps should be close (within ~50ms), confirming concurrency.
        let ts = timestamps.lock().unwrap();
        let a_ts = ts.iter().find(|(k, _)| k == "slow_a").map(|(_, t)| *t).unwrap();
        let b_ts = ts.iter().find(|(k, _)| k == "slow_b").map(|(_, t)| *t).unwrap();
        let diff = (a_ts as i64 - b_ts as i64).unsigned_abs();
        assert!(
            diff < 100,
            "timestamp diff={diff}ms; nodes should have finished at roughly the same time"
        );
    }
}
