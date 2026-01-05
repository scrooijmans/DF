# Theming

Svelte Flow comes with minimal default styles and was designed to be fully customizable. Many of our users fully transform the look and feel of Svelte Flow to match their own brand or design system. This guide will introduce you to the different ways you can customize Svelte Flow’s appearance.

## Default styles<a href="https://svelteflow.dev/learn/customization/theming#default-styles" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

Svelte Flow’s default styles are enough to get going with the built-in nodes. They provide some sensible defaults for styles like padding, border radius, and animated edges. You can see what they look like below:

``` x:group
import '@xyflow/svelte/dist/style.css';
```

<img src="out_svelteflow/learn/customization/theming/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/learn/customization/theming/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/learn/customization/theming/index_media/824817a2c7b3c6650883ed0f84fc74ac655539f5.svg" class="size-4 fill-slate-700 stroke-slate-700" />

## Base styles<a href="https://svelteflow.dev/learn/customization/theming#base-styles" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

If you just want to load the very basic styles that are necessary for Svelte Flow to work, you can import the base styles instead:

``` x:group
import '@xyflow/svelte/dist/base.css';
```

<img src="out_svelteflow/learn/customization/theming/index_media/a7db0f47f02f813c5a264458a8e522bf6320ca5a.svg" class="x:mt-[.3em]" />

These base styles are **required** for Svelte Flow to function correctly. If you don’t import them or you override them with your own styles, some things might not work as expected!

<img src="out_svelteflow/learn/customization/theming/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/learn/customization/theming/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/learn/customization/theming/index_media/824817a2c7b3c6650883ed0f84fc74ac655539f5.svg" class="size-4 fill-slate-700 stroke-slate-700" />

## Customization<a href="https://svelteflow.dev/learn/customization/theming#customization" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

There are different ways how you can customize the appearance of Svelte Flow:

- Work with scoped CSS within your custom nodes and edges
- Override the built-in classes with custom CSS
- Override the CSS variables Svelte Flow uses
- Pass inline styles through `style` props

### Working with built-in classes<a href="https://svelteflow.dev/learn/customization/theming#working-with-built-in-classes" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

If you want to override the default styles, you can do so by overriding the built-in CSS classes. You can find a list of all the classes used by Svelte Flow below:

| Class name | Description |
|:---|:---|
| `.svelte-flow` | The outermost container |
| `.svelte-flow__renderer` | The inner container |
| `.svelte-flow__zoompane` | Zoom & pan pane |
| `.svelte-flow__selectionpane` | Selection pane |
| `.svelte-flow__selection` | User selection |
| `.svelte-flow__edges` | The element containing all edges in the flow |
| `.svelte-flow__edge` | Applied to each <a href="https://svelteflow.dev/api-reference/types/edge" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">Edge</code></a> in the flow |
| `.svelte-flow__edge.selected` | Added to an <a href="https://svelteflow.dev/api-reference/types/edge" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">Edge</code></a> when selected |
| `.svelte-flow__edge.animated` | Added to an <a href="https://svelteflow.dev/api-reference/types/edge" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">Edge</code></a> when its `animated` prop is `true` |
| `.svelte-flow__edge-path` | The SVG `<path />` element of an <a href="https://svelteflow.dev/api-reference/types/edge" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">Edge</code></a> |
| `.svelte-flow__edge-label` | The edge label |
| `.svelte-flow__connection` | Applied to the current connection line |
| `.svelte-flow__connection-path` | The SVG `<path />` of a connection line |
| `.svelte-flow__nodes` | The element containing all nodes in the flow |
| `.svelte-flow__node` | Applied to each <a href="https://svelteflow.dev/api-reference/types/node" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">Node</code></a> in the flow |
| `.svelte-flow__node.selected` | Added to a <a href="https://svelteflow.dev/api-reference/types/node" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">Node</code></a> when selected. |
| `.svelte-flow__node-default` | Added when <a href="https://svelteflow.dev/api-reference/types/node" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">Node</code></a> type is `"default"` |
| `.svelte-flow__node-input` | Added when <a href="https://svelteflow.dev/api-reference/types/node" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">Node</code></a> type is `"input"` |
| `.svelte-flow__node-output` | Added when <a href="https://svelteflow.dev/api-reference/types/node" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">Node</code></a> type is `"output"` |
| `.svelte-flow__node-group` | Added when <a href="https://svelteflow.dev/api-reference/types/node" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">Node</code></a> type is `"group"` |
| `.svelte-flow__nodesselection` | Nodes selection |
| `.svelte-flow__nodesselection-rect` | Nodes selection rect |
| `.svelte-flow__handle` | Applied to each <a href="https://svelteflow.dev/api-reference/components/handle" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">&lt;Handle /&gt;</code></a> component |
| `.svelte-flow__handle-top` | Applied when a handle’s <a href="https://svelteflow.dev/api-reference/types/position" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">Position</code></a> is set to `"top"` |
| `.svelte-flow__handle-right` | Applied when a handle’s <a href="https://svelteflow.dev/api-reference/types/position" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">Position</code></a> is set to `"right"` |
| `.svelte-flow__handle-bottom` | Applied when a handle’s <a href="https://svelteflow.dev/api-reference/types/position" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">Position</code></a> is set to `"bottom"` |
| `.svelte-flow__handle-left` | Applied when a handle’s <a href="https://svelteflow.dev/api-reference/types/position" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">Position</code></a> is set to `"left"` |
| `.svelte-flow__handle.connecting` | Applied when a connection line is above a handle. |
| `.svelte-flow__handle.valid` | Applied when a connection line is above a handle **and** the connection is valid |
| `.svelte-flow__background` | Applied to the <a href="https://svelteflow.dev/api-reference/components/background" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">&lt;Background /&gt;</code></a> component |
| `.svelte-flow__minimap` | Applied to the <a href="https://svelteflow.dev/api-reference/components/mini-map" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">&lt;MiniMap /&gt;</code></a> component |
| `.svelte-flow__controls` | Applied to the <a href="https://svelteflow.dev/api-reference/components/controls" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">&lt;Controls /&gt;</code></a> component |

