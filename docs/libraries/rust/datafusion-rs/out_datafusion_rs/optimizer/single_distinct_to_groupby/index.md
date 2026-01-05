# Module single_distinct_to_groupby Copy item path

<a href="https://docs.rs/datafusion-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_optimizer/lib.rs.html#64" class="src">Source</a>

Expand description

[`SingleDistinctToGroupBy`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/single_distinct_to_groupby/struct.SingleDistinctToGroupBy.html "struct datafusion::optimizer::single_distinct_to_groupby::SingleDistinctToGroupBy") replaces `AGG(DISTINCT ..)` with `AGG(..) GROUP BY ..`

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/single_distinct_to_groupby/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/single_distinct_to_groupby/struct.SingleDistinctToGroupBy.html" class="struct" title="struct datafusion::optimizer::single_distinct_to_groupby::SingleDistinctToGroupBy">SingleDistinctToGroupBy</a>  
single distinct to group by optimizer rule
