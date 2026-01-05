# useNodeConnections()

<a href="https://github.com/xyflow/xyflow/blob/main/packages/svelte/src/lib/hooks/useNodeConnections.svelte.ts" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">Source on GitHub <img src="out_svelteflow/api-reference/hooks/use-node-connections/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a>

This hook returns an array of connections on a specific node, handle type (‘source’, ‘target’) or handle ID.

``` x:group
<script>
  import { useNodeConnections } from '@xyflow/svelte';
  const connections = useNodeConnections({ handleType: 'target', handleId: 'my-handle' });
</script>
 
<div>There are currently {connections.length} incoming connections!</div>
```

## Signature<a href="https://svelteflow.dev/api-reference/hooks/use-node-connections#signature" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

**Parameters:**

| Name | Type | Default |
|----|----|----|
| <a href="https://svelteflow.dev/api-reference/hooks/use-node-connections#0id" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a>`[0]?.id` | `string` |  |
| <a href="https://svelteflow.dev/api-reference/hooks/use-node-connections#0handletype" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a>`[0]?.handleType` | `'source' | 'target'` |  |
| <a href="https://svelteflow.dev/api-reference/hooks/use-node-connections#0handleid" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a>`[0]?.handleId` | `string` |  |
| <a href="https://svelteflow.dev/api-reference/hooks/use-node-connections#0onconnect" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a>`[0]?.onConnect` | `(connections: HandleConnection[]) => void` |  |
| <a href="https://svelteflow.dev/api-reference/hooks/use-node-connections#0ondisconnect" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a>`[0]?.onDisconnect` | `(connections: HandleConnection[]) => void` |  |

**Returns:**

An array with connections

| Name | Type |
|----|----|
| <a href="https://svelteflow.dev/api-reference/hooks/use-node-connections#current" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a>`current` | <a href="https://svelteflow.dev/api-reference/types/node-connection" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">NodeConnection</code></a>`[]` |
