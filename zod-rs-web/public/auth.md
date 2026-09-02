# Auth.md

Instructions for AI agents and bots accessing zod.rs, the documentation site for the zod-rs Rust validation library.

## Registration

Agents do not need to register to use this site. There is no sign-up flow, no account creation, and no approval process. You can start reading immediately — anonymous access is the intended and only mode of access, for both humans and agents acting on their behalf.

## Sign in

There is no login. No credentials, API keys, OAuth flows, or tokens exist for this site, and no endpoint will ever ask you for them. If a page appears to request credentials for zod.rs, it is not operated by us.

## How agents should access content

1. Fetch [/llms.txt](https://zod.rs/llms.txt) for a machine-readable index, or [/llms-full.txt](https://zod.rs/llms-full.txt) for the complete documentation in one file.
2. Request any page URL with an `Accept: text/markdown` header to receive clean markdown instead of HTML.
3. See [/.well-known/api-catalog](https://zod.rs/.well-known/api-catalog) for a directory of these machine-readable resources.

## Scope

This site serves documentation only and exposes no web API that acts on user data. The zod-rs library itself is distributed through [crates.io](https://crates.io/crates/zod-rs) and developed on [GitHub](https://github.com/maulanasdqn/zod-rs), each governed by their own authentication.
