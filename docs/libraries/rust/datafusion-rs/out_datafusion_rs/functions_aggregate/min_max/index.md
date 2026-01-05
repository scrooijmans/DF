# Module min_max Copy item path

<a href="https://docs.rs/datafusion-functions-aggregate/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_aggregate/lib.rs.html#82" class="src">Source</a>

Expand description

[`Max`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/struct.Max.html "struct datafusion::functions_aggregate::min_max::Max") and [`MaxAccumulator`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/struct.MaxAccumulator.html "struct datafusion::functions_aggregate::min_max::MaxAccumulator") accumulator for the `max` function [`Min`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/struct.Min.html "struct datafusion::functions_aggregate::min_max::Min") and [`MinAccumulator`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/struct.MinAccumulator.html "struct datafusion::functions_aggregate::min_max::MinAccumulator") accumulator for the `min` function

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/struct.Max.html" class="struct" title="struct datafusion::functions_aggregate::min_max::Max">Max</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/struct.MaxAccumulator.html" class="struct" title="struct datafusion::functions_aggregate::min_max::MaxAccumulator">MaxAccumulator</a>

An accumulator to compute the maximum value

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/struct.Min.html" class="struct" title="struct datafusion::functions_aggregate::min_max::Min">Min</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/struct.MinAccumulator.html" class="struct" title="struct datafusion::functions_aggregate::min_max::MinAccumulator">MinAccumulator</a>

An accumulator to compute the minimum value

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/struct.MovingMax.html" class="struct" title="struct datafusion::functions_aggregate::min_max::MovingMax">MovingMax</a>

Keep track of the maximum value in a sliding window.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/struct.MovingMin.html" class="struct" title="struct datafusion::functions_aggregate::min_max::MovingMin">MovingMin</a>

Keep track of the minimum value in a sliding window.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/struct.SlidingMaxAccumulator.html" class="struct" title="struct datafusion::functions_aggregate::min_max::SlidingMaxAccumulator">SlidingMaxAccumulator</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/struct.SlidingMinAccumulator.html" class="struct" title="struct datafusion::functions_aggregate::min_max::SlidingMinAccumulator">SlidingMinAccumulator</a>

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/fn.max.html" class="fn" title="fn datafusion::functions_aggregate::min_max::max">max</a>  
Returns the maximum of a group of values.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/fn.max_udaf.html" class="fn" title="fn datafusion::functions_aggregate::min_max::max_udaf">max_udaf</a>  
AggregateFunction that returns a [`AggregateUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html "struct datafusion::logical_expr::AggregateUDF") for [`Max`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/struct.Max.html "struct datafusion::functions_aggregate::min_max::Max")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/fn.min.html" class="fn" title="fn datafusion::functions_aggregate::min_max::min">min</a>  
Returns the minimum of a group of values.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/fn.min_udaf.html" class="fn" title="fn datafusion::functions_aggregate::min_max::min_udaf">min_udaf</a>  
AggregateFunction that returns a [`AggregateUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html "struct datafusion::logical_expr::AggregateUDF") for [`Min`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/struct.Min.html "struct datafusion::functions_aggregate::min_max::Min")
