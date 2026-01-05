# Module regr Copy item path

<a href="https://docs.rs/datafusion-functions-aggregate/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_aggregate/lib.rs.html#84" class="src">Source</a>

Expand description

Defines physical expressions that can evaluated at runtime during query execution

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/struct.Regr.html" class="struct" title="struct datafusion::functions_aggregate::regr::Regr">Regr</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/struct.RegrAccumulator.html" class="struct" title="struct datafusion::functions_aggregate::regr::RegrAccumulator">RegrAccumulator</a>  
`RegrAccumulator` is used to compute linear regression aggregate functions by maintaining statistics needed to compute them in an online fashion.

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/enum.RegrType.html" class="enum" title="enum datafusion::functions_aggregate::regr::RegrType">RegrType</a>

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/fn.regr_avgx.html" class="fn" title="fn datafusion::functions_aggregate::regr::regr_avgx">regr_avgx</a>  
Compute a linear regression of type [RegrType::AvgX](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/enum.RegrType.html#variant.AvgX "variant datafusion::functions_aggregate::regr::RegrType::AvgX")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/fn.regr_avgx_udaf.html" class="fn" title="fn datafusion::functions_aggregate::regr::regr_avgx_udaf">regr_avgx_udaf</a>  
AggregateFunction that returns a [`AggregateUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html "struct datafusion::logical_expr::AggregateUDF") for [`regr_avgx`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/fn.regr_avgx.html "fn datafusion::functions_aggregate::regr::regr_avgx")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/fn.regr_avgy.html" class="fn" title="fn datafusion::functions_aggregate::regr::regr_avgy">regr_avgy</a>  
Compute a linear regression of type [RegrType::AvgY](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/enum.RegrType.html#variant.AvgY "variant datafusion::functions_aggregate::regr::RegrType::AvgY")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/fn.regr_avgy_udaf.html" class="fn" title="fn datafusion::functions_aggregate::regr::regr_avgy_udaf">regr_avgy_udaf</a>  
AggregateFunction that returns a [`AggregateUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html "struct datafusion::logical_expr::AggregateUDF") for [`regr_avgy`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/fn.regr_avgy.html "fn datafusion::functions_aggregate::regr::regr_avgy")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/fn.regr_count.html" class="fn" title="fn datafusion::functions_aggregate::regr::regr_count">regr_count</a>  
Compute a linear regression of type [RegrType::Count](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/enum.RegrType.html#variant.Count "variant datafusion::functions_aggregate::regr::RegrType::Count")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/fn.regr_count_udaf.html" class="fn" title="fn datafusion::functions_aggregate::regr::regr_count_udaf">regr_count_udaf</a>  
AggregateFunction that returns a [`AggregateUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html "struct datafusion::logical_expr::AggregateUDF") for [`regr_count`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/fn.regr_count.html "fn datafusion::functions_aggregate::regr::regr_count")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/fn.regr_intercept.html" class="fn" title="fn datafusion::functions_aggregate::regr::regr_intercept">regr_intercept</a>  
Compute a linear regression of type [RegrType::Intercept](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/enum.RegrType.html#variant.Intercept "variant datafusion::functions_aggregate::regr::RegrType::Intercept")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/fn.regr_intercept_udaf.html" class="fn" title="fn datafusion::functions_aggregate::regr::regr_intercept_udaf">regr_intercept_udaf</a>  
AggregateFunction that returns a [`AggregateUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html "struct datafusion::logical_expr::AggregateUDF") for [`regr_intercept`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/fn.regr_intercept.html "fn datafusion::functions_aggregate::regr::regr_intercept")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/fn.regr_r2.html" class="fn" title="fn datafusion::functions_aggregate::regr::regr_r2">regr_r2</a>  
Compute a linear regression of type [RegrType::R2](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/enum.RegrType.html#variant.R2 "variant datafusion::functions_aggregate::regr::RegrType::R2")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/fn.regr_r2_udaf.html" class="fn" title="fn datafusion::functions_aggregate::regr::regr_r2_udaf">regr_r2_udaf</a>  
AggregateFunction that returns a [`AggregateUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html "struct datafusion::logical_expr::AggregateUDF") for [`regr_r2`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/fn.regr_r2.html "fn datafusion::functions_aggregate::regr::regr_r2")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/fn.regr_slope.html" class="fn" title="fn datafusion::functions_aggregate::regr::regr_slope">regr_slope</a>  
Compute a linear regression of type [RegrType::Slope](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/enum.RegrType.html#variant.Slope "variant datafusion::functions_aggregate::regr::RegrType::Slope")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/fn.regr_slope_udaf.html" class="fn" title="fn datafusion::functions_aggregate::regr::regr_slope_udaf">regr_slope_udaf</a>  
AggregateFunction that returns a [`AggregateUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html "struct datafusion::logical_expr::AggregateUDF") for [`regr_slope`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/fn.regr_slope.html "fn datafusion::functions_aggregate::regr::regr_slope")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/fn.regr_sxx.html" class="fn" title="fn datafusion::functions_aggregate::regr::regr_sxx">regr_sxx</a>  
Compute a linear regression of type [RegrType::SXX](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/enum.RegrType.html#variant.SXX "variant datafusion::functions_aggregate::regr::RegrType::SXX")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/fn.regr_sxx_udaf.html" class="fn" title="fn datafusion::functions_aggregate::regr::regr_sxx_udaf">regr_sxx_udaf</a>  
AggregateFunction that returns a [`AggregateUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html "struct datafusion::logical_expr::AggregateUDF") for [`regr_sxx`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/fn.regr_sxx.html "fn datafusion::functions_aggregate::regr::regr_sxx")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/fn.regr_sxy.html" class="fn" title="fn datafusion::functions_aggregate::regr::regr_sxy">regr_sxy</a>  
Compute a linear regression of type [RegrType::SXY](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/enum.RegrType.html#variant.SXY "variant datafusion::functions_aggregate::regr::RegrType::SXY")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/fn.regr_sxy_udaf.html" class="fn" title="fn datafusion::functions_aggregate::regr::regr_sxy_udaf">regr_sxy_udaf</a>  
AggregateFunction that returns a [`AggregateUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html "struct datafusion::logical_expr::AggregateUDF") for [`regr_sxy`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/fn.regr_sxy.html "fn datafusion::functions_aggregate::regr::regr_sxy")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/fn.regr_syy.html" class="fn" title="fn datafusion::functions_aggregate::regr::regr_syy">regr_syy</a>  
Compute a linear regression of type [RegrType::SYY](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/enum.RegrType.html#variant.SYY "variant datafusion::functions_aggregate::regr::RegrType::SYY")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/fn.regr_syy_udaf.html" class="fn" title="fn datafusion::functions_aggregate::regr::regr_syy_udaf">regr_syy_udaf</a>  
AggregateFunction that returns a [`AggregateUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html "struct datafusion::logical_expr::AggregateUDF") for [`regr_syy`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/fn.regr_syy.html "fn datafusion::functions_aggregate::regr::regr_syy")
