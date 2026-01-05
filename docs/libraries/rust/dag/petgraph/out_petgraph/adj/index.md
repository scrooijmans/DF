# Module adj Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/adj.rs.html#1-657" class="src">Source</a>

Expand description

Simple adjacency list.

## Re-exports<a href="https://docs.rs/petgraph/latest/petgraph/adj/index.html#reexports" class="anchor">§</a>

`pub use crate::graph::`<a href="https://docs.rs/petgraph/latest/petgraph/graph/type.DefaultIx.html" class="type" title="type petgraph::graph::DefaultIx"><code>DefaultIx</code></a>`;`

`pub use crate::graph::`<a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType"><code>IndexType</code></a>`;`

## Structs<a href="https://docs.rs/petgraph/latest/petgraph/adj/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeIndex.html" class="struct" title="struct petgraph::adj::EdgeIndex">EdgeIndex</a>  
Adjacency list edge index type, a pair of integers.

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeIndices.html" class="struct" title="struct petgraph::adj::EdgeIndices">EdgeIndices</a>  
<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeReference.html" class="struct" title="struct petgraph::adj::EdgeReference">EdgeReference</a>  
A reference to an edge of the graph.

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeReferences.html" class="struct" title="struct petgraph::adj::EdgeReferences">EdgeReferences</a>  
An iterator over the [`EdgeReference`](https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeReference.html "struct petgraph::adj::EdgeReference") of all the edges of the graph.

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html" class="struct" title="struct petgraph::adj::List">List</a>  
An adjacency list with labeled edges.

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.Neighbors.html" class="struct" title="struct petgraph::adj::Neighbors">Neighbors</a>  
An iterator over the indices of the neighbors of a node.

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.NodeIndices.html" class="struct" title="struct petgraph::adj::NodeIndices">NodeIndices</a>  
An iterator over all node indices in the graph.

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.OutgoingEdgeIndices.html" class="struct" title="struct petgraph::adj::OutgoingEdgeIndices">OutgoingEdgeIndices</a>  
An Iterator over the indices of the outgoing edges from a node.

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.OutgoingEdgeReferences.html" class="struct" title="struct petgraph::adj::OutgoingEdgeReferences">OutgoingEdgeReferences</a>  
Iterator over the [`EdgeReference`](https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeReference.html "struct petgraph::adj::EdgeReference") of the outgoing edges from a node.

## Type Aliases<a href="https://docs.rs/petgraph/latest/petgraph/adj/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/adj/type.NodeIndex.html" class="type" title="type petgraph::adj::NodeIndex">NodeIndex</a>  
Adjacency list node index type, a plain integer.

<a href="https://docs.rs/petgraph/latest/petgraph/adj/type.UnweightedList.html" class="type" title="type petgraph::adj::UnweightedList">UnweightedList</a>  
A very simple adjacency list with no node or label weights.
