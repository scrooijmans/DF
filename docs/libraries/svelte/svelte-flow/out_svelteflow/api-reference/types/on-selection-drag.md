# OnSelectionDrag

<a href="https://github.com/xyflow/xyflow/blob/main/packages/svelte/src/lib/types/events.ts#L74" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">Source on GitHub <img src="out_svelteflow/api-reference/types/on-selection-drag/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a>

The `OnSelectionDrag` type is a callback function that is called when dragging a selection of nodes. It receives the mouse event and an array of the nodes being dragged.

``` x:group
type OnSelectionDrag<NodeType extends Node = Node> = (
  event: MouseEvent,
  nodes: NodeType[],
) => void;
```

**Parameters:**

| Name | Type | Default |
|----|----|----|
| <a href="https://svelteflow.dev/api-reference/types/on-selection-drag#event" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a>`event` | <a href="https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank"><code class="nextra-code" dir="ltr">MouseEvent</code></a> |  |
| <a href="https://svelteflow.dev/api-reference/types/on-selection-drag#nodes" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a>`nodes` | `NodeBase<`<a href="https://typescriptlang.org/docs/handbook/utility-types.html#recordkeys-type" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank"><code class="nextra-code" dir="ltr">Record</code></a>`<string, unknown>, string | undefined>[]` |  |

**Returns:**

<a href="https://svelteflow.dev/api-reference/types/on-selection-drag#returns" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)]"></a>`void`
