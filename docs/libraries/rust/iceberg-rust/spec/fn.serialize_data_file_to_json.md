# Function serialize_data_file_to_json Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/manifest/mod.rs.html#124-137" class="src">Source</a>

``` rust
pub fn serialize_data_file_to_json(
    data_file: DataFile,
    partition_type: &StructType,
    format_version: FormatVersion,
) -> Result<String>
```

Expand description

Serialize a DataFile to a JSON string.
