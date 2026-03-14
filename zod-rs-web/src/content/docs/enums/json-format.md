---
title: JSON Format
description: JSON representation of Rust enum variants
---

zod-rs uses serde's default externally-tagged format for enums. Here's how each variant type maps to JSON.

## Format table

| Variant Type | Rust | JSON |
|-------------|------|------|
| Unit | `Status::Active` | `{"Active": null}` |
| Tuple (single) | `Message::Text("hi")` | `{"Text": "hi"}` |
| Tuple (multiple) | `Message::Coords(1, 2)` | `{"Coords": [1, 2]}` |
| Struct | `Event::Click { x: 1, y: 2 }` | `{"Click": {"x": 1, "y": 2}}` |

## Unit variants

Unit variants serialize as `{"VariantName": null}`:

```rust
#[derive(Serialize, Deserialize, ZodSchema)]
enum Status { Active, Inactive }

// JSON: {"Active": null}
```

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

Each variant is validated as a union. The value must be a single-key object where the key matches a variant name, and the value matches that variant's expected format.
