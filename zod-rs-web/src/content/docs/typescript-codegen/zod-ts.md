---
title: Generate TypeScript Zod Schemas from Rust
description: Generate TypeScript Zod schemas from Rust types with the ZodTs derive macro to share validation between backend and frontend.
---

The `ZodTs` derive macro generates TypeScript Zod schema code from Rust types, enabling shared validation between your Rust backend and TypeScript frontend.

## Setup

Add the dependency:

```toml
[dependencies]
zod-rs = { version = "1.0", features = ["ts"] }
# Or use the standalone crate
zod-rs-ts = "1.0"
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
import * as z from "zod";

export const UserSchema = z.object({
  username: z.string().min(2).max(50),
  email: z.string().email(),
  age: z.number().int().min(18).max(120),
  bio: z.string().optional()
});

export type User = z.infer<typeof UserSchema>;
```

## Zod version

The generator targets **Zod v4** by default, which uses a namespace import (`import * as z from "zod"`). To emit legacy Zod v3 imports (`import { z } from 'zod'`), enable the `zod-v3` feature:

```toml
[dependencies]
zod-rs = { version = "...", features = ["ts", "zod-v3"] }
# or, directly:
zod-rs-ts = { version = "...", features = ["zod-v3"] }
```

The field/validator output is identical across both versions — only the import statement changes.

## Standard Schema compatibility

Generated schemas are [Standard Schema](https://github.com/standard-schema/standard-schema) compliant out of the box. This is provided by Zod itself: every Zod v3.24+ and Zod v4 schema implements the `~standard` interface natively.

That means the generated output works directly with any Standard Schema consumer — TanStack Form, React Hook Form, and other validation-library-agnostic tooling — with no adapter code.

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
