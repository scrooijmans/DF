# Module string_agg Copy item path

<a href="https://docs.rs/datafusion-functions-aggregate/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_aggregate/lib.rs.html#86" class="src">Source</a>

Expand description

[`StringAgg`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/string_agg/struct.StringAgg.html "struct datafusion::functions_aggregate::string_agg::StringAgg") accumulator for the `string_agg` function

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/string_agg/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/string_agg/struct.StringAgg.html" class="struct" title="struct datafusion::functions_aggregate::string_agg::StringAgg">StringAgg</a>  
STRING_AGG aggregate expression

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/string_agg/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/string_agg/fn.string_agg.html" class="fn" title="fn datafusion::functions_aggregate::string_agg::string_agg">string_agg</a>  
Concatenates the values of string expressions and places separator values between them

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/string_agg/fn.string_agg_udaf.html" class="fn" title="fn datafusion::functions_aggregate::string_agg::string_agg_udaf">string_agg_udaf</a>  
AggregateFunction that returns a [`AggregateUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html "struct datafusion::logical_expr::AggregateUDF") for [`StringAgg`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/string_agg/struct.StringAgg.html "struct datafusion::functions_aggregate::string_agg::StringAgg")
