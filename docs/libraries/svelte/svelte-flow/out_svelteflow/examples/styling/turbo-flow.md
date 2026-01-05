# Turbo Flow

Every part of the Svelte Flow UI is customizable. As the name implies the look is taken from the beautiful <a href="https://turbo.build/pack/docs/core-concepts#function-level-caching" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">turbo.build <img src="out_svelteflow/examples/styling/turbo-flow/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a> website. You can find more information about custom styles in the <a href="https://svelteflow.dev/learn/customization/theming" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]">theming guide</a>. (Svelte Flow uses Turborepo and we love it 💜)

App.svelte

FunctionIcon.svelte

TurboEdge.svelte

TurboNode.svelte

index.css

nodes-and-edges.ts

<img src="out_svelteflow/examples/styling/turbo-flow/index_media/a8615f5af2f285e6489e8b474f5d8c4d43e8c2bd.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/styling/turbo-flow/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/examples/styling/turbo-flow/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/styling/turbo-flow/index_media/824817a2c7b3c6650883ed0f84fc74ac655539f5.svg" class="size-4 fill-slate-700 stroke-slate-700" />

``` x:group
<script lang="ts">
  import { SvelteFlow, Controls, type Node, type Edge } from '@xyflow/svelte';
 
  import '@xyflow/svelte/dist/style.css';
 
  import { initialNodes, initialEdges } from './nodes-and-edges';
  import TurboNode from './TurboNode.svelte';
  import TurboEdge from './TurboEdge.svelte';
 
  let nodes = $state.raw<Node[]>(initialNodes);
  let edges = $state.raw<Edge[]>(initialEdges);
 
  const nodeTypes = {
    turbo: TurboNode,
  };
 
  const edgeTypes = {
    turbo: TurboEdge,
  };
 
  const defaultEdgeOptions = {
    type: 'turbo',
    markerEnd: 'edge-circle',
  };
</script>
 
<SvelteFlow bind:nodes {nodeTypes} bind:edges {edgeTypes} {defaultEdgeOptions} fitView>
  <Controls showLock={false} />
  <svg>
    <defs>
      <linearGradient id="edge-gradient">
        <stop offset="0%" stop-color="#ae53ba" />
        <stop offset="100%" stop-color="#2a8af6" />
      </linearGradient>
      <marker
        id="edge-circle"
        viewBox="-5 -5 10 10"
        refX="0"
        refY="0"
        markerUnits="strokeWidth"
        markerWidth="10"
        markerHeight="10"
        orient="auto"
      >
        <circle stroke="#2a8af6" stroke-opacity="0.75" r="2" cx="0" cy="0" />
      </marker>
    </defs>
  </svg>
</SvelteFlow>
```
