# Module covariance Copy item path

<a href="https://docs.rs/datafusion-functions-aggregate/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_aggregate/lib.rs.html#77" class="src">Source</a>

Expand description

[`CovarianceSample`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/covariance/struct.CovarianceSample.html "struct datafusion::functions_aggregate::covariance::CovarianceSample"): covariance sample aggregations.

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/covariance/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/covariance/struct.CovarianceAccumulator.html" class="struct" title="struct datafusion::functions_aggregate::covariance::CovarianceAccumulator">CovarianceAccumulator</a>

An accumulator to compute covariance The algorithm used is an online implementation and numerically stable. It is derived from the following paper for calculating variance: Welford, B. P. (1962). “Note on a method for calculating corrected sums of squares and products”. Technometrics. 4 (3): 419–420. doi:10.2307/1266577. JSTOR 1266577.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/covariance/struct.CovariancePopulation.html" class="struct" title="struct datafusion::functions_aggregate::covariance::CovariancePopulation">CovariancePopulation</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/covariance/struct.CovarianceSample.html" class="struct" title="struct datafusion::functions_aggregate::covariance::CovarianceSample">CovarianceSample</a>

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/covariance/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/covariance/fn.covar_pop.html" class="fn" title="fn datafusion::functions_aggregate::covariance::covar_pop">covar_pop</a>  
Computes the population covariance.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/covariance/fn.covar_pop_udaf.html" class="fn" title="fn datafusion::functions_aggregate::covariance::covar_pop_udaf">covar_pop_udaf</a>  
AggregateFunction that returns a [`AggregateUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html "struct datafusion::logical_expr::AggregateUDF") for [`CovariancePopulation`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/covariance/struct.CovariancePopulation.html "struct datafusion::functions_aggregate::covariance::CovariancePopulation")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/covariance/fn.covar_samp.html" class="fn" title="fn datafusion::functions_aggregate::covariance::covar_samp">covar_samp</a>  
Computes the sample covariance.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/covariance/fn.covar_samp_udaf.html" class="fn" title="fn datafusion::functions_aggregate::covariance::covar_samp_udaf">covar_samp_udaf</a>  
AggregateFunction that returns a [`AggregateUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html "struct datafusion::logical_expr::AggregateUDF") for [`CovarianceSample`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/covariance/struct.CovarianceSample.html "struct datafusion::functions_aggregate::covariance::CovarianceSample")
