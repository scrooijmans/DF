# Function replace_or_default Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/series/ops/replace.rs.html#64-70" class="src">Source</a>

``` rust
pub fn replace_or_default(
    s: &Series,
    old: &ChunkedArray<ListType>,
    new: &ChunkedArray<ListType>,
    default: &Series,
    return_dtype: Option<DataType>,
) -> Result<Series, PolarsError>
```

Available on **crate feature `polars-ops`** only.

Expand description

Replace all values by different values.

Unmatched values are replaced by a default value.
