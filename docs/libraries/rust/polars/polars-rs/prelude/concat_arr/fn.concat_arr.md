# Function concat_arr Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/series/ops/concat_arr.rs.html#15" class="src">Source</a>

``` rust
pub fn concat_arr(
    args: &[Column],
    dtype: &DataType,
) -> Result<Column, PolarsError>
```

Available on **crate feature `polars-ops`** only.

Expand description

Note: The caller must ensure all columns in `args` have the same type.

## <a href="https://docs.rs/polars/latest/polars/prelude/concat_arr/fn.concat_arr.html#panics" class="doc-anchor">§</a>Panics

Panics if

- `args` is empty
- `dtype` is not a `DataType::Array`
