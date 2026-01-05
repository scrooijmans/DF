# Download Image

This example demonstrates how to export your Svelte Flow diagram as an image. Learn how to capture the current view of your flow and save it as a PNG or other image format, which is useful for sharing, documentation, or printing your diagrams.

<img src="out_svelteflow/examples/misc/download-image/index_media/a7db0f47f02f813c5a264458a8e522bf6320ca5a.svg" class="x:mt-[.3em]" />

The `version` of the <a href="https://www.npmjs.com/package/html-to-image" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">html-to-image <img src="out_svelteflow/examples/misc/download-image/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a> package used in this example, has been locked to `1.11.11`, which is the latest working `version` for the package. The recent versions, after `1.11.11`, are not exporting images properly and there is open <a href="https://github.com/bubkoo/html-to-image/issues/516" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">issue <img src="out_svelteflow/examples/misc/download-image/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a> for this on Github.

App.svelte

CustomNode.svelte

DownloadButton.svelte

xy-theme.css

index.css

nodes-and-edges.ts

<img src="out_svelteflow/examples/misc/download-image/index_media/a8615f5af2f285e6489e8b474f5d8c4d43e8c2bd.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/misc/download-image/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/examples/misc/download-image/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/misc/download-image/index_media/824817a2c7b3c6650883ed0f84fc74ac655539f5.svg" class="size-4 fill-slate-700 stroke-slate-700" />

``` x:group
<script lang="ts">
  import { SvelteFlow, Background, Controls, type Node, type Edge } from '@xyflow/svelte';
 
  import '@xyflow/svelte/dist/style.css';
 
  import DownloadButton from './DownloadButton.svelte';
 
  import { initialNodes, initialEdges } from './nodes-and-edges';
  import CustomNode from './CustomNode.svelte';
  let nodes = $state.raw<Node[]>(initialNodes);
  let edges = $state.raw<Edge[]>(initialEdges);
 
  const nodeTypes = {
    custom: CustomNode,
  };
 
  const defaultEdgeOptions = {
    animated: true,
    type: 'smoothstep',
  };
</script>
 
<SvelteFlow bind:nodes bind:edges {nodeTypes} {defaultEdgeOptions} fitView>
  <Controls />
  <DownloadButton />
  <Background />
</SvelteFlow>
```
