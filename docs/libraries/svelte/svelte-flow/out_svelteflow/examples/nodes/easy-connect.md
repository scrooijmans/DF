# Easy Connect

Fed up with tiny little connection handles? Make your whole node act as one! Keep in mind though that you need to define separate drag handles in this case to still be able to drag the node.

App.svelte

CustomNode.svelte

FloatingEdge.svelte

xy-theme.css

index.css

nodes-and-edges.ts

utils.ts

<img src="out_svelteflow/examples/nodes/easy-connect/index_media/a8615f5af2f285e6489e8b474f5d8c4d43e8c2bd.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/nodes/easy-connect/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/examples/nodes/easy-connect/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/nodes/easy-connect/index_media/824817a2c7b3c6650883ed0f84fc74ac655539f5.svg" class="size-4 fill-slate-700 stroke-slate-700" />

``` x:group
<script lang="ts">
  import {
    ConnectionLineType,
    MarkerType,
    SvelteFlow,
    Background,
    type Node,
    type Edge,
  } from '@xyflow/svelte';
 
  import CustomNode from './CustomNode.svelte';
 
  import '@xyflow/svelte/dist/style.css';
 
  import { initialNodes, initialEdges } from './nodes-and-edges';
  import FloatingEdge from './FloatingEdge.svelte';
 
  let nodes = $state.raw<Node[]>(initialNodes);
  let edges = $state.raw<Edge[]>(initialEdges);
 
  const nodeTypes = {
    custom: CustomNode,
  };
 
  const edgeTypes = {
    floating: FloatingEdge,
  };
 
  const defaultEdgeOptions = {
    type: 'floating',
    markerEnd: {
      type: MarkerType.ArrowClosed,
    },
  };
</script>
 
<SvelteFlow
  bind:nodes
  {nodeTypes}
  bind:edges
  {edgeTypes}
  {defaultEdgeOptions}
  connectionLineType={ConnectionLineType.Straight}
  fitView
>
  <Background />
</SvelteFlow>
```
