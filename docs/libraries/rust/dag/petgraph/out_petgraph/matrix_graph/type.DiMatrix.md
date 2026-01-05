# Type Alias DiMatrix Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/matrix_graph.rs.html#261-268" class="src">Source</a>

``` rust
pub type DiMatrix<N, E, S = RandomState, Null = Option<E>, Ix = u16> = MatrixGraph<N, E, S, Directed, Null, Ix>;
```

Expand description

A `MatrixGraph` with directed edges.

## Aliased Type<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/type.DiMatrix.html#aliased-type" class="anchor">§</a>

``` rust
pub struct DiMatrix<N, E, S = RandomState, Null = Option<E>, Ix = u16> { /* private fields */ }
```
