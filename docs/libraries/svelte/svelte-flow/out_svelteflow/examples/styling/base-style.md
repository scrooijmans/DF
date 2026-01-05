# Base Style

This example shows how to customize the base styling of Svelte Flow components. You can modify the appearance of nodes, edges, handles, and the background to match your application’s design. Learn how to use CSS variables and custom styles to create a consistent look and feel.

App.svelte

index.css

nodes-and-edges.ts

<img src="out_svelteflow/examples/styling/base-style/index_media/a8615f5af2f285e6489e8b474f5d8c4d43e8c2bd.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/styling/base-style/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/examples/styling/base-style/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/styling/base-style/index_media/824817a2c7b3c6650883ed0f84fc74ac655539f5.svg" class="size-4 fill-slate-700 stroke-slate-700" />

``` x:group
<script lang="ts">
  import { SvelteFlow, Background, type Node, type Edge } from '@xyflow/svelte';
 
  import '@xyflow/svelte/dist/base.css';
 
  import { initialNodes, initialEdges } from './nodes-and-edges';
 
  let nodes = $state.raw<Node[]>(initialNodes);
  let edges = $state.raw<Edge[]>(initialEdges);
</script>
 
<SvelteFlow bind:nodes bind:edges fitView>
  <Background />
</SvelteFlow>
```
