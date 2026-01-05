# Updating Nodes

You can update properties of nodes and edges freely as long as you pass a newly created `nodes` or `edges` array to `SvelteFlow`.

<img src="out_svelteflow/examples/nodes/update-node/index_media/a7db0f47f02f813c5a264458a8e522bf6320ca5a.svg" class="x:mt-[.3em]" />

You have to create a new `data` object on a node to notify Svelte Flow about data changes.

  

App.svelte

xy-theme.css

index.css

<img src="out_svelteflow/examples/nodes/update-node/index_media/a8615f5af2f285e6489e8b474f5d8c4d43e8c2bd.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/nodes/update-node/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/examples/nodes/update-node/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/nodes/update-node/index_media/824817a2c7b3c6650883ed0f84fc74ac655539f5.svg" class="size-4 fill-slate-700 stroke-slate-700" />

``` x:group
<script lang="ts">
  import { SvelteFlow, Background, type Edge, type Node, Panel } from '@xyflow/svelte';
 
  import '@xyflow/svelte/dist/style.css';
 
  const initialNodes: Node[] = [
    { id: '1', data: { label: '-' }, position: { x: 100, y: 100 } },
    { id: '2', data: { label: 'Node 2' }, position: { x: 100, y: 200 } },
  ];
 
  const initialEdges: Edge[] = [{ id: 'e1-2', source: '1', target: '2' }];
 
  let nodes = $state.raw<Node[]>(initialNodes);
  let edges = $state.raw(initialEdges);
 
  let nodeName = $state('Node 1');
  let nodeBg = $state('#F58A6A');
  let nodeHidden = $state(false);
 
  function updateNode() {
    nodes = nodes.map((node) => {
      if (node.id === '1') {
        return {
          ...node,
          data: {
            ...node.data,
            label: nodeName,
          },
          style: `background: ${nodeBg}`,
          hidden: nodeHidden,
        };
      }
      return node;
    });
    edges = edges.map((edge) => {
      if (edge.id === 'e1-2') {
        return {
          ...edge,
          hidden: nodeHidden,
        };
      }
      return edge;
    });
  }
 
  updateNode();
 
  function updateNodeName(event) {
    nodeName = event.target.value;
    updateNode();
  }
 
  function updateNodeBg(event) {
    nodeBg = event.target.value;
    updateNode();
  }
 
  function updateNodeHidden(event) {
    nodeHidden = event.target.checked;
    updateNode();
  }
</script>
 
<SvelteFlow bind:nodes bind:edges fitView maxZoom={2}>
  <Panel position="top-left" style="width: 200px;">
    <span class="xy-theme__label">label:</span>
    <input value={nodeName} oninput={updateNodeName} class="xy-theme__input" />
 
    <span class="xy-theme__label">background:</span>
    <input value={nodeBg} oninput={updateNodeBg} class="xy-theme__input" />
 
    <span class="xy-theme__label">hidden:</span>
    <input type="checkbox" oninput={updateNodeHidden} class="xy-theme__checkbox" />
  </Panel>
  <Background />
</SvelteFlow>
```
