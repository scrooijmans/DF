# Function infer_file_schema Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/csv/read/schema_inference.rs.html#521-531" class="src">Source</a>

``` rust
pub fn infer_file_schema(
    reader_bytes: &ReaderBytes<'_>,
    parse_options: &CsvParseOptions,
    max_read_rows: Option<usize>,
    has_header: bool,
    schema_overwrite: Option<&Schema<DataType>>,
    skip_rows: usize,
    skip_lines: usize,
    skip_rows_after_header: usize,
    raise_if_empty: bool,
) -> Result<(Schema<DataType>, usize, usize), PolarsError>
```

Available on **crate feature `polars-io`** only.

Expand description

Infer the schema of a CSV file by reading through the first n rows of the file, with `max_read_rows` controlling the maximum number of rows to read.

If `max_read_rows` is not set, the whole file is read to infer its schema.

Returns - inferred schema - number of rows used for inference. - bytes read
