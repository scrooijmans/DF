# Function write_data_files_to_avro Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/manifest/data_file.rs.html#287-310" class="src">Source</a>

``` rust
pub fn write_data_files_to_avro<W: Write>(
    writer: &mut W,
    data_files: impl IntoIterator<Item = DataFile>,
    partition_type: &StructType,
    version: FormatVersion,
) -> Result<usize>
```

Expand description

Convert data files to avro bytes and write to writer. Return the bytes written.
