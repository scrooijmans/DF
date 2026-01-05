# Function deserialize_data_file_from_json Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/manifest/mod.rs.html#140-155" class="src">Source</a>

``` rust
pub fn deserialize_data_file_from_json(
    json: &str,
    partition_spec_id: i32,
    partition_type: &StructType,
    schema: &Schema,
) -> Result<DataFile>
```

Expand description

Deserialize a DataFile from a JSON string.
