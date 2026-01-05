# Function arrow_schema_to_schema Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/arrow/schema.rs.html#219-222" class="src">Source</a>

``` rust
pub fn arrow_schema_to_schema(schema: &Schema) -> Result<Schema>
```

Expand description

Convert Arrow schema to Iceberg schema.

Iceberg schema fields require a unique field id, and this function assumes that each field in the provided Arrow schema contains a field id in its metadata. If the metadata is missing or the field id is not set, the conversion will fail
