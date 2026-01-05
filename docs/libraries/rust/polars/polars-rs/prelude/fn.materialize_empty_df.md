# Function materialize_empty_df Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/parquet/read/utils.rs.html#11-16" class="src">Source</a>

``` rust
pub fn materialize_empty_df(
    projection: Option<&[usize]>,
    reader_schema: &Schema<Field>,
    hive_partition_columns: Option<&[Series]>,
    row_index: Option<&RowIndex>,
) -> DataFrame
```

Available on **crate feature `polars-io`** only.
