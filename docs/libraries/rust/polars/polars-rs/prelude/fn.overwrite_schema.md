# Function overwrite_schema Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/utils/other.rs.html#147" class="src">Source</a>

``` rust
pub fn overwrite_schema(
    schema: &mut Schema<DataType>,
    overwriting_schema: &Schema<DataType>,
) -> Result<(), PolarsError>
```

Available on **crate feature `polars-io`** only.
