use tokio::time::{timeout, Duration};
use tracing::info;

const GRACEFUL_TIMEOUT_SECS: u64 = 30;

pub async fn graceful_shutdown(has_active_runs: impl std::future::Future<Output = bool>) {
    if has_active_runs.await {
        info!("active runs detected, waiting up to {GRACEFUL_TIMEOUT_SECS}s");
        let _ = timeout(Duration::from_secs(GRACEFUL_TIMEOUT_SECS), async {
            // TODO: wait for active runs to complete
            tokio::time::sleep(Duration::from_secs(1)).await;
        }).await;
    }
    info!("shutdown complete");
}
