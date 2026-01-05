# Function evaluate_group_by Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/aggregates/mod.rs.html#1417-1420" class="src">Source</a>

``` rust
pub fn evaluate_group_by(
    group_by: &PhysicalGroupBy,
    batch: &RecordBatch,
) -> Result<Vec<Vec<Arc<dyn Array>>>, DataFusionError>
```

Expand description

Evaluate a group by expression against a `RecordBatch`

Arguments:

- `group_by`: the expression to evaluate
- `batch`: the `RecordBatch` to evaluate against

Returns: A Vec of Vecs of Array of results The outer Vec appears to be for grouping sets The inner Vec contains the results per expression The inner-inner Array contains the results per row
