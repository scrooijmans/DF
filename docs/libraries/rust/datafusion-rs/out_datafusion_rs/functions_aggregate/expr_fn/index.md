# Module expr_fn Copy item path

<a href="https://docs.rs/datafusion-functions-aggregate/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_aggregate/lib.rs.html#101" class="src">Source</a>

Expand description

Fluent-style API for creating `Expr`s

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/expr_fn/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/expr_fn/fn.approx_distinct.html" class="fn" title="fn datafusion::functions_aggregate::expr_fn::approx_distinct">approx_distinct</a>  
approximate number of distinct input values

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/expr_fn/fn.approx_median.html" class="fn" title="fn datafusion::functions_aggregate::expr_fn::approx_median">approx_median</a>  
Computes the approximate median of a set of numbers

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/expr_fn/fn.approx_percentile_cont.html" class="fn" title="fn datafusion::functions_aggregate::expr_fn::approx_percentile_cont">approx_percentile_cont</a>  
Computes the approximate percentile continuous of a set of numbers

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/expr_fn/fn.approx_percentile_cont_with_weight.html" class="fn" title="fn datafusion::functions_aggregate::expr_fn::approx_percentile_cont_with_weight">approx_percentile_cont_with_weight</a>  
Computes the approximate percentile continuous with weight of a set of numbers

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/expr_fn/fn.array_agg.html" class="fn" title="fn datafusion::functions_aggregate::expr_fn::array_agg">array_agg</a>  
input values, including nulls, concatenated into an array

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/expr_fn/fn.avg.html" class="fn" title="fn datafusion::functions_aggregate::expr_fn::avg">avg</a>  
Returns the avg of a group of values.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/expr_fn/fn.bit_and.html" class="fn" title="fn datafusion::functions_aggregate::expr_fn::bit_and">bit_and</a>  
Returns the bitwiseBitwiseOperationType::Andof a group of values

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/expr_fn/fn.bit_or.html" class="fn" title="fn datafusion::functions_aggregate::expr_fn::bit_or">bit_or</a>  
Returns the bitwiseBitwiseOperationType::Orof a group of values

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/expr_fn/fn.bit_xor.html" class="fn" title="fn datafusion::functions_aggregate::expr_fn::bit_xor">bit_xor</a>  
Returns the bitwiseBitwiseOperationType::Xorof a group of values

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/expr_fn/fn.bool_and.html" class="fn" title="fn datafusion::functions_aggregate::expr_fn::bool_and">bool_and</a>  
The values to combine with `AND`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/expr_fn/fn.bool_or.html" class="fn" title="fn datafusion::functions_aggregate::expr_fn::bool_or">bool_or</a>  
The values to combine with `OR`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/expr_fn/fn.corr.html" class="fn" title="fn datafusion::functions_aggregate::expr_fn::corr">corr</a>  
Correlation between two numeric values.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/expr_fn/fn.count.html" class="fn" title="fn datafusion::functions_aggregate::expr_fn::count">count</a>  
Count the number of non-null values in the column

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/expr_fn/fn.count_distinct.html" class="fn" title="fn datafusion::functions_aggregate::expr_fn::count_distinct">count_distinct</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/expr_fn/fn.covar_pop.html" class="fn" title="fn datafusion::functions_aggregate::expr_fn::covar_pop">covar_pop</a>  
Computes the population covariance.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/expr_fn/fn.covar_samp.html" class="fn" title="fn datafusion::functions_aggregate::expr_fn::covar_samp">covar_samp</a>  
Computes the sample covariance.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/expr_fn/fn.first_value.html" class="fn" title="fn datafusion::functions_aggregate::expr_fn::first_value">first_value</a>  
Returns the first value in a group of values.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/expr_fn/fn.grouping.html" class="fn" title="fn datafusion::functions_aggregate::expr_fn::grouping">grouping</a>  
Returns 1 if the data is aggregated across the specified column or 0 for not aggregated in the result set.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/expr_fn/fn.last_value.html" class="fn" title="fn datafusion::functions_aggregate::expr_fn::last_value">last_value</a>  
Returns the last value in a group of values.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/expr_fn/fn.max.html" class="fn" title="fn datafusion::functions_aggregate::expr_fn::max">max</a>  
Returns the maximum of a group of values.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/expr_fn/fn.median.html" class="fn" title="fn datafusion::functions_aggregate::expr_fn::median">median</a>  
Computes the median of a set of numbers

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/expr_fn/fn.min.html" class="fn" title="fn datafusion::functions_aggregate::expr_fn::min">min</a>  
Returns the minimum of a group of values.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/expr_fn/fn.nth_value.html" class="fn" title="fn datafusion::functions_aggregate::expr_fn::nth_value">nth_value</a>  
Returns the nth value in a group of values.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/expr_fn/fn.regr_avgx.html" class="fn" title="fn datafusion::functions_aggregate::expr_fn::regr_avgx">regr_avgx</a>  
Compute a linear regression of type [RegrType::AvgX](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/enum.RegrType.html#variant.AvgX "variant datafusion::functions_aggregate::regr::RegrType::AvgX")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/expr_fn/fn.regr_avgy.html" class="fn" title="fn datafusion::functions_aggregate::expr_fn::regr_avgy">regr_avgy</a>  
Compute a linear regression of type [RegrType::AvgY](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/enum.RegrType.html#variant.AvgY "variant datafusion::functions_aggregate::regr::RegrType::AvgY")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/expr_fn/fn.regr_count.html" class="fn" title="fn datafusion::functions_aggregate::expr_fn::regr_count">regr_count</a>  
Compute a linear regression of type [RegrType::Count](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/enum.RegrType.html#variant.Count "variant datafusion::functions_aggregate::regr::RegrType::Count")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/expr_fn/fn.regr_intercept.html" class="fn" title="fn datafusion::functions_aggregate::expr_fn::regr_intercept">regr_intercept</a>  
Compute a linear regression of type [RegrType::Intercept](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/enum.RegrType.html#variant.Intercept "variant datafusion::functions_aggregate::regr::RegrType::Intercept")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/expr_fn/fn.regr_r2.html" class="fn" title="fn datafusion::functions_aggregate::expr_fn::regr_r2">regr_r2</a>  
Compute a linear regression of type [RegrType::R2](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/enum.RegrType.html#variant.R2 "variant datafusion::functions_aggregate::regr::RegrType::R2")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/expr_fn/fn.regr_slope.html" class="fn" title="fn datafusion::functions_aggregate::expr_fn::regr_slope">regr_slope</a>  
Compute a linear regression of type [RegrType::Slope](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/enum.RegrType.html#variant.Slope "variant datafusion::functions_aggregate::regr::RegrType::Slope")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/expr_fn/fn.regr_sxx.html" class="fn" title="fn datafusion::functions_aggregate::expr_fn::regr_sxx">regr_sxx</a>  
Compute a linear regression of type [RegrType::SXX](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/enum.RegrType.html#variant.SXX "variant datafusion::functions_aggregate::regr::RegrType::SXX")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/expr_fn/fn.regr_sxy.html" class="fn" title="fn datafusion::functions_aggregate::expr_fn::regr_sxy">regr_sxy</a>  
Compute a linear regression of type [RegrType::SXY](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/enum.RegrType.html#variant.SXY "variant datafusion::functions_aggregate::regr::RegrType::SXY")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/expr_fn/fn.regr_syy.html" class="fn" title="fn datafusion::functions_aggregate::expr_fn::regr_syy">regr_syy</a>  
Compute a linear regression of type [RegrType::SYY](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/enum.RegrType.html#variant.SYY "variant datafusion::functions_aggregate::regr::RegrType::SYY")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/expr_fn/fn.stddev.html" class="fn" title="fn datafusion::functions_aggregate::expr_fn::stddev">stddev</a>  
Compute the standard deviation of a set of numbers

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/expr_fn/fn.stddev_pop.html" class="fn" title="fn datafusion::functions_aggregate::expr_fn::stddev_pop">stddev_pop</a>  
Compute the population standard deviation of a set of numbers

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/expr_fn/fn.sum.html" class="fn" title="fn datafusion::functions_aggregate::expr_fn::sum">sum</a>  
Returns the sum of a group of values.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/expr_fn/fn.var_pop.html" class="fn" title="fn datafusion::functions_aggregate::expr_fn::var_pop">var_pop</a>  
Computes the population variance.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/expr_fn/fn.var_sample.html" class="fn" title="fn datafusion::functions_aggregate::expr_fn::var_sample">var_sample</a>  
Computes the sample variance.
