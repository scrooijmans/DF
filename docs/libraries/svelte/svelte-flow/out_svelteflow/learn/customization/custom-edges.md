# Custom Edges

Like <a href="https://svelteflow.dev/learn/customization/custom-nodes" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]">custom nodes</a>, parts of a custom edge in Svelte Flow are just Svelte components: that means you can render anything you want along an edge! This guide shows you how to implement a custom edge with some additional controls. For a comprehensive reference of props available for custom edges, see the <a href="https://svelteflow.dev/api-reference/types/edge-props" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">EdgeProps</code></a> documentation.

## A basic custom edge<a href="https://svelteflow.dev/learn/customization/custom-edges#a-basic-custom-edge" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

An edge isn’t much use to us if it doesn’t render a path between two connected nodes. These paths are always SVG-based and are typically rendered using the <a href="https://svelteflow.dev/api-reference/components/base-edge" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">&lt;BaseEdge /&gt;</code></a> component. To calculate the actual SVG path to render, Svelte Flow comes with some handy utility functions:

- <a href="https://svelteflow.dev/api-reference/utils/get-bezier-path" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">getBezierPath</code></a>
- <a href="https://svelteflow.dev/api-reference/utils/get-smooth-step-path" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">getSmoothStepPath</code></a>
- <a href="https://svelteflow.dev/api-reference/utils/get-straight-path" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">getStraightPath</code></a>

To kick start our custom edge, we’ll just render a straight path between the source and target.

<img src="out_svelteflow/learn/customization/custom-edges/index_media/0b7eab85abc3682cb59a01670459912dfc0cfbe8.svg" class="x:max-w-6 x:shrink-0" />CustomEdge.svelte

<img src="out_svelteflow/learn/customization/custom-edges/index_media/4ad055c8513380c64c1f5319faccf453f477bb38.svg" class="nextra-copy-icon" />

``` x:group
<script lang="ts">
  import { BaseEdge, getStraightPath, type EdgeProps } from '@xyflow/svelte';
 
  let { id, sourceX, sourceY, targetX, targetY }: EdgeProps = $props();
 
  let [edgePath] = $derived(
    getStraightPath({
      sourceX,
      sourceY,
      targetX,
      targetY,
    })
  );
</script>
 
<BaseEdge {id} path={edgePath} />
```

<img src="out_svelteflow/learn/customization/custom-edges/index_media/a7db0f47f02f813c5a264458a8e522bf6320ca5a.svg" class="x:mt-[.3em]" />

All the props passed to your custom edge component can be found in the API reference under the <a href="https://svelteflow.dev/api-reference/types/edge-props" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">EdgeProps</code></a> type.

This gives us a straight edge that behaves the same as the default `"straight"` <a href="https://svelteflow.dev/api-reference/types/edge#default-edge-types" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]">edge type</a>. To use it, we also need to update the <a href="https://svelteflow.dev/api-reference/svelte-flow#edge-types" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">edgeTypes</code></a> prop on the `<SvelteFlow />` component.

It’s important to define the `edgeTypes` object *outside of the component* or to use Svelte’s `$derived` to prevent unnecessary re-renders. Svelte Flow will show a warning in the console if you forget to do this.

<img src="out_svelteflow/learn/customization/custom-edges/index_media/0b7eab85abc3682cb59a01670459912dfc0cfbe8.svg" class="x:max-w-6 x:shrink-0" />App.svelte

<img src="out_svelteflow/learn/customization/custom-edges/index_media/4ad055c8513380c64c1f5319faccf453f477bb38.svg" class="nextra-copy-icon" />

``` x:group
<script lang="ts">
  import { SvelteFlow, type EdgeTypes } from '@xyflow/svelte';
  import CustomEdge from './CustomEdge.svelte';
 
  const edgeTypes: EdgeTypes = {
    'custom-edge': CustomEdge
  };
</script>
 
<SvelteFlow {edgeTypes} ... />
```

After defining the `edgeTypes` object, we can use our new custom edge by setting the `type` field of an edge to `"custom-edge"`.

<img src="out_svelteflow/learn/customization/custom-edges/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/learn/customization/custom-edges/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/learn/customization/custom-edges/index_media/824817a2c7b3c6650883ed0f84fc74ac655539f5.svg" class="size-4 fill-slate-700 stroke-slate-700" />

## Adding an edge label<a href="https://svelteflow.dev/learn/customization/custom-edges#adding-an-edge-label" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

One of the more common uses for custom edges is rendering some controls or info along an edge’s path. In Svelte Flow we call that an *edge label* and unlike the edge path, edge labels can be any Svelte component!

Because Svelte Flows edges are mounted inside a SVG component, we need to escape it’s context to render a custom edge label. For this, we have a handy <a href="https://svelteflow.dev/api-reference/components/edge-label" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">&lt;EdgeLabel /&gt;</code></a> component. Aside from a couple of extras, like inheriting the edges z-index, it functions as a portal that mounts the child components in the viewport div.

