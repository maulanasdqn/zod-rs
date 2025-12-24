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
    use crate::schema::{literal, number, string};
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

    // ==================== EDGE CASE TESTS ====================

    // Empty Union
    #[test]
    fn test_empty_union_always_fails() {
        let schema: UnionSchema<String> = union();
        assert!(schema.validate(&json!("anything")).is_err());
        assert!(schema.validate(&json!(123)).is_err());
        assert!(schema.validate(&json!(null)).is_err());
    }

    // Single Variant Union
    #[test]
    fn test_single_variant_union() {
        let schema = union().variant(literal("only"));
        assert!(schema.validate(&json!("only")).is_ok());
        assert!(schema.validate(&json!("other")).is_err());
    }

    #[test]
    fn test_single_variant_number_union() {
        let schema = union().variant(number().positive());
        assert!(schema.validate(&json!(5)).is_ok());
        assert!(schema.validate(&json!(-5)).is_err());
    }

    // First Match Wins
    #[test]
    fn test_first_variant_matches_first() {
        // Both variants could match strings, but first should win
        let schema = union()
            .variant(string().min(1))
            .variant(string().min(5));

        // "hi" matches first variant (min 1)
        let result = schema.validate(&json!("hi"));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "hi");
    }

    #[test]
    fn test_order_matters_for_matching() {
        let schema1 = union()
            .variant(literal("a".to_string()))
            .variant(literal("b".to_string()));

        let schema2 = union()
            .variant(literal("b".to_string()))
            .variant(literal("a".to_string()));

        // Both should accept "a" and "b"
        assert!(schema1.validate(&json!("a")).is_ok());
        assert!(schema1.validate(&json!("b")).is_ok());
        assert!(schema2.validate(&json!("a")).is_ok());
        assert!(schema2.validate(&json!("b")).is_ok());
    }

    // Error Aggregation
    #[test]
    fn test_all_variants_fail_error() {
        let schema = union()
            .variant(literal("a".to_string()))
            .variant(literal("b".to_string()))
            .variant(literal("c".to_string()));

        let result = schema.validate(&json!("d"));
        assert!(result.is_err());
        let err = result.unwrap_err();
        // Should have collected errors - the error structure wraps all variant errors
        assert!(!err.issues.is_empty());
    }

    #[test]
    fn test_multiple_type_union() {
        // Union of string and number literals
        let string_schema = union()
            .variant(literal("yes".to_string()))
            .variant(literal("no".to_string()));

        let number_schema = union()
            .variant(literal(1.0))
            .variant(literal(0.0));

        assert!(string_schema.validate(&json!("yes")).is_ok());
        assert!(string_schema.validate(&json!(1)).is_err()); // Type mismatch

        assert!(number_schema.validate(&json!(1)).is_ok());
        assert!(number_schema.validate(&json!("yes")).is_err()); // Type mismatch
    }

    // Constrained Variants
    #[test]
    fn test_union_with_constrained_strings() {
        let schema = union()
            .variant(string().min(5))
            .variant(string().max(2));

        // "hello" matches first (min 5)
        assert!(schema.validate(&json!("hello")).is_ok());
        // "hi" matches second (max 2)
        assert!(schema.validate(&json!("hi")).is_ok());
        // "abc" doesn't match either (too long for second, too short for first)
        assert!(schema.validate(&json!("abc")).is_err());
    }

    // Return Value
    #[test]
    fn test_union_returns_matched_value() {
        let schema = union()
            .variant(literal("test".to_string()))
            .variant(literal("other".to_string()));

        let result = schema.validate(&json!("test"));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test");
    }

    // Type Rejection Across Variants
    #[test]
    fn test_union_rejects_wrong_types() {
        let schema = union()
            .variant(literal("a".to_string()))
            .variant(literal("b".to_string()));

        assert!(schema.validate(&json!(123)).is_err());
        assert!(schema.validate(&json!(true)).is_err());
        assert!(schema.validate(&json!(null)).is_err());
        assert!(schema.validate(&json!([])).is_err());
        assert!(schema.validate(&json!({})).is_err());
    }

    // Many Variants
    #[test]
    fn test_many_variants() {
        let schema = union()
            .variant(literal("a".to_string()))
            .variant(literal("b".to_string()))
            .variant(literal("c".to_string()))
            .variant(literal("d".to_string()))
            .variant(literal("e".to_string()));

        assert!(schema.validate(&json!("a")).is_ok());
        assert!(schema.validate(&json!("e")).is_ok());
        assert!(schema.validate(&json!("f")).is_err());
    }
}
