# Struct ScalarSubqueryToJoin Copy item path

<a href="https://docs.rs/datafusion-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_optimizer/scalar_subquery_to_join.rs.html#41" class="src">Source</a>

``` rust
pub struct ScalarSubqueryToJoin {}
```

Expand description

Optimizer rule for rewriting subquery filters to joins

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/scalar_subquery_to_join/struct.ScalarSubqueryToJoin.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/scalar_subquery_to_join/struct.ScalarSubqueryToJoin.html#impl-ScalarSubqueryToJoin" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/scalar_subquery_to_join/struct.ScalarSubqueryToJoin.html" class="struct" title="struct datafusion::optimizer::scalar_subquery_to_join::ScalarSubqueryToJoin">ScalarSubqueryToJoin</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/scalar_subquery_to_join/struct.ScalarSubqueryToJoin.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/scalar_subquery_to_join/struct.ScalarSubqueryToJoin.html" class="struct" title="struct datafusion::optimizer::scalar_subquery_to_join::ScalarSubqueryToJoin">ScalarSubqueryToJoin</a>

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/scalar_subquery_to_join/struct.ScalarSubqueryToJoin.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/scalar_subquery_to_join/struct.ScalarSubqueryToJoin.html#impl-Debug-for-ScalarSubqueryToJoin" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/scalar_subquery_to_join/struct.ScalarSubqueryToJoin.html" class="struct" title="struct datafusion::optimizer::scalar_subquery_to_join::ScalarSubqueryToJoin">ScalarSubqueryToJoin</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/scalar_subquery_to_join/struct.ScalarSubqueryToJoin.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/scalar_subquery_to_join/struct.ScalarSubqueryToJoin.html#impl-Default-for-ScalarSubqueryToJoin" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/scalar_subquery_to_join/struct.ScalarSubqueryToJoin.html" class="struct" title="struct datafusion::optimizer::scalar_subquery_to_join::ScalarSubqueryToJoin">ScalarSubqueryToJoin</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/scalar_subquery_to_join/struct.ScalarSubqueryToJoin.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/scalar_subquery_to_join/struct.ScalarSubqueryToJoin.html" class="struct" title="struct datafusion::optimizer::scalar_subquery_to_join::ScalarSubqueryToJoin">ScalarSubqueryToJoin</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/scalar_subquery_to_join/struct.ScalarSubqueryToJoin.html#impl-OptimizerRule-for-ScalarSubqueryToJoin" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html" class="trait" title="trait datafusion::optimizer::OptimizerRule">OptimizerRule</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/scalar_subquery_to_join/struct.ScalarSubqueryToJoin.html" class="struct" title="struct datafusion::optimizer::scalar_subquery_to_join::ScalarSubqueryToJoin">ScalarSubqueryToJoin</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/scalar_subquery_to_join/struct.ScalarSubqueryToJoin.html#method.supports_rewrite" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html#method.supports_rewrite" class="fn">supports_rewrite</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

👎Deprecated since 47.0.0: This method is no longer used

Does this rule support rewriting owned plans (rather than by reference)?

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/scalar_subquery_to_join/struct.ScalarSubqueryToJoin.html#method.rewrite" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html#method.rewrite" class="fn">rewrite</a>( &self, plan: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, config: &dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerConfig.html" class="trait" title="trait datafusion::optimizer::OptimizerConfig">OptimizerConfig</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Try to rewrite `plan` to an optimized form, returning `Transformed::yes` if the plan was rewritten and `Transformed::no` if it was not.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/scalar_subquery_to_join/struct.ScalarSubqueryToJoin.html#method.name" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html#tymethod.name" class="fn">name</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

A human readable name for this optimizer rule

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/scalar_subquery_to_join/struct.ScalarSubqueryToJoin.html#method.apply_order" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html#method.apply_order" class="fn">apply_order</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/enum.ApplyOrder.html" class="enum" title="enum datafusion::optimizer::ApplyOrder">ApplyOrder</a>\>

How should the rule be applied by the optimizer? See comments on [`ApplyOrder`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/enum.ApplyOrder.html "enum datafusion::optimizer::ApplyOrder") for details. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html#method.apply_order)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/scalar_subquery_to_join/struct.ScalarSubqueryToJoin.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/scalar_subquery_to_join/struct.ScalarSubqueryToJoin.html#blanket-implementations" class="anchor">§</a>
