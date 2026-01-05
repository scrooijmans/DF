# Installation and Requirements

For this set-up we assume you already have node.js and npm, yarn or pnpm already installed. The Svelte Flow package is published under <a href="https://www.npmjs.com/package/@xyflow/svelte" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank"><code class="nextra-code" dir="ltr">@xyflow/svelte</code></a> on npm and installable via:

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

Now you can import the Svelte Flow component and the styles in your application:

``` x:group
import { SvelteFlow } from '@xyflow/svelte';
import '@xyflow/svelte/dist/style.css';
```

## Hit the ground running<a href="https://svelteflow.dev/learn/getting-started/installation#hit-the-ground-running" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

To get folks building quickly, we have a template repository on GitHub that uses Vite and Svelte. You can find the template <a href="https://github.com/xyflow/vite-svelte-flow-template" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">here <img src="out_svelteflow/learn/getting-started/installation/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a>.

To use it, you can either create a new repository from the template, or use `degit` to grab the template’s files without the git history:

``` x:group
npx degit xyflow/vite-svelte-flow-template your-app-name
```

## Prior Experience Needed<a href="https://svelteflow.dev/learn/getting-started/installation#prior-experience-needed" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

Svelte Flow is a Svelte library. That means Svelte developers will feel comfortable using it. If basic Svelte terms and concepts like reactive state, props, components, and lifecycle methods are unfamiliar to you, you might need to learn more about Svelte before being able to use Svelte Flow fully. If you’ve never used Svelte before, we recommend first getting started on Svelte through the <a href="https://svelte.dev/tutorial/basics" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">official tutorial <img src="out_svelteflow/learn/getting-started/installation/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a>.
