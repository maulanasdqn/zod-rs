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
                Ok(serde_json::to_value(validated).unwrap())
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
                Ok(serde_json::to_value(validated).unwrap())
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
    use crate::schema::{number, string};
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
}
