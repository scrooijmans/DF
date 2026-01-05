# Function ordering_fields Copy item path

<a href="https://docs.rs/datafusion-functions-aggregate-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_aggregate_common/utils.rs.html#40-44" class="src">Source</a>

``` rust
pub fn ordering_fields(
    order_bys: &[PhysicalSortExpr],
    data_types: &[DataType],
) -> Vec<Arc<Field>>
```

Expand description

Construct corresponding fields for the expressions in an ORDER BY clause.
