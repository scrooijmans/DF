# Function read_data_files_from_avro Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/manifest/data_file.rs.html#313-336" class="src">Source</a>

``` rust
pub fn read_data_files_from_avro<R: Read>(
    reader: &mut R,
    schema: &Schema,
    partition_spec_id: i32,
    partition_type: &StructType,
    version: FormatVersion,
) -> Result<Vec<DataFile>>
```

Expand description

Parse data files from avro bytes.
