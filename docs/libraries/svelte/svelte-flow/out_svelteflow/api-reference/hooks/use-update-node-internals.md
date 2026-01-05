# useUpdateNodeInternals()

<a href="https://github.com/xyflow/xyflow/blob/main/packages/svelte/src/lib/hooks/useUpdateNodeInternals.svelte.ts/#L6" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">Source on GitHub <img src="out_svelteflow/api-reference/hooks/use-update-node-internals/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a>

When you programmatically add or remove handles to a node or update a node’s handle position, you need to inform Svelte Flow about it by using this hook. This will update the internal dimensions of the node and properly reposition handles on the canvas if necessary.

``` x:group
<script lang="ts">
  import { Handle, useUpdateNodeInternals } from '@xyflow/svelte';
 
  const updateNodeInternals = useUpdateNodeInternals();
</script>
```

## Signature<a href="https://svelteflow.dev/api-reference/hooks/use-update-node-internals#signature" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

**Parameters:**

This function does not accept any parameters.

**Returns:**

<a href="https://svelteflow.dev/api-reference/hooks/use-update-node-internals#returns" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)]"></a>`(nodeId?: string | string[] | undefined) => void`

A function for telling Svelte Flow to update the internal state of one or more nodes that you have changed programmatically.
