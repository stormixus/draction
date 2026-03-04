use anyhow::Result;

/// Mark any runs with status='running' as 'failed' on startup.
pub fn recover_stale_runs(_db_path: &str) -> Result<u64> {
    // TODO: UPDATE runs SET status='failed' WHERE status='running'
    tracing::info!("crash recovery: checking for stale runs");
    Ok(0)
}
