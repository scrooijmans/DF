# Module correlation Copy item path

<a href="https://docs.rs/datafusion-functions-aggregate/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_aggregate/lib.rs.html#75" class="src">Source</a>

Expand description

[`Correlation`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/correlation/struct.Correlation.html "struct datafusion::functions_aggregate::correlation::Correlation"): correlation sample aggregations.

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/correlation/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/correlation/struct.Correlation.html" class="struct" title="struct datafusion::functions_aggregate::correlation::Correlation">Correlation</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/correlation/struct.CorrelationAccumulator.html" class="struct" title="struct datafusion::functions_aggregate::correlation::CorrelationAccumulator">CorrelationAccumulator</a>

An accumulator to compute correlation

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/correlation/struct.CorrelationGroupsAccumulator.html" class="struct" title="struct datafusion::functions_aggregate::correlation::CorrelationGroupsAccumulator">CorrelationGroupsAccumulator</a>

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/correlation/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/correlation/fn.corr.html" class="fn" title="fn datafusion::functions_aggregate::correlation::corr">corr</a>  
Correlation between two numeric values.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/correlation/fn.corr_udaf.html" class="fn" title="fn datafusion::functions_aggregate::correlation::corr_udaf">corr_udaf</a>  
AggregateFunction that returns a [`AggregateUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html "struct datafusion::logical_expr::AggregateUDF") for [`Correlation`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/correlation/struct.Correlation.html "struct datafusion::functions_aggregate::correlation::Correlation")
