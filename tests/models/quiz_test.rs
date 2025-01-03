use quizmo::models::quiz::Quiz;

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
    let quiz = Quiz::new(
        "".to_string(),
        "Description".to_string(),
        vec![],
    );
    
    assert!(quiz.validate().is_err());
}

#[test]
fn test_quiz_update() {
    let mut quiz = Quiz::new(
        "Original".to_string(),
        "Original Desc".to_string(),
        vec![],
    );
    
    quiz.update_title("Updated".to_string());
    quiz.update_description("Updated Desc".to_string());
    
    assert_eq!(quiz.title, "Updated");
    assert_eq!(quiz.description, "Updated Desc");
}
