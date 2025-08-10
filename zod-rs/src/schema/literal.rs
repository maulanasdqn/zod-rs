use crate::schema::Schema;
use serde_json::Value;
use zod_rs_util::{ValidateResult, ValidationError, ValidationType};

#[derive(Debug, Clone)]
pub struct LiteralSchema<T: Clone + PartialEq + std::fmt::Debug> {
    expected: T,
}

impl<T: Clone + PartialEq + std::fmt::Debug> LiteralSchema<T> {
    pub fn new(expected: T) -> Self {
        Self { expected }
    }
}

impl Schema<String> for LiteralSchema<&'static str> {
    fn validate(&self, value: &Value) -> ValidateResult<String> {
        match value.as_str() {
            Some(s) if s == self.expected => Ok(s.to_string()),
            Some(_) => Err(ValidationError::invalid_value(self.expected).into()),
            None => Err(ValidationError::invalid_type(
                ValidationType::String,
                ValidationType::from(value),
            )
            .into()),
        }
    }
}

impl Schema<String> for LiteralSchema<String> {
    fn validate(&self, value: &Value) -> ValidateResult<String> {
        match value.as_str() {
            Some(s) if s == self.expected => Ok(s.to_string()),
            Some(_) => Err(ValidationError::invalid_value(&self.expected).into()),
            None => Err(ValidationError::invalid_type(
                ValidationType::String,
                ValidationType::from(value),
            )
            .into()),
        }
    }
}

impl Schema<f64> for LiteralSchema<f64> {
    fn validate(&self, value: &Value) -> ValidateResult<f64> {
        match value.as_f64() {
            Some(n) if (n - self.expected).abs() < f64::EPSILON => Ok(n),
            Some(_) => Err(ValidationError::invalid_value(self.expected.to_string()).into()),
            None => Err(ValidationError::invalid_type(
                ValidationType::Number,
                ValidationType::from(value),
            )
            .into()),
        }
    }
}

pub fn literal<T: Clone + PartialEq + std::fmt::Debug>(value: T) -> LiteralSchema<T> {
    LiteralSchema::new(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{literal, union};
    use serde_json::json;

    #[test]
    fn test_string_validation() {
        let string_literals = union()
            .variant(literal("hello".to_string()))
            .variant(literal("world"));

        assert!(string_literals.validate(&json!("hello")).is_ok());
        assert!(string_literals.validate(&json!("world")).is_ok());
        assert!(string_literals.validate(&json!("other")).is_err());
    }

    #[test]
    fn test_f64_validation() {
        let number_literals = union().variant(literal(123.0)).variant(literal(456.0));

        assert!(number_literals.validate(&json!(123)).is_ok());
        assert!(number_literals.validate(&json!(456)).is_ok());
        assert!(number_literals.validate(&json!(789)).is_err());
    }
}
