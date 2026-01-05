# Type Alias UnMatrix Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/matrix_graph.rs.html#271-278" class="src">Source</a>

``` rust
pub type UnMatrix<N, E, S = RandomState, Null = Option<E>, Ix = u16> = MatrixGraph<N, E, S, Undirected, Null, Ix>;
```

Expand description

A `MatrixGraph` with undirected edges.

## Aliased Type<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/type.UnMatrix.html#aliased-type" class="anchor">§</a>

``` rust
pub struct UnMatrix<N, E, S = RandomState, Null = Option<E>, Ix = u16> { /* private fields */ }
```
