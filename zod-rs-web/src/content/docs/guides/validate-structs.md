---
title: How to Validate Structs in Rust
description: Learn how to validate Rust structs with derive macros, enforce business rules like email formats and length constraints, and get typed error messages with full field paths.
head:
  - tag: script
    attrs:
      type: application/ld+json
    content: |
      {
        "@context": "https://schema.org",
        "@type": "HowTo",
        "name": "How to Validate Structs in Rust",
        "description": "Add field-level validation to Rust structs with derive macros, enforce business rules, and get structured errors with full paths.",
        "step": [
          {
            "@type": "HowToStep",
            "name": "Add zod-rs to your project",
            "text": "Add zod-rs with the derive feature to your Cargo.toml: zod-rs = { version = \"1.1\", features = [\"derive\"] }"
          },
          {
            "@type": "HowToStep",
            "name": "Annotate your struct with validation rules",
            "text": "Derive ZodSchema on your struct and add #[zod(...)] attributes to fields: #[zod(email)], #[zod(min_length(3))], #[zod(min(0.0), max(120.0))]."
          },
          {
            "@type": "HowToStep",
            "name": "Validate input data",
            "text": "Call validate_and_parse() with a serde_json::Value or from_json() with a JSON string to validate and deserialize in one step."
          },
          {
            "@type": "HowToStep",
            "name": "Handle validation errors",
            "text": "On failure, iterate over errors.issues to get each error with the full path to the failing field, such as address.city or items[0].name."
          }
        ],
        "totalTime": "PT10M"
      }
---

To validate a struct in Rust, derive `ZodSchema` on the struct and annotate fields with `#[zod(...)]` attributes that express your business rules — email format, string length, numeric range, and more. Call `validate_and_parse()` with JSON input to validate and deserialize in one step, getting either a typed struct or structured errors with the full path to every failing field.

## The problem: types aren't validation

Rust's type system guarantees that a `String` is a string and a `u32` is an unsigned integer, but it says nothing about *meaning*. An email field happily holds `"nope"`, a username can be empty, and an age of 5000 passes the compiler without complaint:

```rust
#[derive(serde::Deserialize)]
struct User {
    username: String,  // empty string? fine.
    email: String,     // "not-an-email"? fine.
    age: u32,          // 5000? fine.
}
```

Deserializing with `serde_json::from_str` gives you a typed struct, but the data inside can still be nonsense. You end up scattering `if` checks throughout your handlers, and when something fails the caller gets a generic "bad request" with no indication of which field caused it.

You need **struct validation** — rules attached to fields, checked automatically, with clear errors.

## Approach 1: Manual validation

The simplest approach is a `validate` method or a `TryFrom` impl:

```rust
use std::convert::TryFrom;

struct User {
    username: String,
    email: String,
    age: u32,
}

impl TryFrom<RawInput> for User {
    type Error = Vec<String>;

    fn try_from(input: RawInput) -> Result<Self, Self::Error> {
        let mut errors = Vec::new();

        if input.username.len() < 3 || input.username.len() > 20 {
            errors.push("username must be 3-20 characters".into());
        }
        if !input.email.contains('@') {
            errors.push("email is invalid".into());
        }
        if input.age > 120 {
            errors.push("age must be 120 or less".into());
        }

        if errors.is_empty() {
            Ok(User { username: input.username, email: input.email, age: input.age })
        } else {
            Err(errors)
        }
    }
}
```

This works for small structs, but it doesn't scale:

- Every field needs hand-written checks and hand-written messages.
- Nested structs require manual plumbing to propagate error paths.
- Rules are spread across procedural code instead of declared next to the fields they apply to.
- There is no standard error shape, so every struct invents its own.

## Approach 2: Derive-macro validation with zod-rs (recommended)

zod-rs lets you declare validation rules as attributes on fields and generates the validation logic at compile time. Add it to your project:

```toml
[dependencies]
zod-rs = { version = "1.1", features = ["derive"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

Then annotate your struct:

```rust
use serde::{Deserialize, Serialize};
use zod_rs::prelude::*;

#[derive(Debug, Serialize, Deserialize, ZodSchema)]
struct User {
    #[zod(min_length(3), max_length(20), regex(r"^[a-zA-Z0-9_]+$"))]
    username: String,

    #[zod(email)]
    email: String,

    #[zod(min(13.0), max(120.0), int)]
    age: u32,

    bio: Option<String>,     // optional — no validation rules needed

    #[zod(nonnegative)]
    score: f64,
}
```

Every `#[zod(...)]` attribute maps to a validation check: `min_length`, `max_length`, `email`, `url`, `regex`, `min`, `max`, `int`, `positive`, `nonnegative`, and more. See the full list in the [attributes reference](/derive-macros/attributes/).

### Validate and parse in one step

The derive macro generates several methods on your struct. The most useful is `validate_and_parse`, which takes a `serde_json::Value`, validates it against the schema, and returns your typed struct:

```rust
use serde_json::json;

let data = json!({
    "username": "alice_dev",
    "email": "alice@example.com",
    "age": 28,
    "score": 95.5
});

match User::validate_and_parse(&data) {
    Ok(user) => println!("Valid: {:?}", user),
    Err(errors) => {
        for issue in &errors.issues {
            println!("  - {}", issue);
        }
    }
}
```

### Parse directly from a JSON string

When the input is a raw JSON string — from an HTTP body, a file, or a message queue — skip the intermediate `Value`:

```rust
let user = User::from_json(r#"{
    "username": "alice_dev",
    "email": "alice@example.com",
    "age": 28,
    "score": 95.5
}"#)?;
```

