# Module streaming Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/lib.rs.html#86" class="src">Source</a>

Expand description

Generic plans for deferred execution: [`StreamingTableExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/streaming/struct.StreamingTableExec.html "struct datafusion::physical_plan::streaming::StreamingTableExec") and [`PartitionStream`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/streaming/trait.PartitionStream.html "trait datafusion::physical_plan::streaming::PartitionStream")

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/streaming/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/streaming/struct.StreamingTableExec.html" class="struct" title="struct datafusion::physical_plan::streaming::StreamingTableExec">StreamingTableExec</a>  
An [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") for one or more [`PartitionStream`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/streaming/trait.PartitionStream.html "trait datafusion::physical_plan::streaming::PartitionStream")s.

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/streaming/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/streaming/trait.PartitionStream.html" class="trait" title="trait datafusion::physical_plan::streaming::PartitionStream">PartitionStream</a>  
A partition that can be converted into a [`SendableRecordBatchStream`](https://docs.rs/datafusion/50.2.0/datafusion/execution/type.SendableRecordBatchStream.html "type datafusion::execution::SendableRecordBatchStream")
