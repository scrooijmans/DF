# Module analyzer Copy item path

<a href="https://docs.rs/datafusion-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_optimizer/lib.rs.html#40" class="src">Source</a>

Expand description

[`Analyzer`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.Analyzer.html "struct datafusion::optimizer::Analyzer") and [`AnalyzerRule`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.AnalyzerRule.html "trait datafusion::optimizer::AnalyzerRule")

## Modules<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/function_rewrite/index.html" class="mod" title="mod datafusion::optimizer::analyzer::function_rewrite">function_rewrite</a>  
[`ApplyFunctionRewrites`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/function_rewrite/struct.ApplyFunctionRewrites.html "struct datafusion::optimizer::analyzer::function_rewrite::ApplyFunctionRewrites") to replace `Expr`s with function calls (e.g `||` to array_concat\`)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/resolve_grouping_function/index.html" class="mod" title="mod datafusion::optimizer::analyzer::resolve_grouping_function">resolve_grouping_function</a>  
Analyzed rule to replace TableScan references such as DataFrames and Views and inlines the LogicalPlan.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/type_coercion/index.html" class="mod" title="mod datafusion::optimizer::analyzer::type_coercion">type_coercion</a>  
Optimizer rule for type validation and coercion

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/struct.Analyzer.html" class="struct" title="struct datafusion::optimizer::analyzer::Analyzer">Analyzer</a>  
Rule-based Analyzer.

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/trait.AnalyzerRule.html" class="trait" title="trait datafusion::optimizer::analyzer::AnalyzerRule">AnalyzerRule</a>  
[`AnalyzerRule`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.AnalyzerRule.html "trait datafusion::optimizer::AnalyzerRule")s transform [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan")s in some way to make the plan valid prior to the rest of the DataFusion optimization process.
