# \<ViewportPortal /\>

<a href="https://github.com/xyflow/xyflow/blob/main/packages/react/src/components/ViewportPortal/index.tsx" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">Source on GitHub <img src="out_svelteflow/api-reference/components/viewport-portal/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a>

`<ViewportPortal />` component can be used to add components to the same viewport of the flow where nodes and edges are rendered. This is useful when you want to render your own components that are adhere to the same coordinate system as the nodes & edges and are also affected by zooming and panning

``` x:group
<script lang="ts">
import { ViewportPortal } from '@xyflow/svelte';
</script>
 
<ViewportPortal>
  <div style:transform="translate(100px, 100px)" style:position="absolute">
    This div is positioned at [100, 100] on the flow.
  </div>
</ViewportPortal>
```

You can also define if you want to render the component below or above the nodes and edges by using the `target` prop.

``` x:group
<script lang="ts">
import { ViewportPortal } from '@xyflow/svelte';
</script>
 
<ViewportPortal target="front">
  <div style:transform="translate(100px, 100px)" style:position="absolute">
    This div is positioned at [100, 100] on the flow.
  </div>
</ViewportPortal>
```

## Props<a href="https://svelteflow.dev/api-reference/components/viewport-portal#props" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

The type for props of `<ViewportPortal />` component is exported as `ViewportPortalProps`. Additionally, it extends the props of `<div />`.

| Name | Type | Default |
|----|----|----|
| <a href="https://svelteflow.dev/api-reference/components/viewport-portal#target" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a>`target` | `"front" | "back"` |  |
| <a href="https://svelteflow.dev/api-reference/components/viewport-portal#props" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a>`...props` | `HTMLAttributes<HTMLDivElement>` |  |
