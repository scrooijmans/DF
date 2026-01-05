# Function rows_to_supertypes Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/frame/row/mod.rs.html#168-171" class="src">Source</a>

``` rust
pub fn rows_to_supertypes(
    rows: &[Row<'_>],
    infer_schema_length: Option<usize>,
) -> Result<Vec<DataType>, PolarsError>
```

Expand description

Infer the schema data types of rows by determining the supertype of the values.
