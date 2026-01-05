# getNodesBounds()

<a href="https://github.com/xyflow/xyflow/blob/main/packages/system/src/utils/graph.ts/#L195-L229" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">Source on GitHub <img src="out_svelteflow/api-reference/utils/get-nodes-bounds/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a>

Returns the bounding box that contains all the given nodes in an array. This can be useful when combined with <a href="https://svelteflow.dev/api-reference/utils/get-viewport-for-bounds" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">getViewportForBounds</code></a> to calculate the correct transform to fit the given nodes in a viewport.

<img src="out_svelteflow/api-reference/utils/get-nodes-bounds/index_media/a7db0f47f02f813c5a264458a8e522bf6320ca5a.svg" class="x:mt-[.3em]" />

This function was previously called `getRectOfNodes`, which will be removed in v12.

``` x:group
import { getNodesBounds } from '@xyflow/svelte';
 
let nodes = $state.raw([
  {
    id: 'a',
    position: { x: 0, y: 0 },
    data: { label: 'a' },
    width: 50,
    height: 25,
  },
  {
    id: 'b',
    position: { x: 100, y: 100 },
    data: { label: 'b' },
    width: 50,
    height: 25,
  },
]);
 
const bounds = getNodesBounds(nodes.value);
```

## Signature<a href="https://svelteflow.dev/api-reference/utils/get-nodes-bounds#signature" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

**Parameters:**

<table class="x:my-8 x:w-full x:text-sm">
<colgroup>
<col style="width: 33%" />
<col style="width: 33%" />
<col style="width: 33%" />
</colgroup>
<thead class="nextra-border x:border-b x:text-left x:max-lg:hidden">
<tr>
<th class="x:py-1.5">Name</th>
<th class="x:p-1.5 x:px-3">Type</th>
<th class="x:py-1.5">Default</th>
</tr>
</thead>
<tbody>
<tr id="nodes" class="x:rounded-xl nextra-border x:hover:bg-primary-50 x:dark:hover:bg-primary-500/10 x:group x:mb-5 x:max-lg:block x:max-lg:border x:lg:border-b x:lg:not-target:[&amp;&gt;td&gt;a]:opacity-0">
<td class="x:relative x:max-lg:block x:py-3 x:max-lg:px-3"><a href="https://svelteflow.dev/api-reference/utils/get-nodes-bounds#nodes" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a><code class="nextra-code x:max-md:break-all" dir="ltr">nodes</code></td>
<td class="x:p-3 x:max-lg:block x:max-lg:before:content-[&quot;Type:_&quot;]"><code class="nextra-code" dir="ltr">(string | </code><a href="https://svelteflow.dev/api-reference/types/node" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">NodeType</code></a><code class="nextra-code" dir="ltr"> | InternalNodeBase&lt;</code><a href="https://svelteflow.dev/api-reference/types/node" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">NodeType</code></a><code class="nextra-code" dir="ltr">&gt;)[]</code>
<p>Nodes to calculate the bounds for.</p></td>
<td class="x:max-lg:block x:lg:after:content-[&quot;–&quot;]"></td>
</tr>
<tr id="paramsnodeorigin" class="x:rounded-xl nextra-border x:hover:bg-primary-50 x:dark:hover:bg-primary-500/10 x:group x:mb-5 x:max-lg:block x:max-lg:border x:lg:border-b x:lg:not-target:[&amp;&gt;td&gt;a]:opacity-0">
<td class="x:relative x:max-lg:block x:py-3 x:max-lg:px-3"><a href="https://svelteflow.dev/api-reference/utils/get-nodes-bounds#paramsnodeorigin" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a><code class="nextra-code x:max-md:break-all x:after:content-[&quot;?&quot;]" dir="ltr">params.nodeOrigin</code></td>
<td class="x:p-3 x:max-lg:block x:max-lg:before:content-[&quot;Type:_&quot;]"><a href="https://svelteflow.dev/api-reference/types/node-origin" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">NodeOrigin</code></a>
<p>Origin of the nodes: <code class="nextra-code" dir="ltr">[0, 0]</code> for top-left, <code class="nextra-code" dir="ltr">[0.5, 0.5]</code> for center.</p></td>
<td class="x:max-lg:block x:py-3 x:max-lg:pt-0 x:max-lg:px-3 x:max-lg:before:content-[&quot;Default:_&quot;]"><code class="nextra-code x:whitespace-pre-wrap x:inline-block" dir="ltr">[0, 0]</code></td>
</tr>
<tr id="paramsnodelookup" class="x:rounded-xl nextra-border x:hover:bg-primary-50 x:dark:hover:bg-primary-500/10 x:group x:mb-5 x:max-lg:block x:max-lg:border x:lg:border-b x:lg:not-target:[&amp;&gt;td&gt;a]:opacity-0">
<td class="x:relative x:max-lg:block x:py-3 x:max-lg:px-3"><a href="https://svelteflow.dev/api-reference/utils/get-nodes-bounds#paramsnodelookup" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a><code class="nextra-code x:max-md:break-all x:after:content-[&quot;?&quot;]" dir="ltr">params.nodeLookup</code></td>
<td class="x:p-3 x:max-lg:block x:max-lg:before:content-[&quot;Type:_&quot;]"><code class="nextra-code" dir="ltr">NodeLookup&lt;InternalNodeBase&lt;</code><a href="https://svelteflow.dev/api-reference/types/node" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">NodeType</code></a><code class="nextra-code" dir="ltr">&gt;&gt;</code></td>
<td class="x:max-lg:block x:lg:after:content-[&quot;–&quot;]"></td>
</tr>
</tbody>
</table>

**Returns:**

<a href="https://svelteflow.dev/api-reference/utils/get-nodes-bounds#returns" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)]"></a><a href="https://svelteflow.dev/api-reference/types/rect" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">Rect</code></a>

Bounding box enclosing all nodes.
