# Trait ToGraph6 Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/graph6/graph6_encoder.rs.html#31-33" class="src">Source</a>

``` rust
pub trait ToGraph6 {
    // Required method
    fn graph6_string(&self) -> String;
}
```

Expand description

A graph that can be converted to graph6 format string.

## Required Methods<a href="https://docs.rs/petgraph/latest/petgraph/graph6/trait.ToGraph6.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/graph6/trait.ToGraph6.html#tymethod.graph6_string" class="fn">graph6_string</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

## Implementors<a href="https://docs.rs/petgraph/latest/petgraph/graph6/trait.ToGraph6.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/graph6/trait.ToGraph6.html#impl-ToGraph6-for-Csr%3CN,+E,+Undirected,+Ix%3E" class="anchor">§</a>

### impl\<N, E, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/graph6/trait.ToGraph6.html" class="trait" title="trait petgraph::graph6::ToGraph6">ToGraph6</a> for <a href="https://docs.rs/petgraph/latest/petgraph/csr/struct.Csr.html" class="struct" title="struct petgraph::csr::Csr">Csr</a>\<N, E, <a href="https://docs.rs/petgraph/latest/petgraph/enum.Undirected.html" class="enum" title="enum petgraph::Undirected">Undirected</a>, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/graph6/trait.ToGraph6.html#impl-ToGraph6-for-Graph%3CN,+E,+Undirected,+Ix%3E" class="anchor">§</a>

### impl\<N, E, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/graph6/trait.ToGraph6.html" class="trait" title="trait petgraph::graph6::ToGraph6">ToGraph6</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.Graph.html" class="struct" title="struct petgraph::graph::Graph">Graph</a>\<N, E, <a href="https://docs.rs/petgraph/latest/petgraph/enum.Undirected.html" class="enum" title="enum petgraph::Undirected">Undirected</a>, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/graph6/trait.ToGraph6.html#impl-ToGraph6-for-StableGraph%3CN,+E,+Undirected,+Ix%3E" class="anchor">§</a>

### impl\<N, E, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/graph6/trait.ToGraph6.html" class="trait" title="trait petgraph::graph6::ToGraph6">ToGraph6</a> for <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html" class="struct" title="struct petgraph::stable_graph::StableGraph">StableGraph</a>\<N, E, <a href="https://docs.rs/petgraph/latest/petgraph/enum.Undirected.html" class="enum" title="enum petgraph::Undirected">Undirected</a>, Ix\>

Available on **crate feature `stable_graph`** only.

<a href="https://docs.rs/petgraph/latest/petgraph/graph6/trait.ToGraph6.html#impl-ToGraph6-for-MatrixGraph%3CN,+E,+S,+Undirected,+Null,+Ix%3E" class="anchor">§</a>

### impl\<N, E, S, Null, Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/graph6/trait.ToGraph6.html" class="trait" title="trait petgraph::graph6::ToGraph6">ToGraph6</a> for <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html" class="struct" title="struct petgraph::matrix_graph::MatrixGraph">MatrixGraph</a>\<N, E, S, <a href="https://docs.rs/petgraph/latest/petgraph/enum.Undirected.html" class="enum" title="enum petgraph::Undirected">Undirected</a>, Null, Ix\>

where N: <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/trait.NodeTrait.html" class="trait" title="trait petgraph::graphmap::NodeTrait">NodeTrait</a>, Null: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Nullable.html" class="trait" title="trait petgraph::matrix_graph::Nullable">Nullable</a>\<Wrapped = E\>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a> + <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a>,

Available on **crate feature `matrix_graph`** only.

<a href="https://docs.rs/petgraph/latest/petgraph/graph6/trait.ToGraph6.html#impl-ToGraph6-for-GraphMap%3CN,+E,+Undirected,+S%3E" class="anchor">§</a>

### impl\<N: <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/trait.NodeTrait.html" class="trait" title="trait petgraph::graphmap::NodeTrait">NodeTrait</a>, E, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/graph6/trait.ToGraph6.html" class="trait" title="trait petgraph::graph6::ToGraph6">ToGraph6</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html" class="struct" title="struct petgraph::graphmap::GraphMap">GraphMap</a>\<N, E, <a href="https://docs.rs/petgraph/latest/petgraph/enum.Undirected.html" class="enum" title="enum petgraph::Undirected">Undirected</a>, S\>

Available on **crate feature `graphmap`** only.
