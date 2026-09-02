import { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js";
import { McpAgent } from "agents/mcp";
import { z } from "zod";

const DOCS_ORIGIN = "https://zod.rs";
const PATH_PATTERN = /^[a-z0-9/-]*$/;

async function fetchText(path: string): Promise<string> {
	const res = await fetch(`${DOCS_ORIGIN}${path}`, {
		cf: { cacheTtl: 300, cacheEverything: true },
	});
	if (!res.ok) throw new Error(`${res.status} fetching ${path}`);
	return res.text();
}

function text(value: string) {
	return { content: [{ type: "text" as const, text: value }] };
}

export class ZodRsDocs extends McpAgent {
	server = new McpServer({
		name: "zod-rs-docs",
		version: "1.0.0",
	});

	async init() {
		this.server.registerTool(
			"list_pages",
			{
				title: "List documentation pages",
				description:
					"List all pages of the zod-rs documentation (a Rust validation library inspired by Zod) as site-relative paths usable with get_page.",
				inputSchema: {},
			},
			async () => {
				const xml = await fetchText("/sitemap-0.xml");
				const pages = [...xml.matchAll(/<loc>([^<]+)<\/loc>/g)]
					.map((m) => new URL(m[1]).pathname)
					.filter((p) => p !== "/404/")
					.sort();
				return text(pages.join("\n"));
			},
		);

		this.server.registerTool(
			"get_page",
			{
				title: "Get a documentation page",
				description:
					"Fetch one zod-rs documentation page as markdown. Pass a site-relative path from list_pages, e.g. /getting-started/ or /primitives/string/.",
				inputSchema: {
					path: z
						.string()
						.describe("Site-relative page path, e.g. /primitives/string/"),
				},
			},
			async ({ path }) => {
				let p = path.trim();
				if (!p.startsWith("/")) p = `/${p}`;
				if (!p.endsWith("/")) p = `${p}/`;
				if (!PATH_PATTERN.test(p)) {
					return text(`Invalid path: ${path}. Use a path from list_pages.`);
				}
				return text(await fetchText(`${p}index.md`));
			},
		);

		this.server.registerTool(
			"search_docs",
			{
				title: "Search the documentation",
				description:
					"Full-text search across the complete zod-rs documentation. Returns the most relevant sections as markdown.",
				inputSchema: {
					query: z.string().describe("Search terms, e.g. 'email validation'"),
					max_results: z
						.number()
						.int()
						.min(1)
						.max(10)
						.default(3)
						.describe("Maximum number of sections to return"),
				},
			},
			async ({ query, max_results }) => {
				const full = await fetchText("/llms-full.txt");
				const sections = full.split(/^(?=# )/m).filter((s) => s.trim());
				const terms = query
					.toLowerCase()
					.split(/\s+/)
					.filter((t) => t.length > 1);
				if (!terms.length) return text("Provide at least one search term.");

				const scored = sections
					.map((section) => {
						const lower = section.toLowerCase();
						const heading = section.split("\n", 1)[0].toLowerCase();
						let score = 0;
						let matched = 0;
						for (const term of terms) {
							const hits = lower.split(term).length - 1;
							if (!hits) continue;
							matched++;
							score += Math.min(hits, 5) + (heading.includes(term) ? 20 : 0);
						}
						return { section, score: score + matched * 1000 };
					})
					.filter((s) => s.score > 0)
					.sort((a, b) => b.score - a.score)
					.slice(0, max_results);

				if (!scored.length) {
					return text(
						`No results for "${query}". Try broader terms, or list_pages to browse.`,
					);
				}
				return text(
					scored
						.map((s) => s.section.slice(0, 6000).trim())
						.join("\n\n---\n\n"),
				);
			},
		);
	}
}

export default {
	fetch(request: Request, env: unknown, ctx: ExecutionContext) {
		const url = new URL(request.url);
		if (url.pathname === "/mcp" || url.pathname.startsWith("/mcp/")) {
			return ZodRsDocs.serve("/mcp", { binding: "ZodRsDocs" }).fetch(
				request,
				env,
				ctx,
			);
		}
		if (url.pathname === "/sse" || url.pathname.startsWith("/sse/")) {
			return ZodRsDocs.serveSSE("/sse", { binding: "ZodRsDocs" }).fetch(
				request,
				env,
				ctx,
			);
		}
		if (url.pathname === "/") {
			return Response.redirect("https://zod.rs/integrations/mcp/", 302);
		}
		return new Response("Not found", { status: 404 });
	},
};
