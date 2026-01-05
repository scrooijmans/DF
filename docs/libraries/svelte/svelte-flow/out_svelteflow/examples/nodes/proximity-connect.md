# Proximity Connect

This example shows how to automatically create edges when a node is dropped in close proximity to another one. While dragging, a dotted connection line is displayed to show which edge will be created if you drop the node.

App.svelte

xy-theme.css

index.css

nodes-and-edges.ts

<img src="out_svelteflow/examples/nodes/proximity-connect/index_media/a8615f5af2f285e6489e8b474f5d8c4d43e8c2bd.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/nodes/proximity-connect/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/examples/nodes/proximity-connect/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/nodes/proximity-connect/index_media/824817a2c7b3c6650883ed0f84fc74ac655539f5.svg" class="size-4 fill-slate-700 stroke-slate-700" />

``` x:group
<script lang="ts">
  import { SvelteFlow, Background, type Node, type Edge } from '@xyflow/svelte';
 
  import '@xyflow/svelte/dist/style.css';
 
  import { initialNodes, initialEdges } from './nodes-and-edges';
 
  let nodes = $state.raw<Node[]>(initialNodes);
  let edges = $state.raw<Edge[]>(initialEdges);
 
  const MIN_DISTANCE = 150;
 
  function getClosestEdge(node: Node, nodes: Node[]) {
    const closestNode = nodes.reduce(
      (res, n) => {
        if (n.id !== node.id) {
          const dx = n.position.x - node.position.x;
          const dy = n.position.y - node.position.y;
          const d = Math.sqrt(dx * dx + dy * dy);
 
          if (d < res.distance && d < MIN_DISTANCE) {
            res.distance = d;
            res.node = n;
          }
        }
 
        return res;
      },
      {
        distance: Number.MAX_VALUE,
        node: null,
      },
    );
 
    if (!closestNode.node) {
      return null;
    }
 
    const closeNodeIsSource = closestNode.node.position.x < node.position.x;
 
    return {
      id: closeNodeIsSource
        ? `${node.id}-${closestNode.node.id}`
        : `${closestNode.node.id}-${node.id}`,
      source: closeNodeIsSource ? closestNode.node.id : node.id,
      target: closeNodeIsSource ? node.id : closestNode.node.id,
      class: 'temp',
    };
  }
 
  function onNodeDrag({ targetNode: node }) {
    const closestEdge = getClosestEdge(node, nodes);
 
    let edgeAlreadyExists = false;
    edges.forEach((edge, i) => {
      if (edgeAlreadyExists) {
        return;
      }
 
      if (closestEdge) {
        // non-temporary edge already exists
        if (
          edge.source === closestEdge.source &&
          edge.target === closestEdge.target
        ) {
          edgeAlreadyExists = true;
          return;
        }
 
        if (edge.class !== 'temp') {
          return;
        }
 
        if (
          edge.source !== closestEdge.source ||
          edge.target !== closestEdge.target
        ) {
          edges[i] = closestEdge; // replace the edge
          edgeAlreadyExists = true;
        }
      } else if (edge.class === 'temp') {
        edges.splice(i, 1); // remove edge
      }
    });
 
    if (closestEdge && !edgeAlreadyExists) {
      edges.push(closestEdge);
    }
 
    edges = edges;
  }
 
  function onNodeDragStop() {
    edges = edges.map((edge) => {
      if (edge.class === 'temp') {
        return { ...edge, class: '' };
      }
      return edge;
    });
  }
</script>
 
<SvelteFlow
  bind:nodes
  bind:edges
  fitView
  onnodedrag={onNodeDrag}
  onnodedragstop={onNodeDragStop}
>
  <Background />
</SvelteFlow>
```
