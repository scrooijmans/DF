# Function ensure_distribution Copy item path

<a href="https://docs.rs/datafusion-physical-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_optimizer/enforce_distribution.rs.html#1170-1173" class="src">Source</a>

``` rust
pub fn ensure_distribution(
    dist_context: PlanContext<bool>,
    config: &ConfigOptions,
) -> Result<Transformed<PlanContext<bool>>, DataFusionError>
```

Expand description

This function checks whether we need to add additional data exchange operators to satisfy distribution requirements. Since this function takes care of such requirements, we should avoid manually adding data exchange operators in other places.

This function is intended to be used in a bottom up traversal, as it can first repartition (or newly partition) at the datasources – these source partitions may be later repartitioned with additional data exchange operators.
