use crate::schema::Schema;
use serde_json::Value;
use std::fmt::Debug;
use zod_rs_util::{
    ValidateResult, ValidationError, ValidationOrigin, ValidationResult, ValidationType,
};

#[derive(Debug, Clone)]
pub struct ArraySchema<S, T> {
    element_schema: S,
    min_length: Option<usize>,
    max_length: Option<usize>,
    _phantom: std::marker::PhantomData<T>,
}

impl<S, T> ArraySchema<S, T> {
    pub fn new(element_schema: S) -> Self {
        Self {
            element_schema,
            min_length: None,
            max_length: None,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn min(mut self, min: usize) -> Self {
        self.min_length = Some(min);
        self
    }

    pub fn max(mut self, max: usize) -> Self {
        self.max_length = Some(max);
        self
    }

    pub fn length(self, len: usize) -> Self {
        self.min(len).max(len)
    }
}

impl<S, T> Schema<Vec<T>> for ArraySchema<S, T>
where
    S: Schema<T>,
    T: Debug,
{
    fn validate(&self, value: &Value) -> ValidateResult<Vec<T>> {
        let array = match value.as_array() {
            Some(arr) => arr,
            None => {
                return Err(ValidationError::invalid_type(
                    ValidationType::Array,
                    ValidationType::from(value),
                )
                .into());
            }
        };

        if let Some(min) = self.min_length {
            if array.len() < min {
                return Err(ValidationError::too_small(
                    ValidationOrigin::Array,
                    min.to_string(),
                    true,
                )
                .into());
            }
        }

        if let Some(max) = self.max_length {
            if array.len() > max {
                return Err(ValidationError::too_big(
                    ValidationOrigin::Array,
                    max.to_string(),
                    true,
                )
                .into());
            }
        }

        let mut results = Vec::new();
        let mut validation_result = ValidationResult::new();

        for (index, item) in array.iter().enumerate() {
            match self.element_schema.validate(item) {
                Ok(validated_item) => results.push(validated_item),
                Err(mut errors) => {
                    errors.prefix_path(index.to_string());
                    validation_result.merge(errors);
                }
            }
        }

        if validation_result.is_empty() {
            Ok(results)
        } else {
            Err(validation_result)
        }
    }
}

pub fn array<S, T>(element_schema: S) -> ArraySchema<S, T> {
    ArraySchema::new(element_schema)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{number, string};
    use serde_json::json;

    #[test]
    fn test_array_validation() {
        let schema = array(string()).min(1).max(3);

        assert!(schema.validate(&json!(["hello", "world"])).is_ok());
        assert!(schema.validate(&json!([])).is_err());
        assert!(schema.validate(&json!(["a", "b", "c", "d"])).is_err());
        assert!(schema.validate(&json!([1, 2, 3])).is_err());
    }

    // ==================== EDGE CASE TESTS ====================

    // Empty Array
    #[test]
    fn test_empty_array_with_min_zero() {
        let schema = array(string()).min(0);
        assert!(schema.validate(&json!([])).is_ok());
    }

    #[test]
    fn test_empty_array_with_min_one() {
        let schema = array(string()).min(1);
        assert!(schema.validate(&json!([])).is_err());
    }

    #[test]
    fn test_empty_array_no_constraints() {
        let schema = array(string());
        assert!(schema.validate(&json!([])).is_ok());
    }

    // Boundary Conditions
    #[test]
    fn test_exactly_at_min_length() {
        let schema = array(string()).min(3);
        assert!(schema.validate(&json!(["a", "b", "c"])).is_ok());
        assert!(schema.validate(&json!(["a", "b"])).is_err());
    }

    #[test]
    fn test_exactly_at_max_length() {
        let schema = array(string()).max(3);
        assert!(schema.validate(&json!(["a", "b", "c"])).is_ok());
        assert!(schema.validate(&json!(["a", "b", "c", "d"])).is_err());
    }

    #[test]
    fn test_exact_length() {
        let schema = array(string()).length(2);
        assert!(schema.validate(&json!(["a", "b"])).is_ok());
        assert!(schema.validate(&json!(["a"])).is_err());
        assert!(schema.validate(&json!(["a", "b", "c"])).is_err());
    }

    // Error Path Tracking
    #[test]
    fn test_error_at_index_zero() {
        let schema = array(string().min(3));
        let result = schema.validate(&json!(["hi"])); // "hi" is too short
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.issues[0].path, vec!["0"]);
    }

    #[test]
    fn test_error_at_middle_index() {
        let schema = array(string().min(3));
        let result = schema.validate(&json!(["hello", "hi", "world"]));
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.issues[0].path, vec!["1"]);
    }

