# Transitions

This example shows how to use <a href="https://svelte.dev/docs/svelte/svelte-transition" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">Svelte’s built-in transitions <img src="out_svelteflow/examples/misc/transitions/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a> to animate creating and removing nodes and edges. Since nodes and edges are rendered as normal DOM elements, you can apply any Svelte transition to them.

Note that you need to use the `|global` modifier with your transitions for them to work properly within Svelte Flow’s component structure.

App.svelte

CustomEdge.svelte

CustomNode.svelte

xy-theme.css

index.css

<img src="out_svelteflow/examples/misc/transitions/index_media/a8615f5af2f285e6489e8b474f5d8c4d43e8c2bd.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/misc/transitions/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/examples/misc/transitions/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/misc/transitions/index_media/824817a2c7b3c6650883ed0f84fc74ac655539f5.svg" class="size-4 fill-slate-700 stroke-slate-700" />

``` x:group
<script>
  import {
    SvelteFlow,
    Background,
    Controls,
    MiniMap,
    Panel,
    ConnectionLineType,
  } from '@xyflow/svelte';
 
  import '@xyflow/svelte/dist/style.css';
  import CustomNode from './CustomNode.svelte';
  import CustomEdge from './CustomEdge.svelte';
 
  const nodeTypes = {
    fadeInOut: CustomNode,
  };
 
  const edgeTypes = {
    fadeOut: CustomEdge,
  };
 
  const initialNodes = [
    {
      id: '1',
      type: 'fadeInOut',
      position: { x: 0, y: 0 },
      data: { label: 'Delete me' },
    },
    {
      id: '2',
      type: 'fadeInOut',
      position: { x: 300, y: 0 },
      data: { label: 'Delete me' },
    },
  ];
 
  const initialEdges = [
    {
      id: '1->2',
      type: 'fadeOut',
      source: '1',
      target: '2',
    },
  ];
 
  let nodes = $state.raw(initialNodes);
  let edges = $state.raw(initialEdges);
</script>
 
<SvelteFlow
  bind:nodes
  bind:edges
  {nodeTypes}
  {edgeTypes}
  connectionLineType={ConnectionLineType.SmoothStep}
  fitView
  defaultEdgeOptions={{
    type: 'fadeOut',
  }}
>
  <Background />
  <MiniMap />
  <Controls />
  <Panel position="top-left">
    <button
      class="xy-theme__button"
      onclick={() => {
        nodes = [
          ...nodes,
          {
            id: window.crypto.randomUUID(),
            type: 'fadeInOut',
            position: {
              x: Math.random() * 300,
              y: Math.random() * 200 - 100,
            },
            data: { label: 'New Node' },
          },
        ];
      }}>Add node</button
    >
  </Panel>
</SvelteFlow>
```
