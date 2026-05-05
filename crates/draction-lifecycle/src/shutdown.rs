use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

use draction_db::DractionDb;
use tokio::time::{sleep, Duration};
use tracing::{info, warn};

const GRACEFUL_TIMEOUT_SECS: u64 = 30;
const POLL_INTERVAL_MS: u64 = 500;

pub async fn graceful_shutdown(db: Arc<DractionDb>, lock_path: PathBuf) {
    info!("Shutdown initiated, waiting for active runs...");

    let start = Instant::now();
    let deadline = Duration::from_secs(GRACEFUL_TIMEOUT_SECS);

    loop {
        match db.get_active_run_count() {
            Ok(0) => {
                info!("No active runs, proceeding with shutdown");
                break;
            }
            Ok(count) => {
                if start.elapsed() >= deadline {
                    warn!("Timeout reached with {count} active run(s), force-exiting");
                    if let Ok(active) = db.list_active_run_ids() {
                        warn!("Stuck run IDs: {active:?}");
                    }
                    break;
                }
                info!(
                    "Waiting for {count} active run(s)... ({:.1}s elapsed)",
                    start.elapsed().as_secs_f64()
                );
                sleep(Duration::from_millis(POLL_INTERVAL_MS)).await;
            }
            Err(e) => {
                warn!("Failed to query active runs: {e}");
                break;
            }
        }
    }

    // Release lock file
    if lock_path.exists() {
        if let Err(e) = std::fs::remove_file(&lock_path) {
            warn!("Failed to remove lock file: {e}");
        } else {
            info!("Lock file released");
        }
    }

    info!("Shutdown complete");
}
