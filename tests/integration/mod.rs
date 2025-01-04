pub mod quiz_tests;
pub mod api_tests;
pub mod auth_tests;
pub mod server_tests;

use crate::common::{TestContext, assertions::*, create_test_quiz};
use axum::http::StatusCode;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;
use serde_json::json;

pub async fn run_lifecycle_test(resource: &str, create_payload: serde_json::Value) {
    let ctx = TestContext::new().await;
    
    // Create
    let create_status = make_test_request(
        &ctx,
        "POST",
        resource,
        Some(create_payload.to_string())
    ).await;
    assert_success_response(create_status);

    // Read, Update, Delete
    // ...existing code...
}

#[tokio::test]
async fn test_full_quiz_lifecycle() -> anyhow::Result<()> {
    let ctx = TestContext::new().await;
    let app = ctx.app();

    // Create quiz
    let quiz_data = json!({
        "title": "Complete Quiz",
        "description": "Full lifecycle test",
        "questions": [{
            "text": "Test Question",
            "options": ["A", "B", "C"],
            "correct_option": 0,
            "points": 10
        }]
    });

    // Create a user
    let user_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/register")
                .header("Content-Type", "application/json")
                .body(Body::from(json!({
                    "username": "testuser",
                    "password": "password123",
                    "email": "test@example.com"
                }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(user_response.status(), StatusCode::CREATED);

    // Create a quiz
    let quiz_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/quizzes")
                .header("Content-Type", "application/json")
                .body(Body::from(json!({
                    "title": "Integration Test Quiz",
                    "description": "Testing full lifecycle",
                    "questions": [{
                        "text": "Test Question",
                        "options": ["A", "B", "C"],
                        "correct_option": 0,
                        "points": 10
                    }]
                }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(quiz_response.status(), StatusCode::CREATED);

    Ok(())
}

// Keep other integration-specific tests
