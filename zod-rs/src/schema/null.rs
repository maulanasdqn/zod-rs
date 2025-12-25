use crate::schema::Schema;
use serde_json::Value;
use zod_rs_util::{ValidateResult, ValidationError, ValidationType};

#[derive(Debug, Clone)]
pub struct NullSchema;

impl NullSchema {
    pub fn new() -> Self {
        Self
    }
}

impl Default for NullSchema {
    fn default() -> Self {
        Self::new()
    }
}

impl Schema<()> for NullSchema {
    fn validate(&self, value: &Value) -> ValidateResult<()> {
        if value.is_null() {
            Ok(())
        } else {
            Err(ValidationError::invalid_type(
                ValidationType::Null,
                ValidationType::from(value),
            )
            .into())
        }
    }
}

pub fn null() -> NullSchema {
    NullSchema::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_null_validation() {
        let schema = null();

        assert!(schema.validate(&json!(null)).is_ok());
        assert!(schema.validate(&json!(true)).is_err());
        assert!(schema.validate(&json!(false)).is_err());
        assert!(schema.validate(&json!("null")).is_err());
        assert!(schema.validate(&json!(0)).is_err());
    }

    #[test]
    fn test_null_returns_unit() {
        let schema = null();
        let result = schema.validate(&json!(null));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ());
    }

    #[test]
    fn test_rejects_empty_string() {
        let schema = null();
        assert!(schema.validate(&json!("")).is_err());
    }

    #[test]
    fn test_rejects_array() {
        let schema = null();
        assert!(schema.validate(&json!([])).is_err());
    }

    #[test]
    fn test_rejects_object() {
        let schema = null();
        assert!(schema.validate(&json!({})).is_err());
    }
}
