# Module variance Copy item path

<a href="https://docs.rs/datafusion-functions-aggregate/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_aggregate/lib.rs.html#88" class="src">Source</a>

Expand description

[`VarianceSample`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VarianceSample.html "struct datafusion::functions_aggregate::variance::VarianceSample"): variance sample aggregations. [`VariancePopulation`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html "struct datafusion::functions_aggregate::variance::VariancePopulation"): variance population aggregations.

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VarianceAccumulator.html" class="struct" title="struct datafusion::functions_aggregate::variance::VarianceAccumulator">VarianceAccumulator</a>

An accumulator to compute variance The algorithm used is an online implementation and numerically stable. It is based on this paper: Welford, B. P. (1962). “Note on a method for calculating corrected sums of squares and products”. Technometrics. 4 (3): 419–420. doi:10.2307/1266577. JSTOR 1266577.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VarianceGroupsAccumulator.html" class="struct" title="struct datafusion::functions_aggregate::variance::VarianceGroupsAccumulator">VarianceGroupsAccumulator</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html" class="struct" title="struct datafusion::functions_aggregate::variance::VariancePopulation">VariancePopulation</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VarianceSample.html" class="struct" title="struct datafusion::functions_aggregate::variance::VarianceSample">VarianceSample</a>

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/fn.var_pop.html" class="fn" title="fn datafusion::functions_aggregate::variance::var_pop">var_pop</a>  
Computes the population variance.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/fn.var_pop_udaf.html" class="fn" title="fn datafusion::functions_aggregate::variance::var_pop_udaf">var_pop_udaf</a>  
AggregateFunction that returns a [`AggregateUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html "struct datafusion::logical_expr::AggregateUDF") for [`VariancePopulation`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html "struct datafusion::functions_aggregate::variance::VariancePopulation")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/fn.var_samp_udaf.html" class="fn" title="fn datafusion::functions_aggregate::variance::var_samp_udaf">var_samp_udaf</a>  
AggregateFunction that returns a [`AggregateUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html "struct datafusion::logical_expr::AggregateUDF") for [`VarianceSample`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VarianceSample.html "struct datafusion::functions_aggregate::variance::VarianceSample")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/fn.var_sample.html" class="fn" title="fn datafusion::functions_aggregate::variance::var_sample">var_sample</a>  
Computes the sample variance.
