use crate::common::{create_sample_quiz, TestContext};
use crate::api::setup_test_context;
use quizmo::models::test_types::TestQuiz;
use reqwest::StatusCode;

#[tokio::test]
async fn test_create_quiz_authenticated() {
    let ctx = setup_test_context().await;
    let test_quiz = create_sample_quiz("Test Quiz");
    let response = ctx.create_test_quiz(&test_quiz).await.expect("Failed to send request");
    assert_eq!(response.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn test_create_quiz_unauthorized() {
    let ctx = setup_test_context().await;
    let test_quiz = create_sample_quiz("Test Quiz");
    let response = ctx.create_test_quiz(&test_quiz).await.expect("Failed to send request");
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_create_quiz() {
    let ctx = setup_test_context().await;
    let test_quiz = create_sample_quiz("Test Quiz");
    let response = ctx.create_test_quiz(&test_quiz).await.expect("Failed to send request");
    assert_eq!(response.status(), StatusCode::CREATED);

    let created_quiz = response.json::<TestQuiz>().await.expect("Failed to parse JSON");
    assert_eq!(created_quiz.title, test_quiz.title);
    assert_eq!(created_quiz.questions.len(), test_quiz.questions.len());
}

#[tokio::test]
async fn test_get_quiz() {
    let ctx = setup_test_context().await;
    
    // Create a quiz first
    let test_quiz = create_sample_quiz("Test Quiz");
    let create_response = ctx.create_test_quiz(&test_quiz).await.expect("Failed to create quiz");
    let created_quiz = create_response.json::<TestQuiz>().await.expect("Failed to parse JSON");
    
    // Then get it
    let response = ctx.get_quiz(&created_quiz.id).await.expect("Failed to get quiz");
    assert_eq!(response.status(), StatusCode::OK);
    
    let quiz_response = response.json::<TestQuiz>().await.expect("Failed to parse JSON");
    assert_eq!(quiz_response.id, created_quiz.id);
    assert_eq!(quiz_response.title, created_quiz.title);
}

#[tokio::test]
async fn test_invalid_quiz_creation() {
    let ctx = setup_test_context().await;
    let mut quiz = create_sample_quiz("Invalid Quiz");
    quiz.questions = vec![]; // Invalid quiz with no questions

    let response = ctx.create_test_quiz(&quiz).await.expect("Failed to send request");
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_quiz_not_found() {
    let ctx = setup_test_context().await;

    let response = ctx.get_quiz("nonexistent_id").await.expect("Failed to get quiz");
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_quiz_validation() {
    let ctx = setup_test_context().await;
    let response = ctx.api_client
        .post(format!("{}/api/v1/quizzes", ctx.base_url))
        .json(&TestQuiz {
            id: "".to_string(),
            title: "".to_string(),
            description: "Test quiz".to_string(),
            questions: vec![],
        })
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
