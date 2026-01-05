# Module repartition Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/lib.rs.html#82" class="src">Source</a>

Expand description

This file implements the [`RepartitionExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/repartition/struct.RepartitionExec.html "struct datafusion::physical_plan::repartition::RepartitionExec") operator, which maps N input partitions to M output partitions based on a partitioning scheme, optionally maintaining the order of the input rows in the output.

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/repartition/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/repartition/struct.BatchPartitioner.html" class="struct" title="struct datafusion::physical_plan::repartition::BatchPartitioner">BatchPartitioner</a>  
A utility that can be used to partition batches based on [`Partitioning`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Partitioning.html "enum datafusion::physical_expr::Partitioning")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/repartition/struct.RepartitionExec.html" class="struct" title="struct datafusion::physical_plan::repartition::RepartitionExec">RepartitionExec</a>  
Maps `N` input partitions to `M` output partitions based on a [`Partitioning`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Partitioning.html "enum datafusion::physical_expr::Partitioning") scheme.
