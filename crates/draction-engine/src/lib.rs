pub mod rule_engine;
pub mod workflow_engine;
pub mod node_registry;
pub mod executors;

/// Build a NodeRegistry with all built-in executors registered.
pub fn default_registry() -> node_registry::NodeRegistry {
    let mut reg = node_registry::NodeRegistry::new();
    reg.register(Box::new(executors::move_node::MoveNode));
    reg.register(Box::new(executors::copy_node::CopyNode));
    reg.register(Box::new(executors::rename_node::RenameNode));
    reg.register(Box::new(executors::transcode_node::TranscodeNode));
    reg.register(Box::new(executors::webhook_node::WebhookNode));
    reg
}
