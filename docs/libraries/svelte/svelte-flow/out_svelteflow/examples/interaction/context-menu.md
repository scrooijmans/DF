# Context Menu

You can add a custom context menu to nodes and edges by using the `onNodeContextMenu` and `onEdgeContextMenu` events. This example shows how to implement a context menu that appears when right-clicking on nodes or edges.

App.svelte

ContextMenu.svelte

xy-theme.css

index.css

nodes-and-edges.ts

<img src="out_svelteflow/examples/interaction/context-menu/index_media/a8615f5af2f285e6489e8b474f5d8c4d43e8c2bd.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/interaction/context-menu/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/examples/interaction/context-menu/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/interaction/context-menu/index_media/824817a2c7b3c6650883ed0f84fc74ac655539f5.svg" class="size-4 fill-slate-700 stroke-slate-700" />

``` x:group
<script lang="ts">
  import {
    SvelteFlow,
    Background,
    type Node,
    type Edge,
    type NodeEventWithPointer,
  } from '@xyflow/svelte';
 
  import ContextMenu from './ContextMenu.svelte';
 
  import { initialNodes, initialEdges } from './nodes-and-edges';
 
  import '@xyflow/svelte/dist/style.css';
 
  let nodes = $state.raw<Node[]>(initialNodes);
  let edges = $state.raw<Edge[]>(initialEdges);
 
  let menu: {
    id: string;
    top?: number;
    left?: number;
    right?: number;
    bottom?: number;
  } | null = $state(null);
  let clientWidth: number = $state(0);
  let clientHeight: number = $state(0);
 
  const handleContextMenu: NodeEventWithPointer<MouseEvent> = ({ event, node }) => {
    // Prevent native context menu from showing
    event.preventDefault();
 
    // Calculate position of the context menu. We want to make sure it
    // doesn't get positioned off-screen.
    menu = {
      id: node.id,
      top: event.clientY < clientHeight - 200 ? event.clientY : undefined,
      left: event.clientX < clientWidth - 200 ? event.clientX : undefined,
      right: event.clientX >= clientWidth - 200 ? clientWidth - event.clientX : undefined,
      bottom:
        event.clientY >= clientHeight - 200 ? clientHeight - event.clientY : undefined,
    };
  };
 
  // Close the context menu if it's open whenever the window is clicked.
  function handlePaneClick() {
    menu = null;
  }
</script>
 
<div style="height:100vh;" bind:clientWidth bind:clientHeight>
  <SvelteFlow
    bind:nodes
    bind:edges
    onnodecontextmenu={handleContextMenu}
    onpaneclick={handlePaneClick}
    onpointerdown={handlePaneClick}
    fitView
  >
    <Background />
    {#if menu}
      <ContextMenu
        onclick={handlePaneClick}
        id={menu.id}
        top={menu.top}
        left={menu.left}
        right={menu.right}
        bottom={menu.bottom}
      />
    {/if}
  </SvelteFlow>
</div>
```
