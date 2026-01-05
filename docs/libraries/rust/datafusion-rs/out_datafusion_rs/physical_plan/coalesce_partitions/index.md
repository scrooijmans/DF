# Module coalesce_partitions Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/lib.rs.html#66" class="src">Source</a>

Expand description

Defines the merge plan for executing partitions in parallel and then merging the results into a single partition

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coalesce_partitions/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coalesce_partitions/struct.CoalescePartitionsExec.html" class="struct" title="struct datafusion::physical_plan::coalesce_partitions::CoalescePartitionsExec">CoalescePartitionsExec</a>  
Merge execution plan executes partitions in parallel and combines them into a single partition. No guarantees are made about the order of the resulting partition.
