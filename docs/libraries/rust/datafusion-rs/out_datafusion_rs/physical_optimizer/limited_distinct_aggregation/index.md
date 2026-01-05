# Module limited_distinct_aggregation Copy item path

<a href="https://docs.rs/datafusion-physical-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_optimizer/lib.rs.html#37" class="src">Source</a>

Expand description

A special-case optimizer rule that pushes limit into a grouped aggregation which has no aggregate expressions or sorting requirements

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/limited_distinct_aggregation/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/limited_distinct_aggregation/struct.LimitedDistinctAggregation.html" class="struct" title="struct datafusion::physical_optimizer::limited_distinct_aggregation::LimitedDistinctAggregation">LimitedDistinctAggregation</a>  
An optimizer rule that passes a `limit` hint into grouped aggregations which don’t require all rows in the group to be processed for correctness. Example queries fitting this description are:
