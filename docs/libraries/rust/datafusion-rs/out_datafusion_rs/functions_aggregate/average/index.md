# Module average Copy item path

<a href="https://docs.rs/datafusion-functions-aggregate/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_aggregate/lib.rs.html#72" class="src">Source</a>

Expand description

Defines `Avg` & `Mean` aggregate & accumulators

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/average/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/average/struct.Avg.html" class="struct" title="struct datafusion::functions_aggregate::average::Avg">Avg</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/average/struct.AvgAccumulator.html" class="struct" title="struct datafusion::functions_aggregate::average::AvgAccumulator">AvgAccumulator</a>  
An accumulator to compute the average

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/average/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/average/fn.avg.html" class="fn" title="fn datafusion::functions_aggregate::average::avg">avg</a>  
Returns the avg of a group of values.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/average/fn.avg_udaf.html" class="fn" title="fn datafusion::functions_aggregate::average::avg_udaf">avg_udaf</a>  
AggregateFunction that returns a [`AggregateUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html "struct datafusion::logical_expr::AggregateUDF") for [`Avg`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/average/struct.Avg.html "struct datafusion::functions_aggregate::average::Avg")
