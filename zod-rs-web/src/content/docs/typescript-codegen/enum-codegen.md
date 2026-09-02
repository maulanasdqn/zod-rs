---
title: Enum Codegen
description: Generate TypeScript Zod union schemas from Rust enums with the ZodTs derive macro.
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
import * as z from "zod";

export const StatusSchema = z.union([
  z.literal("Active"),
  z.literal("Inactive")
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
import * as z from "zod";

export const EventSchema = z.union([
  z.object({ Click: z.object({ x: z.number().int(), y: z.number().int() }) }),
  z.object({ Scroll: z.object({ delta: z.number() }) })
]);
```

See [ZodTs Derive — Zod version](/typescript-codegen/zod-ts/#zod-version) for switching between Zod v3 and v4 output.

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
