use crate::schema::{value_type_name, Schema};
use serde_json::Value;
use zod_rs_util::{ValidateResult, ValidationError};

#[derive(Debug, Clone)]
pub struct LiteralSchema<T: Clone + PartialEq + std::fmt::Debug> {
    expected: T,
}

impl<T: Clone + PartialEq + std::fmt::Debug> LiteralSchema<T> {
    pub fn new(expected: T) -> Self {
        Self { expected }
    }
}

impl Schema<String> for LiteralSchema<String> {
    fn validate(&self, value: &Value) -> ValidateResult<String> {
        match value.as_str() {
            Some(s) if s == self.expected => Ok(s.to_string()),
            Some(s) => Err(ValidationError::custom(format!(
                "Expected '{}', got '{}'",
                self.expected, s
            ))
            .into()),
            None => Err(ValidationError::invalid_type("string", value_type_name(value)).into()),
        }
    }
}

impl Schema<f64> for LiteralSchema<f64> {
    fn validate(&self, value: &Value) -> ValidateResult<f64> {
        match value.as_f64() {
            Some(n) if (n - self.expected).abs() < f64::EPSILON => Ok(n),
            Some(n) => Err(ValidationError::custom(format!(
                "Expected {}, got {}",
                self.expected, n
            ))
            .into()),
            None => Err(ValidationError::invalid_type("number", value_type_name(value)).into()),
        }
    }
}

pub fn literal<T: Clone + PartialEq + std::fmt::Debug>(value: T) -> LiteralSchema<T> {
    LiteralSchema::new(value)
}
