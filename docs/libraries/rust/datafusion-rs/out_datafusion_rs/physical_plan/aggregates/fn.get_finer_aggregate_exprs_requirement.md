# Function get_finer_aggregate_exprs_requirement Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/aggregates/mod.rs.html#1164-1169" class="src">Source</a>

``` rust
pub fn get_finer_aggregate_exprs_requirement(
    aggr_exprs: &mut [Arc<AggregateFunctionExpr>],
    group_by: &PhysicalGroupBy,
    eq_properties: &EquivalenceProperties,
    agg_mode: &AggregateMode,
) -> Result<Vec<PhysicalSortRequirement>, DataFusionError>
```

Expand description

Gets the common requirement that satisfies all the aggregate expressions. When possible, chooses the requirement that is already satisfied by the equivalence properties.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/aggregates/fn.get_finer_aggregate_exprs_requirement.html#parameters" class="doc-anchor">§</a>Parameters

- `aggr_exprs`: A slice of `AggregateFunctionExpr` containing all the aggregate expressions.
- `group_by`: A reference to a `PhysicalGroupBy` instance representing the physical GROUP BY expression.
- `eq_properties`: A reference to an `EquivalenceProperties` instance representing equivalence properties for ordering.
- `agg_mode`: A reference to an `AggregateMode` instance representing the mode of aggregation.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/aggregates/fn.get_finer_aggregate_exprs_requirement.html#returns" class="doc-anchor">§</a>Returns

A `Result<Vec<PhysicalSortRequirement>>` instance, which is the requirement that satisfies all the aggregate requirements. Returns an error in case of conflicting requirements.
