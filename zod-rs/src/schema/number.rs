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

    // ==================== EDGE CASE TESTS ====================

    // Floating Point Edge Cases
    #[test]
    fn test_nan_with_finite() {
        let schema = number().finite();
        assert!(schema.validate(&json!(f64::NAN)).is_err());
    }

    #[test]
    fn test_infinity_with_finite() {
        let schema = number().finite();
        assert!(schema.validate(&json!(f64::INFINITY)).is_err());
    }

    #[test]
    fn test_neg_infinity_with_finite() {
        let schema = number().finite();
        assert!(schema.validate(&json!(f64::NEG_INFINITY)).is_err());
    }

    #[test]
    fn test_finite_accepts_normal_numbers() {
        let schema = number().finite();
        assert!(schema.validate(&json!(0.0)).is_ok());
        assert!(schema.validate(&json!(1.5)).is_ok());
        assert!(schema.validate(&json!(-100.0)).is_ok());
    }

    #[test]
    fn test_very_large_number() {
        let schema = number();
        assert!(schema.validate(&json!(f64::MAX)).is_ok());
    }

    #[test]
    fn test_very_small_positive_number() {
        let schema = number();
        assert!(schema.validate(&json!(f64::MIN_POSITIVE)).is_ok());
    }

    #[test]
    fn test_negative_zero() {
        let schema = number();
        assert!(schema.validate(&json!(-0.0)).is_ok());
        // -0.0 equals 0.0 in IEEE 754
        assert_eq!(schema.validate(&json!(-0.0)).unwrap(), 0.0);
    }

    // Integer Validation Edge Cases
    #[test]
    fn test_zero_as_integer() {
        let schema = number().int();
        assert!(schema.validate(&json!(0)).is_ok());
        assert!(schema.validate(&json!(0.0)).is_ok());
    }

    #[test]
    fn test_positive_integer() {
        let schema = number().int();
        assert!(schema.validate(&json!(5)).is_ok());
        assert!(schema.validate(&json!(5.0)).is_ok());
    }

    #[test]
    fn test_negative_integer() {
        let schema = number().int();
        assert!(schema.validate(&json!(-5)).is_ok());
        assert!(schema.validate(&json!(-5.0)).is_ok());
    }

    #[test]
    fn test_float_fails_int() {
        let schema = number().int();
        assert!(schema.validate(&json!(5.5)).is_err());
        assert!(schema.validate(&json!(0.1)).is_err());
        assert!(schema.validate(&json!(-3.14)).is_err());
    }

    #[test]
    fn test_very_small_fraction_fails_int() {
        let schema = number().int();
        assert!(schema.validate(&json!(5.0000001)).is_err());
    }

    // Positive/Negative Constraint Edge Cases
    #[test]
    fn test_zero_with_positive() {
        let schema = number().positive();
        assert!(schema.validate(&json!(0.0)).is_err()); // 0 is not positive
    }

    #[test]
    fn test_zero_with_negative() {
        let schema = number().negative();
        assert!(schema.validate(&json!(0.0)).is_err()); // 0 is not negative
    }

    #[test]
    fn test_zero_with_nonnegative() {
        let schema = number().nonnegative();
        assert!(schema.validate(&json!(0.0)).is_ok()); // 0 is nonnegative
    }

    #[test]
    fn test_zero_with_nonpositive() {
        let schema = number().nonpositive();
        assert!(schema.validate(&json!(0.0)).is_ok()); // 0 is nonpositive
    }

    #[test]
    fn test_positive_number_with_positive() {
        let schema = number().positive();
        assert!(schema.validate(&json!(0.0001)).is_ok());
        assert!(schema.validate(&json!(100.0)).is_ok());
    }

    #[test]
    fn test_negative_number_with_negative() {
        let schema = number().negative();
        assert!(schema.validate(&json!(-0.0001)).is_ok());
        assert!(schema.validate(&json!(-100.0)).is_ok());
    }

    #[test]
    fn test_positive_with_nonnegative() {
        let schema = number().nonnegative();
        assert!(schema.validate(&json!(5.0)).is_ok());
        assert!(schema.validate(&json!(-5.0)).is_err());
    }

    #[test]
    fn test_negative_with_nonpositive() {
        let schema = number().nonpositive();
        assert!(schema.validate(&json!(-5.0)).is_ok());
        assert!(schema.validate(&json!(5.0)).is_err());
    }

    // Boundary Conditions
    #[test]
    fn test_exactly_at_min() {
        let schema = number().min(5.0);
        assert!(schema.validate(&json!(5.0)).is_ok());
        assert!(schema.validate(&json!(4.9999)).is_err());
    }

    #[test]
    fn test_exactly_at_max() {
        let schema = number().max(5.0);
        assert!(schema.validate(&json!(5.0)).is_ok());
        assert!(schema.validate(&json!(5.0001)).is_err());
    }

    #[test]
    fn test_negative_min() {
        let schema = number().min(-10.0);
        assert!(schema.validate(&json!(-10.0)).is_ok());
        assert!(schema.validate(&json!(-5.0)).is_ok());
        assert!(schema.validate(&json!(-11.0)).is_err());
    }

    #[test]
    fn test_negative_max() {
        let schema = number().max(-1.0);
        assert!(schema.validate(&json!(-1.0)).is_ok());
        assert!(schema.validate(&json!(-5.0)).is_ok());
        assert!(schema.validate(&json!(0.0)).is_err());
    }

    #[test]
    fn test_min_equals_max() {
        let schema = number().min(5.0).max(5.0);
        assert!(schema.validate(&json!(5.0)).is_ok());
        assert!(schema.validate(&json!(5.1)).is_err());
        assert!(schema.validate(&json!(4.9)).is_err());
    }

    // Constraint Conflicts
    #[test]
    fn test_impossible_constraint_min_greater_than_max() {
        let schema = number().min(10.0).max(5.0);
        // All numbers will fail
        assert!(schema.validate(&json!(7.0)).is_err());
        assert!(schema.validate(&json!(3.0)).is_err());
        assert!(schema.validate(&json!(12.0)).is_err());
    }

    #[test]
    fn test_conflicting_positive_and_negative() {
        // Both positive and negative means nothing can pass
        let schema = number().positive().negative();
        assert!(schema.validate(&json!(5.0)).is_err());
        assert!(schema.validate(&json!(-5.0)).is_err());
        assert!(schema.validate(&json!(0.0)).is_err());
    }

    // Type Rejection
    #[test]
    fn test_rejects_null() {
        let schema = number();
        assert!(schema.validate(&json!(null)).is_err());
    }

    #[test]
    fn test_rejects_boolean() {
        let schema = number();
        assert!(schema.validate(&json!(true)).is_err());
        assert!(schema.validate(&json!(false)).is_err());
    }

    #[test]
    fn test_rejects_string() {
        let schema = number();
        assert!(schema.validate(&json!("123")).is_err());
        assert!(schema.validate(&json!("3.14")).is_err());
    }

    #[test]
    fn test_rejects_array() {
        let schema = number();
        assert!(schema.validate(&json!([1, 2, 3])).is_err());
    }

    #[test]
    fn test_rejects_object() {
        let schema = number();
        assert!(schema.validate(&json!({"value": 42})).is_err());
    }

    // Combined Constraints
    #[test]
    fn test_int_and_positive() {
        let schema = number().int().positive();
        assert!(schema.validate(&json!(5)).is_ok());
        assert!(schema.validate(&json!(0)).is_err()); // 0 not positive
        assert!(schema.validate(&json!(-5)).is_err()); // negative
        assert!(schema.validate(&json!(5.5)).is_err()); // not int
    }

    #[test]
    fn test_finite_and_min_max() {
        let schema = number().finite().min(0.0).max(100.0);
        assert!(schema.validate(&json!(50.0)).is_ok());
        assert!(schema.validate(&json!(f64::INFINITY)).is_err());
        assert!(schema.validate(&json!(150.0)).is_err());
    }

    #[test]
    fn test_nonnegative_int() {
        let schema = number().nonnegative().int();
        assert!(schema.validate(&json!(0)).is_ok());
        assert!(schema.validate(&json!(5)).is_ok());
        assert!(schema.validate(&json!(-1)).is_err());
        assert!(schema.validate(&json!(5.5)).is_err());
    }

    // JSON integer vs float
    #[test]
    fn test_json_integer_parsed_as_f64() {
        let schema = number();
        // JSON integers are parsed as f64
        let result = schema.validate(&json!(42));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42.0);
    }
}
