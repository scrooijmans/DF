# Threlte Flow

This example shows how to integrate Svelte Flow with <a href="https://threlte.xyz/" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">Threlte <img src="out_svelteflow/examples/misc/threlte-flow/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a>, a Svelte component library for Three.js. Learn how to create 3D visualizations of your flow graphs and add interactive 3D elements to your nodes and edges.

App.svelte

ColorPickerNode.svelte

NodeWrapper.svelte

SliderNode.svelte

SwitcherNode.svelte

ThrelteNode.svelte

ThrelteScene.svelte

index.css

nodes-and-edges.svelte.ts

tailwind.config.js

<img src="out_svelteflow/examples/misc/threlte-flow/index_media/a8615f5af2f285e6489e8b474f5d8c4d43e8c2bd.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/misc/threlte-flow/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/examples/misc/threlte-flow/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/misc/threlte-flow/index_media/824817a2c7b3c6650883ed0f84fc74ac655539f5.svg" class="size-4 fill-slate-700 stroke-slate-700" />

``` x:group
<script lang="ts">
  import { SvelteFlow, Background, type Node, type Edge } from '@xyflow/svelte';
 
  import '@xyflow/svelte/dist/style.css';
 
  import { initialNodes, initialEdges } from './nodes-and-edges.svelte';
  import ColorPickerNode from './ColorPickerNode.svelte';
  import SliderNode from './SliderNode.svelte';
  import ThrelteNode from './ThrelteNode.svelte';
  import SwitcherNode from './SwitcherNode.svelte';
 
  let nodes = $state.raw<Node[]>(initialNodes);
  let edges = $state.raw<Edge[]>(initialEdges);
 
  const nodeTypes = {
    colorpicker: ColorPickerNode,
    slider: SliderNode,
    hero: ThrelteNode,
    switcher: SwitcherNode,
  };
</script>
 
<SvelteFlow bind:nodes {nodeTypes} bind:edges fitView>
  <Background />
</SvelteFlow>
```
