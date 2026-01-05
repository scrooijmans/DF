Title: SvelteKit | Tauri

Description: The cross-platform app building toolkit

Skip to content

SvelteKit
=========

SvelteKit is a meta framework for Svelte. Learn more about SvelteKit at https://svelte.dev/. This guide is accurate as of SvelteKit 2.20.4 / Svelte 5.25.8.

Checklist
---------

Section titled “Checklist”

*   Use SSG and SPA via `static-adapter`. Tauri doesn’t support server-based solutions.
*   If using SSG **with prerendering**, be aware that `load` functions will not have access to tauri APIs during the build process of your app. Using SPA mode (without prerendering) is recommended since the load functions will only run in the webview with access to tauri APIs.
*   Use `build/` as `frontendDist` in `tauri.conf.json`.

Example Configuration
---------------------

Section titled “Example Configuration”

1.  ##### Install `@sveltejs/adapter-static`

Section titled “Install @sveltejs/adapter-static”

*   npm
*   yarn
*   pnpm
*   deno

```
npm install --save-dev @sveltejs/adapter-static
```

```
yarn add -D @sveltejs/adapter-static
```

```
pnpm add -D @sveltejs/adapter-static
```

```
deno add -D npm:@sveltejs/adapter-static
```

2.  ##### Update Tauri configuration

Section titled “Update Tauri configuration”

*   npm
*   yarn
*   pnpm
*   deno

tauri.conf.json

```
{  "build": {    "beforeDevCommand": "npm run dev",    "beforeBuildCommand": "npm run build",    "devUrl": "http://localhost:5173",    "frontendDist": "../build"  }}
```

tauri.conf.json

```
{  "build": {    "beforeDevCommand": "yarn dev",    "beforeBuildCommand": "yarn build",    "devUrl": "http://localhost:5173",    "frontendDist": "../build"  }}
```

tauri.conf.json

```
{  "build": {    "beforeDevCommand": "pnpm dev",    "beforeBuildCommand": "pnpm build",    "devUrl": "http://localhost:5173",    "frontendDist": "../build"  }}
```

tauri.conf.json

```
{  "build": {    "beforeDevCommand": "deno task dev",    "beforeBuildCommand": "deno task build",    "devUrl": "http://localhost:5173",    "frontendDist": "../build"  }}
```

3.  ##### Update SvelteKit configuration:

Section titled “Update SvelteKit configuration:”

svelte.config.js

```
import adapter from '@sveltejs/adapter-static';import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';
/** @type {import('@sveltejs/kit').Config} */const config = {  // Consult https://svelte.dev/docs/kit/integrations#preprocessors  // for more information about preprocessors  preprocess: vitePreprocess(),
kit: {    adapter: adapter({      fallback: 'index.html',    }),  },};
export default config;
```

4.  ##### Disable SSR

Section titled “Disable SSR”

Lastly, we need to disable SSR by adding a root `+layout.ts` file (or `+layout.js` if you are not using TypeScript) with these contents:

src/routes/+layout.ts

```
export const ssr = false;
```

Note that `static-adapter` doesn’t require you to disable SSR for the whole app but it makes it possible to use APIs that depend on the global window object (like Tauri’s API) without Client-side checks.

Furthermore, if you prefer Static Site Generation (SSG) over Single-Page Application (SPA) mode, you can change the adapter configurations and `+layout.ts` according to the adapter docs.

© 2025 Tauri Contributors. CC-BY / MIT