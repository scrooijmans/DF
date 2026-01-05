# Module coalesce Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/lib.rs.html#64" class="src">Source</a>

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coalesce/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coalesce/struct.BatchCoalescer.html" class="struct" title="struct datafusion::physical_plan::coalesce::BatchCoalescer">BatchCoalescer</a>  
Concatenate multiple [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch")es

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coalesce/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coalesce/enum.CoalescerState.html" class="enum" title="enum datafusion::physical_plan::coalesce::CoalescerState">CoalescerState</a>  
Indicates the state of the [`BatchCoalescer`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coalesce/struct.BatchCoalescer.html "struct datafusion::physical_plan::coalesce::BatchCoalescer") buffer after the [`BatchCoalescer::push_batch()`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coalesce/struct.BatchCoalescer.html#method.push_batch "method datafusion::physical_plan::coalesce::BatchCoalescer::push_batch") operation.
