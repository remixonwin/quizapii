use quizmo::models::quiz::Quiz;
use quizmo::models::question::Question;
use validator::Validate;

#[test]
fn test_quiz_creation() {
    let quiz = Quiz::new(
        "Test Quiz".to_string(),
        "Test Description".to_string(),
        vec![],
    );

    assert_eq!(quiz.title, "Test Quiz");
    assert_eq!(quiz.description, "Test Description");
    assert!(quiz.questions.is_empty());
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
    let question = Question::new(
        "Test Question".to_string(),
        vec!["Answer 1".to_string(), "Answer 2".to_string()],
        0,
    );

    let quiz = Quiz::new(
        "Quiz with Questions".to_string(),
        "Description".to_string(),
        vec![question],
    );

    assert_eq!(quiz.questions.len(), 1);
    assert_eq!(quiz.questions[0].text, "Test Question");
}

#[test]
fn test_quiz_complex_validation() {
    let quiz = Quiz::new(
        "".to_string(),
        "".to_string(),
        vec![Question::new(
            "".to_string(),
            vec![],
            0,
            -1
        )],
    );
    
    let validation_result = quiz.validate();
    assert!(validation_result.is_err());
    let errors = validation_result.unwrap_err();
    assert!(errors.to_string().contains("title"));
    assert!(errors.to_string().contains("questions"));
}

#[test]
fn test_quiz_with_invalid_points() {
    let quiz = Quiz::new(
        "Valid Title".to_string(),
        "Description".to_string(),
        vec![Question::new(
            "Question".to_string(),
            vec!["Option".to_string()],
            0,
            -5, // Invalid negative points
        )],
    );
    assert!(quiz.validate().is_err());
}
