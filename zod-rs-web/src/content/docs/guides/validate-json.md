---
title: How to Validate JSON in Rust
description: A practical guide to JSON validation in Rust - parse untrusted input, enforce rules like email and length constraints, and get typed structs with clear errors.
---

Every Rust service that accepts JSON — an API endpoint, a webhook, a config file — needs to answer two questions: is this JSON shaped correctly, and are the values actually valid? This guide walks through doing both in one step with zod-rs.

## The problem with serde alone

`serde_json` checks structure, but not meaning:

```rust
#[derive(serde::Deserialize)]
struct SignupRequest {
    username: String,
    email: String,
    age: u32,
}

let req: SignupRequest = serde_json::from_str(input)?;
```

This happily accepts an empty username, `"email": "not-an-email"`, and `"age": 5000`. Type-correct, value-wrong. You end up writing ad-hoc `if` checks after deserializing — scattered, untested, and with error messages that don't tell the caller which field failed.

## Step 1: Define a schema

A zod-rs schema states the rules once, as a value:

```rust
use zod_rs::prelude::*;

let schema = object()
    .field("username", string().min(3).max(20))
    .field("email", string().email())
    .field("age", number().min(13.0).max(120.0).int());
```

## Step 2: Validate the JSON

```rust
use serde_json::json;

let data = json!({
    "username": "alice",
    "email": "alice@example.com",
    "age": 25
});

match schema.safe_parse(&data) {
    Ok(value) => println!("Valid: {:?}", value),
    Err(errors) => println!("Invalid: {}", errors),
}
```

Invalid input produces errors with the full path to each failing field:

```text
- username: Too small: expected string to have >= 3 characters
- email: Invalid email address
```

Paths work at any depth — a bad value nested three objects deep reports as `user.profile.email`, and each issue is individually accessible via `errors.issues`. See [Error Handling](/advanced/error-handling/) for the full error model.

## Step 3: Get a typed struct, not just valid JSON

For most applications you want the validated data *as your struct*. Derive the schema from the type instead of writing it by hand:

```rust
use zod_rs::prelude::*;

#[derive(ZodSchema, serde::Deserialize, Debug)]
struct SignupRequest {
    #[zod(min_length(3), max_length(20))]
    username: String,

    #[zod(email)]
    email: String,

    #[zod(min(13.0), max(120.0), int)]
    age: u32,
}

// Validate and deserialize in one call
let req = SignupRequest::validate_and_parse(&data)?;

// Or straight from a JSON string
let req = SignupRequest::from_json(r#"{"username":"alice","email":"alice@example.com","age":25}"#)?;
```

Nested structs validate automatically when their types also derive `ZodSchema` — see [Nested Structs](/derive-macros/nested-structs/) and the [attributes reference](/derive-macros/attributes/) for every available rule.

## Common validation recipes

```rust
// Optional fields
object().field("nickname", string().min(1).optional())

// Arrays with element rules
object().field("tags", array(string().min(1)).max(10))

// One of several allowed values
union()
    .variant(literal("admin".to_string()))
    .variant(literal("member".to_string()))

// Reusable schema functions
fn email_field() -> impl Schema<String> {
    string().email().max(254)
}
```

See [Schema Composition](/advanced/schema-composition/) for structuring larger rule sets.

## Validating HTTP request bodies

If the JSON arrives over HTTP, you usually want validation in the extractor layer rather than in every handler. The [Axum integration](/integrations/axum/) does exactly that with the `axum` feature flag — invalid bodies are rejected with structured errors before your handler runs.
