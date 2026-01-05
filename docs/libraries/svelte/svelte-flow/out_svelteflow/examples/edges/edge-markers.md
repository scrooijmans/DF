# Edge Markers

This example shows how to add markers to the start and end of edges in Svelte Flow. Edge markers are useful for indicating direction, type, or status of connections. You can use SVG markers to create custom arrowheads or other visual indicators.

App.svelte

CustomEdgeMarker.svelte

xy-theme.css

index.css

nodes-and-edges.ts

<img src="out_svelteflow/examples/edges/edge-markers/index_media/a8615f5af2f285e6489e8b474f5d8c4d43e8c2bd.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/edges/edge-markers/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/examples/edges/edge-markers/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/edges/edge-markers/index_media/824817a2c7b3c6650883ed0f84fc74ac655539f5.svg" class="size-4 fill-slate-700 stroke-slate-700" />

``` x:group
<script lang="ts">
  import { SvelteFlow, Background, type Edge, type Node } from '@xyflow/svelte';
 
  import '@xyflow/svelte/dist/style.css';
 
  import { initialNodes, initialEdges } from './nodes-and-edges';
 
  // the custom edge marker is an SVG with a marker element.
  // That marker has the id "logo". We can use that id to reference it,
  // by using the markerStart and markerEnd edge options.
  import CustomEdgeMarker from './CustomEdgeMarker.svelte';
 
  let nodes = $state.raw<Node[]>(initialNodes);
  let edges = $state.raw<Edge[]>(initialEdges);
</script>
 
<SvelteFlow bind:nodes bind:edges fitView>
  <CustomEdgeMarker />
  <Background />
</SvelteFlow>
```
