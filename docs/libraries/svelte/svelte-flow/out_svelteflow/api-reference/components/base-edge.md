# \<BaseEdge /\>

<a href="https://github.com/xyflow/xyflow/blob/main/packages/svelte/src/lib/components/BaseEdge/BaseEdge.svelte" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">Source on GitHub <img src="out_svelteflow/api-reference/components/base-edge/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a>

The `<BaseEdge />` component gets used internally for all the edges. It can be used inside a custom edge and handles the invisible helper edge and the edge label for you.

<img src="out_svelteflow/api-reference/components/base-edge/index_media/0b7eab85abc3682cb59a01670459912dfc0cfbe8.svg" class="x:max-w-6 x:shrink-0" />CustomEdge.svelte

<img src="out_svelteflow/api-reference/components/base-edge/index_media/4ad055c8513380c64c1f5319faccf453f477bb38.svg" class="nextra-copy-icon" />

``` x:group
<script lang="ts">
  import { BaseEdge, getBezierPath, type EdgeProps } from '@xyflow/svelte';
 
  let { sourceX, sourceY, targetX, targetY, ...props } : EdgeProps = $props();
 
  const [edgePath] = $derived(getBezierPath({
    sourceX,
    sourceY,
    targetX,
    targetY,
  }));
 
  const {
    markerStart,
    markerEnd,
    interactionWidth,
    label,
    labelStyle,
  } = props;
</script>
 
<BaseEdge
  path={edgePath}
  {markerStart}
  {markerEnd}
  {interactionWidth}
  {label}
  {labelStyle}
/>
```

## Props<a href="https://svelteflow.dev/api-reference/components/base-edge#props" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

