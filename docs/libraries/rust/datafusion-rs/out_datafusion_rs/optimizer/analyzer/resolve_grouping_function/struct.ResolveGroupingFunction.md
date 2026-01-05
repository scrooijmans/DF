# Struct ResolveGroupingFunction Copy item path

<a href="https://docs.rs/datafusion-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_optimizer/analyzer/resolve_grouping_function.rs.html#44" class="src">Source</a>

``` rust
pub struct ResolveGroupingFunction;
```

Expand description

Replaces grouping aggregation function with value derived from internal grouping id

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/resolve_grouping_function/struct.ResolveGroupingFunction.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/resolve_grouping_function/struct.ResolveGroupingFunction.html#impl-ResolveGroupingFunction" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/resolve_grouping_function/struct.ResolveGroupingFunction.html" class="struct" title="struct datafusion::optimizer::analyzer::resolve_grouping_function::ResolveGroupingFunction">ResolveGroupingFunction</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/resolve_grouping_function/struct.ResolveGroupingFunction.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/resolve_grouping_function/struct.ResolveGroupingFunction.html" class="struct" title="struct datafusion::optimizer::analyzer::resolve_grouping_function::ResolveGroupingFunction">ResolveGroupingFunction</a>

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/resolve_grouping_function/struct.ResolveGroupingFunction.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/resolve_grouping_function/struct.ResolveGroupingFunction.html#impl-AnalyzerRule-for-ResolveGroupingFunction" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.AnalyzerRule.html" class="trait" title="trait datafusion::optimizer::AnalyzerRule">AnalyzerRule</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/resolve_grouping_function/struct.ResolveGroupingFunction.html" class="struct" title="struct datafusion::optimizer::analyzer::resolve_grouping_function::ResolveGroupingFunction">ResolveGroupingFunction</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/resolve_grouping_function/struct.ResolveGroupingFunction.html#method.analyze" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.AnalyzerRule.html#tymethod.analyze" class="fn">analyze</a>( &self, plan: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, \_: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Rewrite `plan`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/resolve_grouping_function/struct.ResolveGroupingFunction.html#method.name" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.AnalyzerRule.html#tymethod.name" class="fn">name</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

A human readable name for this analyzer rule

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/resolve_grouping_function/struct.ResolveGroupingFunction.html#impl-Debug-for-ResolveGroupingFunction" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/resolve_grouping_function/struct.ResolveGroupingFunction.html" class="struct" title="struct datafusion::optimizer::analyzer::resolve_grouping_function::ResolveGroupingFunction">ResolveGroupingFunction</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/resolve_grouping_function/struct.ResolveGroupingFunction.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/resolve_grouping_function/struct.ResolveGroupingFunction.html#impl-Default-for-ResolveGroupingFunction" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/resolve_grouping_function/struct.ResolveGroupingFunction.html" class="struct" title="struct datafusion::optimizer::analyzer::resolve_grouping_function::ResolveGroupingFunction">ResolveGroupingFunction</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/resolve_grouping_function/struct.ResolveGroupingFunction.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/resolve_grouping_function/struct.ResolveGroupingFunction.html" class="struct" title="struct datafusion::optimizer::analyzer::resolve_grouping_function::ResolveGroupingFunction">ResolveGroupingFunction</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/resolve_grouping_function/struct.ResolveGroupingFunction.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/resolve_grouping_function/struct.ResolveGroupingFunction.html#blanket-implementations" class="anchor">§</a>
