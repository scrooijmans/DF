# \<EdgeLabel /\>

<a href="https://github.com/xyflow/xyflow/blob/main/packages/svelte/src/lib/components/EdgeLabel/EdgeLabel.svelte" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">Source on GitHub <img src="out_svelteflow/api-reference/components/edge-label/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a>

The `<EdgeLabel />` component is used in your custom edges to display an edge label that selects the edge when it is clicked.

<img src="out_svelteflow/api-reference/components/edge-label/index_media/0b7eab85abc3682cb59a01670459912dfc0cfbe8.svg" class="x:max-w-6 x:shrink-0" />CustomEdge.svelte

<img src="out_svelteflow/api-reference/components/edge-label/index_media/4ad055c8513380c64c1f5319faccf453f477bb38.svg" class="nextra-copy-icon" />

``` x:group
<script lang="ts">
  import { BaseEdge, getBezierPath, type EdgeProps } from '@xyflow/svelte';
 
  let {
    label,
    labelStyle,
    sourceX,
    sourceY,
    sourcePosition,
    targetX,
    targetY,
    targetPosition
  } : EdgeProps = $props();
 
  let [path, labelX, labelY] = $derived(getBezierPath({
    sourceX,
    sourceY,
    targetX,
    targetY,
    sourcePosition,
    targetPosition,
    curvature: pathOptions?.curvature
  }));
</script>
 
<BaseEdge
  {path}
/>
 
{#if label}
  <EdgeLabel x={labelX} y={labelY} style={labelStyle}>
    {label}
  </EdgeLabel>
{/if}
```

## Props<a href="https://svelteflow.dev/api-reference/components/edge-label#props" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

The type for props of `<EdgeLabel />` component is exported as `EdgeLabelProps`. Additionally, it extends the props of `<div />`.

| Name | Type | Default |
|----|----|----|
| <a href="https://svelteflow.dev/api-reference/components/edge-label#x" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a>`x` | `number` |  |
| <a href="https://svelteflow.dev/api-reference/components/edge-label#y" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a>`y` | `number` |  |
| <a href="https://svelteflow.dev/api-reference/components/edge-label#width" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a>`width` | `number` |  |
| <a href="https://svelteflow.dev/api-reference/components/edge-label#height" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a>`height` | `number` |  |
| <a href="https://svelteflow.dev/api-reference/components/edge-label#selectedgeonclick" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a>`selectEdgeOnClick` | `boolean` |  |
| <a href="https://svelteflow.dev/api-reference/components/edge-label#transparent" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a>`transparent` | `boolean` |  |
| <a href="https://svelteflow.dev/api-reference/components/edge-label#props" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a>`...props` | `HTMLAttributes<HTMLDivElement>` |  |
