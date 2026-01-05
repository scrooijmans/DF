# Type Alias UnGraphMap Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/graphmap.rs.html#41-42" class="src">Source</a>

``` rust
pub type UnGraphMap<N, E, S = RandomState> = GraphMap<N, E, Undirected, S>;
```

Expand description

A `GraphMap` with undirected edges.

For example, an edge between *1* and *2* is equivalent to an edge between *2* and *1*.

## Aliased Type<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/type.UnGraphMap.html#aliased-type" class="anchor">§</a>

``` rust
pub struct UnGraphMap<N, E, S = RandomState> { /* private fields */ }
```
