# Type Alias RowGroupIterColumns Copy item path

<a href="https://docs.rs/polars-parquet/0.51.0/x86_64-unknown-linux-gnu/src/polars_parquet/parquet/write/mod.rs.html#23" class="src">Source</a>

``` rust
pub type RowGroupIterColumns<'a, E> = DynIter<'a, Result<DynStreamingIterator<'a, CompressedPage, E>, E>>;
```

Available on **crate feature `polars-io`** only.

## Aliased Type<a href="https://docs.rs/polars/latest/polars/prelude/type.RowGroupIterColumns.html#aliased-type" class="anchor">§</a>

``` rust
pub struct RowGroupIterColumns<'a, E> { /* private fields */ }
```
