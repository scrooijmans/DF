# Struct Optimizer Copy item path

<a href="https://docs.rs/datafusion-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_optimizer/optimizer.rs.html#199" class="src">Source</a>

``` rust
pub struct Optimizer {
    pub rules: Vec<Arc<dyn OptimizerRule + Sync + Send>>,
}
```

Expand description

A rule-based optimizer.

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.Optimizer.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.Optimizer.html#structfield.rules" class="anchor field">§</a>`rules: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc"><code>Arc</code></a>`<dyn `<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html" class="trait" title="trait datafusion::optimizer::OptimizerRule"><code>OptimizerRule</code></a>` + `<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync"><code>Sync</code></a>` + `<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send"><code>Send</code></a>`>>`

All optimizer rules to apply

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.Optimizer.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.Optimizer.html#impl-Optimizer" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.Optimizer.html" class="struct" title="struct datafusion::optimizer::Optimizer">Optimizer</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.Optimizer.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.Optimizer.html" class="struct" title="struct datafusion::optimizer::Optimizer">Optimizer</a>

Create a new optimizer using the recommended list of rules

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.Optimizer.html#method.with_rules" class="fn">with_rules</a>(rules: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html" class="trait" title="trait datafusion::optimizer::OptimizerRule">OptimizerRule</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>\>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.Optimizer.html" class="struct" title="struct datafusion::optimizer::Optimizer">Optimizer</a>

Create a new optimizer with the given rules

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.Optimizer.html#impl-Optimizer-1" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.Optimizer.html" class="struct" title="struct datafusion::optimizer::Optimizer">Optimizer</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.Optimizer.html#method.optimize" class="fn">optimize</a>\<F\>( &self, plan: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, config: &dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerConfig.html" class="trait" title="trait datafusion::optimizer::OptimizerConfig">OptimizerConfig</a>, observer: F, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(&<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, &dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html" class="trait" title="trait datafusion::optimizer::OptimizerRule">OptimizerRule</a>),

Optimizes the logical plan by applying optimizer rules, and invoking observer function after each call

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.Optimizer.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.Optimizer.html#impl-Clone-for-Optimizer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.Optimizer.html" class="struct" title="struct datafusion::optimizer::Optimizer">Optimizer</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.Optimizer.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.Optimizer.html" class="struct" title="struct datafusion::optimizer::Optimizer">Optimizer</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.Optimizer.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.Optimizer.html#impl-Debug-for-Optimizer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.Optimizer.html" class="struct" title="struct datafusion::optimizer::Optimizer">Optimizer</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.Optimizer.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.Optimizer.html#impl-Default-for-Optimizer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.Optimizer.html" class="struct" title="struct datafusion::optimizer::Optimizer">Optimizer</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.Optimizer.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.Optimizer.html" class="struct" title="struct datafusion::optimizer::Optimizer">Optimizer</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.Optimizer.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.Optimizer.html#blanket-implementations" class="anchor">§</a>