    #[test]
    fn test_multiple_errors_across_indices() {
        let schema = array(string().min(3));
        let result = schema.validate(&json!(["hi", "ok", "x"]));
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.issues.len(), 3);
        // Check that paths are correct
        let paths: Vec<_> = err.issues.iter().map(|i| i.path[0].as_str()).collect();
        assert!(paths.contains(&"0"));
        assert!(paths.contains(&"1"));
        assert!(paths.contains(&"2"));
    }

    #[test]
    fn test_valid_elements_return_correct_values() {
        let schema = array(string());
        let result = schema.validate(&json!(["a", "b", "c"]));
        assert!(result.is_ok());
        let values = result.unwrap();
        assert_eq!(values, vec!["a", "b", "c"]);
    }

    // Nested Arrays
    #[test]
    fn test_nested_array() {
        let schema = array(array(string()));
        let result = schema.validate(&json!([["a", "b"], ["c", "d"]]));
        assert!(result.is_ok());
    }

    #[test]
    fn test_nested_array_error_path() {
        let schema = array(array(string().min(3)));
        let result = schema.validate(&json!([["hello"], ["hi"]]));
        assert!(result.is_err());
        let err = result.unwrap_err();
        // Error should be at path ["1"]["0"] for the "hi" string
        assert_eq!(err.issues[0].path, vec!["1", "0"]);
    }

    #[test]
    fn test_deeply_nested_array() {
        let schema = array(array(array(string())));
        let result = schema.validate(&json!([[["a", "b"]], [["c", "d"]]]));
        assert!(result.is_ok());
    }

    // Mixed Element Types (should fail for wrong types)
    #[test]
    fn test_mixed_types_fail() {
        let schema = array(string());
        assert!(schema.validate(&json!(["hello", 123, "world"])).is_err());
    }

    #[test]
    fn test_number_array() {
        let schema = array(number().positive());
        assert!(schema.validate(&json!([1, 2, 3])).is_ok());
        assert!(schema.validate(&json!([1, -2, 3])).is_err());
    }

    // Constraint Conflicts
    #[test]
    fn test_impossible_constraint_min_greater_than_max() {
        let schema = array(string()).min(5).max(2);
        // All arrays will fail
        assert!(schema.validate(&json!(["a", "b", "c"])).is_err());
        assert!(schema.validate(&json!(["a"])).is_err());
        assert!(schema.validate(&json!(["a", "b", "c", "d", "e", "f"])).is_err());
    }

    // Type Rejection
    #[test]
    fn test_rejects_null() {
        let schema = array(string());
        assert!(schema.validate(&json!(null)).is_err());
    }

    #[test]
    fn test_rejects_string() {
        let schema = array(string());
        assert!(schema.validate(&json!("not an array")).is_err());
    }

    #[test]
    fn test_rejects_number() {
        let schema = array(string());
        assert!(schema.validate(&json!(123)).is_err());
    }

    #[test]
    fn test_rejects_object() {
        let schema = array(string());
        assert!(schema.validate(&json!({"key": "value"})).is_err());
    }

    #[test]
    fn test_rejects_boolean() {
        let schema = array(string());
        assert!(schema.validate(&json!(true)).is_err());
    }

    // Single Element Array
    #[test]
    fn test_single_element_valid() {
        let schema = array(string());
        assert!(schema.validate(&json!(["single"])).is_ok());
    }

    #[test]
    fn test_single_element_invalid() {
        let schema = array(string().min(10));
        let result = schema.validate(&json!(["short"]));
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.issues[0].path, vec!["0"]);
    }

    // Large Array (performance sanity check)
    #[test]
    fn test_large_array() {
        let schema = array(number());
        let large_array: Vec<i32> = (0..1000).collect();
        let json_val = serde_json::to_value(large_array).unwrap();
        assert!(schema.validate(&json_val).is_ok());
    }
}
