use crate::schema::Schema;
use serde_json::Value;
use std::{fmt::Debug, sync::Arc};
use zod_rs_util::{ValidateResult, ValidationError};

#[derive(Debug, Clone)]
pub struct UnionSchema<T>
where
    T: Debug,
{
    schemas: Vec<Arc<dyn Schema<T> + Send + Sync>>,
}

impl<T> UnionSchema<T>
where
    T: Debug,
{
    pub fn new() -> Self {
        Self {
            schemas: Vec::new(),
        }
    }

    pub fn variant<S>(mut self, schema: S) -> Self
    where
        S: Schema<T> + Send + Sync + Debug + 'static,
        T: Debug,
    {
        self.schemas.push(Arc::new(schema));
        self
    }
}

impl<T> Default for UnionSchema<T>
where
    T: Debug,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Schema<T> for UnionSchema<T>
where
    T: Debug,
{
    fn validate(&self, value: &Value) -> ValidateResult<T> {
        let mut issues = vec![];

        for schema in &self.schemas {
            match schema.validate(value) {
                Ok(result) => return Ok(result),
                Err(error) => {
                    issues.extend(error.issues);
                }
            }
        }

        Err(ValidationError::invalid_union(issues).into())
    }
}

pub fn union<T>() -> UnionSchema<T>
where
    T: Debug,
{
    UnionSchema::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::literal;
    use serde_json::json;

    #[test]
    fn test_union_validation() {
        let string_union = union()
            .variant(literal("hello".to_string()))
            .variant(literal("world".to_string()));
        let number_union = union().variant(literal(123.0)).variant(literal(456.0));

        assert!(string_union.validate(&json!("hello")).is_ok());
        assert!(string_union.validate(&json!("world")).is_ok());
        assert!(string_union.validate(&json!("other")).is_err());

        assert!(number_union.validate(&json!(123)).is_ok());
        assert!(number_union.validate(&json!(456)).is_ok());
        assert!(number_union.validate(&json!(789)).is_err());
    }
}
