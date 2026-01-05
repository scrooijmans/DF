# Type Alias DiGraphMap Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/graphmap.rs.html#47-48" class="src">Source</a>

``` rust
pub type DiGraphMap<N, E, S = RandomState> = GraphMap<N, E, Directed, S>;
```

Expand description

A `GraphMap` with directed edges.

For example, an edge from *1* to *2* is distinct from an edge from *2* to *1*.

## Aliased Type<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/type.DiGraphMap.html#aliased-type" class="anchor">§</a>

``` rust
pub struct DiGraphMap<N, E, S = RandomState> { /* private fields */ }
```
