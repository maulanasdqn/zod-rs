mod error;
mod locales;

pub use error::{
    issue::ValidationIssue,
    result::{ValidateResult, ValidationResult},
    NumberConstraint, StringFormat, ValidationError, ValidationOrigin, ValidationType,
};
pub use locales::*;

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{
        error::{StringFormat, ValidationOrigin, ValidationType},
        locales::Locale,
    };

    fn add(left: u64, right: u64) -> u64 {
        left + right
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_validation_error_creation() {
        let error = ValidationError::required();
        assert_eq!(error.to_string(), "Value is required but was not provided");

        let error = ValidationError::invalid_type(ValidationType::String, ValidationType::Number);
        assert_eq!(
            error.to_string(),
            "Invalid input: expected string, received number"
        );
    }

    #[test]
    fn test_validation_result() {
        let mut result = ValidationResult::new();
        assert!(result.is_empty());

        result.add_error(ValidationError::required());
        assert_eq!(result.len(), 1);

        let error_result = ValidationResult::with_error(ValidationError::required());
        assert_eq!(error_result.len(), 1);
    }

    #[test]
    fn test_validation_issue_path() {
        let issue = ValidationIssue {
            path: vec!["user".to_string(), "name".to_string()],
            error: ValidationError::required(),
        };

        assert_eq!(
            issue.to_string(),
            "user.name: Value is required but was not provided"
        );
    }

    #[test]
    fn test_validation_result_display() {
        let mut result = ValidationResult::new();
        result.add_error_at_path(
            vec!["user".to_string(), "email".to_string()],
            ValidationError::invalid_format(StringFormat::custom("email"), None),
        );
        result.add_error_at_path(
            vec!["user".to_string(), "age".to_string()],
            ValidationError::too_small(ValidationOrigin::Number, "15", true),
        );

        let display = result.to_string();

        assert!(display.contains("user.email: Invalid email address"));
        assert!(display.contains("user.age: Too small: expected number to have >= 15"));
    }

    #[test]
    fn test_validation_result_display_ar() {
        let mut result = ValidationResult::new();

        result.add_error_at_path(
            vec!["user".to_string(), "email".to_string()],
            ValidationError::invalid_format(StringFormat::custom("email"), None),
        );
        result.add_error_at_path(
            vec!["user".to_string(), "age".to_string()],
            ValidationError::too_small(ValidationOrigin::Number, "15", true),
        );

        let display = result.local(Locale::Ar);

        assert!(display.contains("user.email: بريد إلكتروني غير مقبول"));
        assert!(display.contains("user.age: أصغر من اللازم: يفترض لـ number أن يكون >= 15"));
    }
}
