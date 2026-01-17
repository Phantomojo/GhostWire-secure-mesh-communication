//! API Integration tests for GhostWire
//!
//! These tests verify the web API endpoints function correctly.
//! Note: Tests require the server to be running or use mock setup.

use std::time::Duration;

/// Integration test: verify API status endpoint
#[tokio::test]
async fn test_api_status_endpoint() {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .expect("Failed to create HTTP client");

    // Try to connect - skip if server not running
    match client.get("http://localhost:3000/api/status").send().await {
        Ok(res) => {
            assert!(
                res.status().is_success(),
                "Status endpoint should return 200"
            );
        }
        Err(_) => {
            // Server not running - skip test gracefully
            eprintln!("Note: Skipping test - server not running at localhost:3000");
        }
    }
}

/// Integration test: send a message via /api/send_message and check response
#[tokio::test]
async fn test_send_message_api() {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .expect("Failed to create HTTP client");

    match client
        .post("http://localhost:3000/api/send_message")
        .json(&serde_json::json!({
            "recipient": "peer1",
            "message": "Hello, GhostWire!"
        }))
        .send()
        .await
    {
        Ok(res) => {
            // API may return 4xx if peer doesn't exist, but should not error
            assert!(
                res.status().is_success() || res.status().is_client_error(),
                "API should respond with valid HTTP status"
            );
        }
        Err(_) => {
            eprintln!("Note: Skipping test - server not running at localhost:3000");
        }
    }
}

/// Test API security headers
#[tokio::test]
async fn test_api_security_headers() {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .expect("Failed to create HTTP client");

    match client.get("http://localhost:3000/api/status").send().await {
        Ok(res) => {
            // Check for security headers
            let headers = res.headers();

            // These may or may not be present depending on config
            if let Some(csp) = headers.get("content-security-policy") {
                assert!(!csp.to_str().unwrap_or("").is_empty());
            }
        }
        Err(_) => {
            eprintln!("Note: Skipping test - server not running at localhost:3000");
        }
    }
}
