# Contextual Zoom

This example demonstrates how to implement contextual zooming in Svelte Flow. You can zoom to specific nodes or areas of the flow using the `fitView` function. This is useful for focusing on particular parts of your flow or implementing navigation features.

App.svelte

ZoomNode.svelte

xy-theme.css

index.css

nodes-and-edges.ts

<img src="out_svelteflow/examples/interaction/contextual-zoom/index_media/a8615f5af2f285e6489e8b474f5d8c4d43e8c2bd.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/interaction/contextual-zoom/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/examples/interaction/contextual-zoom/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/interaction/contextual-zoom/index_media/824817a2c7b3c6650883ed0f84fc74ac655539f5.svg" class="size-4 fill-slate-700 stroke-slate-700" />

``` x:group
<script lang="ts">
  import { SvelteFlow, Background, type Node, type Edge } from '@xyflow/svelte';
 
  import '@xyflow/svelte/dist/style.css';
 
  import { initialNodes, initialEdges } from './nodes-and-edges';
  import ZoomNode from './ZoomNode.svelte';
 
  let nodes = $state.raw<Node[]>(initialNodes);
  let edges = $state.raw<Edge[]>(initialEdges);
 
  const nodeTypes = {
    zoom: ZoomNode,
  };
</script>
 
<SvelteFlow bind:nodes {nodeTypes} bind:edges fitView>
  <Background />
</SvelteFlow>
```
