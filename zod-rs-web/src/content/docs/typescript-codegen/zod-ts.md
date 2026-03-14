---
title: ZodTs Derive
description: Generate TypeScript Zod schemas from Rust types
---

The `ZodTs` derive macro generates TypeScript Zod schema code from Rust types, enabling shared validation between your Rust backend and TypeScript frontend.

## Setup

Add the dependency:

```toml
[dependencies]
zod-rs = { version = "0.4", features = ["ts"] }
# Or use the standalone crate
zod-rs-ts = "0.4"
```

## Basic usage

```rust
use zod_rs_ts::ZodTs;

#[derive(ZodTs)]
struct User {
    #[zod(min_length(2), max_length(50))]
    username: String,

    #[zod(email)]
    email: String,

    #[zod(min(18.0), max(120.0), int)]
    age: u32,

    bio: Option<String>,
}

fn main() {
    let ts_code = User::zod_ts();
    println!("{}", ts_code);

    // Write to file
    std::fs::write("schemas/user.ts", ts_code).unwrap();
}
```

## Generated output

The above generates:

```typescript
import { z } from 'zod';

export const UserSchema = z.object({
  username: z.string().min(2).max(50),
  email: z.string().email(),
  age: z.number().int().min(18).max(120),
  bio: z.string().optional()
});

export type User = z.infer<typeof UserSchema>;
```

## Type mapping

| Rust Type | TypeScript Zod |
|-----------|---------------|
| `String` | `z.string()` |
| `f32`, `f64` | `z.number()` |
| `i8`..`i64`, `u8`..`u64` | `z.number().int()` |
| `bool` | `z.boolean()` |
| `Vec<T>` | `z.array(T)` |
| `Option<T>` | `T.optional()` |

## Validation attributes

The same `#[zod(...)]` attributes used with `ZodSchema` are translated to TypeScript Zod methods. See the [attributes reference](/derive-macros/attributes/) for the full list.
