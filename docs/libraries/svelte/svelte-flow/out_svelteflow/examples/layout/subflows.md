# Subflows

This example shows how to implement subflows in Svelte Flow. Subflows allow you to create nested or grouped flows within your main flow, which is useful for organizing complex diagrams into logical sections or implementing hierarchical structures.

App.svelte

xy-theme.css

index.css

nodes-and-edges.ts

<img src="out_svelteflow/examples/layout/subflows/index_media/a8615f5af2f285e6489e8b474f5d8c4d43e8c2bd.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/layout/subflows/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/examples/layout/subflows/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/layout/subflows/index_media/824817a2c7b3c6650883ed0f84fc74ac655539f5.svg" class="size-4 fill-slate-700 stroke-slate-700" />

``` x:group
<script lang="ts">
  import {
    SvelteFlow,
    Controls,
    Background,
    BackgroundVariant,
    MiniMap,
    type Node,
    type Edge,
  } from '@xyflow/svelte';
 
  import { initialNodes, initialEdges } from './nodes-and-edges';
 
  import '@xyflow/svelte/dist/style.css';
 
  let nodes = $state.raw<Node[]>(initialNodes);
  let edges = $state.raw<Edge[]>(initialEdges);
</script>
 
<main>
  <SvelteFlow
    bind:nodes
    bind:edges
    fitView
    minZoom={0.1}
    maxZoom={2.5}
    defaultEdgeOptions={{ zIndex: 1 }}
  >
    <Controls />
    <Background variant={BackgroundVariant.Dots} />
    <MiniMap />
  </SvelteFlow>
</main>
 
<style>
  main {
    height: 100vh;
  }
 
  :global(.svelte-flow .svelte-flow__node.parent) {
    background-color: rgba(220, 220, 255, 0.4);
  }
</style>
```
