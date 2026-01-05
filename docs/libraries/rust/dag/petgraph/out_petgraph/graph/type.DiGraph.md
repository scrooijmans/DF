# Type Alias DiGraph Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/graph_impl/mod.rs.html#400" class="src">Source</a>

``` rust
pub type DiGraph<N, E, Ix = DefaultIx> = Graph<N, E, Directed, Ix>;
```

Expand description

A `Graph` with directed edges.

For example, an edge from *1* to *2* is distinct from an edge from *2* to *1*.

## Aliased Type<a href="https://docs.rs/petgraph/latest/petgraph/graph/type.DiGraph.html#aliased-type" class="anchor">§</a>

``` rust
pub struct DiGraph<N, E, Ix = DefaultIx> { /* private fields */ }
```
