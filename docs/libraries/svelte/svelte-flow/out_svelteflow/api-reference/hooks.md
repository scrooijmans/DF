# Hooks

## [useConnection()](https://svelteflow.dev/api-reference/hooks/use-connection)

The useConnection hook returns the current connection when there is an active connection interaction. If no connection interaction is active, it returns null for every property. A typical use case for this hook is to colorize handles based on a certain condition (e.g. if the connection is valid or not).

<a href="https://svelteflow.dev/api-reference/hooks/use-connection" class="block _mt-8 text-sm text-primary text-right">Read more <img src="out_svelteflow/api-reference/hooks/index_media/47746133fc075f1f8fa47b8416ecbcc8eab2128f.svg" class="inline w-3 h-3" /></a>

## [useEdges()](https://svelteflow.dev/api-reference/hooks/use-edges)

The useEdges hook returns an array of the current edges store.

<a href="https://svelteflow.dev/api-reference/hooks/use-edges" class="block _mt-8 text-sm text-primary text-right">Read more <img src="out_svelteflow/api-reference/hooks/index_media/47746133fc075f1f8fa47b8416ecbcc8eab2128f.svg" class="inline w-3 h-3" /></a>

## [useInternalNode()](https://svelteflow.dev/api-reference/hooks/use-internal-node)

The useInternalNode hook returns an internal node.

<a href="https://svelteflow.dev/api-reference/hooks/use-internal-node" class="block _mt-8 text-sm text-primary text-right">Read more <img src="out_svelteflow/api-reference/hooks/index_media/47746133fc075f1f8fa47b8416ecbcc8eab2128f.svg" class="inline w-3 h-3" /></a>

## [useNodeConnections()](https://svelteflow.dev/api-reference/hooks/use-node-connections)

This hook returns an array of connected edges. Components that use this hook will re-render whenever any edge changes.

<a href="https://svelteflow.dev/api-reference/hooks/use-node-connections" class="block _mt-8 text-sm text-primary text-right">Read more <img src="out_svelteflow/api-reference/hooks/index_media/47746133fc075f1f8fa47b8416ecbcc8eab2128f.svg" class="inline w-3 h-3" /></a>

## [useNodes()](https://svelteflow.dev/api-reference/hooks/use-nodes)

This hook returns a store of the current nodes array. When you subscribe to this store, it will trigger whenever the nodes array changes. This happens when nodes are added, removed, or updated (dragged for example).

<a href="https://svelteflow.dev/api-reference/hooks/use-nodes" class="block _mt-8 text-sm text-primary text-right">Read more <img src="out_svelteflow/api-reference/hooks/index_media/47746133fc075f1f8fa47b8416ecbcc8eab2128f.svg" class="inline w-3 h-3" /></a>

## [useNodesData()](https://svelteflow.dev/api-reference/hooks/use-nodes-data)

With this hook you can receive the data of the passed node ids.

<a href="https://svelteflow.dev/api-reference/hooks/use-nodes-data" class="block _mt-8 text-sm text-primary text-right">Read more <img src="out_svelteflow/api-reference/hooks/index_media/47746133fc075f1f8fa47b8416ecbcc8eab2128f.svg" class="inline w-3 h-3" /></a>

## [useNodesInitialized()](https://svelteflow.dev/api-reference/hooks/use-nodes-initialized)

The useNodesInitialized hook can be used to check if all nodes are initialized.

<a href="https://svelteflow.dev/api-reference/hooks/use-nodes-initialized" class="block _mt-8 text-sm text-primary text-right">Read more <img src="out_svelteflow/api-reference/hooks/index_media/47746133fc075f1f8fa47b8416ecbcc8eab2128f.svg" class="inline w-3 h-3" /></a>

## [useOnSelectionChange()](https://svelteflow.dev/api-reference/hooks/use-on-selection-change)

This hook lets you listen for changes to both node and edge selection. As the name implies, the callback you provide will be called whenever the selection of either nodes or edges changes.

<a href="https://svelteflow.dev/api-reference/hooks/use-on-selection-change" class="block _mt-8 text-sm text-primary text-right">Read more <img src="out_svelteflow/api-reference/hooks/index_media/47746133fc075f1f8fa47b8416ecbcc8eab2128f.svg" class="inline w-3 h-3" /></a>

## [useStore()](https://svelteflow.dev/api-reference/hooks/use-store)

The useStore hook can be used to access the internal store of the Svelte Flow.

<a href="https://svelteflow.dev/api-reference/hooks/use-store" class="block _mt-8 text-sm text-primary text-right">Read more <img src="out_svelteflow/api-reference/hooks/index_media/47746133fc075f1f8fa47b8416ecbcc8eab2128f.svg" class="inline w-3 h-3" /></a>

## [useSvelteFlow()](https://svelteflow.dev/api-reference/hooks/use-svelte-flow)

The useSvelteFlow hook returns functions to update the viewport, transform coordinates or get node intersections for example.

<a href="https://svelteflow.dev/api-reference/hooks/use-svelte-flow" class="block _mt-8 text-sm text-primary text-right">Read more <img src="out_svelteflow/api-reference/hooks/index_media/47746133fc075f1f8fa47b8416ecbcc8eab2128f.svg" class="inline w-3 h-3" /></a>

## [useUpdateNodeInternals()](https://svelteflow.dev/api-reference/hooks/use-update-node-internals)

When you programmatically add or remove handles to a node or update a node's handle position, you need to inform Svelte Flow about it by using this hook. This will update the internal dimensions of the node and properly reposition handles on the canvas if necessary.

<a href="https://svelteflow.dev/api-reference/hooks/use-update-node-internals" class="block _mt-8 text-sm text-primary text-right">Read more <img src="out_svelteflow/api-reference/hooks/index_media/47746133fc075f1f8fa47b8416ecbcc8eab2128f.svg" class="inline w-3 h-3" /></a>
