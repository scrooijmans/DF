# Function infer_schema_from_files Copy item path

<a href="https://arrow.apache.org/rust/src/arrow_csv/reader/mod.rs.html#410-438" class="src">Source</a>

``` rust
pub fn infer_schema_from_files(
    files: &[String],
    delimiter: u8,
    max_read_records: Option<usize>,
    has_header: bool,
) -> Result<Schema, ArrowError>
```

Expand description

Infer schema from a list of CSV files by reading through first n records with `max_read_records` controlling the maximum number of records to read.

Files will be read in the given order until n records have been reached.

If `max_read_records` is not set, all files will be read fully to infer the schema.
