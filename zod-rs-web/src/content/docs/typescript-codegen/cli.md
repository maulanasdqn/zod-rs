---
title: CLI Tool
description: Generate TypeScript Zod schemas from the command line
---

The `zod-rs-ts` CLI tool generates TypeScript Zod schemas from Rust source files in batch.

## Installation

```bash
cargo install zod-rs-ts --features cli
```

## Usage

### Generate from a directory

```bash
zod-rs-ts generate --input src/ --output schemas/
```

This scans all Rust files in `src/` for types that derive `ZodTs` and generates corresponding TypeScript files in the `schemas/` directory.

### Single-file output

```bash
zod-rs-ts generate --input src/ --output schemas/index.ts --single-file
```

Combines all generated schemas into a single TypeScript file.

## Workflow

A typical workflow for shared validation:

1. Define your types in Rust with `#[derive(ZodTs)]`
2. Run the CLI to generate TypeScript schemas
3. Import the generated schemas in your TypeScript frontend
4. Both sides validate with the same rules

```rust
// src/models/user.rs
use zod_rs_ts::ZodTs;

#[derive(ZodTs)]
struct CreateUserRequest {
    #[zod(min_length(2), max_length(50))]
    name: String,

    #[zod(email)]
    email: String,
}
```

```bash
zod-rs-ts generate --input src/models/ --output frontend/src/schemas/
```

```typescript
// frontend/src/schemas/create_user_request.ts
import { CreateUserRequestSchema } from './schemas';

const result = CreateUserRequestSchema.safeParse(formData);
```
