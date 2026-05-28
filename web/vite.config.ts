import { sveltekit } from '@sveltejs/kit/vite';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import wasm from 'vite-plugin-wasm';
import { defineConfig } from 'vite';

const rootDir = resolve(dirname(fileURLToPath(import.meta.url)), '..');

export default defineConfig({
	build: { target: 'esnext' },
	server: {
		fs: {
			allow: [rootDir]
		}
	},
	plugins: [sveltekit(), wasm()]
});
