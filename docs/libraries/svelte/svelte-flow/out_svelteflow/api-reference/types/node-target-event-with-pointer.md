# NodeTargetEventWithPointer

<a href="https://github.com/xyflow/xyflow/blob/main/packages/svelte/src/lib/types/events.ts#20" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">Source on GitHub <img src="out_svelteflow/api-reference/types/node-target-event-with-pointer/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a>

The `NodeTargetEventWithPointer` type represents an event that occurs during target node interactions, including pointer position and event details. It extends the base node event with pointer-specific information and target node data.

``` x:group
type NodeTargetEventWithPointer<NodeType extends Node = Node> = {
  event: PointerEvent;
  node: NodeType;
  targetNode: NodeType;
};
```

**Parameters:**

| Name | Type | Default |
|----|----|----|
| <a href="https://svelteflow.dev/api-reference/types/node-target-event-with-pointer#__0" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a>`__0` | `{ targetNode: `<a href="https://svelteflow.dev/api-reference/types/node" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">NodeType</code></a>` | null; nodes: `<a href="https://svelteflow.dev/api-reference/types/node" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">NodeType</code></a>`[]; event: T; }` |  |

**Returns:**

<a href="https://svelteflow.dev/api-reference/types/node-target-event-with-pointer#returns" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)]"></a>`void`
