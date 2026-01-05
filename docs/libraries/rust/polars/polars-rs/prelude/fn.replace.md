# Function replace Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/series/ops/replace.rs.html#31" class="src">Source</a>

``` rust
pub fn replace(
    s: &Series,
    old: &ChunkedArray<ListType>,
    new: &ChunkedArray<ListType>,
) -> Result<Series, PolarsError>
```

Available on **crate feature `polars-ops`** only.

Expand description

Replace values by different values of the same data type.
