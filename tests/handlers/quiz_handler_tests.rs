use crate::common::TestContext;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;
use quizmo::models::quiz::Quiz;
use serde_json::json;

#[tokio::test]
async fn test_create_quiz_handler() {
    let ctx = TestContext::new().await;
    let app = ctx.app();

    let quiz_data = json!({
        "title": "Test Quiz",
        "description": "Test Description",
        "questions": []
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/quizzes")
                .header("Content-Type", "application/json")
                .body(Body::from(quiz_data.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn test_get_quiz_handler() {
    let ctx = TestContext::new().await;
    let app = ctx.app();

    // First create a quiz
    let quiz = Quiz::new(
        "Test Quiz".to_string(),
        "Test Description".to_string(),
        vec![],
    );
    let quiz_id = ctx.quiz_repo.create(quiz).await.unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/quizzes/{}", quiz_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
