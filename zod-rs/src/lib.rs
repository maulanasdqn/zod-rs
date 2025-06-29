use serde_json::Value;
use std::collections::HashMap;
use zod_rs_util::{ValidateResult, ValidationError, ValidationResult};

pub mod prelude {
    pub use crate::{
        array, boolean, literal, number, object, optional, string, union, ArraySchema,
        BooleanSchema, LiteralSchema, NumberSchema, ObjectSchema, OptionalSchema, Schema,
        StringSchema, UnionSchema,
    };
    pub use serde_json::Value;
    #[cfg(feature = "macros")]
    pub use zod_rs_macros::ZodSchema;
}

pub trait Schema<T> {
    fn validate(&self, value: &Value) -> ValidateResult<T>;

    fn parse(&self, value: &Value) -> T {
        match self.validate(value) {
            Ok(result) => result,
            Err(errors) => panic!("Validation failed: {}", errors),
        }
    }

    fn safe_parse(&self, value: &Value) -> ValidateResult<T> {
        self.validate(value)
    }

    fn optional(self) -> OptionalSchema<Self, T>
    where
        Self: Sized,
    {
        OptionalSchema::new(self)
    }

    fn array(self) -> ArraySchema<Self, T>
    where
        Self: Sized,
    {
        ArraySchema::new(self)
    }
}

#[derive(Debug)]
pub struct StringSchema {
    min_length: Option<usize>,
    max_length: Option<usize>,
    pattern: Option<regex::Regex>,
    email: bool,
    url: bool,
}

