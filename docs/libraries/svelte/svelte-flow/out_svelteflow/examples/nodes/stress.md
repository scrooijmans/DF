# Stress Test

You doubt we can render a lot of nodes and edges? See for yourself.

App.svelte

xy-theme.css

index.css

utils.js

<img src="out_svelteflow/examples/nodes/stress/index_media/a8615f5af2f285e6489e8b474f5d8c4d43e8c2bd.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/nodes/stress/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/examples/nodes/stress/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/nodes/stress/index_media/824817a2c7b3c6650883ed0f84fc74ac655539f5.svg" class="size-4 fill-slate-700 stroke-slate-700" />

``` x:group
<script>
  import { SvelteFlow, Background, Controls, MiniMap } from '@xyflow/svelte';
  import { createNodesAndEdges } from './utils';
 
  import '@xyflow/svelte/dist/style.css';
 
  const { nodes: initialNodes, edges: initialEdges } = createNodesAndEdges(
    15,
    30,
  );
 
  let nodes = $state.raw(initialNodes);
  let edges = $state.raw(initialEdges);
 
  function updatePos() {
    nodes = nodes.map((node) => {
      return {
        ...node,
        position: {
          x: Math.random() * 1500,
          y: Math.random() * 1500,
        },
      };
    });
  }
</script>
 
<SvelteFlow bind:nodes bind:edges minZoom={0} fitView>
  <Background />
  <MiniMap />
  <Controls />
  <button onclick={updatePos} class="scramble-button"> change pos </button>
</SvelteFlow>
 
<style>
  .scramble-button {
    position: absolute;
    right: 10px;
    top: 30px;
    z-index: 4;
  }
</style>
```
