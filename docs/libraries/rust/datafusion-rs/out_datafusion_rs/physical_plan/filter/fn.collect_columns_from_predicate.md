# Function collect_columns_from_predicate Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/filter.rs.html#719-721" class="src">Source</a>

``` rust
pub fn collect_columns_from_predicate(
    predicate: &Arc<dyn PhysicalExpr>,
) -> (Vec<(&Arc<dyn PhysicalExpr>, &Arc<dyn PhysicalExpr>)>, Vec<(&Arc<dyn PhysicalExpr>, &Arc<dyn PhysicalExpr>)>)
```

Expand description

Return the equals Column-Pairs and Non-equals Column-Pairs
