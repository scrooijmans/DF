# Module approx_median Copy item path

<a href="https://docs.rs/datafusion-functions-aggregate/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_aggregate/lib.rs.html#68" class="src">Source</a>

Expand description

Defines physical expressions for APPROX_MEDIAN that can be evaluated MEDIAN at runtime during query execution

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/approx_median/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/approx_median/struct.ApproxMedian.html" class="struct" title="struct datafusion::functions_aggregate::approx_median::ApproxMedian">ApproxMedian</a>  
APPROX_MEDIAN aggregate expression

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/approx_median/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/approx_median/fn.approx_median.html" class="fn" title="fn datafusion::functions_aggregate::approx_median::approx_median">approx_median</a>  
Computes the approximate median of a set of numbers

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/approx_median/fn.approx_median_udaf.html" class="fn" title="fn datafusion::functions_aggregate::approx_median::approx_median_udaf">approx_median_udaf</a>  
AggregateFunction that returns a [`AggregateUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html "struct datafusion::logical_expr::AggregateUDF") for [`ApproxMedian`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/approx_median/struct.ApproxMedian.html "struct datafusion::functions_aggregate::approx_median::ApproxMedian")
