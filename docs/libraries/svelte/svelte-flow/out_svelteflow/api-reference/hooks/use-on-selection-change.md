# useOnSelectionChange()

<a href="https://github.com/xyflow/xyflow/blob/main/packages/svelte/src/lib/hooks/useOnSelectionChange.svelte.ts" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">Source on GitHub <img src="out_svelteflow/api-reference/hooks/use-on-selection-change/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a>

This hook lets you listen for changes to both node and edge selection. As the name implies, the callback you provide will be called whenever the selection of *either* nodes or edges changes.

<img src="out_svelteflow/api-reference/hooks/use-on-selection-change/index_media/0b7eab85abc3682cb59a01670459912dfc0cfbe8.svg" class="x:max-w-6 x:shrink-0" />Component.svelte

<img src="out_svelteflow/api-reference/hooks/use-on-selection-change/index_media/4ad055c8513380c64c1f5319faccf453f477bb38.svg" class="nextra-copy-icon" />

``` x:group
<script>
  import { useOnSelectionChange } from '@xyflow/svelte';
 
  let selectedNodes = $state.raw([]);
  let selectedEdges = $state.raw([]);
 
  useOnSelectionChange(({ nodes, edges }) => {
    selectedNodes = nodes.map((node) => node.id);
    selectedEdges = edges.map((edge) => edge.id);
  });
</script>
 
<div>
    <p>Selected nodes: {selectedNodes.join(', ')}</p>
    <p>Selected edges: {selectedEdges.join(', ')}</p>
</div>
```

## Signature<a href="https://svelteflow.dev/api-reference/hooks/use-on-selection-change#signature" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

**Parameters:**

| Name | Type | Default |
|----|----|----|
| <a href="https://svelteflow.dev/api-reference/hooks/use-on-selection-change#onselectionchange" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a>`onselectionchange` | `OnSelectionChange` |  |

**Returns:**

<a href="https://svelteflow.dev/api-reference/hooks/use-on-selection-change#returns" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)]"></a>`void`

## Notes<a href="https://svelteflow.dev/api-reference/hooks/use-on-selection-change#notes" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

- This hook can only be used in a component that is a child of a <a href="https://svelteflow.dev/api-reference/svelte-flow-provider" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">&lt;SvelteFlowProvider /&gt;</code></a> or a <a href="https://svelteflow.dev/api-reference/svelte-flow" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">&lt;SvelteFlow /&gt;</code></a> component.
