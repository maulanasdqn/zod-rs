# 🦀 zod-rs

![image](https://github.com/user-attachments/assets/033617ef-3b7d-4c03-87e5-b082f98a26d5)


🦀 **A Rust implementation inspired by Zod for schema validation**

[![Crates.io](https://img.shields.io/crates/v/zod-rs.svg)](https://crates.io/crates/zod-rs)
[![Documentation](https://docs.rs/zod-rs/badge.svg)](https://docs.rs/zod-rs)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)

zod-rs is a TypeScript-first schema validation library with static type inference, inspired by [Zod](https://github.com/colinhacks/zod). It provides a simple and intuitive API for validating JSON data with comprehensive error reporting.

## ✨ Features

- 🔒 **Type-safe validation** - Full type safety with compile-time guarantees
- 🚀 **Zero dependencies** - Lightweight core with optional integrations
- 📝 **Rich error messages** - Detailed validation errors with path information
- 🎯 **Composable schemas** - Build complex validation rules from simple primitives
- 🔗 **Framework integration** - Built-in support for Axum and other web frameworks
- ⚡ **High performance** - Efficient validation with minimal overhead
- 🛠 **Developer friendly** - Intuitive API similar to TypeScript Zod
- 🔄 **Schema inference** - Automatically generate schemas from Rust structs
- 🏷️ **Attribute macros** - Rich validation constraints via `#[zod(...)]` attributes
- 🔧 **Validator replacement** - Drop-in replacement for the `validator` crate

## 📦 Installation

Add zod-rs to your `Cargo.toml`:

```toml
[dependencies]
zod-rs = "0.1.0"

# Optional: for web framework integration
zod-rs = { version = "0.1.0", features = ["axum"] }

# For schema derivation from structs (recommended)
zod-rs = "0.1.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## 🚀 Quick Start

```rust
use serde_json::json;
use zod_rs::prelude::*;

fn main() {
    // Define a schema
    let user_schema = object()
        .field("name", string().min(2).max(50))
        .field("email", string().email())
        .field("age", number().min(0.0).max(120.0).int());

    // Validate data
    let user_data = json!({
        "name": "Alice",
        "email": "alice@example.com",
        "age": 25
    });

    match user_schema.safe_parse(&user_data) {
        Ok(validated_data) => println!("✅ Valid: {:?}", validated_data),
        Err(errors) => println!("❌ Invalid: {}", errors),
    }
}
```

## 📚 API Reference

### Basic Types

#### String Validation

```rust
use zod_rs::prelude::*;
use serde_json::json;

// Basic string
let schema = string();
assert!(schema.safe_parse(&json!("hello")).is_ok());

// String with length constraints
let schema = string().min(3).max(10);
assert!(schema.safe_parse(&json!("hello")).is_ok());
assert!(schema.safe_parse(&json!("hi")).is_err());

// Exact length
let schema = string().length(5);
assert!(schema.safe_parse(&json!("hello")).is_ok());

// Pattern matching
let schema = string().regex(r"^[a-zA-Z]+$");
assert!(schema.safe_parse(&json!("hello")).is_ok());
assert!(schema.safe_parse(&json!("hello123")).is_err());

// Email validation
let schema = string().email();
assert!(schema.safe_parse(&json!("user@example.com")).is_ok());

// URL validation
let schema = string().url();
assert!(schema.safe_parse(&json!("https://example.com")).is_ok());
```

#### Number Validation

```rust
use zod_rs::prelude::*;
use serde_json::json;

// Basic number
let schema = number();
assert!(schema.safe_parse(&json!(42.5)).is_ok());

// Integer only
let schema = number().int();
assert!(schema.safe_parse(&json!(42)).is_ok());
assert!(schema.safe_parse(&json!(42.5)).is_err());

// Range constraints
let schema = number().min(0.0).max(100.0);
assert!(schema.safe_parse(&json!(50)).is_ok());
assert!(schema.safe_parse(&json!(-1)).is_err());

// Positive numbers
let schema = number().positive();
assert!(schema.safe_parse(&json!(1)).is_ok());
assert!(schema.safe_parse(&json!(0)).is_err());

// Non-negative numbers
let schema = number().nonnegative();
assert!(schema.safe_parse(&json!(0)).is_ok());
assert!(schema.safe_parse(&json!(-1)).is_err());

// Finite numbers (excludes NaN, Infinity)
let schema = number().finite();
assert!(schema.safe_parse(&json!(42.0)).is_ok());
```

#### Boolean Validation

```rust
use zod_rs::prelude::*;
use serde_json::json;

let schema = boolean();
assert!(schema.safe_parse(&json!(true)).is_ok());
assert!(schema.safe_parse(&json!(false)).is_ok());
assert!(schema.safe_parse(&json!("true")).is_err());
```

#### Literal Validation

```rust
use zod_rs::prelude::*;
use serde_json::json;

// String literal
let schema = literal("active".to_string());
assert!(schema.safe_parse(&json!("active")).is_ok());
assert!(schema.safe_parse(&json!("inactive")).is_err());

// Number literal
let schema = literal(42.0);
assert!(schema.safe_parse(&json!(42.0)).is_ok());
assert!(schema.safe_parse(&json!(43.0)).is_err());
```

### Complex Types

#### Array Validation

```rust
use zod_rs::prelude::*;
use serde_json::json;

// Array of strings
let schema = array(string());
assert!(schema.safe_parse(&json!(["a", "b", "c"])).is_ok());

// Array with length constraints
let schema = array(string()).min(1).max(5);
assert!(schema.safe_parse(&json!(["a"])).is_ok());
assert!(schema.safe_parse(&json!([])).is_err());

// Array with exact length
let schema = array(number()).length(3);
assert!(schema.safe_parse(&json!([1, 2, 3])).is_ok());

// Nested arrays
let schema = array(array(string()));
assert!(schema.safe_parse(&json!([["a", "b"], ["c", "d"]])).is_ok());
```

#### Object Validation

```rust
use zod_rs::prelude::*;
use serde_json::json;

// Simple object
let schema = object()
    .field("name", string())
    .field("age", number());

let data = json!({"name": "Alice", "age": 25});
assert!(schema.safe_parse(&data).is_ok());

// Object with optional fields
let schema = object()
    .field("name", string())
    .optional_field("bio", string());

let data = json!({"name": "Alice"});
assert!(schema.safe_parse(&data).is_ok());

// Strict mode (no additional properties)
let schema = object()
    .field("name", string())
    .strict();
```

#### Optional Values

```rust
use zod_rs::prelude::*;
use serde_json::json;

let schema = optional(string());
assert!(schema.safe_parse(&json!(null)).is_ok());
assert!(schema.safe_parse(&json!("hello")).is_ok());

// Method chaining
let schema = string().optional();
```

#### Union Types

```rust
use zod_rs::prelude::*;
use serde_json::json;

let schema = union()
    .variant(string())
    .variant(number());

assert!(schema.safe_parse(&json!("hello")).is_ok());
assert!(schema.safe_parse(&json!(42)).is_ok());
assert!(schema.safe_parse(&json!(true)).is_err());

// Literal unions (enums)
let schema = union()
    .variant(literal("small".to_string()))
    .variant(literal("medium".to_string()))
    .variant(literal("large".to_string()));
```

### Schema Methods

All schemas support these methods:

#### `parse(value)` - Parse with panic on error

```rust
let schema = string();
let result = schema.parse(&json!("hello")); // Panics on validation failure
```

#### `safe_parse(value)` - Parse with Result

```rust
let schema = string();
match schema.safe_parse(&json!("hello")) {
    Ok(value) => println!("Valid: {}", value),
    Err(errors) => println!("Invalid: {}", errors),
}
```

#### `validate(value)` - Alias for safe_parse

```rust
let schema = string();
let result = schema.validate(&json!("hello"));
```

## 🏗 Complex Examples

### Struct Validation

```rust
use serde::{Deserialize, Serialize};
use serde_json::json;
use zod_rs::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    username: String,
    email: String,
    age: f64,
    interests: Vec<String>,
}

fn user_schema() -> impl Schema<Value> {
    object()
        .field("username", string().min(3).max(20).regex(r"^[a-zA-Z0-9_]+$"))
        .field("email", string().email())
        .field("age", number().min(13.0).max(120.0).int())
        .field("interests", array(string()).min(1).max(10))
}

fn main() {
    let user_data = json!({
        "username": "alice_dev",
        "email": "alice@example.com",
        "age": 28,
        "interests": ["rust", "programming"]
    });

    // Validate and deserialize
    match user_schema().validate(&user_data) {
        Ok(_) => {
            let user: User = serde_json::from_value(user_data).unwrap();
            println!("Valid user: {:?}", user);
        }
        Err(errors) => {
            println!("Validation failed: {}", errors);
        }
    }
}
```

### Nested Objects

```rust
use zod_rs::prelude::*;
use serde_json::json;

fn address_schema() -> impl Schema<Value> {
    object()
        .field("street", string().min(1))
        .field("city", string().min(1))
        .field("country", string().length(2)) // ISO country code
        .field("zip", string().regex(r"^\d{5}(-\d{4})?$"))
}

fn user_schema() -> impl Schema<Value> {
    object()
        .field("name", string())
        .field("email", string().email())
        .field("address", address_schema())
        .optional_field("billing_address", address_schema())
}

let user_data = json!({
    "name": "John Doe",
    "email": "john@example.com",
    "address": {
        "street": "123 Main St",
        "city": "Boston",
        "country": "US",
        "zip": "02101"
    }
});

assert!(user_schema().safe_parse(&user_data).is_ok());
```

## 🌐 Web Framework Integration

### Axum Integration

Enable the `axum` feature in your `Cargo.toml`:

```toml
[dependencies]
zod-rs = { version = "0.1.0", features = ["axum"] }
axum = "0.7"
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

```rust
use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Json as ResponseJson},
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use zod_rs::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
    age: f64,
}

#[derive(Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    errors: Option<Vec<String>>,
}

fn user_schema() -> impl Schema<Value> {
    object()
        .field("name", string().min(2).max(50))
        .field("email", string().email())
        .field("age", number().min(13.0).max(120.0).int())
}

async fn create_user(Json(payload): Json<Value>) -> impl IntoResponse {
    match user_schema().validate(&payload) {
        Ok(_) => {
            let user: CreateUserRequest = serde_json::from_value(payload).unwrap();
            (
                StatusCode::CREATED,
                ResponseJson(ApiResponse {
                    success: true,
                    data: Some(user),
                    errors: None,
                }),
            )
        }
        Err(validation_result) => {
            let errors: Vec<String> = validation_result
                .issues
                .iter()
                .map(|issue| issue.to_string())
                .collect();

            (
                StatusCode::BAD_REQUEST,
                ResponseJson(ApiResponse::<CreateUserRequest> {
                    success: false,
                    data: None,
                    errors: Some(errors),
                }),
            )
        }
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/users", post(create_user));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

## ⚠️ Error Handling

zod-rs provides detailed error information with path tracking:

```rust
use zod_rs::prelude::*;
use serde_json::json;

let schema = object()
    .field("user", object()
        .field("name", string().min(2))
        .field("email", string().email())
    );

let invalid_data = json!({
    "user": {
        "name": "A",
        "email": "invalid-email"
    }
});

match schema.safe_parse(&invalid_data) {
    Err(errors) => {
        println!("{}", errors);
        // Output:
        // Validation failed with 2 error(s):
        //   - at user.name: String length 1 is invalid: minimum length is 2
        //   - at user.email: Invalid format: invalid email format
    }
    _ => {}
}
```

### Error Types

- `ValidationError::Required` - Missing required field
- `ValidationError::InvalidType` - Wrong data type
- `ValidationError::TooSmall` / `TooBig` - Number out of range
- `ValidationError::InvalidLength` - String/array length issues
- `ValidationError::InvalidFormat` - Format validation (email, URL)
- `ValidationError::PatternMismatch` - Regex pattern mismatch
- `ValidationError::Custom` - Custom validation errors

## 🔧 Advanced Usage

### Schema Inference from Structs

zod-rs provides a powerful derive macro that automatically generates validation schemas from Rust structs, making it an excellent replacement for the `validator` crate.

```rust
use serde::{Deserialize, Serialize};
use serde_json::json;
use zod_rs::prelude::*;

#[derive(Debug, Serialize, Deserialize, ZodSchema)]
struct User {
    #[zod(min_length(3), max_length(20), regex(r"^[a-zA-Z0-9_]+$"))]
    username: String,

    #[zod(email)]
    email: String,

    #[zod(min(13.0), max(120.0), int)]
    age: u32,

    #[zod(min_length(1), max_length(10))]
    interests: Vec<String>,

    bio: Option<String>,

    #[zod(nonnegative)]
    score: f64,

    is_active: bool,
}

let user_data = json!({
    "username": "alice_dev",
    "email": "alice@example.com",
    "age": 28,
    "interests": ["rust", "programming"],
    "score": 95.5,
    "is_active": true
});

match User::validate_and_parse(&user_data) {
    Ok(user) => println!("Valid user: {:?}", user),
    Err(e) => println!("Invalid: {}", e),
}

let schema = User::schema();
match schema.validate(&user_data) {
    Ok(_) => println!("Schema validation passed"),
    Err(e) => println!("Schema validation failed: {}", e),
}

let user_from_json = User::from_json(r#"{"username":"test","email":"test@example.com",...}"#)?;
```

#### Available Validation Attributes

The `#[zod(...)]` attribute supports the following constraints:

**String Validation:**

- `min_length(n)` - Minimum string length
- `max_length(n)` - Maximum string length
- `length(n)` - Exact string length
- `email` - Email format validation
- `url` - URL format validation
- `regex("pattern")` - Regular expression pattern matching

**Number Validation:**

- `min(n)` - Minimum value
- `max(n)` - Maximum value
- `int` - Integer only (no decimals)
- `positive` - Must be positive (> 0)
- `negative` - Must be negative (< 0)
- `nonnegative` - Must be non-negative (>= 0)
- `nonpositive` - Must be non-positive (<= 0)
- `finite` - Must be finite (excludes NaN, Infinity)

**Array Validation:**

- `min_length(n)` - Minimum array length
- `max_length(n)` - Maximum array length
- `length(n)` - Exact array length

#### Nested Structs

The derive macro automatically handles nested structs:

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

    address: Option<Address>,
}
```

#### Generated Methods

The `ZodSchema` derive macro generates the following methods:

- `schema()` - Returns the validation schema
- `validate_and_parse(value)` - Validates and deserializes JSON value
- `from_json(json_str)` - Validates and parses from JSON string
- `validate_json(json_str)` - Validates JSON string (returns Value)

### Custom Validation

```rust
use zod_rs::prelude::*;
use zod_rs_util::{ValidationError, ValidateResult};
use serde_json::Value;

struct CustomSchema {
    min_words: usize,
}

impl Schema<String> for CustomSchema {
    fn validate(&self, value: &Value) -> ValidateResult<String> {
        let string_val = value.as_str()
            .ok_or_else(|| ValidationError::invalid_type("string", "other"))?
            .to_string();

        let word_count = string_val.split_whitespace().count();
        if word_count < self.min_words {
            return Err(ValidationError::custom(
                format!("Must contain at least {} words", self.min_words)
            ).into());
        }

        Ok(string_val)
    }
}

let schema = CustomSchema { min_words: 3 };
assert!(schema.safe_parse(&json!("hello world rust")).is_ok());
assert!(schema.safe_parse(&json!("hello world")).is_err());
```

### Schema Composition

```rust
use zod_rs::prelude::*;

fn email_schema() -> impl Schema<String> {
    string().email()
}

fn password_schema() -> impl Schema<String> {
    string().min(8).regex(r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)")
}

fn login_schema() -> impl Schema<Value> {
    object()
        .field("email", email_schema())
        .field("password", password_schema())
}
```

## 🧪 Testing

Run the test suite:

```bash
cargo test
```

Run examples:

```bash
# Basic usage
cargo run --example basic_usage

# Struct validation
cargo run --example struct_validation

# Derive macro for schema inference
cargo run --example derive_schema

# Validator crate replacement
cargo run --example validator_replacement

# Axum integration
cargo run --example axum_usage --features axum
```

## 📦 Workspace Structure

This project uses a Cargo workspace with the following crates:

- **`zod-rs`** - Main validation library with schema types
- **`zod-rs-util`** - Utility functions and error handling

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### Development Setup

1. Clone the repository
2. Run tests: `cargo test`
3. Run examples: `cargo run --example basic_usage`
4. Format code: `cargo fmt`
5. Check with clippy: `cargo clippy`

## 📄 License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## 🙏 Acknowledgments

- Inspired by [Zod](https://github.com/colinhacks/zod) by Colin McDonnell
- Built with ❤️ for the Rust community

## 📚 Related Projects

- [Zod](https://github.com/colinhacks/zod) - TypeScript-first schema validation
- [Serde](https://github.com/serde-rs/serde) - Rust serialization framework
- [Validator](https://github.com/Keats/validator) - Rust struct validation

## 🎯 zod-rs vs Validator Crate

zod-rs provides significant advantages over the traditional `validator` crate:

| Feature                   | zod-rs                           | validator crate             |
| ------------------------- | -------------------------------- | --------------------------- |
| **Schema Definition**     | Derive macro with attributes     | Struct attributes only      |
| **Runtime Flexibility**   | Dynamic schema creation          | Compile-time only           |
| **Error Messages**        | Detailed with full path context  | Basic field-level errors    |
| **JSON Integration**      | Built-in JSON validation/parsing | Manual serde integration    |
| **Nested Validation**     | Automatic nested struct support  | Manual implementation       |
| **Schema Reuse**          | Composable and reusable schemas  | Struct-bound validation     |
| **Type Safety**           | Full type inference              | Limited type information    |
| **Performance**           | Optimized validation pipeline    | Direct field validation     |
| **Extensibility**         | Custom validators and schemas    | Custom validation functions |
| **Framework Integration** | Built-in web framework support   | Manual integration required |

### Migration from Validator Crate

```rust
// Before: using validator crate
use validator::{Validate, ValidationError};

#[derive(Validate)]
struct User {
    #[validate(length(min = 3, max = 20))]
    username: String,

    #[validate(email)]
    email: String,

    #[validate(range(min = 13, max = 120))]
    age: u32,
}

// After: using zod-rs
use zod_rs::prelude::*;

#[derive(ZodSchema)]
struct User {
    #[zod(min_length(3), max_length(20))]
    username: String,

    #[zod(email)]
    email: String,

    #[zod(min(13.0), max(120.0), int)]
    age: u32,
}

// Enhanced capabilities with zod-rs
let user_data = json!({
    "username": "alice",
    "email": "alice@example.com",
    "age": 25
});

// Validate and parse in one step
let user = User::validate_and_parse(&user_data)?;

// Or validate JSON string directly
let user = User::from_json(r#"{"username":"alice",...}"#)?;

// Reuse schema for different purposes
let schema = User::schema();
let is_valid = schema.validate(&user_data).is_ok();
```

---

Made with 🦀 and ❤️ by [Maulana Sodiqin](https://github.com/maulanasdqn)
