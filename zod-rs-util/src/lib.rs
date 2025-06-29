pub mod error;

pub use error::{ValidateResult, ValidationError, ValidationIssue, ValidationResult};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_validation_error_creation() {
        let error = ValidationError::required();
        assert_eq!(error.to_string(), "Value is required but was not provided");

        let error = ValidationError::invalid_type("string", "number");
        assert_eq!(
            error.to_string(),
            "Invalid type: expected string, got number"
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
            "at user.name: Value is required but was not provided"
        );
    }

    #[test]
    fn test_validation_result_display() {
        let mut result = ValidationResult::new();
        result.add_error_at_path(
            vec!["user".to_string(), "email".to_string()],
            ValidationError::invalid_format("invalid email format".to_string()),
        );
        result.add_error_at_path(
            vec!["user".to_string(), "age".to_string()],
            ValidationError::too_small("15".to_string(), "18".to_string()),
        );

        let display = result.to_string();
        assert!(display.contains("Validation failed with 2 error(s):"));
        assert!(display.contains("at user.email: Invalid format: invalid email format"));
        assert!(display.contains("at user.age: Value '15' is too small: minimum is 18"));
    }
}
