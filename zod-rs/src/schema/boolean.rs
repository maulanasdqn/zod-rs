use crate::schema::Schema;
use serde_json::Value;
use zod_rs_util::{ValidateResult, ValidationError, ValidationType};

#[derive(Debug, Clone)]
pub struct BooleanSchema;

impl BooleanSchema {
    pub fn new() -> Self {
        Self
    }
}

impl Default for BooleanSchema {
    fn default() -> Self {
        Self::new()
    }
}

impl Schema<bool> for BooleanSchema {
    fn validate(&self, value: &Value) -> ValidateResult<bool> {
        match value.as_bool() {
            Some(b) => Ok(b),
            None => Err(ValidationError::invalid_type(
                ValidationType::Bool,
                ValidationType::from(value),
            )
            .into()),
        }
    }
}

pub fn boolean() -> BooleanSchema {
    BooleanSchema::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_boolean_validation() {
        let schema = boolean();

        assert!(schema.validate(&json!(true)).is_ok());
        assert!(schema.validate(&json!(false)).is_ok());
        assert!(schema.validate(&json!("true")).is_err());
    }

    // ==================== EDGE CASE TESTS ====================

    // True/False Return Values
    #[test]
    fn test_true_returns_true() {
        let schema = boolean();
        let result = schema.validate(&json!(true));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn test_false_returns_false() {
        let schema = boolean();
        let result = schema.validate(&json!(false));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), false);
    }

    // Type Confusion - Truthy Values
    #[test]
    fn test_rejects_number_one() {
        let schema = boolean();
        assert!(schema.validate(&json!(1)).is_err());
    }

    #[test]
    fn test_rejects_number_zero() {
        let schema = boolean();
        assert!(schema.validate(&json!(0)).is_err());
    }

    #[test]
    fn test_rejects_string_true() {
        let schema = boolean();
        assert!(schema.validate(&json!("true")).is_err());
    }

    #[test]
    fn test_rejects_string_false() {
        let schema = boolean();
        assert!(schema.validate(&json!("false")).is_err());
    }

    #[test]
    fn test_rejects_string_yes() {
        let schema = boolean();
        assert!(schema.validate(&json!("yes")).is_err());
    }

    #[test]
    fn test_rejects_string_no() {
        let schema = boolean();
        assert!(schema.validate(&json!("no")).is_err());
    }

    // Type Confusion - Falsy Values
    #[test]
    fn test_rejects_empty_string() {
        let schema = boolean();
        assert!(schema.validate(&json!("")).is_err());
    }

    #[test]
    fn test_rejects_null() {
        let schema = boolean();
        assert!(schema.validate(&json!(null)).is_err());
    }

    // Complex Types
    #[test]
    fn test_rejects_array() {
        let schema = boolean();
        assert!(schema.validate(&json!([])).is_err());
        assert!(schema.validate(&json!([true])).is_err());
    }

    #[test]
    fn test_rejects_object() {
        let schema = boolean();
        assert!(schema.validate(&json!({})).is_err());
        assert!(schema.validate(&json!({"value": true})).is_err());
    }

    // Float Values
    #[test]
    fn test_rejects_float() {
        let schema = boolean();
        assert!(schema.validate(&json!(1.0)).is_err());
        assert!(schema.validate(&json!(0.0)).is_err());
    }

    // Negative Numbers
    #[test]
    fn test_rejects_negative_one() {
        let schema = boolean();
        assert!(schema.validate(&json!(-1)).is_err());
    }
}
