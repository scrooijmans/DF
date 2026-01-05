# Function rows_to_schema_first_non_null Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/frame/row/mod.rs.html#190-193" class="src">Source</a>

``` rust
pub fn rows_to_schema_first_non_null(
    rows: &[Row<'_>],
    infer_schema_length: Option<usize>,
) -> Result<Schema<DataType>, PolarsError>
```

Expand description

Infer schema from rows and set the first no null type as column data type.
