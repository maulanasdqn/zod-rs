---
title: JSON Format
description: JSON representation of Rust enum variants
---

zod-rs uses serde's default externally-tagged format for enums. Here's how each variant type maps to JSON.

## Format table

| Variant Type | Rust | JSON |
|-------------|------|------|
| Unit | `Status::Active` | `"Active"` |
| Tuple (single) | `Message::Text("hi")` | `{"Text": "hi"}` |
| Tuple (multiple) | `Message::Coords(1, 2)` | `{"Coords": [1, 2]}` |
| Struct | `Event::Click { x: 1, y: 2 }` | `{"Click": {"x": 1, "y": 2}}` |

## Unit variants

Unit variants serialize as plain strings:

```rust
#[derive(Serialize, Deserialize, ZodSchema)]
enum Status { Active, Inactive }

// JSON: "Active"
```

The legacy `{"Active": null}` form is also accepted during validation, matching serde's deserialization behavior.

## Tuple variants (single)

Single-value tuple variants unwrap the value:

```rust
#[derive(Serialize, Deserialize, ZodSchema)]
enum Msg { Text(String) }

// JSON: {"Text": "hello"}
```

## Tuple variants (multiple)

Multi-value tuple variants use an array:

```rust
#[derive(Serialize, Deserialize, ZodSchema)]
enum Msg { Coords(i32, i32) }

// JSON: {"Coords": [10, 20]}
```

## Struct variants

Struct variants use a nested object:

```rust
#[derive(Serialize, Deserialize, ZodSchema)]
enum Event { Click { x: i32, y: i32 } }

// JSON: {"Click": {"x": 1, "y": 2}}
```

## Validation

Each variant is validated as a union. Unit variants match their string name; variants with data must be a single-key object where the key matches the variant name and the value matches that variant's expected format.

## Serde renames

`#[serde(rename = "...")]` on variants and `#[serde(rename_all = "...")]` on the enum are honored, so the validated names always match what serde accepts:

```rust
#[derive(Serialize, Deserialize, ZodSchema)]
#[serde(rename_all = "snake_case")]
enum Example { SomeValue, AnotherValue }

// JSON: "some_value", "another_value"

#[derive(Serialize, Deserialize, ZodSchema)]
enum AccountVerificationType {
    #[serde(rename = "register")]
    Register,
    #[serde(rename = "login")]
    Login,
}

// JSON: "register", "login"
```
