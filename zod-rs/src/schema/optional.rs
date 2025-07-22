use crate::schema::Schema;
use serde_json::Value;
use std::fmt::Debug;
use zod_rs_util::ValidateResult;

#[derive(Debug, Clone)]
pub struct OptionalSchema<S, T> {
    inner: S,
    _phantom: std::marker::PhantomData<T>,
}

impl<S, T> OptionalSchema<S, T> {
    pub fn new(inner: S) -> Self {
        Self {
            inner,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<S, T> Schema<Option<T>> for OptionalSchema<S, T>
where
    S: Schema<T>,
    T: Debug,
{
    fn validate(&self, value: &Value) -> ValidateResult<Option<T>> {
        if value.is_null() {
            Ok(None)
        } else {
            self.inner.validate(value).map(Some)
        }
    }
}

pub fn optional<S, T>(schema: S) -> OptionalSchema<S, T> {
    OptionalSchema::new(schema)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::string;
    use serde_json::json;

    #[test]
    fn test_optional_validation() {
        let schema = optional(string());

        assert!(schema.validate(&json!(null)).is_ok());
        assert!(schema.validate(&json!("hello")).is_ok());
        assert!(schema.validate(&json!(123)).is_err());
    }
}
