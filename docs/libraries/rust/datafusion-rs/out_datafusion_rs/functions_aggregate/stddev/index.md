# Module stddev Copy item path

<a href="https://docs.rs/datafusion-functions-aggregate/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_aggregate/lib.rs.html#85" class="src">Source</a>

Expand description

Defines physical expressions that can evaluated at runtime during query execution

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/stddev/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/stddev/struct.Stddev.html" class="struct" title="struct datafusion::functions_aggregate::stddev::Stddev">Stddev</a>  
STDDEV and STDDEV_SAMP (standard deviation) aggregate expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/stddev/struct.StddevAccumulator.html" class="struct" title="struct datafusion::functions_aggregate::stddev::StddevAccumulator">StddevAccumulator</a>  
An accumulator to compute the average

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/stddev/struct.StddevGroupsAccumulator.html" class="struct" title="struct datafusion::functions_aggregate::stddev::StddevGroupsAccumulator">StddevGroupsAccumulator</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/stddev/struct.StddevPop.html" class="struct" title="struct datafusion::functions_aggregate::stddev::StddevPop">StddevPop</a>  
STDDEV_POP population aggregate expression

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/stddev/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/stddev/fn.stddev.html" class="fn" title="fn datafusion::functions_aggregate::stddev::stddev">stddev</a>  
Compute the standard deviation of a set of numbers

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/stddev/fn.stddev_pop.html" class="fn" title="fn datafusion::functions_aggregate::stddev::stddev_pop">stddev_pop</a>  
Compute the population standard deviation of a set of numbers

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/stddev/fn.stddev_pop_udaf.html" class="fn" title="fn datafusion::functions_aggregate::stddev::stddev_pop_udaf">stddev_pop_udaf</a>  
AggregateFunction that returns a [`AggregateUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html "struct datafusion::logical_expr::AggregateUDF") for [`StddevPop`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/stddev/struct.StddevPop.html "struct datafusion::functions_aggregate::stddev::StddevPop")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/stddev/fn.stddev_udaf.html" class="fn" title="fn datafusion::functions_aggregate::stddev::stddev_udaf">stddev_udaf</a>  
AggregateFunction that returns a [`AggregateUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html "struct datafusion::logical_expr::AggregateUDF") for [`Stddev`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/stddev/struct.Stddev.html "struct datafusion::functions_aggregate::stddev::Stddev")
