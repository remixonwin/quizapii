pub mod quiz_model_tests;
pub mod user_model_tests;
pub mod question_tests;
pub mod submission_tests;
pub mod error_tests;
pub mod test_types_tests;
pub mod update_quiz_dto_tests;
pub mod quiz_tests;

use validator::Validate;

pub trait ModelTestHelpers {
    fn create_valid_instance() -> Self;
    fn create_invalid_instance() -> Self;
}

pub fn test_model_validation<T: Validate + ModelTestHelpers>() {
    let valid = T::create_valid_instance();
    assert!(valid.validate().is_ok());

    let invalid = T::create_invalid_instance();
    assert!(invalid.validate().is_err());
}
