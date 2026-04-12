//! Regression test: the `ZodSchema` derive macro must expand into code that
//! resolves purely through the `zod-rs` facade — downstream users do not
//! have `zod-rs-util` as a direct dependency, so emitting `zod_rs_util::...`
//! paths (as pre-1.0.1 versions did) would fail to compile in their crates.
//!
//! Integration tests are compiled as a separate crate that only sees
//! `zod-rs`'s public API, which mirrors the downstream user's dep graph.

use serde::{Deserialize, Serialize};
use serde_json::json;
use zod_rs::prelude::*;

#[derive(Debug, Serialize, Deserialize, ZodSchema)]
struct User {
    #[zod(min_length(2), max_length(50))]
    username: String,

    #[zod(email)]
    email: String,

    #[zod(min(18.0), max(120.0), int)]
    age: u32,
}

#[test]
fn downstream_user_can_derive_and_validate() {
    let data = json!({
        "username": "alice",
        "email": "alice@example.com",
        "age": 30
    });

    let user = User::validate_and_parse(&data).expect("valid payload");
    assert_eq!(user.username, "alice");
    assert_eq!(user.age, 30);
}

#[test]
fn downstream_user_surfaces_validation_errors() {
    let data = json!({
        "username": "a",
        "email": "not-an-email",
        "age": 200
    });

    assert!(User::validate_and_parse(&data).is_err());
}

#[test]
fn downstream_user_from_json_string() {
    let json_str = r#"{"username":"alice","email":"alice@example.com","age":30}"#;
    let user = User::from_json(json_str).expect("valid json");
    assert_eq!(user.email, "alice@example.com");
}
