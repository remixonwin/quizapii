use crate::common::{TestQuiz, setup, TestContext, create_test_user, create_sample_quiz};
use reqwest::StatusCode;

#[tokio::test]
async fn test_create_quiz_authenticated() {
    setup();
    let ctx = TestContext::new().await;
    let user = create_test_user();
    let quiz = create_sample_quiz();

    // Register and login user
    ctx.register_user(&user).await;
    let login_response = ctx.login_user(&user).await;
    let login_json = login_response.json::<serde_json::Value>().await.unwrap();
    let token = login_json["token"].as_str().unwrap();

    // Create quiz with authentication
    let response = ctx.create_test_quiz_auth(&quiz, token).await;
    assert_eq!(response.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn test_create_quiz_unauthorized() {
    setup();
    let ctx = TestContext::new().await;
    let quiz = create_sample_quiz();

    let response = ctx.create_test_quiz(&quiz).await;
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_create_quiz() {
    setup();
    let ctx = TestContext::new().await;
    let quiz = create_sample_quiz();

    let response = ctx.create_test_quiz(&quiz).await;
    assert_eq!(response.status(), StatusCode::CREATED);
    
    let created_quiz: TestQuiz = response.json().await.unwrap();
    assert_eq!(created_quiz.title, quiz.title);
    assert_eq!(created_quiz.questions.len(), quiz.questions.len());
}

#[tokio::test]
async fn test_get_quiz() {
    setup();
    let ctx = TestContext::new().await;
    let quiz = create_sample_quiz();

    // First create a quiz
    let create_response = ctx.create_test_quiz(&quiz).await;
    let created_quiz: TestQuiz = create_response.json().await.unwrap();
    
    // Then retrieve it
    let response = ctx.get_quiz(&created_quiz.id).await;
    assert_eq!(response.status(), StatusCode::OK);
    
    let retrieved_quiz: TestQuiz = response.json().await.unwrap();
    assert_eq!(retrieved_quiz.title, quiz.title);
}

#[tokio::test]
async fn test_invalid_quiz_creation() {
    setup();
    let ctx = TestContext::new().await;
    let mut quiz = create_sample_quiz();
    quiz.questions = vec![]; // Invalid quiz with no questions

    let response = ctx.create_test_quiz(&quiz).await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_quiz_not_found() {
    setup();
    let ctx = TestContext::new().await;
    
    let response = ctx.get_quiz("nonexistent_id").await;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_quiz_validation() {
    setup();
    let ctx = TestContext::new().await;
    let mut quiz = create_sample_quiz();
    
    // Test with empty title
    quiz.title = "".to_string();
    let response = ctx.create_test_quiz(&quiz).await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    // Test with too many options
    quiz.title = "Valid Title".to_string();
    quiz.questions[0].options = (0..11).map(|i| i.to_string()).collect();
    let response = ctx.create_test_quiz(&quiz).await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
