# Server side rendering, server side generation

This is an advanced use case and assumes you are already familiar with SvelteFlow. If you’re new to SvelteFlow, check out our <a href="https://svelteflow.dev/learn/getting-started/key-concepts" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]">getting started guide</a>.

In this guide, you’ll learn how to configure SvelteFlow for server-side rendering, enabling you to:

- Generate static HTML diagrams for documentation
- Render SvelteFlow diagrams in non-JavaScript environments
- Create dynamic Open Graph images for social media sharing

(For client-side image generation, check out our <a href="https://svelteflow.dev/examples/misc/download-image" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]">download image example</a>.)

### Why Server-Side Rendering is Complex<a href="https://svelteflow.dev/learn/advanced/server-side-rendering#why-server-side-rendering-is-complex" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

To understand why server-side rendering in Svelte Flow requires special configuration, let’s look at what SvelteFlow typically handles on the client side:

1.  **Node Dimension Calculation**

    - Nodes can contain any content, so their dimensions are determined by the browser’s layout engine
    - This dynamic sizing isn’t available during server-side rendering

2.  **Handle Position Detection**

    - Edge connections require precise handle positions
    - These positions are calculated based on CSS layout, which isn’t available on the server

3.  **Container Size Adaptation**

    - SvelteFlow adapts to its container’s dimensions
    - Server-side rendering needs explicit dimensions

### Node Dimensions<a href="https://svelteflow.dev/learn/advanced/server-side-rendering#node-dimensions" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

The most crucial aspect of server-side rendering is specifying node dimensions. On the client, SvelteFlow automatically measures nodes and stores dimensions in `measured.width` and `measured.height`. For server-side rendering, you must provide these dimensions explicitly using either:

<img src="out_svelteflow/learn/advanced/server-side-rendering/index_media/0fee490687881197462888c84507f0512d9761c1.svg" class="x:mt-[.3em]" />

**Node Dimension Options:**

1.  `width` and `height`: Static dimensions that won’t change
2.  `initialWidth` and `initialHeight`: Dynamic dimensions that may change after client-side hydration

``` x:group
<script>
  const nodes = [
    {
      id: '1',
      type: 'default',
      position: { x: 0, y: 0 },
      data: { label: 'Node 1' },
      width: 100,
      height: 50,
    },
  ];
</script>
```

### Handle Positions<a href="https://svelteflow.dev/learn/advanced/server-side-rendering#handle-positions" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

To render edges on the server, you need to provide handle positions explicitly. On the client, SvelteFlow calculates these positions automatically, but for server-side rendering, you must specify them using the `handles` property:

``` x:group
<script>
  import { Position } from '@xyflow/svelte';
 
  const nodes = [
    {
      id: '1',
      type: 'default',
      position: { x: 0, y: 0 },
      data: { label: 'Node 1' },
      width: 100,
      height: 50,
      handles: [
        {
          type: 'target',
          position: Position.Top,
          x: 100 / 2,
          y: 0,
        },
        {
          type: 'source',
          position: Position.Bottom,
          x: 100 / 2,
          y: 50,
        },
      ],
    },
  ];
</script>
```

### Using `fitView` with Server-Side Rendering<a href="https://svelteflow.dev/learn/advanced/server-side-rendering#using-fitview-with-server-side-rendering" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

If you know your container’s dimensions, you can use `fitView` during server-side rendering by providing the container’s width and height:

``` x:group
<script>
  import { SvelteFlow } from '@xyflow/svelte';
 
  let nodes = $state.raw([/* ... */]);
  let edges = $state.raw([/* ... */]);
</script>
 
<SvelteFlow {nodes} {edges} fitView width={1000} height={500} />
```

### Generating Static HTML<a href="https://svelteflow.dev/learn/advanced/server-side-rendering#generating-static-html" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

To create static HTML output, you can use Svelte’s server-side rendering capabilities. This generates an HTML string that you can use for static files or HTTP responses:

``` x:group
<script>
  import { SvelteFlow, Background } from '@xyflow/svelte';
  import { render } from 'svelte/server';
 
  function toHTML({ nodes, edges, width, height }) {
    const { html } = render(SvelteFlow, {
      props: {
        nodes,
        edges,
        width,
        height,
        minZoom: 0.2,
        fitView: true,
      },
      children: [Background],
    });
 
    return html;
  }
</script>
```
