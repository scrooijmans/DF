# Module bool_and_or Copy item path

<a href="https://docs.rs/datafusion-functions-aggregate/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_aggregate/lib.rs.html#74" class="src">Source</a>

Expand description

Defines physical expressions that can evaluated at runtime during query execution

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/bool_and_or/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/bool_and_or/struct.BoolAnd.html" class="struct" title="struct datafusion::functions_aggregate::bool_and_or::BoolAnd">BoolAnd</a>  
BOOL_AND aggregate expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/bool_and_or/struct.BoolOr.html" class="struct" title="struct datafusion::functions_aggregate::bool_and_or::BoolOr">BoolOr</a>  
BOOL_OR aggregate expression

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/bool_and_or/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/bool_and_or/fn.bool_and.html" class="fn" title="fn datafusion::functions_aggregate::bool_and_or::bool_and">bool_and</a>  
The values to combine with `AND`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/bool_and_or/fn.bool_and_udaf.html" class="fn" title="fn datafusion::functions_aggregate::bool_and_or::bool_and_udaf">bool_and_udaf</a>  
AggregateFunction that returns a [`AggregateUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html "struct datafusion::logical_expr::AggregateUDF") for [`BoolAnd`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/bool_and_or/struct.BoolAnd.html "struct datafusion::functions_aggregate::bool_and_or::BoolAnd")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/bool_and_or/fn.bool_or.html" class="fn" title="fn datafusion::functions_aggregate::bool_and_or::bool_or">bool_or</a>  
The values to combine with `OR`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/bool_and_or/fn.bool_or_udaf.html" class="fn" title="fn datafusion::functions_aggregate::bool_and_or::bool_or_udaf">bool_or_udaf</a>  
AggregateFunction that returns a [`AggregateUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html "struct datafusion::logical_expr::AggregateUDF") for [`BoolOr`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/bool_and_or/struct.BoolOr.html "struct datafusion::functions_aggregate::bool_and_or::BoolOr")
