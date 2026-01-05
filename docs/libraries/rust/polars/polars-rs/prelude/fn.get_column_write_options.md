# Function get_column_write_options Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/parquet/write/writer.rs.html#262-265" class="src">Source</a>

``` rust
pub fn get_column_write_options(
    schema: &Schema<Field>,
    field_overwrites: &[ParquetFieldOverwrites],
) -> Vec<ColumnWriteOptions>
```

Available on **crate feature `polars-io`** only.
