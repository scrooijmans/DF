# Function convert_to_expr Copy item path

<a href="https://docs.rs/datafusion-physical-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr/utils/mod.rs.html#136-138" class="src">Source</a>

``` rust
pub fn convert_to_expr<T>(
    sequence: impl IntoIterator<Item = T>,
) -> Vec<Arc<dyn PhysicalExpr>>where
    T: Borrow<PhysicalSortExpr>,
```

Expand description

This function returns all `Arc<dyn PhysicalExpr>`s inside the given `PhysicalSortExpr` sequence.
