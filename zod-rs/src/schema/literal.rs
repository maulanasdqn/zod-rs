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

    // ==================== EDGE CASE TESTS ====================

    // String Literal Edge Cases
    #[test]
    fn test_empty_string_literal() {
        let schema = literal("");
        assert!(schema.validate(&json!("")).is_ok());
        assert!(schema.validate(&json!("anything")).is_err());
    }

    #[test]
    fn test_unicode_string_literal() {
        let schema = literal("🦀");
        assert!(schema.validate(&json!("🦀")).is_ok());
        assert!(schema.validate(&json!("crab")).is_err());
    }

    #[test]
    fn test_string_literal_with_special_chars() {
        let schema = literal("hello@world.com");
        assert!(schema.validate(&json!("hello@world.com")).is_ok());
        assert!(schema.validate(&json!("hello")).is_err());
    }

    #[test]
    fn test_string_literal_case_sensitive() {
        let schema = literal("Hello");
        assert!(schema.validate(&json!("Hello")).is_ok());
        assert!(schema.validate(&json!("hello")).is_err());
        assert!(schema.validate(&json!("HELLO")).is_err());
    }

    #[test]
    fn test_string_literal_with_spaces() {
        let schema = literal("hello world");
        assert!(schema.validate(&json!("hello world")).is_ok());
        assert!(schema.validate(&json!("helloworld")).is_err());
    }

    #[test]
    fn test_string_literal_with_newline() {
        let schema = literal("hello\nworld");
        assert!(schema.validate(&json!("hello\nworld")).is_ok());
        assert!(schema.validate(&json!("hello world")).is_err());
    }

    #[test]
    fn test_owned_string_literal() {
        let schema = literal("test".to_string());
        assert!(schema.validate(&json!("test")).is_ok());
        assert!(schema.validate(&json!("other")).is_err());
    }

    // Numeric Literal Edge Cases
    #[test]
    fn test_zero_literal() {
        let schema = literal(0.0);
        assert!(schema.validate(&json!(0)).is_ok());
        assert!(schema.validate(&json!(0.0)).is_ok());
        assert!(schema.validate(&json!(1)).is_err());
    }

    #[test]
    fn test_negative_literal() {
        let schema = literal(-5.5);
        assert!(schema.validate(&json!(-5.5)).is_ok());
        assert!(schema.validate(&json!(5.5)).is_err());
    }

    #[test]
    fn test_integer_as_float_literal() {
        let schema = literal(42.0);
        assert!(schema.validate(&json!(42)).is_ok());
        assert!(schema.validate(&json!(42.0)).is_ok());
    }

    #[test]
    fn test_very_small_literal() {
        let schema = literal(0.001);
        assert!(schema.validate(&json!(0.001)).is_ok());
        assert!(schema.validate(&json!(0.002)).is_err());
    }

    // Type Rejection
    #[test]
    fn test_string_literal_rejects_number() {
        let schema = literal("hello");
        assert!(schema.validate(&json!(123)).is_err());
    }

    #[test]
    fn test_number_literal_rejects_string() {
        let schema = literal(123.0);
        assert!(schema.validate(&json!("123")).is_err());
    }

    #[test]
    fn test_string_literal_rejects_null() {
        let schema = literal("hello");
        assert!(schema.validate(&json!(null)).is_err());
    }

    #[test]
    fn test_number_literal_rejects_null() {
        let schema = literal(123.0);
        assert!(schema.validate(&json!(null)).is_err());
    }

    #[test]
    fn test_string_literal_rejects_boolean() {
        let schema = literal("true");
        assert!(schema.validate(&json!(true)).is_err());
    }

    #[test]
    fn test_string_literal_rejects_array() {
        let schema = literal("hello");
        assert!(schema.validate(&json!(["hello"])).is_err());
    }

    #[test]
    fn test_string_literal_rejects_object() {
        let schema = literal("hello");
        assert!(schema.validate(&json!({"value": "hello"})).is_err());
    }

    // Return Value Verification
    #[test]
    fn test_string_literal_returns_value() {
        let schema = literal("test");
        let result = schema.validate(&json!("test"));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test");
    }

    #[test]
    fn test_number_literal_returns_value() {
        let schema = literal(42.5);
        let result = schema.validate(&json!(42.5));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42.5);
    }
}
