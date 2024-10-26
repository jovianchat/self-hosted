import { defineConfig } from 'vite';
import { sveltekit } from '@sveltejs/kit/vite';
import Icons from 'unplugin-icons/vite'

import tailwindcss from 'tailwindcss';
import autoprefixer from 'autoprefixer';

export default defineConfig({
	plugins: [
		sveltekit(),
		Icons({
			compiler: 'svelte',
			autoInstall: true
		})
	],
	server: {
		proxy: {
			'/http-server': {
				target: 'http://localhost:8020',
				changeOrigin: true,
				rewrite: (path) => path.replace(/^\/http-server/, '') // This line rewrites the URL path
			}
		}
	},
	css: {
		postcss: {
			plugins: [
				tailwindcss(),
				autoprefixer()
			]
		}
	}
});
