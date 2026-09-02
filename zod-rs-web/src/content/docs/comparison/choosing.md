---
title: Choosing a Rust Validation Library
description: An honest comparison of the best Rust validation libraries - zod-rs, validator, and garde - and how to pick the right one for your project.
---

The Rust ecosystem has three main approaches to data validation. This page compares them honestly — including where zod-rs is *not* the right choice — so you can pick based on where validation happens in your program.

## The three approaches

| | zod-rs | validator | garde |
|---|--------|-----------|-------|
| **Model** | Schemas as runtime values | Struct attributes | Struct attributes |
| **Validates** | Raw JSON | Deserialized structs | Deserialized structs |
| **Derive macro** | Yes (`ZodSchema`) | Yes (`Validate`) | Yes (`Validate`) |
| **Runtime schema building** | Yes | No | No |
| **Parse + validate in one step** | Yes | No | No |
| **Context-aware rules** | Custom `Schema` impls | Custom functions | Built-in context |
| **Error paths in nested data** | Full paths | Field-level | Path-aware |
| **TypeScript codegen** | Zod schemas | No | No |
| **Built-in i18n** | Yes | No | No |
| **Axum integration** | Built-in feature | Community crates | Community crates |

## Pick by where validation happens

### Your data is already a typed struct

If you construct structs in Rust and only need to check invariants — lengths, ranges, formats — an attribute-based validator is the simplest tool. **garde** is the modern choice here: it's a rewrite of validator with better `Option` handling and built-in context-aware validation. **validator** remains fine if you're already using it.

### Your data arrives as JSON

If validation starts at a boundary — API request bodies, webhooks, config files, message queues — **zod-rs** validates the JSON itself and hands you a typed struct in one step. You get one error shape for both malformed and invalid input, full paths to failing fields (`user.addresses[0].zip`), and schemas you can build, store, and compose at runtime.

### Your validation rules must match a TypeScript frontend

Only **zod-rs** generates [TypeScript Zod schemas](/typescript-codegen/zod-ts/) from Rust types, so a Rust backend and a TypeScript frontend can enforce identical rules from one definition. If you're validating the same payloads on both sides today, this is the differentiator.

## Trying zod-rs

```toml
[dependencies]
zod-rs = "1.1"
```

The [getting started guide](/getting-started/) covers installation and a first schema in a few minutes, and there are step-by-step migration guides from [validator](/comparison/vs-validator/) and [garde](/comparison/vs-garde/).
