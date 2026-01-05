# Function ensure_matching_dtypes_if_found Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/parquet/read/utils.rs.html#69-72" class="src">Source</a>

``` rust
pub fn ensure_matching_dtypes_if_found(
    schema: &Schema<Field>,
    current_schema: &Schema<Field>,
) -> Result<(), PolarsError>
```

Available on **crate feature `polars-io`** only.

Expand description

Utility to ensure the dtype of the column in `current_schema` matches the dtype in `schema` if that column exists in `schema`.
