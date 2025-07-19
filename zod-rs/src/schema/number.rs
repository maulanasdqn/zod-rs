use serde_json::Value;
use zod_rs_util::{ValidateResult, ValidationError};

use crate::schema::{value_type_name, Schema};

#[derive(Debug)]
pub struct NumberSchema {
    min: Option<f64>,
    max: Option<f64>,
    integer: bool,
    positive: bool,
    negative: bool,
    nonnegative: bool,
    nonpositive: bool,
    finite: bool,
}

impl NumberSchema {
    pub fn new() -> Self {
        Self {
            min: None,
            max: None,
            integer: false,
            positive: false,
            negative: false,
            nonnegative: false,
            nonpositive: false,
            finite: false,
        }
    }

    pub fn min(mut self, min: f64) -> Self {
        self.min = Some(min);
        self
    }

    pub fn max(mut self, max: f64) -> Self {
        self.max = Some(max);
        self
    }

    pub fn int(mut self) -> Self {
        self.integer = true;
        self
    }

    pub fn positive(mut self) -> Self {
        self.positive = true;
        self
    }

    pub fn negative(mut self) -> Self {
        self.negative = true;
        self
    }

    pub fn nonnegative(mut self) -> Self {
        self.nonnegative = true;
        self
    }

    pub fn nonpositive(mut self) -> Self {
        self.nonpositive = true;
        self
    }

    pub fn finite(mut self) -> Self {
        self.finite = true;
        self
    }
}

impl Schema<f64> for NumberSchema {
    fn validate(&self, value: &Value) -> ValidateResult<f64> {
        let num = match value.as_f64() {
            Some(n) => n,
            None => {
                return Err(ValidationError::invalid_type("number", value_type_name(value)).into());
            }
        };

        if self.integer && num.fract() != 0.0 {
            return Err(ValidationError::invalid_type("integer", "float").into());
        }

        if self.finite && !num.is_finite() {
            return Err(ValidationError::custom("number must be finite").into());
        }

        if let Some(min) = self.min {
            if num < min {
                return Err(ValidationError::too_small(num.to_string(), min.to_string()).into());
            }
        }

        if let Some(max) = self.max {
            if num > max {
                return Err(ValidationError::too_big(num.to_string(), max.to_string()).into());
            }
        }

        if self.positive && num <= 0.0 {
            return Err(ValidationError::custom("number must be positive").into());
        }

        if self.negative && num >= 0.0 {
            return Err(ValidationError::custom("number must be negative").into());
        }

        if self.nonnegative && num < 0.0 {
            return Err(ValidationError::custom("number must be non-negative").into());
        }

        if self.nonpositive && num > 0.0 {
            return Err(ValidationError::custom("number must be non-positive").into());
        }

        Ok(num)
    }
}

pub fn number() -> NumberSchema {
    NumberSchema::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_number_validation() {
        let schema = number().min(0.0).max(100.0);

        assert!(schema.validate(&json!(50.5)).is_ok());
        assert!(schema.validate(&json!(-1.0)).is_err());
        assert!(schema.validate(&json!(101.0)).is_err());
        assert!(schema.validate(&json!("not a number")).is_err());
    }
}
