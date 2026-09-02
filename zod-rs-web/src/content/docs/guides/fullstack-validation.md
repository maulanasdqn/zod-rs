---
title: Share Validation Between Rust and TypeScript
description: Define validation rules once in Rust and generate matching TypeScript Zod schemas — your backend and frontend enforce identical rules from a single source of truth.
---

To share validation between a Rust backend and a TypeScript frontend, define your schemas once in Rust with `#[derive(ZodTs)]`, run the [zod-rs-ts CLI](/typescript-codegen/cli/) to generate TypeScript Zod schemas, and import them in your frontend code. Both sides enforce identical rules from a single source of truth — no manual syncing, no validation drift.

## The problem: validation drift

In a typical full-stack application the backend and frontend validate the same data independently. The Rust API checks that `username` is 3–20 characters; the React form checks that `username` is 2–30 characters. Nobody notices the mismatch until a user submits a value the frontend accepts and the backend rejects — or worse, the frontend blocks a value the backend would allow.

Keeping two rule sets in sync by hand does not scale. Every time a field changes — a new constraint, a renamed property, a new enum variant — someone has to update both sides and hope the PR reviewer catches the ones they missed.

## The solution: define once, generate TypeScript

zod-rs solves this with a two-step workflow:

1. **Define** validation rules in Rust using `#[derive(ZodSchema, ZodTs)]` on your types.
2. **Generate** matching TypeScript [Zod](https://zod.dev) schemas with the [zod-rs-ts CLI](/typescript-codegen/cli/).

The generated TypeScript code imports from `zod` and mirrors every field name, type, and constraint from your Rust definition. Your frontend `npm run build` always validates with the same rules your Rust backend enforces.

## Step-by-step setup

### 1. Add derives to your Rust types

Annotate each type with both [`ZodSchema`](/derive-macros/zod-schema/) (for Rust-side validation) and `ZodTs` (for TypeScript generation):

```rust
use zod_rs::prelude::*;

#[derive(ZodSchema, ZodTs, serde::Deserialize, Debug)]
struct SignupRequest {
    #[zod(min_length(3), max_length(20))]
    username: String,

    #[zod(email)]
    email: String,

    #[zod(min(13.0), max(120.0), int)]
    age: u32,

    #[zod(min_length(8))]
    password: String,
}
```

Every `#[zod(...)]` attribute you add is reflected in both the Rust schema and the generated TypeScript schema. See the [attributes reference](/derive-macros/attributes/) for the full list.

### 2. Install the CLI

```bash
cargo install zod-rs-ts
```

### 3. Generate TypeScript

Point the CLI at your Rust source files:

```bash
zod-rs-ts src/models.rs -o frontend/src/schemas/
```

This produces a file like `frontend/src/schemas/models.ts`:

```typescript
import { z } from "zod";

export const SignupRequestSchema = z.object({
  username: z.string().min(3).max(20),
  email: z.string().email(),
  age: z.number().int().min(13).max(120),
  password: z.string().min(8),
});

export type SignupRequest = z.infer<typeof SignupRequestSchema>;
```

The generated code is self-contained — it depends only on `zod`, which your frontend likely already uses.

### 4. Use in your frontend

```typescript
import { SignupRequestSchema } from "../schemas/models";

function validateSignup(formData: Record<string, unknown>) {
  const result = SignupRequestSchema.safeParse(formData);
  if (!result.success) {
    return result.error.issues.map((i) => `${i.path.join(".")}: ${i.message}`);
  }
  return result.data; // typed as SignupRequest
}
```

Now your React form, Vue component, or Svelte page validates with the exact same constraints as your Rust API handler. Change a rule in Rust, regenerate, and the frontend follows automatically.

## Full example: signup form

Here is the complete flow for a signup form validated identically on both sides.

**Rust backend (Axum handler):**

```rust
use axum::{Json, response::IntoResponse};
use zod_rs::prelude::*;

#[derive(ZodSchema, ZodTs, serde::Deserialize, Debug)]
struct SignupRequest {
    #[zod(min_length(3), max_length(20))]
    username: String,

    #[zod(email)]
    email: String,

    #[zod(min(13.0), max(120.0), int)]
    age: u32,

    #[zod(min_length(8))]
    password: String,
}

async fn signup(body: Json<serde_json::Value>) -> impl IntoResponse {
    match SignupRequest::validate_and_parse(&body) {
        Ok(req) => { /* create user */ }
        Err(errors) => { /* return 422 with errors */ }
    }
}
```

**TypeScript frontend (generated):**

```typescript
import { SignupRequestSchema } from "./schemas/models";

const result = SignupRequestSchema.safeParse({
  username: "al",       // fails: min 3
  email: "not-email",   // fails: invalid email
  age: 10,              // fails: min 13
  password: "short",    // fails: min 8
});

// result.error.issues matches the Rust-side errors field-for-field
```

Both sides reject the same input for the same reasons with the same field paths.

## Enum codegen

Rust enums generate TypeScript Zod unions automatically. Unit variants become `z.literal()`, and tuple or struct variants become `z.object()` branches:

```rust
#[derive(ZodSchema, ZodTs, serde::Deserialize)]
enum Role {
    Admin,
    Member,
    Guest,
}
```

Generates:

```typescript
export const RoleSchema = z.union([
  z.literal("Admin"),
  z.literal("Member"),
  z.literal("Guest"),
]);
```

Serde rename attributes are respected — if your Rust enum uses `#[serde(rename_all = "snake_case")]`, the generated literals match. See [Enum Codegen](/typescript-codegen/enum-codegen/) for struct and tuple variant examples.

## CI integration: auto-generate on build

Add the generation step to your CI pipeline so generated schemas never go stale:

```yaml
# .github/workflows/ci.yml
- name: Generate TypeScript schemas
  run: |
    cargo install zod-rs-ts
    zod-rs-ts src/models/ -o frontend/src/schemas/
    cd frontend && npx tsc --noEmit  # type-check the generated code
```

Or add it to a `build.rs` or a Makefile target so it runs before every frontend build:

```makefile
.PHONY: schemas
schemas:
	zod-rs-ts src/models.rs -o frontend/src/schemas/

.PHONY: frontend
frontend: schemas
	cd frontend && npm run build
```

If you commit the generated files, your CI can also verify they are up to date:

```bash
zod-rs-ts src/models.rs -o frontend/src/schemas/
git diff --exit-code frontend/src/schemas/ || (echo "Generated schemas are stale" && exit 1)
```

## When to use this approach

**Use single-source validation when:**

- Your Rust API and TypeScript frontend validate the same request or response payloads.
- You have shared types that both sides need to agree on (user profiles, orders, config).
- You want validation errors to be consistent between client and server.
- Your team is tired of manually keeping two schema files in sync.

**Use separate schemas when:**

- The frontend and backend validate fundamentally different shapes (e.g., the frontend form has confirm-password but the API does not).
- You do not use Zod on the TypeScript side.
- Your validation rules are trivial enough that drift is not a real risk.

## Next steps

- [ZodTs derive macro reference](/typescript-codegen/zod-ts/) — all options for controlling the generated output
- [CLI tool reference](/typescript-codegen/cli/) — batch generation, output directory, and file filtering
- [ZodSchema derive](/derive-macros/zod-schema/) — Rust-side validation from the same types
- [Validate structs in Rust](/guides/validate-structs/) — struct validation patterns that pair with TypeScript codegen
