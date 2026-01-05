# Type Alias DefaultIx Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/graph_impl/mod.rs.html#29" class="src">Source</a>

``` rust
pub type DefaultIx = u32;
```

Expand description

The default integer type for graph indices. `u32` is the default to reduce the size of the graph’s data and improve performance in the common case.

Used for node and edge indices in `Graph` and `StableGraph`, used for node indices in `Csr`.
