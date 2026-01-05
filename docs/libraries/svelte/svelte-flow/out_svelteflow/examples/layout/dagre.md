# Dagre Layout

This example shows how to use <a href="https://github.com/dagrejs/dagre" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">Dagre <img src="out_svelteflow/examples/layout/dagre/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a> for automatic hierarchical graph layout in Svelte Flow. Dagre is a JavaScript library that can automatically arrange nodes in a hierarchical structure, making it perfect for creating tree-like diagrams and flowcharts.

App.svelte

xy-theme.css

index.css

nodes-and-edges.ts

<img src="out_svelteflow/examples/layout/dagre/index_media/a8615f5af2f285e6489e8b474f5d8c4d43e8c2bd.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/layout/dagre/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/examples/layout/dagre/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/layout/dagre/index_media/824817a2c7b3c6650883ed0f84fc74ac655539f5.svg" class="size-4 fill-slate-700 stroke-slate-700" />

``` x:group
<script lang="ts">
  import dagre from '@dagrejs/dagre';
  import {
    SvelteFlow,
    Background,
    Position,
    ConnectionLineType,
    Panel,
    type Node,
    type Edge,
  } from '@xyflow/svelte';
 
  import '@xyflow/svelte/dist/style.css';
 
  import { initialNodes, initialEdges } from './nodes-and-edges';
 
  const dagreGraph = new dagre.graphlib.Graph();
  dagreGraph.setDefaultEdgeLabel(() => ({}));
 
  const nodeWidth = 172;
  const nodeHeight = 36;
 
  function getLayoutedElements(nodes: Node[], edges: Edge[], direction = 'TB') {
    const isHorizontal = direction === 'LR';
    dagreGraph.setGraph({ rankdir: direction });
 
    nodes.forEach((node) => {
      dagreGraph.setNode(node.id, { width: nodeWidth, height: nodeHeight });
    });
 
    edges.forEach((edge) => {
      dagreGraph.setEdge(edge.source, edge.target);
    });
 
    dagre.layout(dagreGraph);
 
    const layoutedNodes = nodes.map((node) => {
      const nodeWithPosition = dagreGraph.node(node.id);
      node.targetPosition = isHorizontal ? Position.Left : Position.Top;
      node.sourcePosition = isHorizontal ? Position.Right : Position.Bottom;
 
      // We are shifting the dagre node position (anchor=center center) to the top left
      // so it matches the React Flow node anchor point (top left).
      return {
        ...node,
        position: {
          x: nodeWithPosition.x - nodeWidth / 2,
          y: nodeWithPosition.y - nodeHeight / 2,
        },
      };
    });
 
    return { nodes: layoutedNodes, edges };
  }
 
  const { nodes: layoutedNodes, edges: layoutedEdges } = getLayoutedElements(
    initialNodes,
    initialEdges,
  );
 
  let nodes = $state.raw<Node[]>(layoutedNodes);
  let edges = $state.raw<Edge[]>(layoutedEdges);
 
  function onLayout(direction: string) {
    const layoutedElements = getLayoutedElements(nodes, edges, direction);
 
    nodes = layoutedElements.nodes;
    edges = layoutedElements.edges;
  }
</script>
 
<SvelteFlow
  bind:nodes
  bind:edges
  fitView
  connectionLineType={ConnectionLineType.SmoothStep}
  defaultEdgeOptions={{ type: 'smoothstep', animated: true }}
>
  <Panel position="top-right">
    <button onclick={() => onLayout('TB')}>vertical layout</button>
    <button onclick={() => onLayout('LR')}>horizontal layout</button>
  </Panel>
  <Background />
</SvelteFlow>
```
