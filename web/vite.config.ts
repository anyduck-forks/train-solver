import { sveltekit } from '@sveltejs/kit/vite';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import wasm from 'vite-plugin-wasm';
import { defineConfig } from 'vite';

const rootDir = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const base = process.env.BASE_PATH ?? '/';

export default defineConfig({
	base,
	build: { target: 'esnext' },
	server: {
		fs: {
			allow: [rootDir]
		}
	},
	plugins: [sveltekit(), wasm()]
});
