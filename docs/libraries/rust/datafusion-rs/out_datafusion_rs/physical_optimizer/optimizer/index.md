# Module optimizer Copy item path

<a href="https://docs.rs/datafusion-physical-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_optimizer/lib.rs.html#38" class="src">Source</a>

Expand description

Physical optimizer traits

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/optimizer/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/optimizer/struct.PhysicalOptimizer.html" class="struct" title="struct datafusion::physical_optimizer::optimizer::PhysicalOptimizer">PhysicalOptimizer</a>  
A rule-based physical optimizer.

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/optimizer/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/optimizer/trait.PhysicalOptimizerRule.html" class="trait" title="trait datafusion::physical_optimizer::optimizer::PhysicalOptimizerRule">PhysicalOptimizerRule</a>  
`PhysicalOptimizerRule` transforms one \[‘ExecutionPlan’\] into another which computes the same results, but in a potentially more efficient way.
