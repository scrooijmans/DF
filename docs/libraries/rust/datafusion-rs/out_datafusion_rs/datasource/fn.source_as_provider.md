# Function source_as_provider Copy item path

<a href="https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/default_table_source.rs.html#97-99" class="src">Source</a>

``` rust
pub fn source_as_provider(
    source: &Arc<dyn TableSource>,
) -> Result<Arc<dyn TableProvider>, DataFusionError>
```

Expand description

Attempt to downcast a TableSource to DefaultTableSource and access the TableProvider. This will only work with a TableSource created by DataFusion.