The type for props of `<BaseEdge />` component is exported as `BaseEdgeProps`. Additionally, it extends the props of `<svg />`.

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
<tr id="markerstart" class="x:rounded-xl nextra-border x:hover:bg-primary-50 x:dark:hover:bg-primary-500/10 x:group x:mb-5 x:max-lg:block x:max-lg:border x:lg:border-b x:lg:not-target:[&amp;&gt;td&gt;a]:opacity-0">
<td class="x:relative x:max-lg:block x:py-3 x:max-lg:px-3"><a href="https://svelteflow.dev/api-reference/components/base-edge#markerstart" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a><code class="nextra-code x:max-md:break-all x:after:content-[&quot;?&quot;]" dir="ltr">markerStart</code></td>
<td class="x:p-3 x:max-lg:block x:max-lg:before:content-[&quot;Type:_&quot;]"><code class="nextra-code" dir="ltr">string</code>
<p>The id of the SVG marker to use at the start of the edge. This should be defined in a <code class="nextra-code" dir="ltr">&lt;defs&gt;</code> element in a separate SVG document or element. Use the format “url(#markerId)” where markerId is the id of your marker definition.</p></td>
<td class="x:max-lg:block x:lg:after:content-[&quot;–&quot;]"></td>
</tr>
<tr id="markerend" class="x:rounded-xl nextra-border x:hover:bg-primary-50 x:dark:hover:bg-primary-500/10 x:group x:mb-5 x:max-lg:block x:max-lg:border x:lg:border-b x:lg:not-target:[&amp;&gt;td&gt;a]:opacity-0">
<td class="x:relative x:max-lg:block x:py-3 x:max-lg:px-3"><a href="https://svelteflow.dev/api-reference/components/base-edge#markerend" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a><code class="nextra-code x:max-md:break-all x:after:content-[&quot;?&quot;]" dir="ltr">markerEnd</code></td>
<td class="x:p-3 x:max-lg:block x:max-lg:before:content-[&quot;Type:_&quot;]"><code class="nextra-code" dir="ltr">string</code>
<p>The id of the SVG marker to use at the end of the edge. This should be defined in a <code class="nextra-code" dir="ltr">&lt;defs&gt;</code> element in a separate SVG document or element. Use the format “url(#markerId)” where markerId is the id of your marker definition.</p></td>
<td class="x:max-lg:block x:lg:after:content-[&quot;–&quot;]"></td>
</tr>
<tr id="interactionwidth" class="x:rounded-xl nextra-border x:hover:bg-primary-50 x:dark:hover:bg-primary-500/10 x:group x:mb-5 x:max-lg:block x:max-lg:border x:lg:border-b x:lg:not-target:[&amp;&gt;td&gt;a]:opacity-0">
<td class="x:relative x:max-lg:block x:py-3 x:max-lg:px-3"><a href="https://svelteflow.dev/api-reference/components/base-edge#interactionwidth" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a><code class="nextra-code x:max-md:break-all x:after:content-[&quot;?&quot;]" dir="ltr">interactionWidth</code></td>
<td class="x:p-3 x:max-lg:block x:max-lg:before:content-[&quot;Type:_&quot;]"><code class="nextra-code" dir="ltr">number</code>
<p>ReactFlow renders an invisible path around each edge to make them easier to click or tap on. This property sets the width of that invisible path.</p></td>
<td class="x:max-lg:block x:lg:after:content-[&quot;–&quot;]"></td>
</tr>
<tr id="label" class="x:rounded-xl nextra-border x:hover:bg-primary-50 x:dark:hover:bg-primary-500/10 x:group x:mb-5 x:max-lg:block x:max-lg:border x:lg:border-b x:lg:not-target:[&amp;&gt;td&gt;a]:opacity-0">
<td class="x:relative x:max-lg:block x:py-3 x:max-lg:px-3"><a href="https://svelteflow.dev/api-reference/components/base-edge#label" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a><code class="nextra-code x:max-md:break-all x:after:content-[&quot;?&quot;]" dir="ltr">label</code></td>
<td class="x:p-3 x:max-lg:block x:max-lg:before:content-[&quot;Type:_&quot;]"><code class="nextra-code" dir="ltr">string</code></td>
<td class="x:max-lg:block x:lg:after:content-[&quot;–&quot;]"></td>
</tr>
<tr id="labelstyle" class="x:rounded-xl nextra-border x:hover:bg-primary-50 x:dark:hover:bg-primary-500/10 x:group x:mb-5 x:max-lg:block x:max-lg:border x:lg:border-b x:lg:not-target:[&amp;&gt;td&gt;a]:opacity-0">
<td class="x:relative x:max-lg:block x:py-3 x:max-lg:px-3"><a href="https://svelteflow.dev/api-reference/components/base-edge#labelstyle" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a><code class="nextra-code x:max-md:break-all x:after:content-[&quot;?&quot;]" dir="ltr">labelStyle</code></td>
<td class="x:p-3 x:max-lg:block x:max-lg:before:content-[&quot;Type:_&quot;]"><code class="nextra-code" dir="ltr">string</code></td>
<td class="x:max-lg:block x:lg:after:content-[&quot;–&quot;]"></td>
</tr>
<tr id="path" class="x:rounded-xl nextra-border x:hover:bg-primary-50 x:dark:hover:bg-primary-500/10 x:group x:mb-5 x:max-lg:block x:max-lg:border x:lg:border-b x:lg:not-target:[&amp;&gt;td&gt;a]:opacity-0">
<td class="x:relative x:max-lg:block x:py-3 x:max-lg:px-3"><a href="https://svelteflow.dev/api-reference/components/base-edge#path" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a><code class="nextra-code x:max-md:break-all" dir="ltr">path</code></td>
<td class="x:p-3 x:max-lg:block x:max-lg:before:content-[&quot;Type:_&quot;]"><code class="nextra-code" dir="ltr">string</code>
<p>SVG path of the edge</p></td>
<td class="x:max-lg:block x:lg:after:content-[&quot;–&quot;]"></td>
</tr>
<tr id="labelx" class="x:rounded-xl nextra-border x:hover:bg-primary-50 x:dark:hover:bg-primary-500/10 x:group x:mb-5 x:max-lg:block x:max-lg:border x:lg:border-b x:lg:not-target:[&amp;&gt;td&gt;a]:opacity-0">
<td class="x:relative x:max-lg:block x:py-3 x:max-lg:px-3"><a href="https://svelteflow.dev/api-reference/components/base-edge#labelx" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a><code class="nextra-code x:max-md:break-all x:after:content-[&quot;?&quot;]" dir="ltr">labelX</code></td>
<td class="x:p-3 x:max-lg:block x:max-lg:before:content-[&quot;Type:_&quot;]"><code class="nextra-code" dir="ltr">number</code>
<p>The x coordinate of the label</p></td>
<td class="x:max-lg:block x:lg:after:content-[&quot;–&quot;]"></td>
</tr>
<tr id="labely" class="x:rounded-xl nextra-border x:hover:bg-primary-50 x:dark:hover:bg-primary-500/10 x:group x:mb-5 x:max-lg:block x:max-lg:border x:lg:border-b x:lg:not-target:[&amp;&gt;td&gt;a]:opacity-0">
<td class="x:relative x:max-lg:block x:py-3 x:max-lg:px-3"><a href="https://svelteflow.dev/api-reference/components/base-edge#labely" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a><code class="nextra-code x:max-md:break-all x:after:content-[&quot;?&quot;]" dir="ltr">labelY</code></td>
<td class="x:p-3 x:max-lg:block x:max-lg:before:content-[&quot;Type:_&quot;]"><code class="nextra-code" dir="ltr">number</code>
<p>The y coordinate of the label</p></td>
<td class="x:max-lg:block x:lg:after:content-[&quot;–&quot;]"></td>
</tr>
<tr id="props" class="x:rounded-xl nextra-border x:hover:bg-primary-50 x:dark:hover:bg-primary-500/10 x:group x:mb-5 x:max-lg:block x:max-lg:border x:lg:border-b x:lg:not-target:[&amp;&gt;td&gt;a]:opacity-0">
<td class="x:relative x:max-lg:block x:py-3 x:max-lg:px-3"><a href="https://svelteflow.dev/api-reference/components/base-edge#props" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a><code class="nextra-code x:max-md:break-all" dir="ltr">...props</code></td>
<td class="x:p-3 x:max-lg:block x:max-lg:before:content-[&quot;Type:_&quot;]"><code class="nextra-code" dir="ltr">Omit&lt;HTMLAttributes&lt;SVGPathElement&gt;, "markerStart" | "markerEnd" | "d" | "path"&gt;</code></td>
<td class="x:max-lg:block x:lg:after:content-[&quot;–&quot;]"></td>
</tr>
</tbody>
</table>

## Notes<a href="https://svelteflow.dev/api-reference/components/base-edge#notes" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

- If you want to use an edge marker with the `<BaseEdge />` component, you can pass the `markerStart` or `markerEnd` props passed to your custom edge through to the `<BaseEdge />` component. You can see all the props passed to a custom edge by looking at the <a href="https://svelteflow.dev/api-reference/types/edge-props" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">EdgeProps</code></a> type.
