# Function apply_file_schema_type_coercions Copy item path

<a href="https://docs.rs/datafusion-datasource-parquet/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_datasource_parquet/file_format.rs.html#563-566" class="src">Source</a>

``` rust
pub fn apply_file_schema_type_coercions(
    table_schema: &Schema,
    file_schema: &Schema,
) -> Option<Schema>
```

Available on **crate feature `parquet`** only.

Expand description

Apply necessary schema type coercions to make file schema match table schema.

This function performs two main types of transformations in a single pass:

1.  Binary types to string types conversion - Converts binary data types to their corresponding string types when the table schema expects string data
2.  Regular to view types conversion - Converts standard string/binary types to view types when the table schema uses view types

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/parquet/fn.apply_file_schema_type_coercions.html#arguments" class="doc-anchor">§</a>Arguments

- `table_schema` - The table schema containing the desired types
- `file_schema` - The file schema to be transformed

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/parquet/fn.apply_file_schema_type_coercions.html#returns" class="doc-anchor">§</a>Returns

- `Some(Schema)` - If any transformations were applied, returns the transformed schema
- `None` - If no transformations were needed
