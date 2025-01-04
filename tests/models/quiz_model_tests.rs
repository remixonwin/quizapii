use quizmo::models::quiz::Quiz;
use quizmo::models::question::Question;

pub fn test_quiz_model_operations() {
    let quiz = Quiz {
        id: Some("test-id".to_string()),
        title: "Test Quiz".to_string(),
        description: Some("Test Description".to_string()),
        questions: vec![],
        created_at: None,
        updated_at: None,
    };

    assert_eq!(quiz.title, "Test Quiz");
    assert_eq!(quiz.description.unwrap(), "Test Description");
}
