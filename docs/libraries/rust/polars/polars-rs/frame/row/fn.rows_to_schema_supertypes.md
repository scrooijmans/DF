# Function rows_to_schema_supertypes Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/frame/row/mod.rs.html#158-161" class="src">Source</a>

``` rust
pub fn rows_to_schema_supertypes(
    rows: &[Row<'_>],
    infer_schema_length: Option<usize>,
) -> Result<Schema<DataType>, PolarsError>
```

Expand description

Infer the schema of rows by determining the supertype of the values.

Field names are set as `column_0`, `column_1`, and so on.
