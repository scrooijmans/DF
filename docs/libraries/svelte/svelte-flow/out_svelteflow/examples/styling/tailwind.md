# Tailwind CSS

This example shows how to integrate Tailwind CSS with Svelte Flow. Learn how to use Tailwind’s utility classes to style nodes, edges, and other flow components while maintaining a consistent design system across your application.

App.svelte

CustomNode.svelte

index.css

nodes-and-edges.ts

tailwind.config.js

<img src="out_svelteflow/examples/styling/tailwind/index_media/a8615f5af2f285e6489e8b474f5d8c4d43e8c2bd.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/styling/tailwind/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/examples/styling/tailwind/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/styling/tailwind/index_media/824817a2c7b3c6650883ed0f84fc74ac655539f5.svg" class="size-4 fill-slate-700 stroke-slate-700" />

``` x:group
<script lang="ts">
  import { SvelteFlow, Controls, MiniMap, type Node, type Edge } from '@xyflow/svelte';
  import '@xyflow/svelte/dist/style.css';
 
  import { initialNodes, initialEdges } from './nodes-and-edges';
  import CustomNode from './CustomNode.svelte';
 
  let nodes = $state.raw<Node[]>(initialNodes);
  let edges = $state.raw<Edge[]>(initialEdges);
 
  const nodeTypes = {
    custom: CustomNode,
  };
</script>
 
<SvelteFlow bind:nodes {nodeTypes} bind:edges fitView class="bg-[#f7f9fB]">
  <MiniMap />
  <Controls />
</SvelteFlow>
```
