mod api_tests;
mod auth_tests;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use quizmo::handlers;
use tower::ServiceExt;
use serde_json::json;
use crate::common::create_test_user;

// Shared integration test utilities
async fn create_test_app() -> Router {
    handlers::create_app().await
}

#[tokio::test]
async fn test_protected_endpoints() {
    let app = create_test_app().await;
    let (_, token) = create_test_user();

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/quizzes")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
