# Module median Copy item path

<a href="https://docs.rs/datafusion-functions-aggregate/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_aggregate/lib.rs.html#81" class="src">Source</a>

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/median/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/median/struct.Median.html" class="struct" title="struct datafusion::functions_aggregate::median::Median">Median</a>  
MEDIAN aggregate expression. If using the non-distinct variation, then this uses a lot of memory because all values need to be stored in memory before a result can be computed. If an approximation is sufficient then APPROX_MEDIAN provides a much more efficient solution.

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/median/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/median/fn.median.html" class="fn" title="fn datafusion::functions_aggregate::median::median">median</a>  
Computes the median of a set of numbers

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/median/fn.median_udaf.html" class="fn" title="fn datafusion::functions_aggregate::median::median_udaf">median_udaf</a>  
AggregateFunction that returns a [`AggregateUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html "struct datafusion::logical_expr::AggregateUDF") for [`Median`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/median/struct.Median.html "struct datafusion::functions_aggregate::median::Median")