`from_json` parses the JSON and validates it in one call. If either step fails you get structured errors, not a serde deserialization panic.

### Get the schema as a value

You can also extract the schema and use it independently — pass it around, store it, compose it with other schemas:

```rust
let schema = User::schema();

// Validate without deserializing
let result = schema.validate(&data);

// Reuse the same schema across multiple inputs
for payload in incoming_messages {
    if schema.validate(&payload).is_ok() {
        process(payload);
    }
}
```

This is what makes zod-rs different from attribute-only validators: schemas are runtime values, not just compile-time annotations. See [Schema Composition](/advanced/schema-composition/) for how to combine them.

## Nested struct validation

When a field's type also derives `ZodSchema`, nested validation happens automatically — no extra attributes needed:

```rust
#[derive(Debug, Serialize, Deserialize, ZodSchema)]
struct Address {
    #[zod(min_length(5), max_length(200))]
    street: String,

    #[zod(min_length(2), max_length(50))]
    city: String,

    #[zod(length(2))]
    country_code: String,
}

#[derive(Debug, Serialize, Deserialize, ZodSchema)]
struct UserProfile {
    #[zod(min_length(2), max_length(50))]
    name: String,

    #[zod(email)]
    email: String,

    address: Address,                      // required nested struct
    billing_address: Option<Address>,      // optional nested struct

    #[zod(min_length(1))]
    tags: Vec<String>,
}
```

Vectors of nested structs — like `Vec<Address>` — also validate every element automatically.

### Full-path error messages

When validation fails inside a nested struct, the error includes the complete path to the failing field. This is critical for API responses where the caller needs to know exactly what to fix:

```rust
let data = json!({
    "name": "A",
    "email": "bad",
    "address": {
        "street": "1",
        "city": "B",
        "country_code": "USA"
    },
    "tags": []
});

match UserProfile::validate_and_parse(&data) {
    Err(errors) => {
        for issue in &errors.issues {
            println!("  - {}", issue);
        }
    }
    _ => {}
}
```

Output:

```text
- name: Too small: expected string to have >= 2 characters
- email: Invalid email address
- address.street: Too small: expected string to have >= 5 characters
- address.city: Too small: expected string to have >= 2 characters
- address.country_code: Expected exactly 2 characters
- tags: Too small: expected array to have >= 1 items
```

Every error carries the dotted path (`address.street`, `address.city`), and array elements use bracket notation (`items[0].name`). See [Error Handling](/advanced/error-handling/) for the complete error model.

## Custom validation logic

For rules that go beyond built-in attributes — cross-field checks, database lookups, domain-specific logic — implement the `Schema` trait:

```rust
use zod_rs::prelude::*;
use zod_rs_util::{ValidationError, ValidateResult};
use serde_json::Value;

struct PasswordStrength {
    min_length: usize,
}

impl Schema<String> for PasswordStrength {
    fn validate(&self, value: &Value) -> ValidateResult<String> {
        let s = value.as_str()
            .ok_or_else(|| ValidationError::invalid_type("string", "other"))?
            .to_string();

        if s.len() < self.min_length {
            return Err(ValidationError::custom(
                format!("Password must be at least {} characters", self.min_length)
            ).into());
        }

        let has_digit = s.chars().any(|c| c.is_ascii_digit());
        let has_upper = s.chars().any(|c| c.is_uppercase());

        if !has_digit || !has_upper {
            return Err(ValidationError::custom(
                "Password must contain at least one digit and one uppercase letter"
            ).into());
        }

        Ok(s)
    }
}
```

You can use custom schemas alongside built-in ones via [schema composition](/advanced/schema-composition/).

## Comparison: manual vs zod-rs vs validator

| | Manual (`TryFrom`) | zod-rs | validator crate |
|---|---------------------|--------|-----------------|
| **Declaration style** | Procedural code | `#[zod(...)]` attributes | `#[validate(...)]` attributes |
| **Validates** | Whatever you code | Raw JSON values | Already-deserialized structs |
| **Parse + validate in one step** | You build it | Built in | No — deserialize first |
| **Nested error paths** | You build it | Automatic (`a.b.c`) | Field-level only |
| **Runtime schema building** | N/A | Yes — schemas are values | No |
| **TypeScript codegen** | No | Yes — `#[derive(ZodTs)]` | No |
| **Built-in i18n** | No | Yes | No |
| **Effort for 1 struct** | Low | Low | Low |
| **Effort for 20 nested structs** | Very high | Low | Medium |

**Use manual validation** when you have one or two simple structs and want zero dependencies.

**Use zod-rs** when your data arrives as JSON, when you have nested structs, when you need runtime schema composition, or when your frontend and backend share the same validation rules. See [Choosing a Rust Validation Library](/comparison/choosing/) for a deeper comparison including the garde crate.

**Use the validator crate** when you only validate structs that are already deserialized and don't need runtime schema flexibility.

## What to read next

- [Validate JSON in Rust](/guides/validate-json/) — end-to-end guide from raw JSON input to typed structs
- [ZodSchema Derive Macro](/derive-macros/zod-schema/) — all generated methods and type mappings
- [Attributes Reference](/derive-macros/attributes/) — every available `#[zod(...)]` attribute
- [Nested Structs](/derive-macros/nested-structs/) — automatic nested validation in detail
- [Error Handling](/advanced/error-handling/) — the full error model, error types, and custom validators
- [Choosing a Rust Validation Library](/comparison/choosing/) — zod-rs vs validator vs garde
