# Connection Limit

This is an example of a custom node with a custom handle that can limit the amount of connections a handle can have using the `isConnectable` prop. You can use a boolean, a number (the number of max. connections the handle should have) or a callback function that returns a boolean as an arg for the `isConnectable` prop of the CustomHandle component.

App.svelte

CustomNode.svelte

xy-theme.css

index.css

nodes-and-edges.ts

<img src="out_svelteflow/examples/nodes/connection-limit/index_media/a8615f5af2f285e6489e8b474f5d8c4d43e8c2bd.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/nodes/connection-limit/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/examples/nodes/connection-limit/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/nodes/connection-limit/index_media/824817a2c7b3c6650883ed0f84fc74ac655539f5.svg" class="size-4 fill-slate-700 stroke-slate-700" />

``` x:group
<script lang="ts">
  import { SvelteFlow, Background, type Node, type Edge } from '@xyflow/svelte';
  import '@xyflow/svelte/dist/style.css';
 
  import { initialNodes, initialEdges } from './nodes-and-edges';
  import CustomNode from './CustomNode.svelte';
 
  let nodes = $state.raw<Node[]>(initialNodes);
  let edges = $state.raw<Edge[]>(initialEdges);
 
  const nodeTypes = {
    custom: CustomNode,
  };
</script>
 
<SvelteFlow bind:nodes bind:edges {nodeTypes} fitView>
  <Background />
</SvelteFlow>
```
