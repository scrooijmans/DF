# Module topk_aggregation Copy item path

<a href="https://docs.rs/datafusion-physical-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_optimizer/lib.rs.html#44" class="src">Source</a>

Expand description

An optimizer rule that detects aggregate operations that could use a limited bucket count

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/topk_aggregation/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/topk_aggregation/struct.TopKAggregation.html" class="struct" title="struct datafusion::physical_optimizer::topk_aggregation::TopKAggregation">TopKAggregation</a>  
An optimizer rule that passes a `limit` hint to aggregations if the whole result is not needed
