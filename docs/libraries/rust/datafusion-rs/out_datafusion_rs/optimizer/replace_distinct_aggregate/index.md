# Module replace_distinct_aggregate Copy item path

<a href="https://docs.rs/datafusion-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_optimizer/lib.rs.html#61" class="src">Source</a>

Expand description

[`ReplaceDistinctWithAggregate`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/replace_distinct_aggregate/struct.ReplaceDistinctWithAggregate.html "struct datafusion::optimizer::replace_distinct_aggregate::ReplaceDistinctWithAggregate") replaces `DISTINCT ...` with `GROUP BY ...`

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/replace_distinct_aggregate/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/replace_distinct_aggregate/struct.ReplaceDistinctWithAggregate.html" class="struct" title="struct datafusion::optimizer::replace_distinct_aggregate::ReplaceDistinctWithAggregate">ReplaceDistinctWithAggregate</a>  
Optimizer that replaces logical \[[Distinct](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Distinct.html "enum datafusion::logical_expr::Distinct")\] with a logical \[[Aggregate](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Aggregate.html "struct datafusion::logical_expr::Aggregate")\]
