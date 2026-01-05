# Module count Copy item path

<a href="https://docs.rs/datafusion-functions-aggregate/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_aggregate/lib.rs.html#76" class="src">Source</a>

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/count/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/count/struct.Count.html" class="struct" title="struct datafusion::functions_aggregate::count::Count">Count</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/count/struct.SlidingDistinctCountAccumulator.html" class="struct" title="struct datafusion::functions_aggregate::count::SlidingDistinctCountAccumulator">SlidingDistinctCountAccumulator</a>

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/count/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/count/fn.count.html" class="fn" title="fn datafusion::functions_aggregate::count::count">count</a>  
Count the number of non-null values in the column

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/count/fn.count_all.html" class="fn" title="fn datafusion::functions_aggregate::count::count_all">count_all</a>  
Creates aggregation to count all rows.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/count/fn.count_all_window.html" class="fn" title="fn datafusion::functions_aggregate::count::count_all_window">count_all_window</a>  
Creates window aggregation to count all rows.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/count/fn.count_distinct.html" class="fn" title="fn datafusion::functions_aggregate::count::count_distinct">count_distinct</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/count/fn.count_udaf.html" class="fn" title="fn datafusion::functions_aggregate::count::count_udaf">count_udaf</a>  
AggregateFunction that returns a [`AggregateUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html "struct datafusion::logical_expr::AggregateUDF") for [`Count`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/count/struct.Count.html "struct datafusion::functions_aggregate::count::Count")
