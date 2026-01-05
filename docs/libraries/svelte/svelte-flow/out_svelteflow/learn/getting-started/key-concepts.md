# Key Concepts

At its core, Svelte Flow is about creating interactive flowgraphs - a collection of nodes connected by edges. While this might sound simple in theory, Svelte Flow provides the foundation for building complex interactive diagrams:

- An infinite, interactive canvas for your flowgraph
- The ability to render and connect nodes with edges
- **Everything is built using standard Svelte components**

To help you understand the terminology we use throughout the documentation, take a look at the example flow below.

<img src="out_svelteflow/learn/getting-started/key-concepts/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/learn/getting-started/key-concepts/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

Here are the key terms you’ll encounter when working with Svelte Flow:

- **Svelte Flow Component**: The main component that renders your flowgraph
  - <a href="https://svelteflow.dev/api-reference/svelte-flow" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><strong>Svelte Flow Props</strong></a>: All configuration and data is passed through props
- **Viewport**: The visible area of your flowgraph that can be panned and zoomed
  - When we say something is “in the viewport,” it means it moves with the viewport’s transformation
- <a href="https://svelteflow.dev/learn/customization/theming" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><strong>Base Styles</strong></a>: The essential CSS required for Svelte Flow to function properly
- <a href="https://svelteflow.dev/learn/getting-started/built-in-components" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><strong>Built-In Components</strong></a>: Ready-to-use components like Controls and MiniMap that enhance your flowgraph

With these concepts in mind, you’re ready to <a href="https://svelteflow.dev/learn/getting-started/installation" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]">install Svelte Flow</a> and start <a href="https://svelteflow.dev/learn/getting-started/building-a-flow" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]">building your first flow</a>!
