use anyhow::Result;
use draction_db::DractionDb;

/// Mark any runs with status='running' as 'failed' on startup.
/// Delegates to DractionDb::mark_running_as_failed which runs the SQL UPDATE.
pub fn recover_stale_runs(db: &DractionDb) -> Result<u64> {
    tracing::info!("crash recovery: marking stale runs as failed");
    db.mark_running_as_failed()
}
