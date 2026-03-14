---
title: Enum Codegen
description: TypeScript code generation for Rust enums
---

The `ZodTs` derive macro also supports enums, generating TypeScript Zod union schemas.

## Unit enums

```rust
use zod_rs_ts::ZodTs;

#[derive(ZodTs)]
enum Status {
    Active,
    Inactive,
}

fn main() {
    println!("{}", Status::zod_ts());
}
```

Generated TypeScript:

```typescript
export const StatusSchema = z.union([
  z.object({ Active: z.null() }),
  z.object({ Inactive: z.null() })
]);
```

## Struct variant enums

```rust
#[derive(ZodTs)]
enum Event {
    Click { x: i32, y: i32 },
    Scroll { delta: f64 },
}

fn main() {
    println!("{}", Event::zod_ts());
}
```

Generated TypeScript:

```typescript
export const EventSchema = z.union([
  z.object({ Click: z.object({ x: z.number().int(), y: z.number().int() }) }),
  z.object({ Scroll: z.object({ delta: z.number() }) })
]);
```

## Mixed enums

Enums with mixed variant types are fully supported:

```rust
#[derive(ZodTs)]
enum ApiResponse {
    Success,
    Data(String),
    Error { code: i32, message: String },
}
```

The generated schema creates a union with the appropriate structure for each variant.