impl StringSchema {
    pub fn new() -> Self {
        Self {
            min_length: None,
            max_length: None,
            pattern: None,
            email: false,
            url: false,
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

    pub fn regex(mut self, pattern: &str) -> Self {
        self.pattern = Some(regex::Regex::new(pattern).expect("Invalid regex pattern"));
        self
    }

    pub fn email(mut self) -> Self {
        self.email = true;
        self
    }

    pub fn url(mut self) -> Self {
        self.url = true;
        self
    }
}

impl Schema<String> for StringSchema {
    fn validate(&self, value: &Value) -> ValidateResult<String> {
        let string_val = match value.as_str() {
            Some(s) => s.to_string(),
            None => {
                return Err(ValidationError::invalid_type("string", value_type_name(value)).into());
            }
        };

        if let Some(min) = self.min_length {
            if string_val.len() < min {
                return Err(ValidationError::invalid_length(
                    string_val.len(),
                    format!("minimum length is {}", min),
                )
                .into());
            }
        }

        if let Some(max) = self.max_length {
            if string_val.len() > max {
                return Err(ValidationError::invalid_length(
                    string_val.len(),
                    format!("maximum length is {}", max),
                )
                .into());
            }
        }

        if let Some(pattern) = &self.pattern {
            if !pattern.is_match(&string_val) {
                return Err(ValidationError::pattern_mismatch(pattern.as_str()).into());
            }
        }

        if self.email && !is_valid_email(&string_val) {
            return Err(ValidationError::invalid_format("invalid email format").into());
        }

        if self.url && !is_valid_url(&string_val) {
            return Err(ValidationError::invalid_format("invalid URL format").into());
        }

        Ok(string_val)
    }
}

#[derive(Debug)]
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

impl Schema<f64> for NumberSchema {
    fn validate(&self, value: &Value) -> ValidateResult<f64> {
        let num = match value.as_f64() {
            Some(n) => n,
            None => {
                return Err(ValidationError::invalid_type("number", value_type_name(value)).into());
            }
        };

        if self.integer && num.fract() != 0.0 {
            return Err(ValidationError::invalid_type("integer", "float").into());
        }

        if self.finite && !num.is_finite() {
            return Err(ValidationError::custom("number must be finite").into());
        }

        if let Some(min) = self.min {
            if num < min {
                return Err(ValidationError::too_small(num.to_string(), min.to_string()).into());
            }
        }

        if let Some(max) = self.max {
            if num > max {
                return Err(ValidationError::too_big(num.to_string(), max.to_string()).into());
            }
        }

        if self.positive && num <= 0.0 {
            return Err(ValidationError::custom("number must be positive").into());
        }

        if self.negative && num >= 0.0 {
            return Err(ValidationError::custom("number must be negative").into());
        }

        if self.nonnegative && num < 0.0 {
            return Err(ValidationError::custom("number must be non-negative").into());
        }

        if self.nonpositive && num > 0.0 {
            return Err(ValidationError::custom("number must be non-positive").into());
        }

        Ok(num)
    }
}

#[derive(Debug, Clone)]
pub struct BooleanSchema;

impl BooleanSchema {
    pub fn new() -> Self {
        Self
    }
}

impl Schema<bool> for BooleanSchema {
    fn validate(&self, value: &Value) -> ValidateResult<bool> {
        match value.as_bool() {
            Some(b) => Ok(b),
            None => Err(ValidationError::invalid_type("boolean", value_type_name(value)).into()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LiteralSchema<T: Clone + PartialEq + std::fmt::Debug> {
    expected: T,
}

impl<T: Clone + PartialEq + std::fmt::Debug> LiteralSchema<T> {
    pub fn new(expected: T) -> Self {
        Self { expected }
    }
}

impl Schema<String> for LiteralSchema<String> {
    fn validate(&self, value: &Value) -> ValidateResult<String> {
        match value.as_str() {
            Some(s) if s == self.expected => Ok(s.to_string()),
            Some(s) => Err(ValidationError::custom(format!(
                "Expected '{}', got '{}'",
                self.expected, s
            ))
            .into()),
            None => Err(ValidationError::invalid_type("string", value_type_name(value)).into()),
        }
    }
}

impl Schema<f64> for LiteralSchema<f64> {
    fn validate(&self, value: &Value) -> ValidateResult<f64> {
        match value.as_f64() {
            Some(n) if (n - self.expected).abs() < f64::EPSILON => Ok(n),
            Some(n) => Err(ValidationError::custom(format!(
                "Expected {}, got {}",
                self.expected, n
            ))
            .into()),
            None => Err(ValidationError::invalid_type("number", value_type_name(value)).into()),
        }
    }
}

#[derive(Debug)]
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
{
    fn validate(&self, value: &Value) -> ValidateResult<Vec<T>> {
        let array = match value.as_array() {
            Some(arr) => arr,
            None => {
                return Err(ValidationError::invalid_type("array", value_type_name(value)).into());
            }
        };

        if let Some(min) = self.min_length {
            if array.len() < min {
                return Err(ValidationError::invalid_length(
                    array.len(),
                    format!("minimum length is {}", min),
                )
                .into());
            }
        }

        if let Some(max) = self.max_length {
            if array.len() > max {
                return Err(ValidationError::invalid_length(
                    array.len(),
                    format!("maximum length is {}", max),
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

pub struct ObjectSchema {
    fields: HashMap<String, Box<dyn ObjectFieldValidator>>,
    strict: bool,
}

trait ObjectFieldValidator: Send + Sync {
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
    S: Schema<T> + Send + Sync,
    T: serde::Serialize + Send + Sync,
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
    S: Schema<T> + Send + Sync,
    T: serde::Serialize + Send + Sync,
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
        T: serde::Serialize + Send + Sync + 'static,
    {
        self.fields.insert(
            name.to_string(),
            Box::new(RequiredFieldValidator::new(schema)),
        );
        self
    }

    pub fn optional_field<S, T>(mut self, name: &str, schema: S) -> Self
    where
        S: Schema<T> + Send + Sync + 'static,
        T: serde::Serialize + Send + Sync + 'static,
    {
        self.fields.insert(
            name.to_string(),
            Box::new(OptionalFieldValidator::new(schema)),
        );
        self
    }

    pub fn strict(mut self) -> Self {
        self.strict = true;
        self
    }
}

impl Schema<Value> for ObjectSchema {
    fn validate(&self, value: &Value) -> ValidateResult<Value> {
        let obj = match value.as_object() {
            Some(o) => o,
            None => {
                return Err(ValidationError::invalid_type("object", value_type_name(value)).into());
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
            for key in obj.keys() {
                if !self.fields.contains_key(key) {
                    validation_result.add_error_at_path(
                        vec![key.clone()],
                        ValidationError::custom(format!("Unknown field '{}'", key)),
                    );
                }
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

#[derive(Debug)]
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
{
    fn validate(&self, value: &Value) -> ValidateResult<Option<T>> {
        if value.is_null() {
            Ok(None)
        } else {
            self.inner.validate(value).map(Some)
        }
    }
}

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

pub fn string() -> StringSchema {
    StringSchema::new()
}

pub fn number() -> NumberSchema {
    NumberSchema::new()
}

pub fn boolean() -> BooleanSchema {
    BooleanSchema::new()
}

pub fn literal<T: Clone + PartialEq + std::fmt::Debug>(value: T) -> LiteralSchema<T> {
    LiteralSchema::new(value)
}

pub fn array<S, T>(element_schema: S) -> ArraySchema<S, T> {
    ArraySchema::new(element_schema)
}

pub fn object() -> ObjectSchema {
    ObjectSchema::new()
}

pub fn optional<S, T>(schema: S) -> OptionalSchema<S, T> {
    OptionalSchema::new(schema)
}

pub fn union<T>() -> UnionSchema<T> {
    UnionSchema::new()
}

fn value_type_name(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

fn is_valid_email(email: &str) -> bool {
    let email_regex = regex::Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();
    email_regex.is_match(email)
}

fn is_valid_url(url: &str) -> bool {
    url.starts_with("http://") || url.starts_with("https://")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_string_validation() {
        let schema = string().min(3).max(10);

        assert!(schema.validate(&json!("hello")).is_ok());
        assert!(schema.validate(&json!("hi")).is_err());
        assert!(schema.validate(&json!("this is too long")).is_err());
        assert!(schema.validate(&json!(123)).is_err());
    }

    #[test]
    fn test_number_validation() {
        let schema = number().min(0.0).max(100.0);

        assert!(schema.validate(&json!(50.5)).is_ok());
        assert!(schema.validate(&json!(-1.0)).is_err());
        assert!(schema.validate(&json!(101.0)).is_err());
        assert!(schema.validate(&json!("not a number")).is_err());
    }

    #[test]
    fn test_boolean_validation() {
        let schema = boolean();

        assert!(schema.validate(&json!(true)).is_ok());
        assert!(schema.validate(&json!(false)).is_ok());
        assert!(schema.validate(&json!("true")).is_err());
    }

    #[test]
    fn test_array_validation() {
        let schema = array(string()).min(1).max(3);

        assert!(schema.validate(&json!(["hello", "world"])).is_ok());
        assert!(schema.validate(&json!([])).is_err());
        assert!(schema.validate(&json!(["a", "b", "c", "d"])).is_err());
        assert!(schema.validate(&json!([1, 2, 3])).is_err());
    }

    #[test]
    fn test_object_validation() {
        let schema = object()
            .field("name", string().min(1))
            .field("age", number().min(0.0))
            .optional_field("email", string().email());

        assert!(schema
            .validate(&json!({
                "name": "John",
                "age": 25,
                "email": "john@example.com"
            }))
            .is_ok());

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

    #[test]
    fn test_optional_validation() {
        let schema = optional(string());

        assert!(schema.validate(&json!(null)).is_ok());
        assert!(schema.validate(&json!("hello")).is_ok());
        assert!(schema.validate(&json!(123)).is_err());
    }

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
