# Type Alias ChunkLenIter Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/mod.rs.html#66" class="src">Source</a>

``` rust
pub type ChunkLenIter<'a> = Map<Iter<'a, Box<dyn Array>>, fn(&Box<dyn Array>) -> usize>;
```

## Aliased Type<a href="https://docs.rs/polars/latest/polars/chunked_array/type.ChunkLenIter.html#aliased-type" class="anchor">§</a>

``` rust
pub struct ChunkLenIter<'a> { /* private fields */ }
```
