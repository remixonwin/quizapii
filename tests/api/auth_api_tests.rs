use anyhow::Result;
use axum::http::StatusCode;
use reqwest::Client;
use serde_json::json;

pub async fn test_auth_api_operations() -> Result<()> {
    let client = Client::new();
    let base_url = "http://localhost:3000/api/v1";

    // Test login endpoint
    let response = client
        .post(&format!("{}/auth/login", base_url))
        .json(&json!({
            "username": "test_user",
            "password": "test_password"
        }))
        .send()
        .await?;

    assert_eq!(response.status(), StatusCode::OK);
    let token = response.json::<serde_json::Value>().await?;
    assert!(token.get("token").is_some());

    Ok(())
}
