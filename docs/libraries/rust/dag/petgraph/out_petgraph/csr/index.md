# Module csr Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/csr.rs.html#1-1241" class="src">Source</a>

Expand description

Compressed Sparse Row (CSR) is a sparse adjacency matrix graph.

## Re-exports<a href="https://docs.rs/petgraph/latest/petgraph/csr/index.html#reexports" class="anchor">§</a>

`pub use crate::graph::`<a href="https://docs.rs/petgraph/latest/petgraph/graph/type.DefaultIx.html" class="type" title="type petgraph::graph::DefaultIx"><code>DefaultIx</code></a>`;`

`pub use crate::graph::`<a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType"><code>IndexType</code></a>`;`

## Structs<a href="https://docs.rs/petgraph/latest/petgraph/csr/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/csr/struct.Csr.html" class="struct" title="struct petgraph::csr::Csr">Csr</a>  
Compressed Sparse Row ([`CSR`](https://en.wikipedia.org/wiki/Sparse_matrix#Compressed_sparse_row_(CSR,_CRS_or_Yale_format))) is a sparse adjacency matrix graph.

<a href="https://docs.rs/petgraph/latest/petgraph/csr/struct.EdgeReference.html" class="struct" title="struct petgraph::csr::EdgeReference">EdgeReference</a>  
<a href="https://docs.rs/petgraph/latest/petgraph/csr/struct.EdgeReferences.html" class="struct" title="struct petgraph::csr::EdgeReferences">EdgeReferences</a>  
<a href="https://docs.rs/petgraph/latest/petgraph/csr/struct.Edges.html" class="struct" title="struct petgraph::csr::Edges">Edges</a>  
<a href="https://docs.rs/petgraph/latest/petgraph/csr/struct.EdgesNotSorted.html" class="struct" title="struct petgraph::csr::EdgesNotSorted">EdgesNotSorted</a>  
Csr creation error: edges were not in sorted order.

<a href="https://docs.rs/petgraph/latest/petgraph/csr/struct.Neighbors.html" class="struct" title="struct petgraph::csr::Neighbors">Neighbors</a>  
<a href="https://docs.rs/petgraph/latest/petgraph/csr/struct.NodeIdentifiers.html" class="struct" title="struct petgraph::csr::NodeIdentifiers">NodeIdentifiers</a>  
<a href="https://docs.rs/petgraph/latest/petgraph/csr/struct.NodeReferences.html" class="struct" title="struct petgraph::csr::NodeReferences">NodeReferences</a>  
Iterator over all nodes of a graph.

## Enums<a href="https://docs.rs/petgraph/latest/petgraph/csr/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/csr/enum.CsrError.html" class="enum" title="enum petgraph::csr::CsrError">CsrError</a>  
The error type for fallible operations with `Csr`.

## Type Aliases<a href="https://docs.rs/petgraph/latest/petgraph/csr/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/csr/type.EdgeIndex.html" class="type" title="type petgraph::csr::EdgeIndex">EdgeIndex</a>  
Csr edge index type, a plain integer.

<a href="https://docs.rs/petgraph/latest/petgraph/csr/type.NodeIndex.html" class="type" title="type petgraph::csr::NodeIndex">NodeIndex</a>  
Csr node index type, a plain integer.
