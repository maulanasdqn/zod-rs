mod schema;

pub use schema::*;

pub mod prelude {
    pub use crate::schema::{
        array, boolean, literal, number, object, optional, string, union, ArraySchema,
        BooleanSchema, LiteralSchema, NumberSchema, ObjectSchema, OptionalSchema, Schema,
        StringSchema, UnionSchema,
    };
    pub use serde_json::Value;
    #[cfg(feature = "macros")]
    pub use zod_rs_macros::ZodSchema;
}
