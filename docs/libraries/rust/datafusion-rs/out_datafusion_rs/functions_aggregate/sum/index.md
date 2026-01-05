# Module sum Copy item path

<a href="https://docs.rs/datafusion-functions-aggregate/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_aggregate/lib.rs.html#87" class="src">Source</a>

Expand description

Defines `SUM` and `SUM DISTINCT` aggregate accumulators

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/sum/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/sum/struct.SlidingDistinctSumAccumulator.html" class="struct" title="struct datafusion::functions_aggregate::sum::SlidingDistinctSumAccumulator">SlidingDistinctSumAccumulator</a>

A sliding‐window accumulator for `SUM(DISTINCT)` over Int64 columns. Maintains a running sum so that `evaluate()` is O(1).

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/sum/struct.Sum.html" class="struct" title="struct datafusion::functions_aggregate::sum::Sum">Sum</a>

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/sum/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/sum/fn.sum.html" class="fn" title="fn datafusion::functions_aggregate::sum::sum">sum</a>  
Returns the sum of a group of values.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/sum/fn.sum_udaf.html" class="fn" title="fn datafusion::functions_aggregate::sum::sum_udaf">sum_udaf</a>  
AggregateFunction that returns a [`AggregateUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html "struct datafusion::logical_expr::AggregateUDF") for [`Sum`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/sum/struct.Sum.html "struct datafusion::functions_aggregate::sum::Sum")
