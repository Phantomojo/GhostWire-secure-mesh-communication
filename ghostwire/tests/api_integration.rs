/// Integration test: send a message via /api/send_message and check response.
#[tokio::test]
async fn test_send_message_api() {
    // TODO: Start server in test mode (or assume running)
    let client = reqwest::Client::new();
    let res = client.post("http://localhost:8080/api/send_message")
        .json(&serde_json::json!({
            "recipient": "peer1",
            "message": "Hello, GhostWire!"
        }))
        .send()
        .await
        .expect("Failed to send request");
    assert!(res.status().is_success());
    // TODO: Check response body for success
} 