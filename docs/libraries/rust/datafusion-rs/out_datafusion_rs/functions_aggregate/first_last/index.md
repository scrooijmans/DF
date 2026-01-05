# Module first_last Copy item path

<a href="https://docs.rs/datafusion-functions-aggregate/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_aggregate/lib.rs.html#78" class="src">Source</a>

Expand description

Defines the FIRST_VALUE/LAST_VALUE aggregations.

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/first_last/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/first_last/struct.FirstValue.html" class="struct" title="struct datafusion::functions_aggregate::first_last::FirstValue">FirstValue</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/first_last/struct.FirstValueAccumulator.html" class="struct" title="struct datafusion::functions_aggregate::first_last::FirstValueAccumulator">FirstValueAccumulator</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/first_last/struct.LastValue.html" class="struct" title="struct datafusion::functions_aggregate::first_last::LastValue">LastValue</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/first_last/struct.TrivialFirstValueAccumulator.html" class="struct" title="struct datafusion::functions_aggregate::first_last::TrivialFirstValueAccumulator">TrivialFirstValueAccumulator</a>  
This accumulator is used when there is no ordering specified for the `FIRST_VALUE` aggregation. It simply returns the first value it sees according to the pre-existing ordering of the input data, and provides a fast path for this case without needing to maintain any ordering state.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/first_last/struct.TrivialLastValueAccumulator.html" class="struct" title="struct datafusion::functions_aggregate::first_last::TrivialLastValueAccumulator">TrivialLastValueAccumulator</a>  
This accumulator is used when there is no ordering specified for the `LAST_VALUE` aggregation. It simply updates the last value it sees according to the pre-existing ordering of the input data, and provides a fast path for this case without needing to maintain any ordering state.

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/first_last/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/first_last/fn.first_value.html" class="fn" title="fn datafusion::functions_aggregate::first_last::first_value">first_value</a>  
Returns the first value in a group of values.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/first_last/fn.first_value_udaf.html" class="fn" title="fn datafusion::functions_aggregate::first_last::first_value_udaf">first_value_udaf</a>  
AggregateFunction that returns a [`AggregateUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html "struct datafusion::logical_expr::AggregateUDF") for [`FirstValue`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/first_last/struct.FirstValue.html "struct datafusion::functions_aggregate::first_last::FirstValue")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/first_last/fn.last_value.html" class="fn" title="fn datafusion::functions_aggregate::first_last::last_value">last_value</a>  
Returns the last value in a group of values.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/first_last/fn.last_value_udaf.html" class="fn" title="fn datafusion::functions_aggregate::first_last::last_value_udaf">last_value_udaf</a>  
AggregateFunction that returns a [`AggregateUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html "struct datafusion::logical_expr::AggregateUDF") for [`LastValue`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/first_last/struct.LastValue.html "struct datafusion::functions_aggregate::first_last::LastValue")
