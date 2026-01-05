# Computing Flows

This example shows how to compute and visualize different paths between nodes in your flow. You can use this to implement features like dependency analysis, path finding, or flow validation. The example demonstrates how to use Svelte Flow’s built-in utilities to calculate paths and highlight them in the flow.

App.svelte

ResultNode.svelte

TextNode.svelte

UppercaseNode.svelte

xy-theme.css

index.css

<img src="out_svelteflow/examples/interaction/computing-flows/index_media/a8615f5af2f285e6489e8b474f5d8c4d43e8c2bd.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/interaction/computing-flows/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/examples/interaction/computing-flows/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/interaction/computing-flows/index_media/824817a2c7b3c6650883ed0f84fc74ac655539f5.svg" class="size-4 fill-slate-700 stroke-slate-700" />

``` x:group
<script module>
  import TextNode, { type TextNodeType } from './TextNode.svelte';
  import UppercaseNode, { type UppercaseNodeType } from './UppercaseNode.svelte';
  import ResultNode, { type ResultNodeType } from './ResultNode.svelte';
 
  export type CustomNodes = TextNodeType | UppercaseNodeType | ResultNodeType;
 
  export function isTextNode(node: any): node is TextNodeType | UppercaseNodeType {
    return !node || !node.type
      ? false
      : node.type === 'text' || node.type === 'uppercase';
  }
</script>
 
<script lang="ts">
  import {
    SvelteFlow,
    Controls,
    Background,
    BackgroundVariant,
    MiniMap,
    type Node,
    type NodeTypes,
    type Edge,
  } from '@xyflow/svelte';
  import '@xyflow/svelte/dist/style.css';
 
  const nodeTypes: NodeTypes = {
    text: TextNode,
    uppercase: UppercaseNode,
    result: ResultNode,
  };
 
  let nodes = $state.raw<Node[]>([
    {
      id: '1',
      type: 'text',
      data: {
        text: 'hello',
      },
      position: { x: -100, y: -50 },
    },
    {
      id: '1a',
      type: 'uppercase',
      data: {},
      position: { x: 100, y: 0 },
    },
    {
      id: '2',
      type: 'text',
      data: {
        text: 'world',
      },
      position: { x: 0, y: 100 },
    },
 
    {
      id: '3',
      type: 'result',
      data: {},
      position: { x: 300, y: 50 },
    },
  ]);
 
  let edges = $state.raw<Edge[]>([
    {
      id: 'e1-1a',
      source: '1',
      target: '1a',
    },
    {
      id: 'e1a-3',
      source: '1a',
      target: '3',
    },
    {
      id: 'e2-3',
      source: '2',
      target: '3',
    },
  ]);
</script>
 
<main>
  <SvelteFlow bind:nodes bind:edges {nodeTypes} fitView>
    <Controls />
    <Background variant={BackgroundVariant.Dots} />
    <MiniMap />
  </SvelteFlow>
</main>
 
<style>
  main {
    height: 100vh;
    display: flex;
    flex-direction: column-reverse;
  }
</style>
```
