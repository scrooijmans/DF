# Struct ApplyFunctionRewrites Copy item path

<a href="https://docs.rs/datafusion-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_optimizer/analyzer/function_rewrite.rs.html#33" class="src">Source</a>

``` rust
pub struct ApplyFunctionRewrites { /* private fields */ }
```

Expand description

Analyzer rule that invokes [`FunctionRewrite`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_rewriter/trait.FunctionRewrite.html "trait datafusion::logical_expr::expr_rewriter::FunctionRewrite")s on expressions

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/function_rewrite/struct.ApplyFunctionRewrites.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/function_rewrite/struct.ApplyFunctionRewrites.html#impl-ApplyFunctionRewrites" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/function_rewrite/struct.ApplyFunctionRewrites.html" class="struct" title="struct datafusion::optimizer::analyzer::function_rewrite::ApplyFunctionRewrites">ApplyFunctionRewrites</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/function_rewrite/struct.ApplyFunctionRewrites.html#method.new" class="fn">new</a>( function_rewrites: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_rewriter/trait.FunctionRewrite.html" class="trait" title="trait datafusion::logical_expr::expr_rewriter::FunctionRewrite">FunctionRewrite</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>\>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/function_rewrite/struct.ApplyFunctionRewrites.html" class="struct" title="struct datafusion::optimizer::analyzer::function_rewrite::ApplyFunctionRewrites">ApplyFunctionRewrites</a>

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/function_rewrite/struct.ApplyFunctionRewrites.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/function_rewrite/struct.ApplyFunctionRewrites.html#impl-AnalyzerRule-for-ApplyFunctionRewrites" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.AnalyzerRule.html" class="trait" title="trait datafusion::optimizer::AnalyzerRule">AnalyzerRule</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/function_rewrite/struct.ApplyFunctionRewrites.html" class="struct" title="struct datafusion::optimizer::analyzer::function_rewrite::ApplyFunctionRewrites">ApplyFunctionRewrites</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/function_rewrite/struct.ApplyFunctionRewrites.html#method.name" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.AnalyzerRule.html#tymethod.name" class="fn">name</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

A human readable name for this analyzer rule

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/function_rewrite/struct.ApplyFunctionRewrites.html#method.analyze" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.AnalyzerRule.html#tymethod.analyze" class="fn">analyze</a>( &self, plan: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, options: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Rewrite `plan`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/function_rewrite/struct.ApplyFunctionRewrites.html#impl-Debug-for-ApplyFunctionRewrites" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/function_rewrite/struct.ApplyFunctionRewrites.html" class="struct" title="struct datafusion::optimizer::analyzer::function_rewrite::ApplyFunctionRewrites">ApplyFunctionRewrites</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/function_rewrite/struct.ApplyFunctionRewrites.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/function_rewrite/struct.ApplyFunctionRewrites.html#impl-Default-for-ApplyFunctionRewrites" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/function_rewrite/struct.ApplyFunctionRewrites.html" class="struct" title="struct datafusion::optimizer::analyzer::function_rewrite::ApplyFunctionRewrites">ApplyFunctionRewrites</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/function_rewrite/struct.ApplyFunctionRewrites.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/function_rewrite/struct.ApplyFunctionRewrites.html" class="struct" title="struct datafusion::optimizer::analyzer::function_rewrite::ApplyFunctionRewrites">ApplyFunctionRewrites</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/function_rewrite/struct.ApplyFunctionRewrites.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/function_rewrite/struct.ApplyFunctionRewrites.html#blanket-implementations" class="anchor">§</a>
