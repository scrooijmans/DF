# Module functions_aggregate Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/lib.rs.html#842" class="src">Source</a>

Expand description

re-export of [`datafusion_functions_aggregate`](https://docs.rs/datafusion-functions-aggregate/50.2.0/x86_64-unknown-linux-gnu/datafusion_functions_aggregate/index.html "mod datafusion_functions_aggregate") crate

## Modules<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/approx_distinct/index.html" class="mod" title="mod datafusion::functions_aggregate::approx_distinct">approx_distinct</a>  
Defines physical expressions that can evaluated at runtime during query execution

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/approx_median/index.html" class="mod" title="mod datafusion::functions_aggregate::approx_median">approx_median</a>  
Defines physical expressions for APPROX_MEDIAN that can be evaluated MEDIAN at runtime during query execution

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/approx_percentile_cont/index.html" class="mod" title="mod datafusion::functions_aggregate::approx_percentile_cont">approx_percentile_cont</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/approx_percentile_cont_with_weight/index.html" class="mod" title="mod datafusion::functions_aggregate::approx_percentile_cont_with_weight">approx_percentile_cont_with_weight</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/array_agg/index.html" class="mod" title="mod datafusion::functions_aggregate::array_agg">array_agg</a>  
`ARRAY_AGG` aggregate implementation: [`ArrayAgg`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/array_agg/struct.ArrayAgg.html "struct datafusion::functions_aggregate::array_agg::ArrayAgg")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/average/index.html" class="mod" title="mod datafusion::functions_aggregate::average">average</a>  
Defines `Avg` & `Mean` aggregate & accumulators

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/bit_and_or_xor/index.html" class="mod" title="mod datafusion::functions_aggregate::bit_and_or_xor">bit_and_or_xor</a>  
Defines `BitAnd`, `BitOr`, `BitXor` and `BitXor DISTINCT` aggregate accumulators

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/bool_and_or/index.html" class="mod" title="mod datafusion::functions_aggregate::bool_and_or">bool_and_or</a>  
Defines physical expressions that can evaluated at runtime during query execution

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/correlation/index.html" class="mod" title="mod datafusion::functions_aggregate::correlation">correlation</a>  
[`Correlation`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/correlation/struct.Correlation.html "struct datafusion::functions_aggregate::correlation::Correlation"): correlation sample aggregations.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/count/index.html" class="mod" title="mod datafusion::functions_aggregate::count">count</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/covariance/index.html" class="mod" title="mod datafusion::functions_aggregate::covariance">covariance</a>  
[`CovarianceSample`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/covariance/struct.CovarianceSample.html "struct datafusion::functions_aggregate::covariance::CovarianceSample"): covariance sample aggregations.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/expr_fn/index.html" class="mod" title="mod datafusion::functions_aggregate::expr_fn">expr_fn</a>  
Fluent-style API for creating `Expr`s

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/first_last/index.html" class="mod" title="mod datafusion::functions_aggregate::first_last">first_last</a>  
Defines the FIRST_VALUE/LAST_VALUE aggregations.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/grouping/index.html" class="mod" title="mod datafusion::functions_aggregate::grouping">grouping</a>  
Defines physical expressions that can evaluated at runtime during query execution

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/hyperloglog/index.html" class="mod" title="mod datafusion::functions_aggregate::hyperloglog">hyperloglog</a>  
HyperLogLog

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/macros/index.html" class="mod" title="mod datafusion::functions_aggregate::macros">macros</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/median/index.html" class="mod" title="mod datafusion::functions_aggregate::median">median</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/index.html" class="mod" title="mod datafusion::functions_aggregate::min_max">min_max</a>  
[`Max`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/struct.Max.html "struct datafusion::functions_aggregate::min_max::Max") and [`MaxAccumulator`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/struct.MaxAccumulator.html "struct datafusion::functions_aggregate::min_max::MaxAccumulator") accumulator for the `max` function [`Min`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/struct.Min.html "struct datafusion::functions_aggregate::min_max::Min") and [`MinAccumulator`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/struct.MinAccumulator.html "struct datafusion::functions_aggregate::min_max::MinAccumulator") accumulator for the `min` function

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/nth_value/index.html" class="mod" title="mod datafusion::functions_aggregate::nth_value">nth_value</a>  
Defines NTH_VALUE aggregate expression which may specify ordering requirement that can evaluated at runtime during query execution

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/planner/index.html" class="mod" title="mod datafusion::functions_aggregate::planner">planner</a>  
SQL planning extensions like [`AggregateFunctionPlanner`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/planner/struct.AggregateFunctionPlanner.html "struct datafusion::functions_aggregate::planner::AggregateFunctionPlanner")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/index.html" class="mod" title="mod datafusion::functions_aggregate::regr">regr</a>  
Defines physical expressions that can evaluated at runtime during query execution

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/stddev/index.html" class="mod" title="mod datafusion::functions_aggregate::stddev">stddev</a>  
Defines physical expressions that can evaluated at runtime during query execution

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/string_agg/index.html" class="mod" title="mod datafusion::functions_aggregate::string_agg">string_agg</a>  
[`StringAgg`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/string_agg/struct.StringAgg.html "struct datafusion::functions_aggregate::string_agg::StringAgg") accumulator for the `string_agg` function

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/sum/index.html" class="mod" title="mod datafusion::functions_aggregate::sum">sum</a>  
Defines `SUM` and `SUM DISTINCT` aggregate accumulators

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/index.html" class="mod" title="mod datafusion::functions_aggregate::variance">variance</a>  
[`VarianceSample`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VarianceSample.html "struct datafusion::functions_aggregate::variance::VarianceSample"): variance sample aggregations. [`VariancePopulation`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html "struct datafusion::functions_aggregate::variance::VariancePopulation"): variance population aggregations.

## Macros<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/index.html#macros" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/macro.create_func.html" class="macro" title="macro datafusion::functions_aggregate::create_func">create_func</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/macro.make_udaf_expr.html" class="macro" title="macro datafusion::functions_aggregate::make_udaf_expr">make_udaf_expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/macro.make_udaf_expr_and_func.html" class="macro" title="macro datafusion::functions_aggregate::make_udaf_expr_and_func">make_udaf_expr_and_func</a>

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/fn.all_default_aggregate_functions.html" class="fn" title="fn datafusion::functions_aggregate::all_default_aggregate_functions">all_default_aggregate_functions</a>  
Returns all default aggregate functions

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/fn.register_all.html" class="fn" title="fn datafusion::functions_aggregate::register_all">register_all</a>  
Registers all enabled packages with a [`FunctionRegistry`](https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html "trait datafusion::execution::FunctionRegistry")
