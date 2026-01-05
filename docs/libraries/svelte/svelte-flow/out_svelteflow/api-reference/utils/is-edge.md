# isEdge()

<a href="https://github.com/xyflow/xyflow/blob/v11/packages/core/src/utils/graph.ts/#L19" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">Source on GitHub <img src="out_svelteflow/api-reference/utils/is-edge/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a>

Test whether an object is usable as an <a href="https://svelteflow.dev/api-reference/types/edge" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">Edge</code></a>. In TypeScript this is a type guard that will narrow the type of whatever you pass in to <a href="https://svelteflow.dev/api-reference/types/edge" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">Edge</code></a> if it returns `true`.

``` x:group
import { isEdge } from '@xyflow/svelte';
 
const edge = {
  id: 'edge-a',
  source: 'a',
  target: 'b',
};
 
if (isEdge(edge)) {
  // ..
}
```

## Signature<a href="https://svelteflow.dev/api-reference/utils/is-edge#signature" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

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
<tr id="element" class="x:rounded-xl nextra-border x:hover:bg-primary-50 x:dark:hover:bg-primary-500/10 x:group x:mb-5 x:max-lg:block x:max-lg:border x:lg:border-b x:lg:not-target:[&amp;&gt;td&gt;a]:opacity-0">
<td class="x:relative x:max-lg:block x:py-3 x:max-lg:px-3"><a href="https://svelteflow.dev/api-reference/utils/is-edge#element" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a><code class="nextra-code x:max-md:break-all" dir="ltr">element</code></td>
<td class="x:p-3 x:max-lg:block x:max-lg:before:content-[&quot;Type:_&quot;]"><code class="nextra-code" dir="ltr">unknown</code>
<p>The element to test</p></td>
<td class="x:max-lg:block x:lg:after:content-[&quot;–&quot;]"></td>
</tr>
</tbody>
</table>

**Returns:**

<a href="https://svelteflow.dev/api-reference/utils/is-edge#returns" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)]"></a>`boolean`

A boolean indicating whether the element is an Edge
