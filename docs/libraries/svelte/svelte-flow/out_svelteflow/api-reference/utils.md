# Utils

## [addEdge()](https://svelteflow.dev/api-reference/utils/add-edge)

This util is a convenience function to add a new Edge to an array of edges. It also performs some validation to make sure you don't add an invalid edge or duplicate an existing one.

<a href="https://svelteflow.dev/api-reference/utils/add-edge" class="block _mt-8 text-sm text-primary text-right">Read more <img src="out_svelteflow/api-reference/utils/index_media/47746133fc075f1f8fa47b8416ecbcc8eab2128f.svg" class="inline w-3 h-3" /></a>

## [getBezierPath()](https://svelteflow.dev/api-reference/utils/get-bezier-path)

The getBezierPath util returns everything you need to render a bezier edge between two nodes.

<a href="https://svelteflow.dev/api-reference/utils/get-bezier-path" class="block _mt-8 text-sm text-primary text-right">Read more <img src="out_svelteflow/api-reference/utils/index_media/47746133fc075f1f8fa47b8416ecbcc8eab2128f.svg" class="inline w-3 h-3" /></a>

## [getConnectedEdges()](https://svelteflow.dev/api-reference/utils/get-connected-edges)

Given an array of nodes that may be connected to one another and an array of all your edges, this util gives you an array of edges that connect any of the given nodes together.

<a href="https://svelteflow.dev/api-reference/utils/get-connected-edges" class="block _mt-8 text-sm text-primary text-right">Read more <img src="out_svelteflow/api-reference/utils/index_media/47746133fc075f1f8fa47b8416ecbcc8eab2128f.svg" class="inline w-3 h-3" /></a>

## [getIncomers()](https://svelteflow.dev/api-reference/utils/get-incomers)

This util is used to tell you what nodes, if any, are connected to the given node as the source of an edge.

<a href="https://svelteflow.dev/api-reference/utils/get-incomers" class="block _mt-8 text-sm text-primary text-right">Read more <img src="out_svelteflow/api-reference/utils/index_media/47746133fc075f1f8fa47b8416ecbcc8eab2128f.svg" class="inline w-3 h-3" /></a>

## [getNodesBounds()](https://svelteflow.dev/api-reference/utils/get-nodes-bounds)

Returns the bounding box that contains all the given nodes in an array. This can be useful when combined with getTransformForBounds to calculate the correct transform to fit the given nodes in a viewport.

<a href="https://svelteflow.dev/api-reference/utils/get-nodes-bounds" class="block _mt-8 text-sm text-primary text-right">Read more <img src="out_svelteflow/api-reference/utils/index_media/47746133fc075f1f8fa47b8416ecbcc8eab2128f.svg" class="inline w-3 h-3" /></a>

## [getOutgoers()](https://svelteflow.dev/api-reference/utils/get-outgoers)

This util is used to tell you what nodes, if any, are connected to the given node as the target of an edge.

<a href="https://svelteflow.dev/api-reference/utils/get-outgoers" class="block _mt-8 text-sm text-primary text-right">Read more <img src="out_svelteflow/api-reference/utils/index_media/47746133fc075f1f8fa47b8416ecbcc8eab2128f.svg" class="inline w-3 h-3" /></a>

## [getSmoothStepPath()](https://svelteflow.dev/api-reference/utils/get-smooth-step-path)

The getSmoothStepPath util returns everything you need to render a stepped path between two nodes. The borderRadius property can be used to choose how rounded the corners of those steps are.

<a href="https://svelteflow.dev/api-reference/utils/get-smooth-step-path" class="block _mt-8 text-sm text-primary text-right">Read more <img src="out_svelteflow/api-reference/utils/index_media/47746133fc075f1f8fa47b8416ecbcc8eab2128f.svg" class="inline w-3 h-3" /></a>

## [getStraightPath()](https://svelteflow.dev/api-reference/utils/get-straight-path)

Calculates the straight line path between two points.

<a href="https://svelteflow.dev/api-reference/utils/get-straight-path" class="block _mt-8 text-sm text-primary text-right">Read more <img src="out_svelteflow/api-reference/utils/index_media/47746133fc075f1f8fa47b8416ecbcc8eab2128f.svg" class="inline w-3 h-3" /></a>

## [getViewportForBounds()](https://svelteflow.dev/api-reference/utils/get-viewport-for-bounds)

This util returns the viewport for the given bounds. You might use this to pre-calculate the viewport for a given set of nodes on the server or calculate the viewport for the given bounds \_without\_ changing the viewport directly.

<a href="https://svelteflow.dev/api-reference/utils/get-viewport-for-bounds" class="block _mt-8 text-sm text-primary text-right">Read more <img src="out_svelteflow/api-reference/utils/index_media/47746133fc075f1f8fa47b8416ecbcc8eab2128f.svg" class="inline w-3 h-3" /></a>

## [isEdge()](https://svelteflow.dev/api-reference/utils/is-edge)

Test whether an object is usable as an Edge. In TypeScript this is a type guard that will narrow the type of whatever you pass in to Edge if it returns true.

<a href="https://svelteflow.dev/api-reference/utils/is-edge" class="block _mt-8 text-sm text-primary text-right">Read more <img src="out_svelteflow/api-reference/utils/index_media/47746133fc075f1f8fa47b8416ecbcc8eab2128f.svg" class="inline w-3 h-3" /></a>

## [isNode()](https://svelteflow.dev/api-reference/utils/is-node)

Test whether an object is usable as an Node. In TypeScript this is a type guard that will narrow the type of whatever you pass in to Node if it returns true.

<a href="https://svelteflow.dev/api-reference/utils/is-node" class="block _mt-8 text-sm text-primary text-right">Read more <img src="out_svelteflow/api-reference/utils/index_media/47746133fc075f1f8fa47b8416ecbcc8eab2128f.svg" class="inline w-3 h-3" /></a>
