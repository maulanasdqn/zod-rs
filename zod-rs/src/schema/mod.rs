mod array;
mod boolean;
mod literal;
mod null;
mod number;
mod object;
mod optional;
mod string;
mod tuple;
mod union;

pub use array::*;
pub use boolean::*;
pub use literal::*;
pub use null::*;
pub use number::*;
pub use object::*;
pub use optional::*;
pub use string::*;
pub use tuple::*;
pub use union::*;

use serde_json::Value;
use std::fmt::Debug;
use zod_rs_util::ValidateResult;

pub trait Schema<T>: Debug
where
    T: std::fmt::Debug,
{
    /// Validates the value against this schema and returns the validated result.
    fn validate(&self, value: &Value) -> ValidateResult<T>;

    /// Validates and returns the result, panicking on validation failure.
    ///
    /// # Panics
    /// Panics if validation fails. Use `safe_parse()` or `validate()` for non-panicking alternatives.
    ///
    /// # Example
    /// ```should_panic
    /// use zod_rs::prelude::*;
    /// use serde_json::json;
    ///
    /// let schema = string().min(5);
    /// let result = schema.parse(&json!("hi")); // panics: string too short
    /// ```
    fn parse(&self, value: &Value) -> T {
        match self.validate(value) {
            Ok(result) => result,
            Err(errors) => panic!("Validation failed: {errors}"),
        }
    }

    /// Validates the value and returns a Result. This is the recommended method for handling
    /// validation in production code.
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
