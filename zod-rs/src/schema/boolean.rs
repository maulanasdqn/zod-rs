use crate::schema::{value_type_name, Schema};
use serde_json::Value;
use zod_rs_util::{ValidateResult, ValidationError};

#[derive(Debug, Clone)]
pub struct BooleanSchema;

impl BooleanSchema {
    pub fn new() -> Self {
        Self
    }
}

impl Schema<bool> for BooleanSchema {
    fn validate(&self, value: &Value) -> ValidateResult<bool> {
        match value.as_bool() {
            Some(b) => Ok(b),
            None => Err(ValidationError::invalid_type("boolean", value_type_name(value)).into()),
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
}
