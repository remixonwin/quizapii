use crate::models::error::AppError;

pub trait Validatable {
    fn validate(&self) -> Result<(), AppError>;
}

pub fn validate_string_not_empty(field: &str, value: &str) -> Result<(), AppError> {
    if value.trim().is_empty() {
        return Err(AppError::ValidationError(
            format!("{} cannot be empty", field)
        ));
    }
    Ok(())
}

pub fn validate_vec_not_empty<T>(field: &str, value: &[T]) -> Result<(), AppError> {
    if value.is_empty() {
        return Err(AppError::ValidationError(
            format!("{} cannot be empty", field)
        ));
    }
    Ok(())
}
