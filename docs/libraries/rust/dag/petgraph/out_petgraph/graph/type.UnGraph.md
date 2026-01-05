# Type Alias UnGraph Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/graph_impl/mod.rs.html#406" class="src">Source</a>

``` rust
pub type UnGraph<N, E, Ix = DefaultIx> = Graph<N, E, Undirected, Ix>;
```

Expand description

A `Graph` with undirected edges.

For example, an edge between *1* and *2* is equivalent to an edge between *2* and *1*.

## Aliased Type<a href="https://docs.rs/petgraph/latest/petgraph/graph/type.UnGraph.html#aliased-type" class="anchor">§</a>

``` rust
pub struct UnGraph<N, E, Ix = DefaultIx> { /* private fields */ }
```
