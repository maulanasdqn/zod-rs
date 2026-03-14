---
title: Attributes Reference
description: Complete reference for #[zod(...)] validation attributes
---

The `#[zod(...)]` attribute configures validation constraints on struct fields.

## String attributes

| Attribute | Description | Example |
|-----------|-------------|---------|
| `min_length(n)` | Minimum string length | `#[zod(min_length(3))]` |
| `max_length(n)` | Maximum string length | `#[zod(max_length(50))]` |
| `length(n)` | Exact string length | `#[zod(length(2))]` |
| `email` | Email format | `#[zod(email)]` |
| `url` | URL format | `#[zod(url)]` |
| `regex("pattern")` | Regex pattern | `#[zod(regex(r"^[a-z]+$"))]` |
| `starts_with("value")` | Starts with prefix | `#[zod(starts_with("hello"))]` |
| `ends_with("value")` | Ends with suffix | `#[zod(ends_with("@domain.com"))]` |
| `includes("value")` | Contains substring | `#[zod(includes("test"))]` |

## Number attributes

| Attribute | Description | Example |
|-----------|-------------|---------|
| `min(n)` | Minimum value | `#[zod(min(0.0))]` |
| `max(n)` | Maximum value | `#[zod(max(100.0))]` |
| `int` | Integer only | `#[zod(int)]` |
| `positive` | Must be > 0 | `#[zod(positive)]` |
| `negative` | Must be < 0 | `#[zod(negative)]` |
| `nonnegative` | Must be >= 0 | `#[zod(nonnegative)]` |
| `nonpositive` | Must be <= 0 | `#[zod(nonpositive)]` |
| `finite` | Must be finite | `#[zod(finite)]` |

## Array attributes

| Attribute | Description | Example |
|-----------|-------------|---------|
| `min_length(n)` | Minimum array length | `#[zod(min_length(1))]` |
| `max_length(n)` | Maximum array length | `#[zod(max_length(10))]` |
| `length(n)` | Exact array length | `#[zod(length(3))]` |

## Combining attributes

Multiple attributes can be combined in a single `#[zod(...)]`:

```rust
#[derive(ZodSchema)]
struct Registration {
    #[zod(min_length(3), max_length(20), regex(r"^[a-zA-Z0-9_]+$"))]
    username: String,

    #[zod(min(13.0), max(120.0), int)]
    age: u32,

    #[zod(min_length(1), max_length(5))]
    tags: Vec<String>,
}
```

## Fields without attributes

Fields without `#[zod(...)]` attributes use the default schema for their type:

```rust
#[derive(ZodSchema)]
struct Simple {
    name: String,      // string() — no extra validation
    count: u32,        // number().int()
    active: bool,      // boolean()
    items: Vec<String>, // array(string())
    bio: Option<String>, // optional(string())
}
```
