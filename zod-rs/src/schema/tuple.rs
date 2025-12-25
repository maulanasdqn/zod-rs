use crate::schema::Schema;
use serde_json::Value;
use std::{fmt::Debug, sync::Arc};
use zod_rs_util::{ValidateResult, ValidationError, ValidationResult, ValidationType};

#[derive(Debug, Clone)]
pub struct TupleSchema {
    elements: Vec<Arc<dyn TupleElementValidator>>,
}

impl TupleSchema {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn element<S, T>(mut self, schema: S) -> Self
    where
        S: Schema<T> + Send + Sync + 'static,
        T: serde::Serialize + Send + Sync + Debug + 'static,
    {
        self.elements
            .push(Arc::new(TupleElementValidatorImpl::new(schema)));
        self
    }
}

impl Default for TupleSchema {
    fn default() -> Self {
        Self::new()
    }
}

trait TupleElementValidator: Send + Sync + Debug {
    fn validate_element(&self, value: &Value) -> ValidateResult<Value>;
}

#[derive(Debug)]
struct TupleElementValidatorImpl<S, T> {
    schema: S,
    _phantom: std::marker::PhantomData<T>,
}

impl<S, T> TupleElementValidatorImpl<S, T> {
    fn new(schema: S) -> Self {
        Self {
            schema,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<S, T> TupleElementValidator for TupleElementValidatorImpl<S, T>
where
    S: Schema<T> + Send + Sync + Debug,
    T: serde::Serialize + Send + Sync + Debug,
{
    fn validate_element(&self, value: &Value) -> ValidateResult<Value> {
        let validated = self.schema.validate(value)?;
        serde_json::to_value(validated).map_err(|e| {
            ValidationError::custom(format!("Failed to serialize validated value: {}", e)).into()
        })
    }
}

impl Schema<Value> for TupleSchema {
    fn validate(&self, value: &Value) -> ValidateResult<Value> {
        let arr = value.as_array().ok_or_else(|| {
            ValidationResult::from(ValidationError::invalid_type(
                ValidationType::Array,
                ValidationType::from(value),
            ))
        })?;

        if arr.len() != self.elements.len() {
            return Err(ValidationError::custom(format!(
                "Expected tuple of {} elements, got {}",
                self.elements.len(),
                arr.len()
            ))
            .into());
        }

        let mut result = Vec::with_capacity(arr.len());
        let mut validation_result = ValidationResult::new();

        for (i, (element, schema)) in arr.iter().zip(&self.elements).enumerate() {
            match schema.validate_element(element) {
                Ok(validated) => result.push(validated),
                Err(mut errors) => {
                    errors.prefix_path(i.to_string());
                    validation_result.merge(errors);
                }
            }
        }

        if validation_result.is_empty() {
            Ok(Value::Array(result))
        } else {
            Err(validation_result)
        }
    }
}

pub fn tuple() -> TupleSchema {
    TupleSchema::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{boolean, number, string};
    use serde_json::json;

    #[test]
    fn test_empty_tuple() {
        let schema = tuple();
        assert!(schema.validate(&json!([])).is_ok());
        assert!(schema.validate(&json!([1])).is_err());
    }

    #[test]
    fn test_single_element_tuple() {
        let schema = tuple().element(string());
        assert!(schema.validate(&json!(["hello"])).is_ok());
        assert!(schema.validate(&json!([123])).is_err());
        assert!(schema.validate(&json!([])).is_err());
        assert!(schema.validate(&json!(["a", "b"])).is_err());
    }

    #[test]
    fn test_two_element_tuple() {
        let schema = tuple().element(string()).element(number());
        assert!(schema.validate(&json!(["hello", 42])).is_ok());
        assert!(schema.validate(&json!([42, "hello"])).is_err());
        assert!(schema.validate(&json!(["hello"])).is_err());
        assert!(schema.validate(&json!(["hello", 42, true])).is_err());
    }

    #[test]
    fn test_three_element_tuple() {
        let schema = tuple()
            .element(string())
            .element(number())
            .element(boolean());

        assert!(schema.validate(&json!(["test", 123, true])).is_ok());
        assert!(schema.validate(&json!(["test", 123, false])).is_ok());
        assert!(schema.validate(&json!(["test", 123])).is_err());
    }

    #[test]
    fn test_tuple_with_constraints() {
        let schema = tuple()
            .element(string().min(3))
            .element(number().positive());

        assert!(schema.validate(&json!(["abc", 1])).is_ok());
        assert!(schema.validate(&json!(["ab", 1])).is_err()); // String too short
        assert!(schema.validate(&json!(["abc", -1])).is_err()); // Number not positive
    }

    #[test]
    fn test_tuple_returns_array_value() {
        let schema = tuple().element(string()).element(number());
        let result = schema.validate(&json!(["hello", 42]));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), json!(["hello", 42.0]));
    }

    #[test]
    fn test_tuple_rejects_non_array() {
        let schema = tuple().element(string());
        assert!(schema.validate(&json!("hello")).is_err());
        assert!(schema.validate(&json!(123)).is_err());
        assert!(schema.validate(&json!(null)).is_err());
        assert!(schema.validate(&json!({})).is_err());
    }

    #[test]
    fn test_tuple_error_includes_index() {
        let schema = tuple().element(string()).element(number());
        let result = schema.validate(&json!(["hello", "not a number"]));
        assert!(result.is_err());
        let err = result.unwrap_err();
        // Error should include path information about which element failed
        assert!(!err.issues.is_empty());
    }

    #[test]
    fn test_homogeneous_tuple() {
        let schema = tuple()
            .element(number())
            .element(number())
            .element(number());

        assert!(schema.validate(&json!([1, 2, 3])).is_ok());
        assert!(schema.validate(&json!([1, 2])).is_err());
        assert!(schema.validate(&json!([1, 2, "3"])).is_err());
    }
}
