# Module combine_partial_final_agg Copy item path

<a href="https://docs.rs/datafusion-physical-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_optimizer/lib.rs.html#30" class="src">Source</a>

Expand description

CombinePartialFinalAggregate optimizer rule checks the adjacent Partial and Final AggregateExecs and try to combine them if necessary

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/combine_partial_final_agg/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/combine_partial_final_agg/struct.CombinePartialFinalAggregate.html" class="struct" title="struct datafusion::physical_optimizer::combine_partial_final_agg::CombinePartialFinalAggregate">CombinePartialFinalAggregate</a>  
CombinePartialFinalAggregate optimizer rule combines the adjacent Partial and Final AggregateExecs into a Single AggregateExec if their grouping exprs and aggregate exprs equal.
