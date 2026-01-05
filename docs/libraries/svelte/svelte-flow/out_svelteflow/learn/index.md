# Quickstart

If you want to get up and running as soon as possible, you’re in the right place! This page will take you from zero to a working Svelte Flow app in a few minutes. From there, you can take a deeper look at what Svelte Flow is all about, check out the examples, or dive into the API docs.

## Dependency<a href="https://svelteflow.dev/learn#dependency" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

Svelte Flow is published on npm as <a href="https://npmjs.com/package/@xyflow/svelte" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank"><code class="nextra-code" dir="ltr">@xyflow/svelte</code></a>.

npm

pnpm

yarn

bun

``` x:group
npm install @xyflow/svelte
```

``` x:group
pnpm add @xyflow/svelte
```

``` x:group
yarn add @xyflow/svelte
```

``` x:group
bun add @xyflow/svelte
```

## Play online<a href="https://svelteflow.dev/learn#play-online" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

You can try Svelte Flow without setting anything up locally by checking out the starter projects we have on <a href="https://stackblitz.com" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">Stackblitz <img src="out_svelteflow/learn/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a>:

<a href="https://new.svelteflow.dev/js" class="x:group x:focus-visible:nextra-focus nextra-card x:flex x:flex-col x:justify-start x:overflow-hidden x:rounded-lg x:border x:border-gray-200 x:text-current x:no-underline x:dark:shadow-none x:hover:shadow-gray-100 x:dark:hover:shadow-none x:shadow-gray-100 x:active:shadow-sm x:active:shadow-gray-200 x:transition-all x:duration-200 x:hover:border-gray-300 x:bg-transparent x:shadow-sm x:dark:border-neutral-800 x:hover:bg-slate-50 x:hover:shadow-md x:dark:hover:border-neutral-700 x:dark:hover:bg-neutral-900"></a>

JS

new.svelteflow.dev/js<a href="https://new.svelteflow.dev/ts" class="x:group x:focus-visible:nextra-focus nextra-card x:flex x:flex-col x:justify-start x:overflow-hidden x:rounded-lg x:border x:border-gray-200 x:text-current x:no-underline x:dark:shadow-none x:hover:shadow-gray-100 x:dark:hover:shadow-none x:shadow-gray-100 x:active:shadow-sm x:active:shadow-gray-200 x:transition-all x:duration-200 x:hover:border-gray-300 x:bg-transparent x:shadow-sm x:dark:border-neutral-800 x:hover:bg-slate-50 x:hover:shadow-md x:dark:hover:border-neutral-700 x:dark:hover:bg-neutral-900"></a>

TS

new.svelteflow.dev/ts

## Vite template<a href="https://svelteflow.dev/learn#vite-template" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

If you want to get started right away, you can use our <a href="https://github.com/xyflow/vite-svelte-flow-template" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">vite template <img src="out_svelteflow/learn/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a>:

npm

pnpm

yarn

bun

``` x:group
npx degit xyflow/vite-svelte-flow-template app-name
```

``` x:group
pnpm dlx degit xyflow/vite-svelte-flow-template app-name
```

``` x:group
yarn dlx degit xyflow/vite-svelte-flow-template app-name
```

``` x:group
bun x degit xyflow/vite-svelte-flow-template app-name
```

## Project Setup<a href="https://svelteflow.dev/learn#project-setup" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

To get started locally, you should have a few things:

- <a href="https://nodejs.org/en/" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">Node.js <img src="out_svelteflow/learn/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a> installed.
- Either npm or another package manager like <a href="https://yarnpkg.com/" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">yarn <img src="out_svelteflow/learn/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a> or <a href="https://pnpm.io/" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">pnpm <img src="out_svelteflow/learn/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a>.
- Some knowledge of <a href="https://svelte.dev/" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">Svelte <img src="out_svelteflow/learn/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a>. You don’t need to be an expert, but you should be comfortable with the basics.

First, spin up a new <a href="https://svelte.dev/" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">Svelte <img src="out_svelteflow/learn/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a> project however you like; we recommend using <a href="https://vitejs.dev/" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">Vite <img src="out_svelteflow/learn/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a> and <a href="https://svelte.dev/docs/kit/introduction" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">SvelteKit <img src="out_svelteflow/learn/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a> but the choice is yours.

npm

pnpm

yarn

bun

``` x:group
npx sv create my-svelte-flow-app
```

``` x:group
pnpm dlx sv create my-svelte-flow-app
```

``` x:group
yarn dlx sv create my-svelte-flow-app
```

``` x:group
bun x sv create my-svelte-flow-app
```

Then, navigate to your project directory and install the Svelte Flow package:

npm

pnpm

yarn

bun

``` x:group
npm install @xyflow/svelte
```

``` x:group
pnpm add @xyflow/svelte
```

``` x:group
yarn add @xyflow/svelte
```

``` x:group
bun add @xyflow/svelte
```

## Creating your first flow<a href="https://svelteflow.dev/learn#creating-your-first-flow" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

The `@xyflow/svelte` package exports the `<SvelteFlow />` component, which is the entrypoint for you flow. Importing the default styles and defining a handful of nodes and edges are all we need to get started!

There are a few things to pay attention to here:

- 🎨 You must import the Svelte Flow stylesheet.
- 📐 `<SvelteFlow />` inherits the size of its parent. Wrap it in an element with dimensions.
- ⚡ Use <a href="https://svelte.dev/docs/svelte/$state#$state.raw" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank"><code class="nextra-code" dir="ltr">$state.raw</code></a> instead of deeply reactive state for the `nodes` and `edges` for <a href="https://github.com/sveltejs/svelte/issues/11851" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">performance reasons <img src="out_svelteflow/learn/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a>.

``` x:group
<script>
  import { SvelteFlow } from '@xyflow/svelte';
 
  import '@xyflow/svelte/dist/style.css';
 
  let nodes = $state.raw([
    { id: '1', position: { x: 0, y: 0 }, data: { label: '1' } },
    { id: '2', position: { x: 0, y: 100 }, data: { label: '2' } },
  ]);
 
  let edges = $state.raw([
    { id: 'e1-2', source: '1', target: '2' },
  ]);
</script>
 
<div style:width="100vw" style:height="100vh">
  <SvelteFlow bind:nodes bind:edges />
</div>
```