<img src="out_svelteflow/learn/customization/theming/index_media/9ad8526a9514e5c58a2e4d8bd814f6e82c5f7eb2.svg" class="x:mt-[.3em]" />

Be careful if you go poking around the source code looking for other classes to override. Some classes are used internally and are required in order for the library to be functional. If you replace them you may end up with unexpected bugs or errors!

### CSS variables<a href="https://svelteflow.dev/learn/customization/theming#css-variables" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

If you don’t want to replace the default styles entirely but just want to tweak the overall look and feel, you can override some of the CSS variables we use throughout the library.

These variables are mostly self-explanatory. Below is a table of all the variables you might want to tweak and their default values for reference:

| Variable name | Default |
|:---|:---|
| `--xy-edge-stroke-default` | `#b1b1b7` |
| `--xy-edge-stroke-width-default` | `1` |
| `--xy-edge-stroke-selected-default` | `#555` |
| `--xy-connectionline-stroke-default` | `#b1b1b7` |
| `--xy-connectionline-stroke-width-default` | `1` |
| `--xy-attribution-background-color-default` | `rgba(255, 255, 255, 0.5)` |
| `--xy-minimap-background-color-default` | `#fff` |
| `--xy-background-pattern-dot-color-default` | `#91919a` |
| `--xy-background-pattern-line-color-default` | `#eee` |
| `--xy-background-pattern-cross-color-default` | `#e2e2e2` |
| `--xy-node-color-default` | `inherit` |
| `--xy-node-border-default` | `1px solid #1a192b` |
| `--xy-node-background-color-default` | `#fff` |
| `--xy-node-group-background-color-default` | `rgba(240, 240, 240, 0.25)` |
| `--xy-node-boxshadow-hover-default` | `0 1px 4px 1px rgba(0, 0, 0, 0.08)` |
| `--xy-node-boxshadow-selected-default` | `0 0 0 0.5px #1a192b` |
| `--xy-handle-background-color-default` | `#1a192b` |
| `--xy-handle-border-color-default` | `#fff` |
| `--xy-selection-background-color-default` | `rgba(0, 89, 220, 0.08)` |
| `--xy-selection-border-default` | `1px dotted rgba(0, 89, 220, 0.8)` |
| `--xy-controls-button-background-color-default` | `#fefefe` |
| `--xy-controls-button-background-color-hover-default` | `#f4f4f4` |
| `--xy-controls-button-color-default` | `inherit` |
| `--xy-controls-button-color-hover-default` | `inherit` |
| `--xy-controls-button-border-color-default` | `#eee` |
| `--xy-controls-box-shadow-default` | `0 0 2px 1px rgba(0, 0, 0, 0.08)` |

These variables are used to define the *defaults* for the various elements of Svelte Flow. This means they can still be overridden by inline styles or custom classes on a per-element basis. If you want to override a variable, you can do so by adding:

``` x:group
.svelte-flow {
  --xy-node-background-color-default: #ff5050;
}
```

### Tailwind<a href="https://svelteflow.dev/learn/customization/theming#tailwind" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

Custom nodes and edges are just Svelte components, and you can use any styling solution you’d like to style them. For example, you might want to use <a href="https://tailwindcss.com/" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">Tailwind <img src="out_svelteflow/learn/customization/theming/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a> to style your nodes:

``` x:group
<script lang="ts">
  import { Handle, Position, type NodeProps } from '@xyflow/svelte';
 
  let { data }: NodeProps = $props();
</script>
 
<div class="px-4 py-2 shadow-md rounded-md bg-white border-2 border-stone-400">
  <div class="flex">
    <div class="rounded-full w-12 h-12 flex justify-center items-center bg-gray-100">
      {data.emoji}
    </div>
    <div class="ml-2">
      <div class="text-lg font-bold">{data.name}</div>
      <div class="text-gray-500">{data.job}</div>
    </div>
  </div>
  <Handle
    type="target"
    position={Position.Top}
    class="w-16 !bg-teal-500 rounded-none border-none"
  />
  <Handle
    type="source"
    position={Position.Bottom}
    class="w-16 !bg-teal-500 rounded-none border-none"
  />
</div>
```

<img src="out_svelteflow/learn/customization/theming/index_media/a7db0f47f02f813c5a264458a8e522bf6320ca5a.svg" class="x:mt-[.3em]" />

If you want to overwrite default styles, make sure to import Tailwinds entry last!

``` x:group
import '@xyflow/svelte/dist/style.css';
import 'tailwind.css';
```

For a complete example of using Tailwind with Svelte Flow, check out the <a href="https://svelteflow.dev/examples/styling/tailwind" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]">tailwind example</a>!
