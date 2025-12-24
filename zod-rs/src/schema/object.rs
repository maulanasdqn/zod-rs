use crate::schema::Schema;
use serde_json::Value;
use std::{collections::HashMap, fmt::Debug, sync::Arc};
use zod_rs_util::{ValidateResult, ValidationError, ValidationResult, ValidationType};

#[derive(Debug, Clone)]
pub struct ObjectSchema {
    fields: HashMap<String, Arc<dyn ObjectFieldValidator>>,
    strict: bool,
}

impl ObjectSchema {
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
            strict: false,
        }
    }

    pub fn field<S, T>(mut self, name: &str, schema: S) -> Self
    where
        S: Schema<T> + Send + Sync + 'static,
        T: serde::Serialize + Send + Sync + Debug + 'static,
    {
        self.fields.insert(
            name.to_string(),
            Arc::new(RequiredFieldValidator::new(schema)),
        );
        self
    }

    pub fn optional_field<S, T>(mut self, name: &str, schema: S) -> Self
    where
        S: Schema<T> + Send + Sync + 'static,
        T: serde::Serialize + Send + Sync + Debug + 'static,
    {
        self.fields.insert(
            name.to_string(),
            Arc::new(OptionalFieldValidator::new(schema)),
        );
        self
    }

    pub fn strict(mut self) -> Self {
        self.strict = true;
        self
    }
}

impl Default for ObjectSchema {
    fn default() -> Self {
        Self::new()
    }
}

trait ObjectFieldValidator: Send + Sync + Debug {
    fn validate_field(&self, value: Option<&Value>) -> ValidateResult<Value>;
    fn is_optional(&self) -> bool;
}

#[derive(Debug)]
struct RequiredFieldValidator<S, T> {
    schema: S,
    _phantom: std::marker::PhantomData<T>,
}

