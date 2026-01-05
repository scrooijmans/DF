# Reconnect Edge

You can make edges reconnectable by using the `<EdgeReconnectAnchor />` component. This component is used to create a reconnection point on your custom edges. They behave similar to handles:

1.  You can start dragging on an `<EdgeReconnectAnchor />`
2.  This starts a new connection process and from the oppsite side of the edge
3.  You can finish the connection the same way as it had been started from a handle

App.svelte

CustomEdge.svelte

xy-theme.css

index.css

nodes-and-edges.ts

<img src="out_svelteflow/examples/edges/reconnect-edge/index_media/a8615f5af2f285e6489e8b474f5d8c4d43e8c2bd.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/edges/reconnect-edge/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/examples/edges/reconnect-edge/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/edges/reconnect-edge/index_media/824817a2c7b3c6650883ed0f84fc74ac655539f5.svg" class="size-4 fill-slate-700 stroke-slate-700" />

``` x:group
<script lang="ts">
  import {
    SvelteFlow,
    Background,
    type Node,
    type Edge,
    type EdgeTypes,
  } from '@xyflow/svelte';
 
  import '@xyflow/svelte/dist/style.css';
 
  import { initialNodes, initialEdges } from './nodes-and-edges';
 
  import CustomEdge from './CustomEdge.svelte';
 
  let nodes = $state.raw<Node[]>(initialNodes);
  let edges = $state.raw<Edge[]>(initialEdges);
 
  const edgeTypes: EdgeTypes = {
    custom: CustomEdge,
  };
</script>
 
<SvelteFlow bind:nodes bind:edges {edgeTypes} fitView>
  <Background />
</SvelteFlow>
```
