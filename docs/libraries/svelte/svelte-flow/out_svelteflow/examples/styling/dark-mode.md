# Dark Mode

This example demonstrates how to implement dark mode in Svelte Flow. You can customize the appearance of nodes, edges, and the background to create a cohesive dark theme that matches your application’s design system.

App.svelte

index.css

nodes-and-edges.ts

<img src="out_svelteflow/examples/styling/dark-mode/index_media/a8615f5af2f285e6489e8b474f5d8c4d43e8c2bd.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/styling/dark-mode/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/examples/styling/dark-mode/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/styling/dark-mode/index_media/824817a2c7b3c6650883ed0f84fc74ac655539f5.svg" class="size-4 fill-slate-700 stroke-slate-700" />

``` x:group
<script lang="ts">
  import {
    SvelteFlow,
    Background,
    Controls,
    MiniMap,
    Panel,
    type Node,
    type Edge,
    type ColorMode,
  } from '@xyflow/svelte';
 
  import '@xyflow/svelte/dist/style.css';
 
  import { initialNodes, initialEdges } from './nodes-and-edges';
 
  let nodes = $state.raw<Node[]>(initialNodes);
  let edges = $state.raw<Edge[]>(initialEdges);
 
  let colorMode: ColorMode = $state('system');
</script>
 
<SvelteFlow bind:nodes bind:edges {colorMode} fitView>
  <Background />
  <Controls />
  <MiniMap />
 
  <Panel>
    <select bind:value={colorMode}>
      <option value="dark">dark</option>
      <option value="light">light</option>
      <option value="system">system</option>
    </select>
  </Panel>
</SvelteFlow>
```
