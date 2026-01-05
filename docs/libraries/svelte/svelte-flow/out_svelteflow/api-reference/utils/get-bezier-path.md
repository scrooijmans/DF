# getBezierPath()

<a href="https://github.com/xyflow/xyflow/blob/main/packages/system/src/utils/edges/bezier-edge.ts/#L95" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">Source on GitHub <img src="out_svelteflow/api-reference/utils/get-bezier-path/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a>

The `getBezierPath` util returns everything you need to render a bezier edge between two nodes.

``` x:group
import { Position, getBezierPath } from '@xyflow/svelte';
 
const source = { x: 0, y: 20 };
const target = { x: 150, y: 100 };
 
const [path, labelX, labelY, offsetX, offsetY] = getBezierPath({
  sourceX: source.x,
  sourceY: source.y,
  sourcePosition: Position.Right,
  targetX: target.x,
  targetY: target.y,
  targetPosition: Position.Left,
});
 
console.log(path); //=> "M0,20 C75,20 75,100 150,100"
console.log(labelX, labelY); //=> 75, 60
console.log(offsetX, offsetY); //=> 75, 40
```

## Signature<a href="https://svelteflow.dev/api-reference/utils/get-bezier-path#signature" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

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
<tr id="0sourcex" class="x:rounded-xl nextra-border x:hover:bg-primary-50 x:dark:hover:bg-primary-500/10 x:group x:mb-5 x:max-lg:block x:max-lg:border x:lg:border-b x:lg:not-target:[&amp;&gt;td&gt;a]:opacity-0">
<td class="x:relative x:max-lg:block x:py-3 x:max-lg:px-3"><a href="https://svelteflow.dev/api-reference/utils/get-bezier-path#0sourcex" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a><code class="nextra-code x:max-md:break-all" dir="ltr">[0].sourceX</code></td>
<td class="x:p-3 x:max-lg:block x:max-lg:before:content-[&quot;Type:_&quot;]"><code class="nextra-code" dir="ltr">number</code>
<p>The <code class="nextra-code" dir="ltr">x</code> position of the source handle.</p></td>
<td class="x:max-lg:block x:lg:after:content-[&quot;–&quot;]"></td>
</tr>
<tr id="0sourcey" class="x:rounded-xl nextra-border x:hover:bg-primary-50 x:dark:hover:bg-primary-500/10 x:group x:mb-5 x:max-lg:block x:max-lg:border x:lg:border-b x:lg:not-target:[&amp;&gt;td&gt;a]:opacity-0">
<td class="x:relative x:max-lg:block x:py-3 x:max-lg:px-3"><a href="https://svelteflow.dev/api-reference/utils/get-bezier-path#0sourcey" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a><code class="nextra-code x:max-md:break-all" dir="ltr">[0].sourceY</code></td>
<td class="x:p-3 x:max-lg:block x:max-lg:before:content-[&quot;Type:_&quot;]"><code class="nextra-code" dir="ltr">number</code>
<p>The <code class="nextra-code" dir="ltr">y</code> position of the source handle.</p></td>
<td class="x:max-lg:block x:lg:after:content-[&quot;–&quot;]"></td>
</tr>
<tr id="0sourceposition" class="x:rounded-xl nextra-border x:hover:bg-primary-50 x:dark:hover:bg-primary-500/10 x:group x:mb-5 x:max-lg:block x:max-lg:border x:lg:border-b x:lg:not-target:[&amp;&gt;td&gt;a]:opacity-0">
<td class="x:relative x:max-lg:block x:py-3 x:max-lg:px-3"><a href="https://svelteflow.dev/api-reference/utils/get-bezier-path#0sourceposition" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a><code class="nextra-code x:max-md:break-all x:after:content-[&quot;?&quot;]" dir="ltr">[0].sourcePosition</code></td>
<td class="x:p-3 x:max-lg:block x:max-lg:before:content-[&quot;Type:_&quot;]"><a href="https://svelteflow.dev/api-reference/types/position" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">Position</code></a>
<p>The position of the source handle.</p></td>
<td class="x:max-lg:block x:py-3 x:max-lg:pt-0 x:max-lg:px-3 x:max-lg:before:content-[&quot;Default:_&quot;]"><a href="https://svelteflow.dev/api-reference/types/position" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code x:whitespace-pre-wrap x:inline-block" dir="ltr">Position</code></a><code class="nextra-code x:whitespace-pre-wrap x:inline-block" dir="ltr">.Bottom</code></td>
</tr>
<tr id="0targetx" class="x:rounded-xl nextra-border x:hover:bg-primary-50 x:dark:hover:bg-primary-500/10 x:group x:mb-5 x:max-lg:block x:max-lg:border x:lg:border-b x:lg:not-target:[&amp;&gt;td&gt;a]:opacity-0">
<td class="x:relative x:max-lg:block x:py-3 x:max-lg:px-3"><a href="https://svelteflow.dev/api-reference/utils/get-bezier-path#0targetx" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a><code class="nextra-code x:max-md:break-all" dir="ltr">[0].targetX</code></td>
<td class="x:p-3 x:max-lg:block x:max-lg:before:content-[&quot;Type:_&quot;]"><code class="nextra-code" dir="ltr">number</code>
<p>The <code class="nextra-code" dir="ltr">x</code> position of the target handle.</p></td>
<td class="x:max-lg:block x:lg:after:content-[&quot;–&quot;]"></td>
</tr>
<tr id="0targety" class="x:rounded-xl nextra-border x:hover:bg-primary-50 x:dark:hover:bg-primary-500/10 x:group x:mb-5 x:max-lg:block x:max-lg:border x:lg:border-b x:lg:not-target:[&amp;&gt;td&gt;a]:opacity-0">
<td class="x:relative x:max-lg:block x:py-3 x:max-lg:px-3"><a href="https://svelteflow.dev/api-reference/utils/get-bezier-path#0targety" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a><code class="nextra-code x:max-md:break-all" dir="ltr">[0].targetY</code></td>
<td class="x:p-3 x:max-lg:block x:max-lg:before:content-[&quot;Type:_&quot;]"><code class="nextra-code" dir="ltr">number</code>
<p>The <code class="nextra-code" dir="ltr">y</code> position of the target handle.</p></td>
<td class="x:max-lg:block x:lg:after:content-[&quot;–&quot;]"></td>
</tr>
<tr id="0targetposition" class="x:rounded-xl nextra-border x:hover:bg-primary-50 x:dark:hover:bg-primary-500/10 x:group x:mb-5 x:max-lg:block x:max-lg:border x:lg:border-b x:lg:not-target:[&amp;&gt;td&gt;a]:opacity-0">
<td class="x:relative x:max-lg:block x:py-3 x:max-lg:px-3"><a href="https://svelteflow.dev/api-reference/utils/get-bezier-path#0targetposition" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a><code class="nextra-code x:max-md:break-all x:after:content-[&quot;?&quot;]" dir="ltr">[0].targetPosition</code></td>
<td class="x:p-3 x:max-lg:block x:max-lg:before:content-[&quot;Type:_&quot;]"><a href="https://svelteflow.dev/api-reference/types/position" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">Position</code></a>
<p>The position of the target handle.</p></td>
<td class="x:max-lg:block x:py-3 x:max-lg:pt-0 x:max-lg:px-3 x:max-lg:before:content-[&quot;Default:_&quot;]"><a href="https://svelteflow.dev/api-reference/types/position" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code x:whitespace-pre-wrap x:inline-block" dir="ltr">Position</code></a><code class="nextra-code x:whitespace-pre-wrap x:inline-block" dir="ltr">.Top</code></td>
</tr>
<tr id="0curvature" class="x:rounded-xl nextra-border x:hover:bg-primary-50 x:dark:hover:bg-primary-500/10 x:group x:mb-5 x:max-lg:block x:max-lg:border x:lg:border-b x:lg:not-target:[&amp;&gt;td&gt;a]:opacity-0">
<td class="x:relative x:max-lg:block x:py-3 x:max-lg:px-3"><a href="https://svelteflow.dev/api-reference/utils/get-bezier-path#0curvature" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a><code class="nextra-code x:max-md:break-all x:after:content-[&quot;?&quot;]" dir="ltr">[0].curvature</code></td>
<td class="x:p-3 x:max-lg:block x:max-lg:before:content-[&quot;Type:_&quot;]"><code class="nextra-code" dir="ltr">number</code>
<p>The curvature of the bezier edge.</p></td>
<td class="x:max-lg:block x:py-3 x:max-lg:pt-0 x:max-lg:px-3 x:max-lg:before:content-[&quot;Default:_&quot;]"><code class="nextra-code x:whitespace-pre-wrap x:inline-block" dir="ltr">0.25</code></td>
</tr>
</tbody>
</table>

**Returns:**

<a href="https://svelteflow.dev/api-reference/utils/get-bezier-path#returns" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)]"></a>`[path: string, labelX: number, labelY: number, offsetX: number, offsetY: number]`

A path string you can use in an SVG, the `labelX` and `labelY` position (center of path) and `offsetX`, `offsetY` between source handle and label.

- `path`: the path to use in an SVG `<path>` element.
- `labelX`: the `x` position you can use to render a label for this edge.
- `labelY`: the `y` position you can use to render a label for this edge.
- `offsetX`: the absolute difference between the source `x` position and the `x` position of the middle of this path.
- `offsetY`: the absolute difference between the source `y` position and the `y` position of the middle of this path.

## Notes<a href="https://svelteflow.dev/api-reference/utils/get-bezier-path#notes" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

- This function returns a tuple (aka a fixed-size array) to make it easier to work with multiple edge paths at once.
