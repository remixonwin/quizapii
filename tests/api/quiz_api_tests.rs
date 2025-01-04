use anyhow::Result;
use axum::http::StatusCode;
use reqwest::Client;
use serde_json::json;

pub async fn test_quiz_api_operations() -> Result<()> {
    let client = Client::new();
    let base_url = "http://localhost:3000/api/v1";

    // Test create quiz via API
    let response = client
        .post(&format!("{}/quizzes", base_url))
        .json(&json!({
            "title": "API Test Quiz",
            "description": "Created via API test",
            "questions": []
        }))
        .send()
        .await?;

    assert_eq!(response.status(), StatusCode::CREATED);

    // Test get quiz list
    let response = client
        .get(&format!("{}/quizzes", base_url))
        .send()
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    Ok(())
}
