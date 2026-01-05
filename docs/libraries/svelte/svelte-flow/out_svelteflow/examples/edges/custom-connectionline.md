# Custom Connection Line

This example demonstrates how to customize the connection line that appears while dragging from a handle. You can modify the appearance and behavior of the connection line by implementing a custom connection line component.

App.svelte

ConnectionLine.svelte

CustomNode.svelte

xy-theme.css

index.css

<img src="out_svelteflow/examples/edges/custom-connectionline/index_media/a8615f5af2f285e6489e8b474f5d8c4d43e8c2bd.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/edges/custom-connectionline/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/examples/edges/custom-connectionline/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/edges/custom-connectionline/index_media/824817a2c7b3c6650883ed0f84fc74ac655539f5.svg" class="size-4 fill-slate-700 stroke-slate-700" />

``` x:group
<script lang="ts">
  import {
    SvelteFlow,
    Background,
    type Edge,
    type Node,
    MiniMap,
  } from '@xyflow/svelte';
 
  import CustomNode from './CustomNode.svelte';
  import ConnectionLine from './ConnectionLine.svelte';
 
  import '@xyflow/svelte/dist/style.css';
 
  const nodeTypes = {
    custom: CustomNode,
  };
 
  let nodes = $state.raw<Node[]>([
    {
      id: 'connectionline-1',
      type: 'custom',
      data: { label: 'Node 1' },
      position: { x: 250, y: 5 },
    },
  ]);
 
  let edges = $state.raw<Edge[]>([]);
</script>
 
<SvelteFlow
  bind:nodes
  bind:edges
  {nodeTypes}
  fitView
  connectionLineComponent={ConnectionLine}
>
  <Background />
  <MiniMap />
</SvelteFlow>
```
