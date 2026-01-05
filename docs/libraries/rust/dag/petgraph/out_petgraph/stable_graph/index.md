# Module stable_graph Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/graph_impl/stable_graph/mod.rs.html#1-2616" class="src">Source</a>

Expand description

`StableGraph` keeps indices stable across removals.

Depends on `feature = "stable_graph"`.

## Re-exports<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/index.html#reexports" class="anchor">§</a>

`pub use crate::graph::`<a href="https://docs.rs/petgraph/latest/petgraph/graph/fn.edge_index.html" class="fn" title="fn petgraph::graph::edge_index"><code>edge_index</code></a>`;`

`pub use crate::graph::`<a href="https://docs.rs/petgraph/latest/petgraph/graph/fn.node_index.html" class="fn" title="fn petgraph::graph::node_index"><code>node_index</code></a>`;`

`pub use crate::graph::`<a href="https://docs.rs/petgraph/latest/petgraph/graph/type.DefaultIx.html" class="type" title="type petgraph::graph::DefaultIx"><code>DefaultIx</code></a>`;`

`pub use crate::graph::`<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeIndex.html" class="struct" title="struct petgraph::graph::EdgeIndex"><code>EdgeIndex</code></a>`;`

`pub use crate::graph::`<a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.GraphIndex.html" class="trait" title="trait petgraph::graph::GraphIndex"><code>GraphIndex</code></a>`;`

`pub use crate::graph::`<a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType"><code>IndexType</code></a>`;`

`pub use crate::graph::`<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex"><code>NodeIndex</code></a>`;`

## Structs<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.EdgeIndices.html" class="struct" title="struct petgraph::stable_graph::EdgeIndices">EdgeIndices</a>  
Iterator over the edge indices of a graph.

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.EdgeReference.html" class="struct" title="struct petgraph::stable_graph::EdgeReference">EdgeReference</a>  
Reference to a `StableGraph` edge.

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.EdgeReferences.html" class="struct" title="struct petgraph::stable_graph::EdgeReferences">EdgeReferences</a>  
Iterator over all edges of a graph.

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.Edges.html" class="struct" title="struct petgraph::stable_graph::Edges">Edges</a>  
Iterator over the edges of from or to a node

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.EdgesConnecting.html" class="struct" title="struct petgraph::stable_graph::EdgesConnecting">EdgesConnecting</a>  
Iterator over the multiple directed edges connecting a source node to a target node

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.Externals.html" class="struct" title="struct petgraph::stable_graph::Externals">Externals</a>  
An iterator over either the nodes without edges to them or from them.

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.Neighbors.html" class="struct" title="struct petgraph::stable_graph::Neighbors">Neighbors</a>  
Iterator over the neighbors of a node.

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.NodeIndices.html" class="struct" title="struct petgraph::stable_graph::NodeIndices">NodeIndices</a>  
Iterator over the node indices of a graph.

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.NodeReferences.html" class="struct" title="struct petgraph::stable_graph::NodeReferences">NodeReferences</a>  
Iterator over all nodes of a graph.

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html" class="struct" title="struct petgraph::stable_graph::StableGraph">StableGraph</a>  
`StableGraph<N, E, Ty, Ix>` is a graph datastructure using an adjacency list representation.

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraphEdge.html" class="struct" title="struct petgraph::stable_graph::StableGraphEdge">StableGraphEdge</a>  
<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraphNode.html" class="struct" title="struct petgraph::stable_graph::StableGraphNode">StableGraphNode</a>  
<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.WalkNeighbors.html" class="struct" title="struct petgraph::stable_graph::WalkNeighbors">WalkNeighbors</a>  
A “walker” object that can be used to step through the edge list of a node.

## Type Aliases<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/type.StableDiGraph.html" class="type" title="type petgraph::stable_graph::StableDiGraph">StableDiGraph</a>  
A `StableGraph` with directed edges.

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/type.StableUnGraph.html" class="type" title="type petgraph::stable_graph::StableUnGraph">StableUnGraph</a>  
A `StableGraph` with undirected edges.
