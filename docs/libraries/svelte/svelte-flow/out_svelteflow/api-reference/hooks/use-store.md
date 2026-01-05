# useStore()

<a href="https://github.com/xyflow/xyflow/blob/main/packages/svelte/src/lib/store/index.ts/" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">Source on GitHub <img src="out_svelteflow/api-reference/hooks/use-store/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a>

This hook can be used to access the internal store of the Svelte Flow.

<img src="out_svelteflow/api-reference/hooks/use-store/index_media/a7db0f47f02f813c5a264458a8e522bf6320ca5a.svg" class="x:mt-[.3em]" />

This hook is only needed for advanced use cases. It should only be used if there is no other way to access the internal state. For many of the common use cases, there are dedicated hooks available such as <a href="https://svelteflow.dev/api-reference/hooks/use-connection" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">useConnection</code></a>, <a href="https://svelteflow.dev/api-reference/hooks/use-nodes" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">useNodes</code></a>, etc.

``` x:group
<script lang="ts">
  import { useStore } from '@xyflow/svelte';
 
  // lots of props that you pass to the <SvelteFlow> component end up in the internal store.
  // Here we are accessing the current connectionMode.
  const { connectionMode } = useStore();
</script>
```

## Signature<a href="https://svelteflow.dev/api-reference/hooks/use-store#signature" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

**Parameters:**

This function does not accept any parameters.

**Returns:**

<a href="https://svelteflow.dev/api-reference/hooks/use-store#returns" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)]"></a><a href="https://svelteflow.dev/api-reference/types/svelte-flow-store" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">SvelteFlowStore</code></a>`<`<a href="https://svelteflow.dev/api-reference/types/node" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">NodeType</code></a>`, `<a href="https://svelteflow.dev/api-reference/types/edge" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">EdgeType</code></a>`>`
