# Function replace_order_preserving_variants Copy item path

<a href="https://docs.rs/datafusion-physical-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_optimizer/enforce_distribution.rs.html#1026-1028" class="src">Source</a>

``` rust
pub fn replace_order_preserving_variants(
    context: PlanContext<bool>,
) -> Result<PlanContext<bool>, DataFusionError>
```

Expand description

Updates the [`DistributionContext`](https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_distribution/type.DistributionContext.html "type datafusion::physical_optimizer::enforce_distribution::DistributionContext") if preserving ordering while changing partitioning is not helpful or desirable.

Assume that following plan is given:

``` text
"SortPreservingMergeExec: \[a@0 ASC]"
"  RepartitionExec: partitioning=RoundRobinBatch(10), input_partitions=10, preserve_order=true",
"    RepartitionExec: partitioning=RoundRobinBatch(10), input_partitions=2, preserve_order=true",
"      DataSourceExec: file_groups={2 groups: \[\[x], \[y]]}, projection=\[a, b, c, d, e], output_ordering=\[a@0 ASC], file_type=parquet",
```

This function converts plan above to the following:

``` text
"CoalescePartitionsExec"
"  RepartitionExec: partitioning=RoundRobinBatch(10), input_partitions=10",
"    RepartitionExec: partitioning=RoundRobinBatch(10), input_partitions=2",
"      DataSourceExec: file_groups={2 groups: \[\[x], \[y]]}, projection=\[a, b, c, d, e], output_ordering=\[a@0 ASC], file_type=parquet",
```
