# Built-In Components

Svelte Flow comes with several additional components that you can plug into your flow. All you have to do is import and add them as children to the `SvelteFlow` component.

<img src="out_svelteflow/learn/getting-started/built-in-components/index_media/0b7eab85abc3682cb59a01670459912dfc0cfbe8.svg" class="x:max-w-6 x:shrink-0" />BuiltInComponents.svelte

<img src="out_svelteflow/learn/getting-started/built-in-components/index_media/4ad055c8513380c64c1f5319faccf453f477bb38.svg" class="nextra-copy-icon" />

``` x:group
<script>
  import { SvelteFlow, MiniMap, Controls, Background, Panel } from '@xyflow/svelte';
 
  import '@xyflow/svelte/dist/style.css';
</script>
 
<SvelteFlow>
  <MiniMap />
  <Controls />
  <Background />
  <Panel position="top-left">
    <h1>My Flow</h1>
  </Panel>
</SvelteFlow>
```

## MiniMap<a href="https://svelteflow.dev/learn/getting-started/built-in-components#minimap" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

The <a href="https://svelteflow.dev/api-reference/components/minimap" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">MiniMap</code></a> provides a bird’s-eye view of your flowgraph, making navigation easier, especially for larger flows. You can customize the appearance of nodes in the minimap by providing a `nodeColor` function.

<img src="out_svelteflow/learn/getting-started/built-in-components/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/learn/getting-started/built-in-components/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

## Controls<a href="https://svelteflow.dev/learn/getting-started/built-in-components#controls" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

Svelte Flow comes with a set of customizable <a href="https://svelteflow.dev/api-reference/components/controls" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">Controls</code></a> for the viewport. You can zoom in and out, fit the viewport and toggle if the user can move, select and edit the nodes.

<img src="out_svelteflow/learn/getting-started/built-in-components/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/learn/getting-started/built-in-components/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

## Background<a href="https://svelteflow.dev/learn/getting-started/built-in-components#background" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

The <a href="https://svelteflow.dev/api-reference/components/background" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">Background</code></a> component adds a visual grid pattern to your flowgraph, helping users maintain orientation. You can choose from different pattern variants, or if you need more advanced customization, you can explore the <a href="https://github.com/xyflow/xyflow/blob/main/packages/svelte/src/lib/plugins/Background/Background.svelte" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">source code <img src="out_svelteflow/learn/getting-started/built-in-components/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a> to implement your own pattern.

<img src="out_svelteflow/learn/getting-started/built-in-components/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/learn/getting-started/built-in-components/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

## Panel<a href="https://svelteflow.dev/learn/getting-started/built-in-components#panel" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

The <a href="https://svelteflow.dev/api-reference/components/panel" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">Panel</code></a> component allows you to add fixed overlays to your flowgraph, perfect for titles, controls, or any other UI elements that should remain stationary.

<img src="out_svelteflow/learn/getting-started/built-in-components/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/learn/getting-started/built-in-components/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />
