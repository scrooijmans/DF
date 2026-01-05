# Migrate to Svelte Flow 1.0

Svelte Flow 1.0 is built from the ground up with Svelte 5 and includes many new features and improvements. This guide will help you migrate from Svelte Flow 0.x to 1.0.

<img src="out_svelteflow/learn/troubleshooting/migrate-to-v1/index_media/a7db0f47f02f813c5a264458a8e522bf6320ca5a.svg" class="x:mt-[.3em]" />

If you are looking for the Svelte Flow 0.x docs, please refer to <a href="https://legacy.svelteflow.dev" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">legacy.svelteflow.dev <img src="out_svelteflow/learn/troubleshooting/migrate-to-v1/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a>.

## New features<a href="https://svelteflow.dev/learn/troubleshooting/migrate-to-v1#new-features" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

- **<a href="https://svelteflow.dev/examples/edges/reconnect-edge" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]">Reconnect edges</a>**: You can reconnect your edges by using the new `<EdgeReconnectAnchor />` component. It can be used to add custom reconnection points on custom edges.
- **Keyboard navigation & A11y**: We added support for keyboard navigation and improved accessibility for screen readers. You can now tab through nodes and edges and move nodes with the arrow keys. Can be disabled via <a href="https://svelteflow.dev/api-reference/svelte-flow#disablekeyboarda11y" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><strong>disableKeyboardA11y</strong></a>
- **<a href="https://svelteflow.dev/examples/edges/click-connect" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]">Click connect</a>**: You can now create a new connection by clicking on a handle one by one.
- **<a href="https://svelteflow.dev/api-reference/components/viewport-portal" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]">Enhanced ViewportPortal</a>**: You can now decide if you want to render something below or above the nodes & edges in the viewport.
- **Improved <a href="https://svelteflow.dev/api-reference/hooks/use-svelte-flow#fitview" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]">fitView</a>**: We finetuned the `fitView` function to better work with dynamically added nodes.
- **colorModeSSR** prop: You can pass a fallback color mode for server side rendering when colorMode is set to ‘system’.
- <a href="https://svelteflow.dev/api-reference/svelte-flow#elevateNodesOnSelect" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><strong>elevateNodesOnSelect</strong></a> & <a href="https://svelteflow.dev/api-reference/svelte-flow#elevateEdgesOnSelect" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><strong>elevateEdgesOnSelect</strong></a>: Control if nodes & edges should be elevated via z-index when selected.
- <a href="https://svelteflow.dev/api-reference/svelte-flow#style-props" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><strong>noDragClass, noWheelClass, noPanClass</strong></a>: You can now modify the class name used to disable dragging, panning and zooming.
- <a href="https://svelteflow.dev/api-reference/svelte-flow#onselectionchange" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><strong>onselectionchange</strong></a> & <a href="https://svelteflow.dev/api-reference/hooks/use-on-selection-change" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><strong>useOnSelectionChange</strong></a>: You can now listen to selection changes via a callback

## Breaking changes<a href="https://svelteflow.dev/learn/troubleshooting/migrate-to-v1#breaking-changes" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

### `nodes` & `edges` are now using `$state.raw` instead of `writable`<a href="https://svelteflow.dev/learn/troubleshooting/migrate-to-v1#nodes--edges-are-now-using-stateraw-instead-of-writable" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

Svelte 5 introduces runes which are now getting used for nodes and edges.

**Old API**

``` x:group
const nodes = writable([...]);
const edges = writable([...]);
```

**New API**

``` x:group
let nodes = $state.raw([...]);
let edges = $state.raw([...]);
```

### Updating Nodes & Edges<a href="https://svelteflow.dev/learn/troubleshooting/migrate-to-v1#updating-nodes--edges" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

Previously it was possible to update single node properties. Theoretically, this would also be possible with `$state`, however the <a href="https://svelte.dev/playground/e6f804ba6da348bc8b6a0a13c59672cb?version=5.19.0" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">performance implications <img src="out_svelteflow/learn/troubleshooting/migrate-to-v1/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a> of this are unfortunately too great, so we opted to using `$state.raw`.

This means that `nodes` and `edges` are to be treated as immutable from now on. If you are making updates manually make sure you:

1.  create a new node/edge object, when updating a property.
2.  reassign the nodes/edges array (this was technically required before anyway)

``` x:group
nodes[0].position.x = 100; // won't work
 
const newNode = { ...nodes[0] };
newNode.position.x = 100;
nodes[0] = newNode; // not enough to trigger an update
nodes = [...nodes]; // this will make it work
 
nodes = nodes.map((node) => {
  if (node.id === '1') {
    return { ...node, position: { ...node.position, x: 100 } };
  }
  return node;
}); // also works
 
updateNode('1', (node) => ({
  ...node,
  position: { ...node.position, x: 100 },
})); // using the updateNode helper from useSvelteFlow
```

### `nodes` & `edges` need to be bound from `<SvelteFlow />`<a href="https://svelteflow.dev/learn/troubleshooting/migrate-to-v1#nodes--edges-need-to-be-bound-from-svelteflow-" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

**Old API**

``` x:group
<SvelteFlow {nodes} {edges} />
```

**New API**

``` x:group
<SvelteFlow bind:nodes bind:edges />
```

If `nodes` and `edges` live in a separate module, you can use <a href="https://svelte.dev/docs/svelte/bind#Function-bindings" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">function bindings <img src="out_svelteflow/learn/troubleshooting/migrate-to-v1/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a>.

``` x:group
// store.svelte.js
 
let nodes = $state.raw([...]);
let edges = $state.raw([...]);
 
export const getNodes = () => nodes;
export const getEdges = () => edges;
export const setNodes = (newNodes) => nodes = newNodes;
export const setEdges = (newEdges) => edges = newEdges;
```

``` x:group
// BaseComponent.svelte
 
<script>
  import { getNodes, getEdges, setNodes, setEdges } from 'store.svelte.js';
</script>
 
<SvelteFlow bind:nodes={getNodes, setNodes} bind:edges={getEdges, setEdges} />
```

### Custom Node & Edge Props<a href="https://svelteflow.dev/learn/troubleshooting/migrate-to-v1#custom-node--edge-props" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

This is by enlarge a general change in Svelte 5, but it does have quite a big impact on typing the props of Custom Nodes & Edges.

**Old API**

``` x:group
// CustomNode.svelte
 
type $$Props = NodeProps;
 
export let data: $$Props['data'];
export let position: $$Props['position'];
export let selected: $$Props['selected'];
```

**New API**

``` x:group
let { data, position, selected } : NodeProps = $props();
```

### Hooks<a href="https://svelteflow.dev/learn/troubleshooting/migrate-to-v1#hooks" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

Hooks now return reactive values instead of writables. Because `$state` values cannot be <a href="https://svelte.dev/docs/svelte/$state#Passing-state-into-functions" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">returned by functions directly <img src="out_svelteflow/learn/troubleshooting/migrate-to-v1/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a> we have to return an object with a `.current` property to keep reactivity. In this regard, we are <a href="https://svelte.dev/docs/svelte/svelte-reactivity#MediaQuery" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">following the official trend <img src="out_svelteflow/learn/troubleshooting/migrate-to-v1/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a> set by the Svelte library authors.

**Old API**

``` x:group
const edges = useEdges();
$: console.log(edges);
```

**New API**

``` x:group
const edges = useEdges();
$inspect(edges.current);
```

Note that in case of `useNodes`, `useEdges` and `useViewport` reassignments to `.current` work!

``` x:group
const nodes = useNodes();
 
function updateNodes() {
   nodes.current = [...]
}
```

### Binding the viewport<a href="https://svelteflow.dev/learn/troubleshooting/migrate-to-v1#binding-the-viewport" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

Binding the viewport now works natively in Svelte 5. You can either access the internal viewport or bind your very own viewport object to be used instead.

**Old API**

``` x:group
const viewport = writable<Viewport>({ x: 100, y: 100, zoom: 1.25 });
 
<SvelteFlow {viewport} />
```

**New API**

``` x:group
let viewport = $state < Viewport > { x: 100, y: 100, zoom: 1.25 };
 
<SvelteFlow bind:viewport />;
```

### Custom Connection Line<a href="https://svelteflow.dev/learn/troubleshooting/migrate-to-v1#custom-connection-line" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

Using a custom Connection Line was possible before by passing it to a <a href="https://svelte.dev/docs/svelte/legacy-slots" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">slot <img src="out_svelteflow/learn/troubleshooting/migrate-to-v1/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a>. In Svelte Flow 1.0 we introduced a new prop called `connectionLineComponent`for this.

**Old API**

``` x:group
<SvelteFlow {nodes} {edges}>
  <ConnectionLine slot="connectionLine" />
  <Background variant={BackgroundVariant.Lines} />
</SvelteFlow>
```

**New API**

``` x:group
<SvelteFlow {nodes} {edges} connectionLineComponent={ConnectionLine}>
  <Background variant={BackgroundVariant.Lines} />
</SvelteFlow>
```

### `onEdgeCreate` becomes `onbeforeconnect`<a href="https://svelteflow.dev/learn/troubleshooting/migrate-to-v1#onedgecreate-becomes-onbeforeconnect" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

`onedgecreate` was called before a new edge was created. This is now called `onbeforeconnect` to better align with events like <a href="https://svelteflow.dev/api-reference/svelte-flow#onbeforedelete" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">onbeforeconnect</code></a>.

**Old API**

``` x:group
<SvelteFlow
  {nodes}
  {edges}
  onEdgeCreate={(connection) => ({...connection, id: crypto.randomUUID()})}
/>
```

**New API**

``` x:group
<SvelteFlow
  bind:nodes
  bind:edges
  onbeforeconnect={(connection) => ({ ...connection, id: crypto.randomUUID() })}
/>
```

### `<EdgeLabelRenderer/>` becomes <a href="https://svelteflow.dev/api-reference/components/edge-label" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">&lt;EdgeLabel /&gt;</code></a><a href="https://svelteflow.dev/learn/troubleshooting/migrate-to-v1#edgelabelrenderer-becomes-edgelabel-" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

The `EdgeLabelRenderer` component is now called `EdgeLabel`. As it was just a simple Portal to begin with, the naming of it being a “renderer” was a bit misleading. To add to this, the new `EdgeLabel` component now also handles clicks on the label automatically and is aware of what edge it belongs to.

**Old API**

<img src="out_svelteflow/learn/troubleshooting/migrate-to-v1/index_media/5b0cb536cd682d27624b010ea8c901ce4ae78adc.svg" class="x:max-w-6 x:shrink-0" />CustomEdge.svelte

<img src="out_svelteflow/learn/troubleshooting/migrate-to-v1/index_media/4ad055c8513380c64c1f5319faccf453f477bb38.svg" class="nextra-copy-icon" />

``` x:group
<BaseEdge id={id} path={path} />
<EdgeLabelRenderer>
  <div
    style:transform={`translate(${labelX}px, ${labelY}px)`}
  >
    My Edge Label
  </div>
</EdgeLabelRenderer>
```

**New API**

<img src="out_svelteflow/learn/troubleshooting/migrate-to-v1/index_media/5b0cb536cd682d27624b010ea8c901ce4ae78adc.svg" class="x:max-w-6 x:shrink-0" />CustomEdge.svelte

<img src="out_svelteflow/learn/troubleshooting/migrate-to-v1/index_media/4ad055c8513380c64c1f5319faccf453f477bb38.svg" class="nextra-copy-icon" />

``` x:group
<BaseEdge path={path} />
<EdgeLabel x={labelX} y={labelY} selectEdgeOnClick>
  <div>My Edge Label</div>
</EdgeLabel>
```
