# Module visit Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/visit/mod.rs.html#1-521" class="src">Source</a>

Expand description

Graph traits and graph traversals.

#### <a href="https://docs.rs/petgraph/latest/petgraph/visit/index.html#the-into--traits" class="doc-anchor">§</a>The `Into-` Traits

Graph traits like [`IntoNeighbors`](https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighbors.html) create iterators and use the same pattern that `IntoIterator` does: the trait takes a reference to a graph, and produces an iterator. These traits are quite composable, but with the limitation that they only use shared references to graphs.

#### <a href="https://docs.rs/petgraph/latest/petgraph/visit/index.html#graph-traversal" class="doc-anchor">§</a>Graph Traversal

[`Dfs`](https://docs.rs/petgraph/latest/petgraph/visit/struct.Dfs.html), [`Bfs`](https://docs.rs/petgraph/latest/petgraph/visit/struct.Bfs.html), [`DfsPostOrder`](https://docs.rs/petgraph/latest/petgraph/visit/struct.DfsPostOrder.html) and [`Topo`](https://docs.rs/petgraph/latest/petgraph/visit/struct.Topo.html) are basic visitors and they use “walker” methods: the visitors don’t hold the graph as borrowed during traversal, only for the `.next()` call on the walker. They can be converted to iterators through the [`Walker`](https://docs.rs/petgraph/latest/petgraph/visit/trait.Walker.html) trait.

There is also the callback based traversal [`depth_first_search`](https://docs.rs/petgraph/latest/petgraph/visit/fn.depth_first_search.html).

#### <a href="https://docs.rs/petgraph/latest/petgraph/visit/index.html#other-graph-traits" class="doc-anchor">§</a>Other Graph Traits

The traits are rather loosely coupled at the moment (which is intentional, but will develop a bit), and there are traits missing that could be added.

Not much is needed to be able to use the visitors on a graph. A graph needs to define [`GraphBase`](https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html), [`IntoNeighbors`](https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighbors.html) and [`Visitable`](https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html) as a minimum.

#### <a href="https://docs.rs/petgraph/latest/petgraph/visit/index.html#graph-trait-implementations" class="doc-anchor">§</a>Graph Trait Implementations

The following table lists the traits that are implemented for each graph type:

|                       | Graph | StableGraph | GraphMap | MatrixGraph | Csr | List |
|-----------------------|:-----:|:-----------:|:--------:|:-----------:|:---:|:----:|
| GraphBase             |   x   |      x      |    x     |      x      |  x  |  x   |
| GraphProp             |   x   |      x      |    x     |      x      |  x  |  x   |
| NodeCount             |   x   |      x      |    x     |      x      |  x  |  x   |
| NodeIndexable         |   x   |      x      |    x     |      x      |  x  |  x   |
| NodeCompactIndexable  |   x   |             |    x     |             |  x  |  x   |
| EdgeCount             |   x   |      x      |    x     |      x      |  x  |  x   |
| EdgeIndexable         |   x   |      x      |    x     |             |     |      |
| Data                  |   x   |      x      |    x     |      x      |  x  |  x   |
| IntoNodeIdentifiers   |   x   |      x      |    x     |      x      |  x  |  x   |
| IntoNodeReferences    |   x   |      x      |    x     |      x      |  x  |  x   |
| IntoEdgeReferences    |   x   |      x      |    x     |      x      |  x  |  x   |
| IntoNeighbors         |   x   |      x      |    x     |      x      |  x  |  x   |
| IntoNeighborsDirected |   x   |      x      |    x     |      x      |     |      |
| IntoEdges             |   x   |      x      |    x     |      x      |  x  |  x   |
| IntoEdgesDirected     |   x   |      x      |    x     |      x      |     |      |
| Visitable             |   x   |      x      |    x     |      x      |  x  |  x   |
| GetAdjacencyMatrix    |   x   |      x      |    x     |      x      |  x  |  x   |

## Structs<a href="https://docs.rs/petgraph/latest/petgraph/visit/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Bfs.html" class="struct" title="struct petgraph::visit::Bfs">Bfs</a>  
A breadth first search (BFS) of a graph.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Dfs.html" class="struct" title="struct petgraph::visit::Dfs">Dfs</a>  
Visit nodes of a graph in a depth-first-search (DFS) emitting nodes in preorder (when they are first discovered).

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.DfsPostOrder.html" class="struct" title="struct petgraph::visit::DfsPostOrder">DfsPostOrder</a>  
Visit nodes in a depth-first-search (DFS) emitting nodes in postorder (each node after all its descendants have been emitted).

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html" class="struct" title="struct petgraph::visit::EdgeFiltered">EdgeFiltered</a>  
An edge-filtering graph adaptor.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFilteredEdges.html" class="struct" title="struct petgraph::visit::EdgeFilteredEdges">EdgeFilteredEdges</a>  
A filtered edges iterator.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFilteredNeighbors.html" class="struct" title="struct petgraph::visit::EdgeFilteredNeighbors">EdgeFilteredNeighbors</a>  
A filtered neighbors iterator.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFilteredNeighborsDirected.html" class="struct" title="struct petgraph::visit::EdgeFilteredNeighborsDirected">EdgeFilteredNeighborsDirected</a>  
A filtered neighbors-directed iterator.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.MaybeReversedEdgeReference.html" class="struct" title="struct petgraph::visit::MaybeReversedEdgeReference">MaybeReversedEdgeReference</a>  
An edge reference which may reverse the edge orientation.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.MaybeReversedEdgeReferences.html" class="struct" title="struct petgraph::visit::MaybeReversedEdgeReferences">MaybeReversedEdgeReferences</a>  
An edges iterator which may reverse the edge orientation.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.MaybeReversedEdges.html" class="struct" title="struct petgraph::visit::MaybeReversedEdges">MaybeReversedEdges</a>  
An edges iterator which may reverse the edge orientation.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.NodeFiltered.html" class="struct" title="struct petgraph::visit::NodeFiltered">NodeFiltered</a>  
A node-filtering graph adaptor.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.NodeFilteredEdgeReferences.html" class="struct" title="struct petgraph::visit::NodeFilteredEdgeReferences">NodeFilteredEdgeReferences</a>  
A filtered edges iterator.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.NodeFilteredEdges.html" class="struct" title="struct petgraph::visit::NodeFilteredEdges">NodeFilteredEdges</a>  
A filtered edges iterator.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.NodeFilteredNeighbors.html" class="struct" title="struct petgraph::visit::NodeFilteredNeighbors">NodeFilteredNeighbors</a>  
A filtered neighbors iterator.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.NodeFilteredNodes.html" class="struct" title="struct petgraph::visit::NodeFilteredNodes">NodeFilteredNodes</a>  
A filtered node references iterator.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Reversed.html" class="struct" title="struct petgraph::visit::Reversed">Reversed</a>  
An edge-reversing graph adaptor.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdgeReference.html" class="struct" title="struct petgraph::visit::ReversedEdgeReference">ReversedEdgeReference</a>  
A reversed edge reference

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdgeReferences.html" class="struct" title="struct petgraph::visit::ReversedEdgeReferences">ReversedEdgeReferences</a>  
A reversed edge references iterator.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html" class="struct" title="struct petgraph::visit::ReversedEdges">ReversedEdges</a>  
A reversed edges iterator.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Time.html" class="struct" title="struct petgraph::visit::Time">Time</a>  
Strictly monotonically increasing event time for a depth first search.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Topo.html" class="struct" title="struct petgraph::visit::Topo">Topo</a>  
A topological order traversal for a graph.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html" class="struct" title="struct petgraph::visit::UndirectedAdaptor">UndirectedAdaptor</a>  
An edge direction removing graph adaptor.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.WalkerIter.html" class="struct" title="struct petgraph::visit::WalkerIter">WalkerIter</a>  
A walker and its context wrapped into an iterator.

## Enums<a href="https://docs.rs/petgraph/latest/petgraph/visit/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/enum.Control.html" class="enum" title="enum petgraph::visit::Control">Control</a>  
Control flow for `depth_first_search` callbacks.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/enum.DfsEvent.html" class="enum" title="enum petgraph::visit::DfsEvent">DfsEvent</a>  
A depth first search (DFS) visitor event.

## Traits<a href="https://docs.rs/petgraph/latest/petgraph/visit/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.ControlFlow.html" class="trait" title="trait petgraph::visit::ControlFlow">ControlFlow</a>  
Control flow for callbacks.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html" class="trait" title="trait petgraph::visit::Data">Data</a>  
Define associated data for nodes and edges

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeCount.html" class="trait" title="trait petgraph::visit::EdgeCount">EdgeCount</a>  
A graph with a known edge count.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeIndexable.html" class="trait" title="trait petgraph::visit::EdgeIndexable">EdgeIndexable</a>  
The graph’s `NodeId`s map to indices

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html" class="trait" title="trait petgraph::visit::EdgeRef">EdgeRef</a>  
An edge reference.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.FilterEdge.html" class="trait" title="trait petgraph::visit::FilterEdge">FilterEdge</a>  
A graph filter for edges

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.FilterNode.html" class="trait" title="trait petgraph::visit::FilterNode">FilterNode</a>  
A graph filter for nodes.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html" class="trait" title="trait petgraph::visit::GetAdjacencyMatrix">GetAdjacencyMatrix</a>  
Create or access the adjacency matrix of a graph.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html" class="trait" title="trait petgraph::visit::GraphBase">GraphBase</a>  
Base graph trait: defines the associated node identifier and edge identifier types.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphProp.html" class="trait" title="trait petgraph::visit::GraphProp">GraphProp</a>  
Edge kind property (directed or undirected edges)

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphRef.html" class="trait" title="trait petgraph::visit::GraphRef">GraphRef</a>  
A copyable reference to a graph.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgeReferences.html" class="trait" title="trait petgraph::visit::IntoEdgeReferences">IntoEdgeReferences</a>  
Access to the sequence of the graph’s edges

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdges.html" class="trait" title="trait petgraph::visit::IntoEdges">IntoEdges</a>  
Access to the edges of each node.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgesDirected.html" class="trait" title="trait petgraph::visit::IntoEdgesDirected">IntoEdgesDirected</a>  
Access to all edges of each node, in the specified direction.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighbors.html" class="trait" title="trait petgraph::visit::IntoNeighbors">IntoNeighbors</a>  
Access to the neighbors of each node

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighborsDirected.html" class="trait" title="trait petgraph::visit::IntoNeighborsDirected">IntoNeighborsDirected</a>  
Access to the neighbors of each node, through incoming or outgoing edges.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeIdentifiers.html" class="trait" title="trait petgraph::visit::IntoNodeIdentifiers">IntoNodeIdentifiers</a>  
Access to the sequence of the graph’s `NodeId`s.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeReferences.html" class="trait" title="trait petgraph::visit::IntoNodeReferences">IntoNodeReferences</a>  
Access to the sequence of the graph’s nodes

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeCompactIndexable.html" class="trait" title="trait petgraph::visit::NodeCompactIndexable">NodeCompactIndexable</a>  
The graph’s `NodeId`s map to indices, in a range without holes.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeCount.html" class="trait" title="trait petgraph::visit::NodeCount">NodeCount</a>  
A graph with a known node count.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeIndexable.html" class="trait" title="trait petgraph::visit::NodeIndexable">NodeIndexable</a>  
The graph’s `NodeId`s map to indices

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeRef.html" class="trait" title="trait petgraph::visit::NodeRef">NodeRef</a>  
A node reference.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.VisitMap.html" class="trait" title="trait petgraph::visit::VisitMap">VisitMap</a>  
A mapping for storing the visited status for NodeId `N`.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html" class="trait" title="trait petgraph::visit::Visitable">Visitable</a>  
A graph that can create a map that tracks the visited status of its nodes.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Walker.html" class="trait" title="trait petgraph::visit::Walker">Walker</a>  
A walker is a traversal state, but where part of the traversal information is supplied manually to each next call.

## Functions<a href="https://docs.rs/petgraph/latest/petgraph/visit/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/fn.depth_first_search.html" class="fn" title="fn petgraph::visit::depth_first_search">depth_first_search</a>  
A recursive depth first search.
