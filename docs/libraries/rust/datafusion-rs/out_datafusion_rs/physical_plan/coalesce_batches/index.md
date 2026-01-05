# Module coalesce_batches Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/lib.rs.html#65" class="src">Source</a>

Expand description

[`CoalesceBatchesExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coalesce_batches/struct.CoalesceBatchesExec.html "struct datafusion::physical_plan::coalesce_batches::CoalesceBatchesExec") combines small batches into larger batches.

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coalesce_batches/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coalesce_batches/struct.CoalesceBatchesExec.html" class="struct" title="struct datafusion::physical_plan::coalesce_batches::CoalesceBatchesExec">CoalesceBatchesExec</a>  
`CoalesceBatchesExec` combines small batches into larger batches for more efficient vectorized processing by later operators.
