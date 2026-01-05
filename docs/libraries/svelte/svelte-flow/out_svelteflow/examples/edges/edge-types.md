# Edge Types

Svelte Flow comes with several built-in edge types: `default` (bezier), `straight`, `step`, and `smoothstep`. This example demonstrates how to use these different edge types and how to switch between them. Each edge type has its own characteristics and is suitable for different visualization needs.

App.svelte

xy-theme.css

index.css

<img src="out_svelteflow/examples/edges/edge-types/index_media/a8615f5af2f285e6489e8b474f5d8c4d43e8c2bd.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/edges/edge-types/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/examples/edges/edge-types/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/edges/edge-types/index_media/824817a2c7b3c6650883ed0f84fc74ac655539f5.svg" class="size-4 fill-slate-700 stroke-slate-700" />

``` x:group
<script lang="ts">
  import { SvelteFlow, Background, type Node, type Edge } from '@xyflow/svelte';
 
  import '@xyflow/svelte/dist/style.css';
 
  const initialNodes: Node[] = [
    {
      id: '1',
      data: { label: 'choose' },
      position: {
        x: 0,
        y: 0,
      },
    },
    {
      id: '2',
      data: { label: 'your' },
      position: {
        x: 100,
        y: 100,
      },
    },
    {
      id: '3',
      data: { label: 'desired' },
      position: {
        x: 0,
        y: 200,
      },
    },
    {
      id: '4',
      data: { label: 'edge' },
      position: {
        x: 100,
        y: 300,
      },
    },
    {
      id: '5',
      data: { label: 'type' },
      position: {
        x: 0,
        y: 400,
      },
    },
  ];
 
  const initialEdges: Edge[] = [
    {
      type: 'straight',
      source: '1',
      target: '2',
      id: '1',
      label: 'straight',
    },
    {
      type: 'step',
      source: '2',
      target: '3',
      id: '2',
      label: 'step',
    },
    {
      type: 'smoothstep',
      source: '3',
      target: '4',
      id: '3',
      label: 'smoothstep',
    },
    {
      type: 'bezier',
      source: '4',
      target: '5',
      id: '4',
      label: 'bezier',
    },
  ];
 
  let nodes = $state.raw<Node[]>(initialNodes);
  let edges = $state.raw<Edge[]>(initialEdges);
</script>
 
<SvelteFlow bind:nodes bind:edges fitView>
  <Background />
</SvelteFlow>
```
