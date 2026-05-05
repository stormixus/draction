use draction_app_core::DractionRuntime;
use reqwest::StatusCode;
use tempfile::TempDir;

#[tokio::test]
async fn test_health_returns_ok() {
    let tmp = TempDir::new().unwrap();
    let runtime = DractionRuntime::bootstrap_with_base(tmp.path().to_path_buf())
        .await
        .unwrap();

    // Health should return 200 without auth
    let resp = reqwest::get(format!("http://127.0.0.1:{}/api/v1/health", runtime.api_port))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["status"], "ok");
}

#[tokio::test]
async fn test_rules_requires_auth() {
    let tmp = TempDir::new().unwrap();
    let runtime = DractionRuntime::bootstrap_with_base(tmp.path().to_path_buf())
        .await
        .unwrap();

    let base = format!("http://127.0.0.1:{}", runtime.api_port);

    // Without auth → 401
    let resp = reqwest::get(format!("{}/api/v1/rules", base))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

    // With auth → 200
    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{}/api/v1/rules", base))
        .bearer_auth(&runtime.auth_token)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let rules: Vec<serde_json::Value> = resp.json().await.unwrap();
    assert_eq!(rules.len(), 3); // 3 default rules
}

#[tokio::test]
async fn test_ingest_creates_event() {
    let tmp = TempDir::new().unwrap();
    let base = tmp.path().to_path_buf();

    // Create a test file
    let test_file = base.join("test.txt");
    std::fs::write(&test_file, "hello world").unwrap();

    let runtime = DractionRuntime::bootstrap_with_base(base).await.unwrap();

    // Ingest the file
    let results = runtime
        .ingest_paths(vec![test_file], None)
        .await
        .unwrap();

    assert_eq!(results.len(), 1);
    assert!(results[0].inbox_path.contains("Inbox"));

    // Verify event exists in DB
    let runs = runtime.list_runs(10).unwrap();
    assert!(!runs.is_empty());
}

#[tokio::test]
async fn test_undo_after_ingest() {
    let tmp = TempDir::new().unwrap();
    let base = tmp.path().to_path_buf();

    // Create a test file
    let test_file = base.join("undo_me.txt");
    std::fs::write(&test_file, "to be undone").unwrap();

    let runtime = DractionRuntime::bootstrap_with_base(base).await.unwrap();

    // Ingest the file
    runtime
        .ingest_paths(vec![test_file], None)
        .await
        .unwrap();

    // Try undo (the entry should be in the stack)
    let client = reqwest::Client::new();
    let events_resp = client
        .get(format!(
            "http://127.0.0.1:{}/api/v1/events",
            runtime.api_port
        ))
        .bearer_auth(&runtime.auth_token)
        .send()
        .await
        .unwrap();

    let events: Vec<serde_json::Value> = events_resp.json().await.unwrap();
    if let Some(event) = events.first() {
        let event_id = event["id"].as_str().unwrap();

        let undo_url = format!(
            "http://127.0.0.1:{}/api/v1/events/{}/undo",
            runtime.api_port, event_id
        );
        let undo_resp = client
            .post(&undo_url)
            .bearer_auth(&runtime.auth_token)
            .send()
            .await
            .unwrap();

        let status = undo_resp.status();
        let body: serde_json::Value = undo_resp.json().await.unwrap();
        // Should not return 501 (the old stub)
        assert!(status.is_success(), "Undo failed: {status} - {body:?}");
        // Verifies undo endpoint is wired, not stubbed
    }
}
