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
    use crate::schema::string;
    use serde_json::json;

    #[test]
    fn test_array_validation() {
        let schema = array(string()).min(1).max(3);

        assert!(schema.validate(&json!(["hello", "world"])).is_ok());
        assert!(schema.validate(&json!([])).is_err());
        assert!(schema.validate(&json!(["a", "b", "c", "d"])).is_err());
        assert!(schema.validate(&json!([1, 2, 3])).is_err());
    }
}
