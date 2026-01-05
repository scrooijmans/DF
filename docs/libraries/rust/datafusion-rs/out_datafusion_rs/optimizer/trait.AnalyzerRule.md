# Trait AnalyzerRule Copy item path

<a href="https://docs.rs/datafusion-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_optimizer/analyzer/mod.rs.html#56" class="src">Source</a>

``` rust
pub trait AnalyzerRule: Debug {
    // Required methods
    fn analyze(
        &self,
        plan: LogicalPlan,
        config: &ConfigOptions,
    ) -> Result<LogicalPlan, DataFusionError>;
    fn name(&self) -> &str;
}
```

Expand description

[`AnalyzerRule`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.AnalyzerRule.html "trait datafusion::optimizer::AnalyzerRule")s transform [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan")s in some way to make the plan valid prior to the rest of the DataFusion optimization process.

`AnalyzerRule`s are different than an [`OptimizerRule`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html "trait datafusion::optimizer::OptimizerRule")s which must preserve the semantics of the `LogicalPlan`, while computing results in a more optimal way.

For example, an `AnalyzerRule` may resolve [`Expr`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr")s into more specific forms such as a subquery reference, or do type coercion to ensure the types of operands are correct.

Use [`SessionState::add_analyzer_rule`](https://docs.rs/datafusion/latest/datafusion/execution/session_state/struct.SessionState.html#method.add_analyzer_rule) to register additional `AnalyzerRule`s.

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.AnalyzerRule.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.AnalyzerRule.html#tymethod.analyze" class="fn">analyze</a>( &self, plan: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, config: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Rewrite `plan`

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.AnalyzerRule.html#tymethod.name" class="fn">name</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

A human readable name for this analyzer rule

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.AnalyzerRule.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.AnalyzerRule.html#impl-AnalyzerRule-for-ApplyFunctionRewrites" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.AnalyzerRule.html" class="trait" title="trait datafusion::optimizer::AnalyzerRule">AnalyzerRule</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/function_rewrite/struct.ApplyFunctionRewrites.html" class="struct" title="struct datafusion::optimizer::analyzer::function_rewrite::ApplyFunctionRewrites">ApplyFunctionRewrites</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.AnalyzerRule.html#impl-AnalyzerRule-for-ResolveGroupingFunction" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.AnalyzerRule.html" class="trait" title="trait datafusion::optimizer::AnalyzerRule">AnalyzerRule</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/resolve_grouping_function/struct.ResolveGroupingFunction.html" class="struct" title="struct datafusion::optimizer::analyzer::resolve_grouping_function::ResolveGroupingFunction">ResolveGroupingFunction</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.AnalyzerRule.html#impl-AnalyzerRule-for-TypeCoercion" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.AnalyzerRule.html" class="trait" title="trait datafusion::optimizer::AnalyzerRule">AnalyzerRule</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/type_coercion/struct.TypeCoercion.html" class="struct" title="struct datafusion::optimizer::analyzer::type_coercion::TypeCoercion">TypeCoercion</a>
