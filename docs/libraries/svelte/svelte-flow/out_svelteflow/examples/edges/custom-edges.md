# Custom Edges

This example shows how to create and use custom edge types in Svelte Flow. You can define your own edge components with custom rendering, animations, and interactions. Custom edges are useful for creating specialized visualizations or implementing complex edge behaviors.

App.svelte

BiDirectionalEdge.svelte

BiDirectionalNode.svelte

ButtonEdge.svelte

SelfConnectingEdge.svelte

xy-theme.css

index.css

nodes-and-edges.ts

<img src="out_svelteflow/examples/edges/custom-edges/index_media/a8615f5af2f285e6489e8b474f5d8c4d43e8c2bd.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/edges/custom-edges/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/examples/edges/custom-edges/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/edges/custom-edges/index_media/824817a2c7b3c6650883ed0f84fc74ac655539f5.svg" class="size-4 fill-slate-700 stroke-slate-700" />

``` x:group
<script lang="ts">
  import {
    SvelteFlow,
    Background,
    ConnectionMode,
    Controls,
    type Node,
    type Edge,
  } from '@xyflow/svelte';
 
  import '@xyflow/svelte/dist/style.css';
 
  import { initialNodes, initialEdges } from './nodes-and-edges';
 
  import ButtonEdge from './ButtonEdge.svelte';
  import BiDirectionalEdge from './BiDirectionalEdge.svelte';
  import BiDirectionalNode from './BiDirectionalNode.svelte';
  import SelfConnectingEdge from './SelfConnectingEdge.svelte';
 
  let nodes = $state.raw<Node[]>(initialNodes);
  let edges = $state.raw<Edge[]>(initialEdges);
 
  const nodeTypes = {
    bidirectional: BiDirectionalNode,
  };
 
  const edgeTypes = {
    buttonedge: ButtonEdge,
    bidirectional: BiDirectionalEdge,
    selfconnecting: SelfConnectingEdge,
  };
</script>
 
<SvelteFlow
  bind:nodes
  {nodeTypes}
  bind:edges
  {edgeTypes}
  connectionMode={ConnectionMode.Loose}
  fitView
>
  <Controls />
  <Background />
</SvelteFlow>
```
