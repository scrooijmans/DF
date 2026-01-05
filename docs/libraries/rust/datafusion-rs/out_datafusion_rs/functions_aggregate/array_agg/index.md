# Module array_agg Copy item path

<a href="https://docs.rs/datafusion-functions-aggregate/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_aggregate/lib.rs.html#71" class="src">Source</a>

Expand description

`ARRAY_AGG` aggregate implementation: [`ArrayAgg`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/array_agg/struct.ArrayAgg.html "struct datafusion::functions_aggregate::array_agg::ArrayAgg")

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/array_agg/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/array_agg/struct.ArrayAgg.html" class="struct" title="struct datafusion::functions_aggregate::array_agg::ArrayAgg">ArrayAgg</a>

ARRAY_AGG aggregate expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/array_agg/struct.ArrayAggAccumulator.html" class="struct" title="struct datafusion::functions_aggregate::array_agg::ArrayAggAccumulator">ArrayAggAccumulator</a>

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/array_agg/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/array_agg/fn.array_agg.html" class="fn" title="fn datafusion::functions_aggregate::array_agg::array_agg">array_agg</a>  
input values, including nulls, concatenated into an array

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/array_agg/fn.array_agg_udaf.html" class="fn" title="fn datafusion::functions_aggregate::array_agg::array_agg_udaf">array_agg_udaf</a>  
AggregateFunction that returns a [`AggregateUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html "struct datafusion::logical_expr::AggregateUDF") for [`ArrayAgg`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/array_agg/struct.ArrayAgg.html "struct datafusion::functions_aggregate::array_agg::ArrayAgg")
