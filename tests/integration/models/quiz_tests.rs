use quizapii::models::quiz::Quiz;
use quizapii::models::question::Question;
use quizapii::models::update_quiz_dto::UpdateQuizDto;

#[test]
fn test_quiz_creation() {
    let quiz = Quiz::new(
        "Test Quiz".to_string(),
        vec![
            Question::new("Q1".to_string(), vec!["A".to_string()], 0),
            Question::new("Q2".to_string(), vec!["B".to_string()], 1),
        ],
    );
    
    assert_eq!(quiz.title, "Test Quiz");
    assert_eq!(quiz.questions.unwrap().len(), 2);
}

#[test]
fn test_quiz_update() {
    let mut quiz = Quiz::new("Old Title".to_string(), vec![]);
    let update = UpdateQuizDto {
        title: Some("New Title".to_string()),
        questions: Some(vec![Question::new("Q1".to_string(), vec![], 0)]),
    };
    
    quiz.update_from_dto(&update);
    assert_eq!(quiz.title, "New Title");
    assert_eq!(quiz.questions.unwrap().len(), 1);
}

#[test]
fn test_quiz_validation() {
    let quiz = Quiz::new("".to_string(), vec![]);
    assert!(quiz.validate().is_err());
}
