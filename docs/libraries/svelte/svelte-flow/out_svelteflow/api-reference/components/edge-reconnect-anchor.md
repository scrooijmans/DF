# \<EdgeReconnectAnchor /\>

If you want a way to make your edges updatable, you can use the `<EdgeReconnectAnchor />` component. This component is used to create a reconnection point on your custom edges. They behave similar to handles:

1.  You can start dragging on an `<EdgeReconnectAnchor />`
2.  This starts a new connection process and from the oppsite side of the edge
3.  You can finish the connection the same way as it had been started from a handle

<img src="out_svelteflow/api-reference/components/edge-reconnect-anchor/index_media/0b7eab85abc3682cb59a01670459912dfc0cfbe8.svg" class="x:max-w-6 x:shrink-0" />CustomEdge.svelte

<img src="out_svelteflow/api-reference/components/edge-reconnect-anchor/index_media/4ad055c8513380c64c1f5319faccf453f477bb38.svg" class="nextra-copy-icon" />

``` x:group
<script lang="ts">
  import {
    BaseEdge,
    EdgeReconnectAnchor,
    getBezierPath,
    type EdgeProps,
  } from '@xyflow/svelte';
 
  let { sourceX, sourceY, targetX, targetY, selected, data, ...props }: EdgeProps = $props();
 
  const [edgePath] = $derived(getBezierPath({
    sourceX,
    sourceY,
    targetX,
    targetY,
  }));
 
  let reconnecting = $state(false);
</script>
 
<!-- We want to hide the initial edge while reconnecting -->
{#if !reconnecting}
  <BaseEdge path={edgePath} {...props} />
{/if}
 
<!-- We only want to be able to reconnect when an edge is selected  -->
{#if selected}
  <EdgeReconnectAnchor
    bind:reconnecting
    type="source"
    position={{ x: sourceX, y: sourceY }}
  />
  <EdgeReconnectAnchor
    bind:reconnecting
    type="target"
    position={{ x: targetX, y: targetY }}
  />
{/if}
```

This example renders invisible reconnection points. Naturally, you can also render an icon inside the `<EdgeReconnectAnchor />` component.

## Props<a href="https://svelteflow.dev/api-reference/components/edge-reconnect-anchor#props" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

The type for props of `<EdgeReconnectAnchor />` component is exported as `EdgeReconnectAnchorProps`. Additionally, it extends the props of `<div />`.

| Name | Type | Default |
|----|----|----|
| <a href="https://svelteflow.dev/api-reference/components/edge-reconnect-anchor#reconnecting" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a>`reconnecting` | `boolean` |  |
| <a href="https://svelteflow.dev/api-reference/components/edge-reconnect-anchor#type" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a>`type` | `'source' | 'target'` |  |
| <a href="https://svelteflow.dev/api-reference/components/edge-reconnect-anchor#position" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a>`position` | <a href="https://svelteflow.dev/api-reference/types/xy-position" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">XYPosition</code></a> |  |
| <a href="https://svelteflow.dev/api-reference/components/edge-reconnect-anchor#size" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a>`size` | `number` |  |
| <a href="https://svelteflow.dev/api-reference/components/edge-reconnect-anchor#dragthreshold" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a>`dragThreshold` | `number` |  |
| <a href="https://svelteflow.dev/api-reference/components/edge-reconnect-anchor#props" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a>`...props` | `HTMLAttributes<HTMLDivElement>` |  |
