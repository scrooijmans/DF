# Function provider_as_source Copy item path

<a href="https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/default_table_source.rs.html#89-91" class="src">Source</a>

``` rust
pub fn provider_as_source(
    table_provider: Arc<dyn TableProvider>,
) -> Arc<dyn TableSource>
```

Expand description

Wrap TableProvider in TableSource
