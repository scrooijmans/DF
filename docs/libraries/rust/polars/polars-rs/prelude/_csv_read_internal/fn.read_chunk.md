# Function read_chunk Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/csv/read/read_impl.rs.html#535-547" class="src">Source</a>

``` rust
pub fn read_chunk(
    bytes: &[u8],
    parse_options: &CsvParseOptions,
    schema: &Schema<DataType>,
    ignore_errors: bool,
    projection: &[usize],
    bytes_offset_thread: usize,
    capacity: usize,
    null_values: Option<&NullValuesCompiled>,
    chunk_size: usize,
    stop_at_nbytes: usize,
    starting_point_offset: Option<usize>,
) -> Result<DataFrame, PolarsError>
```

Available on **crate feature `polars-io`** only.
