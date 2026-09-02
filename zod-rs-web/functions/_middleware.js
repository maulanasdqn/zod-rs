export async function onRequest(context) {
	const { request, next, env } = context;
	const accept = request.headers.get('accept') || '';
	if (request.method === 'GET' && accept.includes('text/markdown')) {
		const url = new URL(request.url);
		const path = url.pathname.endsWith('/') ? url.pathname : url.pathname + '/';
		const asset = await env.ASSETS.fetch(new URL(path + 'index.md', url));
		if (asset.ok) {
			return new Response(asset.body, {
				headers: {
					'content-type': 'text/markdown; charset=utf-8',
					vary: 'accept',
				},
			});
		}
	}
	return next();
}
