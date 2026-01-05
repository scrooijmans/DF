import { sveltekit } from '@sveltejs/kit/vite';
import tailwindcss from '@tailwindcss/vite';
import { defineConfig } from 'vite';
import type { Plugin } from 'vite';

// Plugin to exclude docs directory from being processed
// Optimized with early returns and cached path checks
function excludeDocsPlugin(): Plugin {
	// Cache common patterns for faster matching
	const docsPatterns = ['/docs/', '\\docs\\', '/markdown-guides/', '\\markdown-guides\\'];

	const isDocsFile = (id: string): boolean => {
		// Early return for non-docs paths
		if (!id.includes('docs') && !id.includes('markdown-guides')) {
			return false;
		}
		// Check against cached patterns
		return (
			docsPatterns.some((pattern) => id.includes(pattern)) ||
			(id.includes('@fs') && (id.includes('/docs/') || id.includes('\\docs\\')))
		);
	};

	return {
		name: 'exclude-docs',
		enforce: 'pre', // Run before other plugins
		resolveId(id: string) {
			// Skip resolving files in docs directory
			if (isDocsFile(id)) {
				// Return a virtual empty module to prevent errors
				return `\0excluded:${id}`;
			}
			return null;
		},
		load(id: string) {
			// Return empty module for excluded files
			if (id.startsWith('\0excluded:')) {
				return 'export {}';
			}
			return null;
		}
	};
}

export default defineConfig({
	plugins: [
		// Add exclude plugin first (before Tailwind)
		excludeDocsPlugin(),
		tailwindcss(),
		sveltekit()
	],
	clearScreen: false,
	server: {
		port: 1420,
		strictPort: true,
		watch: {
			ignored: [
				'**/src-tauri/**',
				'**/docs/**',
				'**/markdown-guides/**',
				'**/node_modules/**',
				'**/.git/**'
			]
		},
		fs: {
			// Deny serving files from docs folder
			deny: ['**/docs/**', '**/markdown-guides/**']
		},
		// Warmup frequently used files for faster initial load
		warmup: {
			clientFiles: [
				'./src/routes/+layout.svelte',
				'./src/routes/+layout.ts',
				'./src/routes/(app)/+page.svelte',
				'./src/lib/stores/auth.svelte.ts',
				'./src/app.css'
			]
		}
	},
	ssr: {
		// Exclude docs directory from SSR processing
		noExternal: []
	},
	optimizeDeps: {
		// Pre-bundle dependencies for faster startup
		include: [
			'@tauri-apps/api',
			'@tauri-apps/api/core',
			'@tauri-apps/plugin-dialog',
			'@tauri-apps/plugin-fs',
			'@tauri-apps/plugin-shell'
		],
		// Exclude SvelteKit internals and virtual modules from optimization
		exclude: ['@sveltejs/kit', 'svelte']
	},
	// Mark CSS files in docs as static assets to prevent processing
	assetsInclude: ['**/docs/**/*.css', '**/markdown-guides/**/*.css'],
	// Build optimizations
	build: {
		// Increase chunk size warning limit
		chunkSizeWarningLimit: 1000,
		// Optimize chunk splitting
		rollupOptions: {
			output: {
				manualChunks: {
					// Separate vendor chunks for better caching
					'vendor-svelte': ['svelte', '@sveltejs/kit'],
					'vendor-tauri': ['@tauri-apps/api', '@tauri-apps/api/core']
				}
			}
		}
	},
	// ESBuild options for faster transforms
	esbuild: {
		target: 'esnext',
		// Drop console and debugger in production
		drop: process.env.NODE_ENV === 'production' ? ['console', 'debugger'] : []
	}
});
