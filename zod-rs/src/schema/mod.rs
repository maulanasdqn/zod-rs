mod array;
mod boolean;
mod literal;
mod number;
mod object;
mod optional;
mod string;
mod union;

pub use array::*;
pub use boolean::*;
pub use literal::*;
pub use number::*;
pub use object::*;
pub use optional::*;
pub use string::*;
pub use union::*;

use serde_json::Value;
use zod_rs_util::ValidateResult;

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

pub fn value_type_name(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}
