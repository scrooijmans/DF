# Building a Flow

Ready to create your first flow? This guide will walk you through the process step by step. If you haven’t reviewed our <a href="https://svelteflow.dev/learn/getting-started/key-concepts" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]">Key Concepts</a> yet, we recommend doing that first.

## Getting Started<a href="https://svelteflow.dev/learn/getting-started/building-a-flow#getting-started" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

First, import the Svelte Flow Component and its required styles into your project. We’ll also import the `Background` component for visual enhancement.

``` x:group
<script>
import { SvelteFlow, Background } from '@xyflow/svelte';
import '@xyflow/svelte/dist/style.css';
</script>
```

Next, render the main component inside an element with defined dimensions and place the `Background` component inside `SvelteFlow`.

<img src="out_svelteflow/learn/getting-started/building-a-flow/index_media/a7db0f47f02f813c5a264458a8e522bf6320ca5a.svg" class="x:mt-[.3em]" />

Content inside `SvelteFlow` stays fixed on top of the viewport. The `Background` component transforms its pattern to match viewport movement.

``` x:group
<div style:width="100vw" style:height="100vh">
    <SvelteFlow>
        <Background />
    </SvelteFlow>
</div>
```

If everything is set up correctly, you should see a blank canvas like this:

<img src="out_svelteflow/learn/getting-started/building-a-flow/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/learn/getting-started/building-a-flow/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

## Adding nodes<a href="https://svelteflow.dev/learn/getting-started/building-a-flow#adding-nodes" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

Now that the flow is set up, let’s add some nodes. Create an array of <a href="https://svelteflow.dev/api-reference/types/node" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]">node objects</a> with these **required** properties:

- `id`: A unique identifier for each node
- `position`: The x and y coordinates
- `data`: An object for storing custom data

We’ll use the <a href="https://svelte.dev/docs/svelte/$state" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank"><code class="nextra-code" dir="ltr">$state.raw</code></a> rune to make the array reactive.

<img src="out_svelteflow/learn/getting-started/building-a-flow/index_media/0fee490687881197462888c84507f0512d9761c1.svg" class="x:mt-[.3em]" />

Simply using `$state` would **not only** make the array reactive, but **every property of each node**, too. This could lead to <a href="https://github.com/sveltejs/svelte/issues/11851" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">performance issues <img src="out_svelteflow/learn/getting-started/building-a-flow/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a>, so we use <a href="https://svelte.dev/docs/svelte/$state#$state.raw" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank"><code class="nextra-code" dir="ltr">$state.raw</code></a> instead.

``` x:group
let nodes = $state.raw([
  {
    id: '1',
    position: { x: 0, y: 0 },
    data: { label: 'Hello' },
  },
  {
    id: '2',
    position: { x: 100, y: 100 },
    data: { label: 'World' },
  },
]);
```

Next, we <a href="https://svelte.dev/docs/svelte/$bindable" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">bind <img src="out_svelteflow/learn/getting-started/building-a-flow/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a> the nodes to the `SvelteFlow` component. This two-way binding allows both the component and your code to modify the nodes.

``` x:group
<SvelteFlow bind:nodes>
```

If you’ve followed these steps, you should see a flow that looks like this:

<img src="out_svelteflow/learn/getting-started/building-a-flow/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/learn/getting-started/building-a-flow/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

## Adding edges<a href="https://svelteflow.dev/learn/getting-started/building-a-flow#adding-edges" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

Let’s connect the nodes with an edge. Initialize a <a href="https://svelte.dev/docs/svelte/$state#$state.raw" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank"><code class="nextra-code" dir="ltr">$state.raw</code></a> with an array of <a href="https://svelteflow.dev/api-reference/types/edge" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]">edge objects</a> that have these **required** properties:

- `id`: A unique identifier for the edge
- `source`: The ID of the source node
- `target`: The ID of the target node

``` x:group
let edges = $state.raw([{ id: 'e1-2', source: '1', target: '2' }]);
```

As with nodes, we <a href="https://svelte.dev/docs/svelte/$bindable" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">bind <img src="out_svelteflow/learn/getting-started/building-a-flow/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a> the edges to the `SvelteFlow` component.

``` x:group
<SvelteFlow bind:nodes bind:edges>
```

Your flow should now look like this:

<img src="out_svelteflow/learn/getting-started/building-a-flow/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/learn/getting-started/building-a-flow/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

## Polishing the flow<a href="https://svelteflow.dev/learn/getting-started/building-a-flow#polishing-the-flow" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

This all might already go into the right drection but it still looks a little bland and lopsided, doesn’t it?

#### `fitView`<a href="https://svelteflow.dev/learn/getting-started/building-a-flow#fitview" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

Add the <a href="https://svelteflow.dev/api-reference/components/svelte-flow#fitview" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">fitView</code></a> prop to automatically fit the initial viewport to the visible nodes.

``` x:group
<SvelteFlow bind:nodes bind:edges fitView>
```

#### Built-in node types<a href="https://svelteflow.dev/learn/getting-started/building-a-flow#built-in-node-types" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

Let’s change the `type` of the first node to `input` and the second node to `output`. These are built-in node types, that come with a different set of handles.

``` x:group
let nodes = $state.raw([
  {
    id: '1',
    type: 'input',
    position: { x: 0, y: 0 },
    data: { label: 'Hello' },
  },
  {
    id: '2',
    type: 'output',
    position: { x: 100, y: 100 },
    data: { label: 'World' },
  },
]);
```

#### Built-in edge types<a href="https://svelteflow.dev/learn/getting-started/building-a-flow#built-in-edge-types" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

We change the edge to type `smoothstep` and also give it a label!

``` x:group
let edges = $state.raw([
  { id: 'e1-2', source: '1', target: '2', type: 'smoothstep', label: 'to the' },
]);
```

## Finished Flow<a href="https://svelteflow.dev/learn/getting-started/building-a-flow#finished-flow" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

And there you have it! Your completed flow should look like this:

<img src="out_svelteflow/learn/getting-started/building-a-flow/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/learn/getting-started/building-a-flow/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />
