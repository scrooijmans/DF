# Function check_all_columns_from_schema Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/utils.rs.html#852-855" class="src">Source</a>

``` rust
pub fn check_all_columns_from_schema(
    columns: &HashSet<&Column>,
    schema: &DFSchema,
) -> Result<bool, DataFusionError>
```

Expand description

Check whether all columns are from the schema.
