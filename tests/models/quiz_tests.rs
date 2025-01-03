use quizmo::models::quiz::Quiz;
use quizmo::models::question::Question;
use chrono::Utc;
use uuid::Uuid;

#[test]
fn test_quiz_creation() {
    let quiz = Quiz::new("Test Quiz".to_string(), "Description".to_string());
    assert_eq!(quiz.title, "Test Quiz");
    assert_eq!(quiz.description, "Description");
    assert!(!quiz.id.is_nil());
}

#[test]
fn test_quiz_validation() {
    let quiz = Quiz::new("".to_string(), "Description".to_string());
    assert!(quiz.validate().is_err());

    let valid_quiz = Quiz::new("Valid Title".to_string(), "Description".to_string());
    assert!(valid_quiz.validate().is_ok());
}

#[test]
fn test_quiz_with_questions() {
    let mut quiz = Quiz::new("Test Quiz".to_string(), "Description".to_string());
    let question = Question::new(
        "Test Question?".to_string(),
        vec!["A".to_string(), "B".to_string()],
        0
    );
    
    quiz.add_question(question);
    assert_eq!(quiz.questions.len(), 1);
}
