import { readdirSync, readFileSync, writeFileSync, mkdirSync } from 'node:fs';
import { join, relative, sep } from 'node:path';

const srcDir = new URL('../src/content/docs', import.meta.url).pathname;
const distDir = new URL('../dist', import.meta.url).pathname;

const walk = (dir) =>
	readdirSync(dir, { withFileTypes: true }).flatMap((entry) =>
		entry.isDirectory() ? walk(join(dir, entry.name)) : [join(dir, entry.name)],
	);

let count = 0;
for (const file of walk(srcDir)) {
	if (!/\.mdx?$/.test(file)) continue;
	const slug = relative(srcDir, file)
		.replace(/\.mdx?$/, '')
		.split(sep)
		.join('/')
		.replace(/(^|\/)index$/, '');
	const outDir = join(distDir, slug);
	mkdirSync(outDir, { recursive: true });
	const content = readFileSync(file, 'utf8').replace(/^import .*$\n?/gm, '');
	writeFileSync(join(outDir, 'index.md'), content);
	count++;
}
console.log(`copy-md: wrote ${count} markdown files to dist`);
