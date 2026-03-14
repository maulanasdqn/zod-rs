// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

// https://astro.build/config
export default defineConfig({
	integrations: [
		starlight({
			title: 'zod-rs',
			logo: {
				light: './src/assets/logo-light.svg',
				dark: './src/assets/logo-dark.svg',
				replacesTitle: false,
			},
			social: [
				{ icon: 'github', label: 'GitHub', href: 'https://github.com/maulanasdqn/zod-rs' },
			],
			customCss: ['./src/styles/custom.css'],
			sidebar: [
				{
					label: 'Getting Started',
					items: [
						{ label: 'Introduction', slug: '' },
						{ label: 'Installation & Quick Start', slug: 'getting-started' },
					],
				},
				{
					label: 'Primitives',
					items: [
						{ label: 'String', slug: 'primitives/string' },
						{ label: 'Number', slug: 'primitives/number' },
						{ label: 'Boolean', slug: 'primitives/boolean' },
						{ label: 'Literal', slug: 'primitives/literal' },
						{ label: 'Null', slug: 'primitives/null' },
					],
				},
				{
					label: 'Complex Types',
					items: [
						{ label: 'Object', slug: 'complex-types/object' },
						{ label: 'Array', slug: 'complex-types/array' },
						{ label: 'Optional', slug: 'complex-types/optional' },
						{ label: 'Union', slug: 'complex-types/union' },
						{ label: 'Tuple', slug: 'complex-types/tuple' },
					],
				},
				{
					label: 'Derive Macros',
					items: [
						{ label: 'ZodSchema', slug: 'derive-macros/zod-schema' },
						{ label: 'Attributes Reference', slug: 'derive-macros/attributes' },
						{ label: 'Nested Structs', slug: 'derive-macros/nested-structs' },
					],
				},
				{
					label: 'Enums',
					items: [
						{ label: 'Overview', slug: 'enums/overview' },
						{ label: 'JSON Format', slug: 'enums/json-format' },
					],
				},
				{
					label: 'TypeScript Codegen',
					items: [
						{ label: 'ZodTs Derive', slug: 'typescript-codegen/zod-ts' },
						{ label: 'CLI Tool', slug: 'typescript-codegen/cli' },
						{ label: 'Enum Codegen', slug: 'typescript-codegen/enum-codegen' },
					],
				},
				{
					label: 'Integrations',
					items: [
						{ label: 'Axum', slug: 'integrations/axum' },
					],
				},
				{
					label: 'Advanced',
					items: [
						{ label: 'Error Handling', slug: 'advanced/error-handling' },
						{ label: 'Schema Composition', slug: 'advanced/schema-composition' },
						{ label: 'Internationalization', slug: 'advanced/i18n' },
					],
				},
				{
					label: 'Comparison',
					items: [
						{ label: 'vs Validator Crate', slug: 'comparison/vs-validator' },
					],
				},
			],
		}),
	],
});
