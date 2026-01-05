import adapter from '@sveltejs/adapter-static'
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte'

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: vitePreprocess(),
	kit: {
		adapter: adapter({
			fallback: 'index.html'
		}),
		alias: {
			$lib: './src/lib'
		}
		// Note: files configuration removed - using SvelteKit defaults:
		// - routes: 'src/routes'
		// - lib: 'src/lib'
		// - appTemplate: 'src/app.html'
		// - errorTemplate: 'src/error.html'
		// - hooks.client: 'src/hooks.client'
		// - hooks.server: 'src/hooks.server'
		// Docs directory exclusion is handled by vite.config.ts
	}
}

export default config