Let’s add a button to our custom edge that can be used to delete the edge it’s attached to:

<img src="out_svelteflow/learn/customization/custom-edges/index_media/0b7eab85abc3682cb59a01670459912dfc0cfbe8.svg" class="x:max-w-6 x:shrink-0" />CustomEdge.svelte

<img src="out_svelteflow/learn/customization/custom-edges/index_media/4ad055c8513380c64c1f5319faccf453f477bb38.svg" class="nextra-copy-icon" />

``` x:group
<script lang="ts">
  import {
    BaseEdge,
    EdgeLabel,
    getStraightPath,
    useEdges,
    type EdgeProps,
  } from '@xyflow/svelte';
 
  let { id, sourceX, sourceY, targetX, targetY }: EdgeProps = $props();
 
  let [edgePath, labelX, labelY] = $derived(
    getStraightPath({
      sourceX,
      sourceY,
      targetX,
      targetY,
    })
  );
 
  const edges = useEdges();
</script>
 
<BaseEdge {id} path={edgePath} />
<EdgeLabel x={labelX} y={labelY}>
  <button
    class="nodrag nopan"
    onclick={() => {
      edges.update((eds) => eds.filter((edge) => edge.id !== id));
    }}
  >
    delete
  </button>
</EdgeLabel>
```

<img src="out_svelteflow/learn/customization/custom-edges/index_media/a7db0f47f02f813c5a264458a8e522bf6320ca5a.svg" class="x:mt-[.3em]" />

To make sure our edge labels are interactive and not just for presentation, it is important to add the `nodrag` and `nopan` classes to the label to stop mouse events from controlling the canvas.

Here’s an interactive example with our updated custom edge. Clicking the delete button will remove that edge from the flow. Creating a new edge will use the custom node.

<img src="out_svelteflow/learn/customization/custom-edges/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/learn/customization/custom-edges/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/learn/customization/custom-edges/index_media/824817a2c7b3c6650883ed0f84fc74ac655539f5.svg" class="size-4 fill-slate-700 stroke-slate-700" />

## Making Custom SVG Edge Paths<a href="https://svelteflow.dev/learn/customization/custom-edges#making-custom-svg-edge-paths" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

As discussed previously, if you want to make a custom edge in Svelte Flow, you have to use either of the four path creation functions discussed above (e.g <a href="https://svelteflow.dev/api-reference/utils/get-bezier-path" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">getBezierPath</code></a>). However if you want to make some other path shape like a Sinusoidal edge or some other edge type then you will have to make the edge path yourself.

The edge path we get from functions like <a href="https://svelteflow.dev/api-reference/utils/get-bezier-path" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">getBezierPath</code></a> is just a path string which we pass into the `path` prop of the `<BaseEdge />` component. It contains the necessary information needed in order to draw that path, like where it should start from, where it should curve, where it should end, etc. A simple straight path string between two points `(x1, y1)` to `(x2, y2)` would look like:

``` x:group
M x1 y1 L x2 y2
```

An SVG path is a concatenated list of commands like `M`, `L`, `Q`, etc, along with their values. Some of these commands are listed below, along with their supported values.

- `M x1 y1` is the Move To command which moves the current point to the x1, y1 coordinate.
- `L x1 y1` is the Line To command which draws a line from the current point to x1, y1 coordinate.
- `Q x1 y1 x2 y2` is the Quadratic Bezier Curve command which draws a bezier curve from the current point to the x2, y2 coordinate. x1, y1 is the control point of the curve which determines the curviness of the curve.

Whenever we want to start a path for our custom edge, we use the `M` command to move our current point to `sourceX, sourceY` which we get as props in the custom edge component. Then based on the shape we want, we will use other commands like `L`(to make lines), `Q`(to make curves) and then finally end our path at `targetX, targetY` which we get as props in the custom edge component.

If you want to learn more about SVG paths, you can check out <a href="https://yqnn.github.io/svg-path-editor/" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">SVG-Path-Editor <img src="out_svelteflow/learn/customization/custom-edges/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a>. You can paste any SVG path there and analyze individual path commands via an intuitive UI.

Here is an example with two types of custom edge paths, a Step edge and a Sinusoidal edge. You should look at the Step edge first to get your hands dirty with custom SVG paths since it’s simple, and then look at how the Sinusoidal edge is made. After going through this example, you will have the necessary knowledge to make custom SVG paths for your custom edges.

<img src="out_svelteflow/learn/customization/custom-edges/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/learn/customization/custom-edges/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/learn/customization/custom-edges/index_media/824817a2c7b3c6650883ed0f84fc74ac655539f5.svg" class="size-4 fill-slate-700 stroke-slate-700" />
