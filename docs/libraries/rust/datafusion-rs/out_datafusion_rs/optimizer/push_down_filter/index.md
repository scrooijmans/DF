# Module push_down_filter Copy item path

<a href="https://docs.rs/datafusion-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_optimizer/lib.rs.html#59" class="src">Source</a>

Expand description

[`PushDownFilter`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/push_down_filter/struct.PushDownFilter.html "struct datafusion::optimizer::push_down_filter::PushDownFilter") applies filters as early as possible

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/push_down_filter/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/push_down_filter/struct.PushDownFilter.html" class="struct" title="struct datafusion::optimizer::push_down_filter::PushDownFilter">PushDownFilter</a>  
Optimizer rule for pushing (moving) filter expressions down in a plan so they are applied as early as possible.

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/push_down_filter/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/push_down_filter/fn.make_filter.html" class="fn" title="fn datafusion::optimizer::push_down_filter::make_filter">make_filter</a>  
Creates a new LogicalPlan::Filter node.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/push_down_filter/fn.replace_cols_by_name.html" class="fn" title="fn datafusion::optimizer::push_down_filter::replace_cols_by_name">replace_cols_by_name</a>  
replaces columns by its name on the projection.
