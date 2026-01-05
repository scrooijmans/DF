# OnReconnect

<a href="https://github.com/xyflow/xyflow/blob/main/packages/svelte/src/lib/types/general.ts#L83" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">Source on GitHub <img src="out_svelteflow/api-reference/types/on-reconnect/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a>

The `OnReconnect` type is a callback function that is called when an existing edge is reconnected to a different node or handle. It receives the old edge and the new connection details.

``` x:group
type OnReconnect<EdgeType extends EdgeBase = EdgeBase> = (
  oldEdge: EdgeType,
  newConnection: Connection,
) => void;
```

**Parameters:**

| Name | Type | Default |
|----|----|----|
| <a href="https://svelteflow.dev/api-reference/types/on-reconnect#oldedge" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a>`oldEdge` | <a href="https://svelteflow.dev/api-reference/types/edge" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">EdgeType</code></a> |  |
| <a href="https://svelteflow.dev/api-reference/types/on-reconnect#newconnection" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a>`newConnection` | <a href="https://svelteflow.dev/api-reference/types/connection" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">Connection</code></a> |  |

**Returns:**

<a href="https://svelteflow.dev/api-reference/types/on-reconnect#returns" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)]"></a>`void`
