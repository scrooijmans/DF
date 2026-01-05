# Type Alias StableUnGraph Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/graph_impl/stable_graph/mod.rs.html#100" class="src">Source</a>

``` rust
pub type StableUnGraph<N, E, Ix = DefaultIx> = StableGraph<N, E, Undirected, Ix>;
```

Expand description

A `StableGraph` with undirected edges.

For example, an edge between *1* and *2* is equivalent to an edge between *2* and *1*.

## Aliased Type<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/type.StableUnGraph.html#aliased-type" class="anchor">§</a>

``` rust
pub struct StableUnGraph<N, E, Ix = DefaultIx> { /* private fields */ }
```
