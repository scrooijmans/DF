# Trait FilterEdge Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/visit/filter.rs.html#358-361" class="src">Source</a>

``` rust
pub trait FilterEdge<Edge> {
    // Required method
    fn include_edge(&self, edge: Edge) -> bool;
}
```

Expand description

A graph filter for edges

## Required Methods<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.FilterEdge.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.FilterEdge.html#tymethod.include_edge" class="fn">include_edge</a>(&self, edge: Edge) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Return true to have the edge be part of the graph

## Implementors<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.FilterEdge.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.FilterEdge.html#impl-FilterEdge%3CN%3E-for-F" class="anchor">§</a>

### impl\<F, N\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.FilterEdge.html" class="trait" title="trait petgraph::visit::FilterEdge">FilterEdge</a>\<N\> for F

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(N) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,
