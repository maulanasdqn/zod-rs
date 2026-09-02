---
title: MCP Server
description: Connect AI assistants to the zod-rs docs with the official MCP server - accurate answers about the Rust validation library in Claude, Cursor, and other MCP clients.
---

zod-rs ships an official [Model Context Protocol](https://modelcontextprotocol.io) server at `https://mcp.zod.rs/mcp`. Connect it to Claude, Cursor, or any MCP client and your AI assistant can search and read the current zod-rs documentation instead of relying on training data.

The server is free, requires no authentication, and always serves the latest published docs.

## Claude Code

```bash
claude mcp add --transport http zod-rs https://mcp.zod.rs/mcp
```

## Claude (web and desktop)

Add a custom connector in **Settings → Connectors** with the URL:

```
https://mcp.zod.rs/mcp
```

## Cursor

Add to `.cursor/mcp.json`:

```json
{
  "mcpServers": {
    "zod-rs": {
      "url": "https://mcp.zod.rs/mcp"
    }
  }
}
```

## Available tools

| Tool | Description |
|------|-------------|
| `search_docs` | Full-text search across the complete documentation, returning the most relevant sections as markdown |
| `get_page` | Fetch one documentation page as markdown by its path, e.g. `/primitives/string/` |
| `list_pages` | List every documentation page path |

## Discovery

The server publishes a machine-readable card at [`/.well-known/mcp/server-card.json`](https://zod.rs/.well-known/mcp/server-card.json), so MCP-aware agents can discover it without configuration. Legacy SSE clients can connect at `https://mcp.zod.rs/sse`.
