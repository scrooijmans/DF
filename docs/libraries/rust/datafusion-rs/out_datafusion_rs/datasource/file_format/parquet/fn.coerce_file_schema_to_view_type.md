# Function coerce_file_schema_to_view_type Copy item path

<a href="https://docs.rs/datafusion-datasource-parquet/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_datasource_parquet/file_format.rs.html#844-847" class="src">Source</a>

``` rust
pub fn coerce_file_schema_to_view_type(
    table_schema: &Schema,
    file_schema: &Schema,
) -> Option<Schema>
```

👎Deprecated since 47.0.0: Use `apply_file_schema_type_coercions` instead

Available on **crate feature `parquet`** only.

Expand description

Coerces the file schema if the table schema uses a view type.
