mod schema;

pub use schema::*;
pub use zod_rs_util::Locale;

#[doc(hidden)]
pub mod __private {
    pub use zod_rs_util::{ParseError, ValidationError, ValidationResult};
}

pub mod prelude {
    pub use crate::schema::{
        array, boolean, literal, null, number, object, optional, string, tuple, union,
        ArraySchema, BooleanSchema, LiteralSchema, NullSchema, NumberSchema, ObjectSchema,
        OptionalSchema, Schema, StringSchema, TupleSchema, UnionSchema,
    };
    pub use serde_json::Value;
    #[cfg(feature = "macros")]
    pub use zod_rs_macros::ZodSchema;
    pub use zod_rs_util::Locale;
}
