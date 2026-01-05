# Hello World

This is a basic example that shows how to get started with Svelte Flow. Learn how to create a simple flow with nodes and edges, handle basic interactions, and understand the core concepts of Svelte Flow.

App.svelte

xy-theme.css

index.css

<img src="out_svelteflow/examples/misc/hello-world/index_media/a8615f5af2f285e6489e8b474f5d8c4d43e8c2bd.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/misc/hello-world/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/examples/misc/hello-world/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/misc/hello-world/index_media/824817a2c7b3c6650883ed0f84fc74ac655539f5.svg" class="size-4 fill-slate-700 stroke-slate-700" />

``` x:group
<script lang="ts">
  import { SvelteFlow, Controls, Background, MiniMap } from '@xyflow/svelte';
 
  import '@xyflow/svelte/dist/style.css';
 
  let nodes = $state.raw([
    {
      id: '1',
      data: { label: 'Hello' },
      position: { x: 0, y: 0 },
    },
    {
      id: '2',
      data: { label: 'World' },
      position: { x: 100, y: 100 },
    },
  ]);
 
  let edges = $state.raw([
    {
      id: '1-2',
      source: '1',
      target: '2',
    },
  ]);
</script>
 
<SvelteFlow bind:nodes bind:edges fitView>
  <Controls />
  <Background />
  <MiniMap />
</SvelteFlow>
```
