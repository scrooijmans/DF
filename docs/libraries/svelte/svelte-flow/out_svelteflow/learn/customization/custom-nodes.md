# Custom Nodes

A powerful feature of Svelte Flow is the ability to create custom nodes. This gives you the flexibility to render anything you want within your nodes. We generally recommend creating your own custom nodes rather than relying on built-in ones. With custom nodes, you can add as many source and target handles as you like—or even embed form inputs, charts, and other interactive elements.

In this section, we’ll walk through creating a custom node featuring an input field that updates text elsewhere in your application. For further examples, we recommend checking out our <a href="https://svelteflow.dev/examples/nodes/custom-node" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]">Custom Node Example</a>.

## Creating a Custom Node<a href="https://svelteflow.dev/learn/customization/custom-nodes#creating-a-custom-node" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

To create a custom node, all you need to do is create a Svelte component. Svelte Flow will automatically wrap it in an interactive container that injects essential props like the node’s id, position, and data, and provides functionality for selection, dragging, and connecting handles. For a full reference on all available custom node props, take a look at the <a href="https://svelteflow.dev/api-reference/types/node-props" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]">Node Props</a>.

Let’s dive into an example by creating a custom node called `TextUpdaterNode`. For this, we’ve added a controlled input field with a oninput handler. We simply use the ‘text’ property from the node’s data for the input and we update the node’s data via the <a href="https://svelteflow.dev/api-reference/hooks/use-svelte-flow#update-node-data" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">updateNodeData</code></a> function, that can be accessed through the <a href="https://svelteflow.dev/api-reference/hooks/use-svelte-flow" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">useSvelteFlow</code></a> hook.

<img src="out_svelteflow/learn/customization/custom-nodes/index_media/0b7eab85abc3682cb59a01670459912dfc0cfbe8.svg" class="x:max-w-6 x:shrink-0" />TextUpdaterNode.svelte

<img src="out_svelteflow/learn/customization/custom-nodes/index_media/4ad055c8513380c64c1f5319faccf453f477bb38.svg" class="nextra-copy-icon" />

``` x:group
<script lang="ts">
  import { Position, useSvelteFlow, type NodeProps } from '@xyflow/svelte';
 
  let { id, data }: NodeProps = $props();
 
  let { updateNodeData } = useSvelteFlow();
</script>
 
<div class="text-updater-node">
  <div>
    <label for="text">Text:</label>
    <input
      id="text"
      name="text"
      value={data.text}
      oninput={(evt) => {
        updateNodeData(id, { text: evt.target.value });
      }}
      class="nodrag"
    />
  </div>
</div>
```

## Adding the Node Type<a href="https://svelteflow.dev/learn/customization/custom-nodes#adding-the-node-type" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

Now we need to communicate the new custom node to Svelte Flow. You can add custom nodes by passing the <a href="https://svelteflow.dev/api-reference/svelte-flow#node-types" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">nodeTypes</code></a> prop.

<img src="out_svelteflow/learn/customization/custom-nodes/index_media/0b7eab85abc3682cb59a01670459912dfc0cfbe8.svg" class="x:max-w-6 x:shrink-0" />App.svelte

<img src="out_svelteflow/learn/customization/custom-nodes/index_media/4ad055c8513380c64c1f5319faccf453f477bb38.svg" class="nextra-copy-icon" />

``` x:group
<script>
  import { SvelteFlow } from '@xyflow/svelte';
  import '@xyflow/svelte/dist/style.css';
 
  import TextUpdaterNode from './TextUpdaterNode.svelte';
 
  const nodeTypes = { textUpdater: TextUpdaterNode };
 
  // [...]
 
</script>
 
<SvelteFlow
  bind:nodes
  bind:edges
  {nodeTypes}
  fitView
>
  <!-- [...] -->
</SvelteFlow>
```

After defining your new node type, you can refer to it by using the `type` node option:

``` x:group
let nodes = $state.raw([
  {
    id: 'node-1',
    type: 'textUpdater',
    position: { x: 0, y: 0 },
    data: { text: 'some text' },
  },
]);
```

After putting it all together and adding some basic styles we get a custom node that prints text to the console:

<img src="out_svelteflow/learn/customization/custom-nodes/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/learn/customization/custom-nodes/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/learn/customization/custom-nodes/index_media/824817a2c7b3c6650883ed0f84fc74ac655539f5.svg" class="size-4 fill-slate-700 stroke-slate-700" />

## Adding Handles<a href="https://svelteflow.dev/learn/customization/custom-nodes#adding-handles" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

Svelte Flow provides a <a href="https://svelteflow.dev/api-reference/components/handle" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">Handle</code></a> component that can be used to add handles to your custom nodes. It’s as easy as mounting the component.

<img src="out_svelteflow/learn/customization/custom-nodes/index_media/0b7eab85abc3682cb59a01670459912dfc0cfbe8.svg" class="x:max-w-6 x:shrink-0" />CustomNode.svelte

<img src="out_svelteflow/learn/customization/custom-nodes/index_media/4ad055c8513380c64c1f5319faccf453f477bb38.svg" class="nextra-copy-icon" />

``` x:group
<script>
  import { Handle } from '@xyflow/svelte';
</script>
 
<Handle type="target" position={Position.Top} />
<Handle type="source" position={Position.Bottom} />
```

#### Multiple Handles<a href="https://svelteflow.dev/learn/customization/custom-nodes#multiple-handles" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

If you need more than just one source and target handle, you can use the `id`
