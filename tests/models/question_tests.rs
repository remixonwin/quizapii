use quizmo::models::question::Question;

#[test]
fn test_question_creation() {
    let question = Question::new(
        "What is Rust?".to_string(),
        vec!["Language".to_string(), "Metal".to_string()],
        0
    );
    
    assert_eq!(question.text, "What is Rust?");
    assert_eq!(question.options.len(), 2);
    assert_eq!(question.correct_answer, 0);
}

#[test]
fn test_question_validation() {
    let invalid_question = Question::new(
        "".to_string(),
        vec!["A".to_string()],
        0
    );
    assert!(invalid_question.validate().is_err());

    let invalid_options = Question::new(
        "Question".to_string(),
        vec![],
        0
    );
    assert!(invalid_options.validate().is_err());
}

#[test]
fn test_check_answer() {
    let question = Question::new(
        "Question?".to_string(),
        vec!["A".to_string(), "B".to_string()],
        1
    );
    
    assert!(question.check_answer(1));
    assert!(!question.check_answer(0));
}
