# Feature Overview

This example provides an overview of Svelte Flow’s main features. See demonstrations of node types, edge types, interactions, layouts, and other capabilities in one comprehensive example. Perfect for understanding what’s possible with Svelte Flow.

AnnotationNode.svelte

App.svelte

ButtonEdge.svelte

CircleNode.svelte

Message.svelte

ResizerNode.svelte

TextNode.svelte

ToolbarNode.svelte

xy-theme.css

index.css

nodes-and-edges.ts

<img src="out_svelteflow/examples/overview/index_media/a8615f5af2f285e6489e8b474f5d8c4d43e8c2bd.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/overview/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/examples/overview/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/overview/index_media/824817a2c7b3c6650883ed0f84fc74ac655539f5.svg" class="size-4 fill-slate-700 stroke-slate-700" />

``` x:group
<script module>
  import type { Node, NodeProps } from '@xyflow/svelte';
 
  export type AnnotationNode = Node<{
    label: string;
    level: number;
    arrowStyle?: string;
  }>;
</script>
 
<script lang="ts">
  let { data }: NodeProps<AnnotationNode> = $props();
</script>
 
<div class="annotation-content">
  <div class="annotation-level">{data.level}.</div>
  <div>{data.label}</div>
</div>
{#if data.arrowStyle}
  <div class="annotation-arrow" style={data.arrowStyle}>⤹</div>
{/if}
```
