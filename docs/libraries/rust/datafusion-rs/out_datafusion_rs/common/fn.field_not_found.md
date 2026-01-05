# Function field_not_found Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/error.rs.html#913-917" class="src">Source</a>

``` rust
pub fn field_not_found<R>(
    qualifier: Option<R>,
    name: &str,
    schema: &DFSchema,
) -> DataFusionErrorwhere
    R: Into<TableReference>,
```

Expand description

Create a “field not found” DataFusion::SchemaError
