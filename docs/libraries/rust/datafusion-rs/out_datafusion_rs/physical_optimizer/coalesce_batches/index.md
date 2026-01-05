# Module coalesce_batches Copy item path

<a href="https://docs.rs/datafusion-physical-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_optimizer/lib.rs.html#29" class="src">Source</a>

Expand description

CoalesceBatches optimizer that groups batches together rows in bigger batches to avoid overhead with small batches

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/coalesce_batches/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/coalesce_batches/struct.CoalesceBatches.html" class="struct" title="struct datafusion::physical_optimizer::coalesce_batches::CoalesceBatches">CoalesceBatches</a>  
Optimizer rule that introduces CoalesceBatchesExec to avoid overhead with small batches that are produced by highly selective filters
