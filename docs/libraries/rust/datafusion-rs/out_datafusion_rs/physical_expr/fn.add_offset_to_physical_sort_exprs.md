# Function add_offset_to_physical_sort_exprs Copy item path

<a href="https://docs.rs/datafusion-physical-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr/physical_expr.rs.html#190-193" class="src">Source</a>

``` rust
pub fn add_offset_to_physical_sort_exprs(
    sort_exprs: impl IntoIterator<Item = PhysicalSortExpr>,
    offset: isize,
) -> Result<Vec<PhysicalSortExpr>, DataFusionError>
```
