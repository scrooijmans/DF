# Module matrix_graph Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/matrix_graph.rs.html#1-2090" class="src">Source</a>

Expand description

`MatrixGraph<N, E, Ty, NullN, NullE, Ix>` is a graph datastructure backed by an adjacency matrix.

## Re-exports<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/index.html#reexports" class="anchor">§</a>

`pub use crate::graph::`<a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType"><code>IndexType</code></a>`;`

## Structs<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.EdgeReferences.html" class="struct" title="struct petgraph::matrix_graph::EdgeReferences">EdgeReferences</a>  
Iterator over all edges of a graph.

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.Edges.html" class="struct" title="struct petgraph::matrix_graph::Edges">Edges</a>  
Iterator over the edges of from or to a node

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html" class="struct" title="struct petgraph::matrix_graph::MatrixGraph">MatrixGraph</a>  
`MatrixGraph<N, E, Ty, Null>` is a graph datastructure using an adjacency matrix representation.

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.Neighbors.html" class="struct" title="struct petgraph::matrix_graph::Neighbors">Neighbors</a>  
Iterator over the neighbors of a node.

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.NodeIdentifiers.html" class="struct" title="struct petgraph::matrix_graph::NodeIdentifiers">NodeIdentifiers</a>  
Iterator over the node identifiers of a graph.

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.NodeReferences.html" class="struct" title="struct petgraph::matrix_graph::NodeReferences">NodeReferences</a>  
Iterator over all nodes of a graph.

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.NotZero.html" class="struct" title="struct petgraph::matrix_graph::NotZero">NotZero</a>  
`NotZero` is used to optimize the memory usage of edge weights `E` in a [`MatrixGraph`](https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html), replacing the default `Option<E>` sentinel.

## Enums<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/enum.MatrixError.html" class="enum" title="enum petgraph::matrix_graph::MatrixError">MatrixError</a>  
The error type for fallible `MatrixGraph` operations.

## Traits<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Nullable.html" class="trait" title="trait petgraph::matrix_graph::Nullable">Nullable</a>  
Wrapper trait for an `Option`, allowing user-defined structs to be input as containers when defining a null element.

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html" class="trait" title="trait petgraph::matrix_graph::Zero">Zero</a>  
Base trait for types that can be wrapped in a [`NotZero`](https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.NotZero.html).

## Functions<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/fn.node_index.html" class="fn" title="fn petgraph::matrix_graph::node_index">node_index</a>  
Short version of `NodeIndex::new` (with Ix = `DefaultIx`)

## Type Aliases<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/type.DiMatrix.html" class="type" title="type petgraph::matrix_graph::DiMatrix">DiMatrix</a>  
A `MatrixGraph` with directed edges.

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/type.NodeIndex.html" class="type" title="type petgraph::matrix_graph::NodeIndex">NodeIndex</a>  
Node identifier.

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/type.UnMatrix.html" class="type" title="type petgraph::matrix_graph::UnMatrix">UnMatrix</a>  
A `MatrixGraph` with undirected edges.
