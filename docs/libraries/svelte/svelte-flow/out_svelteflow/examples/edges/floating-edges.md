# Floating Edges

This example demonstrates how to create floating edges that connect nodes to the viewport or other fixed points. Floating edges are useful for creating connections to external elements or implementing special edge behaviors that don’t require a target node.

App.svelte

CustomNode.svelte

SimpleFloatingEdge.svelte

xy-theme.css

index.css

nodes-and-edges.ts

utils.ts

<img src="out_svelteflow/examples/edges/floating-edges/index_media/a8615f5af2f285e6489e8b474f5d8c4d43e8c2bd.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/edges/floating-edges/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/examples/edges/floating-edges/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/edges/floating-edges/index_media/824817a2c7b3c6650883ed0f84fc74ac655539f5.svg" class="size-4 fill-slate-700 stroke-slate-700" />

``` x:group
<script lang="ts">
  import {
    SvelteFlow,
    Background,
    ConnectionMode,
    type Node,
    type Edge,
  } from '@xyflow/svelte';
 
  import '@xyflow/svelte/dist/style.css';
 
  import { initialNodes, initialEdges } from './nodes-and-edges';
  import CustomNode from './CustomNode.svelte';
  import SimpleFloatingEdge from './SimpleFloatingEdge.svelte';
 
  let nodes = $state.raw<Node[]>(initialNodes);
  let edges = $state.raw<Edge[]>(initialEdges);
 
  const nodeTypes = {
    custom: CustomNode,
  };
 
  const edgeTypes = {
    floating: SimpleFloatingEdge,
  };
</script>
 
<SvelteFlow
  bind:nodes
  {nodeTypes}
  bind:edges
  {edgeTypes}
  fitView
  connectionMode={ConnectionMode.Loose}
>
  <Background />
</SvelteFlow>
```
