# Layouting

We regularly get asked how to handle layouting in Svelte Flow. While we could build some basic layouting into Svelte Flow, we believe that **you know your app’s requirements best** and with so many options out there we think it’s better you choose the best right tool for the job (not to mention it’d be a whole bunch of work for us).

That doesn’t help very much if you don’t know what the options _are_, so this guide is here to help! We’ll split things up into resources for layouting nodes and resources for routing edges.

To start, let’s explore several different layouting libraries and how they can be used with Svelte Flow.

## Layouting Nodes<a href="https://svelteflow.dev/learn/layouting/layouting-libraries#layouting-nodes" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

For layouting nodes, there are a few third-party libraries that we think are worth checking out:

| Library                                                                                                                                                                                                                                                                                                                                                                                                                     | Dynamic node sizes | Sub-flow layouting | Edge routing | Bundle size                                                                                                                                                                              |
| --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------ | ------------------ | ------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| <a href="https://github.com/dagrejs/dagre" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">Dagre <img src="out_svelteflow/learn/layouting/layouting-libraries/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a>          | Yes                | Yes¹               | No           | [![](out_svelteflow/learn/layouting/layouting-libraries/index_media/3328b892e04c21c9c3b442e690bb1341c6a337b5.svg "Bundle size for @dagrejs/dagre")](https://pkg-size.dev/@dagrejs/dagre) |
| <a href="https://github.com/d3/d3-hierarchy" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">D3-Hierarchy <img src="out_svelteflow/learn/layouting/layouting-libraries/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a> | No                 | No                 | No           | [![](out_svelteflow/learn/layouting/layouting-libraries/index_media/e2dce4a3f3f10e985c40adcd426eac3d90eae3b4.svg "Bundle size for d3-hierarchy")](https://pkg-size.dev/d3-hierarchy)     |
| <a href="https://github.com/d3/d3-force" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">D3-Force <img src="out_svelteflow/learn/layouting/layouting-libraries/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a>         | Yes                | No                 | No           | [![](out_svelteflow/learn/layouting/layouting-libraries/index_media/b90d3de40708a1f1ef9206a6efa947855bf6f1f5.svg "Bundle size for d3-force")](https://pkg-size.dev/d3-force)             |
| <a href="https://github.com/kieler/elkjs" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">ELK <img src="out_svelteflow/learn/layouting/layouting-libraries/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a>             | Yes                | Yes                | Yes          | [![](out_svelteflow/learn/layouting/layouting-libraries/index_media/68a3beb91c2b4e6c98441d98ebdbff0c9d383030.svg "Bundle size for elkjs")](https://pkg-size.dev/elkjs)                   |

¹ Dagre currently has an <a href="https://github.com/dagrejs/dagre/issues/238" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">open issue <img src="out_svelteflow/learn/layouting/layouting-libraries/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a> that prevents it from laying out sub-flows correctly if any nodes in the sub-flow are connected to nodes outside the sub-flow.

We’ve loosely ordered these options from simplest to most complex, where dagre is largely a drop-in solution and elkjs is a full-blown highly configurable layouting engine. Below, we’ll take a look at a brief example of how each of these libraries can be used with Svelte Flow. For dagre and elkjs specifically, we have some separate examples you can refer back to <a href="https://svelteflow.dev/examples/layout/dagre" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]">here</a> and <a href="https://svelteflow.dev/examples/layout/elkjs" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]">here</a>.

### Dagre<a href="https://svelteflow.dev/learn/layouting/layouting-libraries#dagre" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

- Repo: <a href="https://github.com/dagrejs/dagre" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">https://github.com/dagrejs/dagre <img src="out_svelteflow/learn/layouting/layouting-libraries/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a>
- Docs: <a href="https://github.com/dagrejs/dagre/wiki#configuring-the-layout" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">https://github.com/dagrejs/dagre/wiki#configuring-the-layout <img src="out_svelteflow/learn/layouting/layouting-libraries/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a>

Dagre is a simple library for layouting directed graphs. It has minimal configuration options and a focus on speed over choosing the most optimal layout. If you need to organize your flows into a tree, _we highly recommend dagre_.

<img src="out_svelteflow/learn/layouting/layouting-libraries/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/learn/layouting/layouting-libraries/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/learn/layouting/layouting-libraries/index_media/824817a2c7b3c6650883ed0f84fc74ac655539f5.svg" class="size-4 fill-slate-700 stroke-slate-700" />

With no effort at all we get a well-organized tree layout! Whenever `getLayoutedElements` is called, we’ll reset the dagre graph and set the graph’s direction (either left-to-right or top-to-bottom) based on the `direction` prop. Dagre needs to know the dimensions of each node in order to lay them out, so we iterate over our list of nodes and add them to dagre’s internal graph.

After laying out the graph, we’ll return an object with the layouted nodes and edges. We do this by mapping over the original list of nodes and updating each node’s position according to node stored in the dagre graph.

Documentation for dagre’s configuration options can be found <a href="https://github.com/dagrejs/dagre/wiki#configuring-the-layout" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">here <img src="out_svelteflow/learn/layouting/layouting-libraries/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a>, including properties to set for spacing and alignment.

### D3-Hierarchy<a href="https://svelteflow.dev/learn/layouting/layouting-libraries#d3-hierarchy" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

- Repo: <a href="https://github.com/d3/d3-hierarchy" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">https://github.com/d3/d3-hierarchy <img src="out_svelteflow/learn/layouting/layouting-libraries/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a>
- Docs: <a href="https://d3js.org/d3-hierarchy" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">https://d3js.org/d3-hierarchy <img src="out_svelteflow/learn/layouting/layouting-libraries/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a>

When you know your graph is a tree with a single root node, d3-hierarchy can provide a handful of interesting layouting options. While the library can layout a simple tree just fine, it also has layouting algorithms for tree maps, partition layouts, and enclosure diagrams.

<img src="out_svelteflow/learn/layouting/layouting-libraries/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/learn/layouting/layouting-libraries/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/learn/layouting/layouting-libraries/index_media/824817a2c7b3c6650883ed0f84fc74ac655539f5.svg" class="size-4 fill-slate-700 stroke-slate-700" />

<img src="out_svelteflow/learn/layouting/layouting-libraries/index_media/0fee490687881197462888c84507f0512d9761c1.svg" class="x:mt-[.3em]" />

D3-hierarchy expects your graphs to have a single root node, so it won’t work in all cases. It’s also important to note that d3-hierarchy assigns the same width and height to _all_ nodes when calculating the layout, so it’s not the best choice if you’re displaying lots of different node types.

### D3-Force<a href="https://svelteflow.dev/learn/layouting/layouting-libraries#d3-force" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

- Repo: <a href="https://github.com/d3/d3-force" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">https://github.com/d3/d3-force <img src="out_svelteflow/learn/layouting/layouting-libraries/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a>
- Docs: <a href="https://d3js.org/d3-force" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">https://d3js.org/d3-force <img src="out_svelteflow/learn/layouting/layouting-libraries/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a>

For something more interesting than a tree, a force-directed layout might be the way to go. D3-Force is a physics-based layouting library that can be used to position nodes by applying different forces to them.

As a consequence, it’s a little more complicated to configure and use compared to dagre and d3-hierarchy. Importantly, d3-force’s layouting algorithm is iterative, so we need a way to keep computing the layout across multiple renders.

First, let’s see what it does:

<img src="out_svelteflow/learn/layouting/layouting-libraries/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/learn/layouting/layouting-libraries/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/learn/layouting/layouting-libraries/index_media/824817a2c7b3c6650883ed0f84fc74ac655539f5.svg" class="size-4 fill-slate-700 stroke-slate-700" />

In our Svelte implementation, we use Svelte’s lifecycle hooks and reactive state to manage the force simulation. The simulation is configured with a number of different forces applied so you can see how they interact: play around in your own code to see how you want to configure those forces. You can find the documentation and some different examples of d3-force <a href="https://d3js.org/d3-force" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">here <img src="out_svelteflow/learn/layouting/layouting-libraries/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a>.

<img src="out_svelteflow/learn/layouting/layouting-libraries/index_media/0fee490687881197462888c84507f0512d9761c1.svg" class="x:mt-[.3em]" />

**Rectangular collisions** D3-Force has a built-in collision force, but it assumes nodes are circles. We’ve thrown together a custom force in `collision.js` that uses a similar algorithm but accounts for our rectangular nodes instead. Feel free to steal it or let us know if you have any suggestions for improvements!

The tick function progresses the simulation by one step and then updates Svelte Flow with the new node positions. We’ve also included a demonstration on how to handle node dragging while the simulation is running: if your flow isn’t interactive you can ignore that part!

<img src="out_svelteflow/learn/layouting/layouting-libraries/index_media/0fee490687881197462888c84507f0512d9761c1.svg" class="x:mt-[.3em]" />

For larger graphs, computing the force layout every render forever is going to incur a big performance hit. In this example we have a simple toggle to turn the layouting on and off, but you might want to come up with some other approach to only compute the layout when necessary.

### Elkjs<a href="https://svelteflow.dev/learn/layouting/layouting-libraries#elkjs" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

- Repo: <a href="https://github.com/kieler/elkjs" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">https://github.com/kieler/elkjs <img src="out_svelteflow/learn/layouting/layouting-libraries/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a>
- Docs: <a href="https://eclipse.dev/elk/reference.html" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">https://eclipse.dev/elk/reference.html <img src="out_svelteflow/learn/layouting/layouting-libraries/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a> (good luck!)

Elkjs is certainly the most configurable option available, but it’s also the most complicated. Elkjs is a Java library that’s been ported to JavaScript, and it provides a huge number of options for configuring the layout of your graph.

<img src="out_svelteflow/learn/layouting/layouting-libraries/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/learn/layouting/layouting-libraries/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/learn/layouting/layouting-libraries/index_media/824817a2c7b3c6650883ed0f84fc74ac655539f5.svg" class="size-4 fill-slate-700 stroke-slate-700" />

At its most basic we can compute layouts similar to dagre, but because the layouting algorithm runs asynchronously we need to handle promises and update our Svelte state accordingly.

<img src="out_svelteflow/learn/layouting/layouting-libraries/index_media/0fee490687881197462888c84507f0512d9761c1.svg" class="x:mt-[.3em]" />

**The ELK reference is your new best friend** We don’t often recommend elkjs because its complexity makes it difficult for us to support folks when they need it. If you do decide to use it, you’ll want to keep the original <a href="https://eclipse.dev/elk/reference.html" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">Java API reference <img src="out_svelteflow/learn/layouting/layouting-libraries/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a> handy.

We’ve also included a few examples of some of the other layouting algorithms available, including a non-interactive force layout.

### Honourable Mentions<a href="https://svelteflow.dev/learn/layouting/layouting-libraries#honourable-mentions" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

Of course, we can’t go through every layouting library out there: we’d never work on anything else! Here are some other libraries we’ve come across that might be worth taking a look at:

- If you want to use dagre or d3-hierarchy but need to support nodes with different dimensions, both <a href="https://github.com/klortho/d3-flextree" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">d3-flextree <img src="out_svelteflow/learn/layouting/layouting-libraries/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a> and <a href="https://github.com/codeledge/entitree-flex" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">entitree-flex <img src="out_svelteflow/learn/layouting/layouting-libraries/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a> look promising.

- <a href="https://github.com/tgdwyer/WebCola" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">Cola.js <img src="out_svelteflow/learn/layouting/layouting-libraries/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a> looks like a promising option for so-called “constraint-based” layouts. We haven’t had time to properly investigate it yet, but it looks like you can achieve results similar to d3-force but with a lot more control.

## Routing Edges<a href="https://svelteflow.dev/learn/layouting/layouting-libraries#routing-edges" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

If you don’t have any requirements for edge routing, you can use one of the layouting libraries above to position nodes and let the edges fall wherever they may. Otherwise, you’ll want to look into some libraries and techniques for edge routing.

Your options here are more limited than for node layouting, but here are some resources we thought looked promising:

- <a href="https://medium.com/swlh/routing-orthogonal-diagram-connectors-in-javascript-191dc2c5ff70" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">Routing Orthogonal Diagram Connectors in JavaScript <img src="out_svelteflow/learn/layouting/layouting-libraries/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a>

If you do explore some custom edge routing options, consider contributing back to the community by writing a blog post or creating a library!

You can use the editable edges functionality in Svelte Flow as a starting point for implementing a custom edge that can be routed along a specific path.
