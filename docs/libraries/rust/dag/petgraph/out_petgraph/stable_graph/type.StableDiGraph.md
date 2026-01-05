# Type Alias StableDiGraph Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/graph_impl/stable_graph/mod.rs.html#94" class="src">Source</a>

``` rust
pub type StableDiGraph<N, E, Ix = DefaultIx> = StableGraph<N, E, Directed, Ix>;
```

Expand description

A `StableGraph` with directed edges.

For example, an edge from *1* to *2* is distinct from an edge from *2* to *1*.

## Aliased Type<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/type.StableDiGraph.html#aliased-type" class="anchor">§</a>

``` rust
pub struct StableDiGraph<N, E, Ix = DefaultIx> { /* private fields */ }
```
