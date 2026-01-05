# useNodesInitialized()

<a href="https://github.com/xyflow/xyflow/blob/main/packages/svelte/src/lib/hooks/useInitialized.svelte.ts" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">Source on GitHub <img src="out_svelteflow/api-reference/hooks/use-nodes-initialized/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a>

This hook can be used to check if all nodes are initialized. It returns a signal with a boolean.

``` x:group
<script lang="ts">
  import { useNodesInitialized } from '@xyflow/svelte';
 
  const useNodesInitialized = useNodesInitialized();
</script>
```

## Signature<a href="https://svelteflow.dev/api-reference/hooks/use-nodes-initialized#signature" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

**Parameters:**

This function does not accept any parameters.

**Returns:**

A boolean that indicates if nodes are initialized

| Name | Type |
|----|----|
| <a href="https://svelteflow.dev/api-reference/hooks/use-nodes-initialized#current" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a>`current` | `boolean` |
