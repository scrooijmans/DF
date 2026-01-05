# Struct Analyzer Copy item path

<a href="https://docs.rs/datafusion-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_optimizer/analyzer/mod.rs.html#72" class="src">Source</a>

``` rust
pub struct Analyzer {
    pub function_rewrites: Vec<Arc<dyn FunctionRewrite + Sync + Send>>,
    pub rules: Vec<Arc<dyn AnalyzerRule + Sync + Send>>,
}
```

Expand description

Rule-based Analyzer.

Applies [`FunctionRewrite`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_rewriter/trait.FunctionRewrite.html "trait datafusion::logical_expr::expr_rewriter::FunctionRewrite")s and [`AnalyzerRule`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.AnalyzerRule.html "trait datafusion::optimizer::AnalyzerRule")s to transform a [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan") in preparation for execution.

For example, the `Analyzer` applies type coercion to ensure the types of operands match the types required by functions.

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/struct.Analyzer.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/struct.Analyzer.html#structfield.function_rewrites" class="anchor field">§</a>`function_rewrites: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc"><code>Arc</code></a>`<dyn `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_rewriter/trait.FunctionRewrite.html" class="trait" title="trait datafusion::logical_expr::expr_rewriter::FunctionRewrite"><code>FunctionRewrite</code></a>` + `<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync"><code>Sync</code></a>` + `<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send"><code>Send</code></a>`>>`

Expr –\> Function writes to apply prior to analysis passes

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/struct.Analyzer.html#structfield.rules" class="anchor field">§</a>`rules: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc"><code>Arc</code></a>`<dyn `<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.AnalyzerRule.html" class="trait" title="trait datafusion::optimizer::AnalyzerRule"><code>AnalyzerRule</code></a>` + `<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync"><code>Sync</code></a>` + `<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send"><code>Send</code></a>`>>`

All rules to apply

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/struct.Analyzer.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/struct.Analyzer.html#impl-Analyzer" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.Analyzer.html" class="struct" title="struct datafusion::optimizer::Analyzer">Analyzer</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/struct.Analyzer.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.Analyzer.html" class="struct" title="struct datafusion::optimizer::Analyzer">Analyzer</a>

Create a new analyzer using the recommended list of rules

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/struct.Analyzer.html#method.with_rules" class="fn">with_rules</a>(rules: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.AnalyzerRule.html" class="trait" title="trait datafusion::optimizer::AnalyzerRule">AnalyzerRule</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>\>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.Analyzer.html" class="struct" title="struct datafusion::optimizer::Analyzer">Analyzer</a>

Create a new analyzer with the given rules

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/struct.Analyzer.html#method.add_function_rewrite" class="fn">add_function_rewrite</a>( &mut self, rewrite: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_rewriter/trait.FunctionRewrite.html" class="trait" title="trait datafusion::logical_expr::expr_rewriter::FunctionRewrite">FunctionRewrite</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>\>, )

Add a function rewrite rule

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/struct.Analyzer.html#method.function_rewrites" class="fn">function_rewrites</a>(&self) -\> &\[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_rewriter/trait.FunctionRewrite.html" class="trait" title="trait datafusion::logical_expr::expr_rewriter::FunctionRewrite">FunctionRewrite</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>\>\]

return the list of function rewrites in this analyzer

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/struct.Analyzer.html#method.execute_and_check" class="fn">execute_and_check</a>\<F\>( &self, plan: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, config: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>, observer: F, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(&<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, &dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.AnalyzerRule.html" class="trait" title="trait datafusion::optimizer::AnalyzerRule">AnalyzerRule</a>),

Analyze the logical plan by applying analyzer rules, and do necessary check and fail the invalid plans

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/struct.Analyzer.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/struct.Analyzer.html#impl-Clone-for-Analyzer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.Analyzer.html" class="struct" title="struct datafusion::optimizer::Analyzer">Analyzer</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/struct.Analyzer.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.Analyzer.html" class="struct" title="struct datafusion::optimizer::Analyzer">Analyzer</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/struct.Analyzer.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/struct.Analyzer.html#impl-Debug-for-Analyzer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.Analyzer.html" class="struct" title="struct datafusion::optimizer::Analyzer">Analyzer</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/struct.Analyzer.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/struct.Analyzer.html#impl-Default-for-Analyzer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.Analyzer.html" class="struct" title="struct datafusion::optimizer::Analyzer">Analyzer</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/struct.Analyzer.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.Analyzer.html" class="struct" title="struct datafusion::optimizer::Analyzer">Analyzer</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/struct.Analyzer.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/struct.Analyzer.html#blanket-implementations" class="anchor">§</a>
