# Module grouping Copy item path

<a href="https://docs.rs/datafusion-functions-aggregate/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_aggregate/lib.rs.html#79" class="src">Source</a>

Expand description

Defines physical expressions that can evaluated at runtime during query execution

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/grouping/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/grouping/struct.Grouping.html" class="struct" title="struct datafusion::functions_aggregate::grouping::Grouping">Grouping</a>

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/grouping/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/grouping/fn.grouping.html" class="fn" title="fn datafusion::functions_aggregate::grouping::grouping">grouping</a>  
Returns 1 if the data is aggregated across the specified column or 0 for not aggregated in the result set.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/grouping/fn.grouping_udaf.html" class="fn" title="fn datafusion::functions_aggregate::grouping::grouping_udaf">grouping_udaf</a>  
AggregateFunction that returns a [`AggregateUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html "struct datafusion::logical_expr::AggregateUDF") for [`Grouping`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/grouping/struct.Grouping.html "struct datafusion::functions_aggregate::grouping::Grouping")
