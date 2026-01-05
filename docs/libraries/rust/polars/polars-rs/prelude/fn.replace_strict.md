# Function replace_strict Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/series/ops/replace.rs.html#123-128" class="src">Source</a>

``` rust
pub fn replace_strict(
    s: &Series,
    old: &ChunkedArray<ListType>,
    new: &ChunkedArray<ListType>,
    return_dtype: Option<DataType>,
) -> Result<Series, PolarsError>
```

Available on **crate feature `polars-ops`** only.

Expand description

Replace all values by different values.

Raises an error if not all values were replaced.
