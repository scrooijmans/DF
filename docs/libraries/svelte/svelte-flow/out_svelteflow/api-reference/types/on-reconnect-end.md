# OnReconnectEnd

<a href="https://github.com/xyflow/xyflow/blob/main/packages/svelte/src/lib/types/general.ts#L89" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">Source on GitHub <img src="out_svelteflow/api-reference/types/on-reconnect-end/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a>

The `OnReconnectEnd` type is a callback that fires when the reconnection process for an edge is completed or canceled. It receives the triggering event, the edge, the handle type, and the final connection state.

``` x:group
type OnReconnectEnd<EdgeType extends EdgeBase = EdgeBase> = (
  event: MouseEvent | TouchEvent,
  edge: EdgeType,
  handleType: HandleType,
  connectionState: FinalConnectionState,
) => void;
```

**Parameters:**

| Name | Type | Default |
|----|----|----|
| <a href="https://svelteflow.dev/api-reference/types/on-reconnect-end#event" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a>`event` | <a href="https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank"><code class="nextra-code" dir="ltr">MouseEvent</code></a>` | `<a href="https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank"><code class="nextra-code" dir="ltr">TouchEvent</code></a> |  |
| <a href="https://svelteflow.dev/api-reference/types/on-reconnect-end#edge" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a>`edge` | <a href="https://svelteflow.dev/api-reference/types/edge" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">EdgeType</code></a> |  |
| <a href="https://svelteflow.dev/api-reference/types/on-reconnect-end#handletype" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a>`handleType` | `'source' | 'target'` |  |
| <a href="https://svelteflow.dev/api-reference/types/on-reconnect-end#connectionstate" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a>`connectionState` | `FinalConnectionState<InternalNodeBase>` |  |

**Returns:**

<a href="https://svelteflow.dev/api-reference/types/on-reconnect-end#returns" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)]"></a>`void`
