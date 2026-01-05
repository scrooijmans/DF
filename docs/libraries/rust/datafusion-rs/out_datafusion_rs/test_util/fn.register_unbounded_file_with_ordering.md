# Function register_unbounded_file_with_ordering Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/test_util/mod.rs.html#229-242" class="src">Source</a>

``` rust
pub fn register_unbounded_file_with_ordering(
    ctx: &SessionContext,
    schema: SchemaRef,
    file_path: &Path,
    table_name: &str,
    file_sort_order: Vec<Vec<SortExpr>>,
) -> Result<()>
```

Expand description

This function creates an unbounded sorted file for testing purposes.
