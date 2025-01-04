use quizapii::models::submission::Submission;

#[test]
fn test_submission_creation() {
    let submission = Submission::new(1, 1, vec![("q1".to_string(), "a1".to_string())]);
    assert_eq!(submission.quiz_id, 1);
    assert_eq!(submission.user_id, 1);
}

#[test]
fn test_submission_validation() {
    let submission = Submission::new(1, 1, vec![]);
    assert!(submission.validate().is_err());
}

#[test]
fn test_submission_scoring() {
    let answers = vec![("q1".to_string(), "correct".to_string())];
    let submission = Submission::new(1, 1, answers);
    assert!(submission.calculate_score() >= 0.0);
}
