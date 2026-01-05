# Module eliminate_filter Copy item path

<a href="https://docs.rs/datafusion-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_optimizer/lib.rs.html#47" class="src">Source</a>

Expand description

[`EliminateFilter`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_filter/struct.EliminateFilter.html "struct datafusion::optimizer::eliminate_filter::EliminateFilter") replaces `where false` or `where null` with an empty relation.

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_filter/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_filter/struct.EliminateFilter.html" class="struct" title="struct datafusion::optimizer::eliminate_filter::EliminateFilter">EliminateFilter</a>  
Optimization rule that eliminate the scalar value (true/false/null) filter with an [LogicalPlan::EmptyRelation](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html#variant.EmptyRelation "variant datafusion::logical_expr::LogicalPlan::EmptyRelation")
