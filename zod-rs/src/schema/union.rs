use crate::schema::Schema;
use serde_json::Value;
use zod_rs_util::{ValidateResult, ValidationError};

pub struct UnionSchema<T> {
    schemas: Vec<Box<dyn Schema<T> + Send + Sync>>,
}

impl<T> UnionSchema<T> {
    pub fn new() -> Self {
        Self {
            schemas: Vec::new(),
        }
    }

    pub fn variant<S>(mut self, schema: S) -> Self
    where
        S: Schema<T> + Send + Sync + 'static,
    {
        self.schemas.push(Box::new(schema));
        self
    }
}

impl<T> Schema<T> for UnionSchema<T> {
    fn validate(&self, value: &Value) -> ValidateResult<T> {
        for schema in &self.schemas {
            if let Ok(result) = schema.validate(value) {
                return Ok(result);
            }
        }
        Err(ValidationError::union_mismatch().into())
    }
}

pub fn union<T>() -> UnionSchema<T> {
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
