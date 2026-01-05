# Intersections

The `useSvelteFlow` hook exports <a href="https://svelteflow.dev/api-reference/types/svelte-flow-instance#intersections" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]">helpers to check intersections</a> of nodes and areas. In this example you can drag a node and get a visual feedback when it intersects with another node.

App.svelte

Flow.svelte

xy-theme.css

index.css

nodes-and-edges.ts

<img src="out_svelteflow/examples/nodes/intersections/index_media/a8615f5af2f285e6489e8b474f5d8c4d43e8c2bd.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/nodes/intersections/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/examples/nodes/intersections/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/nodes/intersections/index_media/824817a2c7b3c6650883ed0f84fc74ac655539f5.svg" class="size-4 fill-slate-700 stroke-slate-700" />

``` x:group
<script lang="ts">
  import { SvelteFlowProvider } from '@xyflow/svelte';
  import Flow from './Flow.svelte';
</script>
 
<SvelteFlowProvider>
  <Flow />
</SvelteFlowProvider>
```
