# Module nth_value Copy item path

<a href="https://docs.rs/datafusion-functions-aggregate/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_aggregate/lib.rs.html#83" class="src">Source</a>

Expand description

Defines NTH_VALUE aggregate expression which may specify ordering requirement that can evaluated at runtime during query execution

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/nth_value/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/nth_value/struct.NthValueAccumulator.html" class="struct" title="struct datafusion::functions_aggregate::nth_value::NthValueAccumulator">NthValueAccumulator</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/nth_value/struct.NthValueAgg.html" class="struct" title="struct datafusion::functions_aggregate::nth_value::NthValueAgg">NthValueAgg</a>

Expression for a `NTH_VALUE(..., ... ORDER BY ...)` aggregation. In a multi partition setting, partial aggregations are computed for every partition, and then their results are merged.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/nth_value/struct.TrivialNthValueAccumulator.html" class="struct" title="struct datafusion::functions_aggregate::nth_value::TrivialNthValueAccumulator">TrivialNthValueAccumulator</a>

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/nth_value/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/nth_value/fn.nth_value.html" class="fn" title="fn datafusion::functions_aggregate::nth_value::nth_value">nth_value</a>  
Returns the nth value in a group of values.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/nth_value/fn.nth_value_udaf.html" class="fn" title="fn datafusion::functions_aggregate::nth_value::nth_value_udaf">nth_value_udaf</a>  
AggregateFunction that returns a [`AggregateUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html "struct datafusion::logical_expr::AggregateUDF") for [`NthValueAgg`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/nth_value/struct.NthValueAgg.html "struct datafusion::functions_aggregate::nth_value::NthValueAgg")