impl<S, T> RequiredFieldValidator<S, T> {
    fn new(schema: S) -> Self {
        Self {
            schema,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<S, T> ObjectFieldValidator for RequiredFieldValidator<S, T>
where
    S: Schema<T> + Send + Sync + Debug,
    T: serde::Serialize + Send + Sync + Debug,
{
    fn validate_field(&self, value: Option<&Value>) -> ValidateResult<Value> {
        match value {
            Some(v) => {
                let validated = self.schema.validate(v)?;
                serde_json::to_value(validated).map_err(|e| {
                    ValidationError::custom(format!("Failed to serialize validated value: {}", e))
                        .into()
                })
            }
            None => Err(ValidationError::required().into()),
        }
    }

    fn is_optional(&self) -> bool {
        false
    }
}

#[derive(Debug)]
struct OptionalFieldValidator<S, T> {
    schema: S,
    _phantom: std::marker::PhantomData<T>,
}

impl<S, T> OptionalFieldValidator<S, T> {
    fn new(schema: S) -> Self {
        Self {
            schema,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<S, T> ObjectFieldValidator for OptionalFieldValidator<S, T>
where
    S: Schema<T> + Send + Sync + Debug,
    T: serde::Serialize + Send + Sync + Debug,
{
    fn validate_field(&self, value: Option<&Value>) -> ValidateResult<Value> {
        match value {
            Some(v) if !v.is_null() => {
                let validated = self.schema.validate(v)?;
                serde_json::to_value(validated).map_err(|e| {
                    ValidationError::custom(format!("Failed to serialize validated value: {}", e))
                        .into()
                })
            }
            _ => Ok(Value::Null),
        }
    }

    fn is_optional(&self) -> bool {
        true
    }
}

impl Schema<Value> for ObjectSchema {
    fn validate(&self, value: &Value) -> ValidateResult<Value> {
        let obj = match value.as_object() {
            Some(o) => o,
            None => {
                return Err(ValidationError::invalid_type(
                    ValidationType::Object,
                    ValidationType::from(value),
                )
                .into());
            }
        };

        let mut result = serde_json::Map::new();
        let mut validation_result = ValidationResult::new();

        for (field_name, validator) in &self.fields {
            let field_value = obj.get(field_name);
            match validator.validate_field(field_value) {
                Ok(validated_value) => {
                    if !validated_value.is_null() || !validator.is_optional() {
                        result.insert(field_name.clone(), validated_value);
                    }
                }
                Err(mut errors) => {
                    errors.prefix_path(field_name.clone());
                    validation_result.merge(errors);
                }
            }
        }

        if self.strict {
            let mut unrecognized_keys = vec![];

            for key in obj.keys() {
                if !self.fields.contains_key(key) {
                    unrecognized_keys.push(key.clone());
                }
            }

            if !unrecognized_keys.is_empty() {
                validation_result.add_error_at_path(
                    vec![],
                    ValidationError::unrecognized_keys(unrecognized_keys),
                );
            }
        } else {
            for (key, value) in obj {
                if !self.fields.contains_key(key) {
                    result.insert(key.clone(), value.clone());
                }
            }
        }

        if validation_result.is_empty() {
            Ok(Value::Object(result))
        } else {
            Err(validation_result)
        }
    }
}

pub fn object() -> ObjectSchema {
    ObjectSchema::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{array, number, string};
    use serde_json::json;

    #[test]
    fn test_object_validation() {
        let schema = object()
            .field("name", string().min(1))
            .field("age", number().min(0.0))
            .optional_field("email", string().email());

        let schema_strict = schema.clone().strict();

        let schema_with_username = schema.clone().field("username", string().min(3));

        assert!(schema_with_username
            .validate(&json!({
                "name": "John",
                "age": 25,
                "email": "john@example.com",
                "username":"j.doe"
            }))
            .is_ok());

        assert!(schema_with_username
            .validate(&json!({
                "name": "John",
                "age": 25,
                "email": "john@example.com",
            }))
            .is_err());

        assert!(schema
            .validate(&json!({
                "name": "John",
                "age": 25,
                "email": "john@example.com"
            }))
            .is_ok());

        assert!(schema_strict
            .validate(&json!({
                "name": "John",
                "age": 25,
                "email": "john@example.com",
                "username":"j.doe"
            }))
            .is_err());

        assert!(schema
            .validate(&json!({
                "name": "John",
                "age": 25
            }))
            .is_ok());

        assert!(schema
            .validate(&json!({
                "age": 25
            }))
            .is_err());
    }

    // ==================== EDGE CASE TESTS ====================

    // Required Field Edge Cases
    #[test]
    fn test_null_for_required_field() {
        let schema = object().field("name", string());
        assert!(schema.validate(&json!({"name": null})).is_err());
    }

    #[test]
    fn test_missing_required_field() {
        let schema = object().field("name", string());
        assert!(schema.validate(&json!({})).is_err());
    }

    #[test]
    fn test_empty_string_for_required_field() {
        let schema = object().field("name", string());
        assert!(schema.validate(&json!({"name": ""})).is_ok());
    }

    #[test]
    fn test_empty_string_with_min_constraint() {
        let schema = object().field("name", string().min(1));
        assert!(schema.validate(&json!({"name": ""})).is_err());
        assert!(schema.validate(&json!({"name": "a"})).is_ok());
    }

    // Optional Field Edge Cases
    #[test]
    fn test_optional_field_null() {
        let schema = object().optional_field("email", string());
        let result = schema.validate(&json!({"email": null}));
        assert!(result.is_ok());
    }

    #[test]
    fn test_optional_field_missing() {
        let schema = object().optional_field("email", string());
        let result = schema.validate(&json!({}));
        assert!(result.is_ok());
    }

    #[test]
    fn test_optional_field_with_value() {
        let schema = object().optional_field("email", string());
        let result = schema.validate(&json!({"email": "test@example.com"}));
        assert!(result.is_ok());
    }

    #[test]
    fn test_optional_field_wrong_type() {
        let schema = object().optional_field("email", string());
        assert!(schema.validate(&json!({"email": 123})).is_err());
    }

    // Nested Objects
    #[test]
    fn test_nested_object() {
        let schema = object()
            .field("user", object().field("name", string()));

        assert!(schema.validate(&json!({
            "user": {"name": "John"}
        })).is_ok());

        assert!(schema.validate(&json!({
            "user": {"name": 123}
        })).is_err());
    }

    #[test]
    fn test_deeply_nested_object() {
        let schema = object()
            .field("level1", object()
                .field("level2", object()
                    .field("level3", string())));

        assert!(schema.validate(&json!({
            "level1": {
                "level2": {
                    "level3": "value"
                }
            }
        })).is_ok());
    }

    #[test]
    fn test_nested_object_error_path() {
        let schema = object()
            .field("user", object()
                .field("profile", object()
                    .field("name", string().min(5))));

        let result = schema.validate(&json!({
            "user": {
                "profile": {
                    "name": "hi"
                }
            }
        }));

        assert!(result.is_err());
        let err = result.unwrap_err();
        // Error path should be ["user", "profile", "name"]
        assert_eq!(err.issues[0].path, vec!["user", "profile", "name"]);
    }

    // Strict Mode
    #[test]
    fn test_strict_no_extra_keys() {
        let schema = object().field("name", string()).strict();
        assert!(schema.validate(&json!({"name": "John"})).is_ok());
    }

    #[test]
    fn test_strict_with_extra_key() {
        let schema = object().field("name", string()).strict();
        assert!(schema.validate(&json!({"name": "John", "extra": "value"})).is_err());
    }

    #[test]
    fn test_strict_multiple_extra_keys() {
        let schema = object().field("name", string()).strict();
        let result = schema.validate(&json!({
            "name": "John",
            "extra1": "a",
            "extra2": "b"
        }));
        assert!(result.is_err());
    }

    #[test]
    fn test_strict_empty_object() {
        let schema = object().strict();
        assert!(schema.validate(&json!({})).is_ok());
        assert!(schema.validate(&json!({"any": "key"})).is_err());
    }

    // Non-Strict Mode (extra keys allowed)
    #[test]
    fn test_non_strict_extra_keys_preserved() {
        let schema = object().field("name", string());
        let result = schema.validate(&json!({
            "name": "John",
            "extra": "preserved"
        }));
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value.get("extra").unwrap(), "preserved");
    }

    // Key Edge Cases
    #[test]
    fn test_empty_string_key() {
        let schema = object().field("", string());
        assert!(schema.validate(&json!({"": "value"})).is_ok());
    }

    #[test]
    fn test_key_with_special_chars() {
        let schema = object().field("user-id", string());
        assert!(schema.validate(&json!({"user-id": "123"})).is_ok());
    }

    #[test]
    fn test_unicode_key() {
        let schema = object().field("用户名", string());
        assert!(schema.validate(&json!({"用户名": "张三"})).is_ok());
    }

    #[test]
    fn test_key_with_spaces() {
        let schema = object().field("user name", string());
        assert!(schema.validate(&json!({"user name": "John"})).is_ok());
    }

    // Type Rejection
    #[test]
    fn test_rejects_array() {
        let schema = object().field("name", string());
        assert!(schema.validate(&json!([])).is_err());
    }

    #[test]
    fn test_rejects_string() {
        let schema = object().field("name", string());
        assert!(schema.validate(&json!("not an object")).is_err());
    }

    #[test]
    fn test_rejects_number() {
        let schema = object().field("name", string());
        assert!(schema.validate(&json!(123)).is_err());
    }

    #[test]
    fn test_rejects_null() {
        let schema = object().field("name", string());
        assert!(schema.validate(&json!(null)).is_err());
    }

    #[test]
    fn test_rejects_boolean() {
        let schema = object().field("name", string());
        assert!(schema.validate(&json!(true)).is_err());
    }

    // Multiple Fields with Errors
    #[test]
    fn test_multiple_field_errors() {
        let schema = object()
            .field("name", string().min(5))
            .field("age", number().min(18.0));

        let result = schema.validate(&json!({
            "name": "hi",
            "age": 10
        }));

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.issues.len(), 2);
    }

    // Object with Array Field
    #[test]
    fn test_object_with_array_field() {
        let schema = object()
            .field("tags", array(string()));

        assert!(schema.validate(&json!({
            "tags": ["a", "b", "c"]
        })).is_ok());

        assert!(schema.validate(&json!({
            "tags": ["a", 123, "c"]
        })).is_err());
    }

    #[test]
    fn test_array_of_objects_error_path() {
        let schema = object()
            .field("items", array(object().field("name", string().min(3))));

        let result = schema.validate(&json!({
            "items": [
                {"name": "hello"},
                {"name": "hi"}
            ]
        }));

        assert!(result.is_err());
        let err = result.unwrap_err();
        // Error should be at path ["items", "1", "name"]
        assert_eq!(err.issues[0].path, vec!["items", "1", "name"]);
    }

    // Empty Object
    #[test]
    fn test_empty_schema_accepts_empty_object() {
        let schema = object();
        assert!(schema.validate(&json!({})).is_ok());
    }

    #[test]
    fn test_empty_schema_accepts_any_object() {
        let schema = object();
        assert!(schema.validate(&json!({"any": "thing"})).is_ok());
    }

    // Single Field Object
    #[test]
    fn test_single_required_field() {
        let schema = object().field("id", number());
        assert!(schema.validate(&json!({"id": 1})).is_ok());
        assert!(schema.validate(&json!({})).is_err());
    }

    #[test]
    fn test_single_optional_field() {
        let schema = object().optional_field("id", number());
        assert!(schema.validate(&json!({"id": 1})).is_ok());
        assert!(schema.validate(&json!({})).is_ok());
    }
}
