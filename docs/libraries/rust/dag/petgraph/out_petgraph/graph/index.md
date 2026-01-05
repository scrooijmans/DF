# Module graph Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/lib.rs.html#528" class="src">Source</a>

Expand description

`Graph<N, E, Ty, Ix>` is a graph datastructure using an adjacency list representation.

## Structs<a href="https://docs.rs/petgraph/latest/petgraph/graph/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.Edge.html" class="struct" title="struct petgraph::graph::Edge">Edge</a>  
The graph’s edge type.

<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeIndex.html" class="struct" title="struct petgraph::graph::EdgeIndex">EdgeIndex</a>  
Edge identifier.

<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeIndices.html" class="struct" title="struct petgraph::graph::EdgeIndices">EdgeIndices</a>  
Iterator over the edge indices of a graph.

<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeReference.html" class="struct" title="struct petgraph::graph::EdgeReference">EdgeReference</a>  
Reference to a `Graph` edge.

<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeReferences.html" class="struct" title="struct petgraph::graph::EdgeReferences">EdgeReferences</a>  
Iterator over all edges of a graph.

<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeWeightsMut.html" class="struct" title="struct petgraph::graph::EdgeWeightsMut">EdgeWeightsMut</a>  
Iterator yielding mutable access to all edge weights.

<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.Edges.html" class="struct" title="struct petgraph::graph::Edges">Edges</a>  
Iterator over the edges of from or to a node

<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgesConnecting.html" class="struct" title="struct petgraph::graph::EdgesConnecting">EdgesConnecting</a>  
Iterator over the multiple directed edges connecting a source node to a target node

<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.Externals.html" class="struct" title="struct petgraph::graph::Externals">Externals</a>  
An iterator over either the nodes without edges to them or from them.

<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.Frozen.html" class="struct" title="struct petgraph::graph::Frozen">Frozen</a>  
`Frozen` is a graph wrapper.

<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.Graph.html" class="struct" title="struct petgraph::graph::Graph">Graph</a>  
`Graph<N, E, Ty, Ix>` is a graph datastructure using an adjacency list representation.

<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.Neighbors.html" class="struct" title="struct petgraph::graph::Neighbors">Neighbors</a>  
Iterator over the neighbors of a node.

<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.Node.html" class="struct" title="struct petgraph::graph::Node">Node</a>  
The graph’s node type.

<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>  
Node identifier.

<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndices.html" class="struct" title="struct petgraph::graph::NodeIndices">NodeIndices</a>  
Iterator over the node indices of a graph.

<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeReferences.html" class="struct" title="struct petgraph::graph::NodeReferences">NodeReferences</a>  
Iterator over all nodes of a graph.

<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeWeightsMut.html" class="struct" title="struct petgraph::graph::NodeWeightsMut">NodeWeightsMut</a>  
Iterator yielding mutable access to all node weights.

<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.WalkNeighbors.html" class="struct" title="struct petgraph::graph::WalkNeighbors">WalkNeighbors</a>  
A “walker” object that can be used to step through the edge list of a node.

## Enums<a href="https://docs.rs/petgraph/latest/petgraph/graph/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/graph/enum.GraphError.html" class="enum" title="enum petgraph::graph::GraphError">GraphError</a>  
The error type for fallible `Graph` & `StableGraph` operations.

## Traits<a href="https://docs.rs/petgraph/latest/petgraph/graph/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.GraphIndex.html" class="trait" title="trait petgraph::graph::GraphIndex">GraphIndex</a>  
A `GraphIndex` is a node or edge index.

<a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>  
Trait for the unsigned integer type used for node and edge indices.

## Functions<a href="https://docs.rs/petgraph/latest/petgraph/graph/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/graph/fn.edge_index.html" class="fn" title="fn petgraph::graph::edge_index">edge_index</a>  
Short version of `EdgeIndex::new`

<a href="https://docs.rs/petgraph/latest/petgraph/graph/fn.node_index.html" class="fn" title="fn petgraph::graph::node_index">node_index</a>  
Short version of `NodeIndex::new`

## Type Aliases<a href="https://docs.rs/petgraph/latest/petgraph/graph/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/graph/type.DefaultIx.html" class="type" title="type petgraph::graph::DefaultIx">DefaultIx</a>  
The default integer type for graph indices. `u32` is the default to reduce the size of the graph’s data and improve performance in the common case.

<a href="https://docs.rs/petgraph/latest/petgraph/graph/type.DiGraph.html" class="type" title="type petgraph::graph::DiGraph">DiGraph</a>  
A `Graph` with directed edges.

<a href="https://docs.rs/petgraph/latest/petgraph/graph/type.UnGraph.html" class="type" title="type petgraph::graph::UnGraph">UnGraph</a>  
A `Graph` with undirected edges.
