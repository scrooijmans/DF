# Function init_buffers Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/csv/read/buffer.rs.html#490-497" class="src">Source</a>

``` rust
pub fn init_buffers(
    projection: &[usize],
    capacity: usize,
    schema: &Schema<DataType>,
    quote_char: Option<u8>,
    encoding: CsvEncoding,
    decimal_comma: bool,
) -> Result<Vec<Buffer>, PolarsError>
```

Available on **crate feature `polars-io`** only.
