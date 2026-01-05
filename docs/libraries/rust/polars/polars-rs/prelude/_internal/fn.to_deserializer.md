# Function to_deserializer Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/parquet/read/mmap.rs.html#52-56" class="src">Source</a>

``` rust
pub fn to_deserializer(
    columns: Vec<(&ColumnChunkMetadata, MemSlice)>,
    field: Field,
    filter: Option<Filter>,
) -> Result<(Vec<Box<dyn Array>>, Bitmap), PolarsError>
```

Available on **crate feature `polars-io`** only.
