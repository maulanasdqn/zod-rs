use crate::schema::Schema;
use serde_json::Value;
use zod_rs_util::{
    NumberConstraint, ValidateResult, ValidationError, ValidationOrigin, ValidationType,
};

#[derive(Debug, Clone)]
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

impl Default for NumberSchema {
    fn default() -> Self {
        Self::new()
    }
}

impl Schema<f64> for NumberSchema {
    fn validate(&self, value: &Value) -> ValidateResult<f64> {
        let num = match value.as_f64() {
            Some(n) => n,
            None => {
                return Err(ValidationError::invalid_type(
                    ValidationType::Number,
                    ValidationType::from(value),
                )
                .into());
            }
        };

        if self.integer && num.fract() != 0.0 {
            return Err(ValidationError::invalid_type(
                ValidationType::custom("integer"),
                ValidationType::custom("float"),
            )
            .into());
        }

        if self.finite && !num.is_finite() {
            return Err(ValidationError::invalid_number(NumberConstraint::Finite).into());
        }

        if let Some(min) = self.min {
            if num < min {
                return Err(ValidationError::too_small(
                    ValidationOrigin::Number,
                    min.to_string(),
                    true,
                )
                .into());
            }
        }

        if let Some(max) = self.max {
            if num > max {
                return Err(ValidationError::too_big(
                    ValidationOrigin::Number,
                    max.to_string(),
                    true,
                )
                .into());
            }
        }

        if self.positive && num <= 0.0 {
            return Err(ValidationError::invalid_number(NumberConstraint::Positive).into());
        }

        if self.negative && num >= 0.0 {
            return Err(ValidationError::invalid_number(NumberConstraint::Negative).into());
        }

        if self.nonnegative && num < 0.0 {
            return Err(ValidationError::invalid_number(NumberConstraint::NonNegative).into());
        }

        if self.nonpositive && num > 0.0 {
            return Err(ValidationError::invalid_number(NumberConstraint::NonPositive).into());
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
