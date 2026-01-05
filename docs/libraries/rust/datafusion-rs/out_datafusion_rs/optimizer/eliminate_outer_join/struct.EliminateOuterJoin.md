# Struct EliminateOuterJoin Copy item path

<a href="https://docs.rs/datafusion-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_optimizer/eliminate_outer_join.rs.html#52" class="src">Source</a>

``` rust
pub struct EliminateOuterJoin;
```

Expand description

Attempt to replace outer joins with inner joins.

Outer joins are typically more expensive to compute at runtime than inner joins and prevent various forms of predicate pushdown and other optimizations, so removing them if possible is beneficial.

Inner joins filter out rows that do match. Outer joins pass rows that do not match padded with nulls. If there is a filter in the query that would filter any such null rows after the join the rows introduced by the outer join are filtered.

For example, in the `select ... from a left join b on ... where b.xx = 100;`

For rows when `b.xx` is null (as it would be after an outer join), the `b.xx = 100` predicate filters them out and there is no need to produce null rows for output.

Generally, an outer join can be rewritten to inner join if the filters from the WHERE clause return false while any inputs are null and columns of those quals are come from nullable side of outer join.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_outer_join/struct.EliminateOuterJoin.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_outer_join/struct.EliminateOuterJoin.html#impl-EliminateOuterJoin" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_outer_join/struct.EliminateOuterJoin.html" class="struct" title="struct datafusion::optimizer::eliminate_outer_join::EliminateOuterJoin">EliminateOuterJoin</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_outer_join/struct.EliminateOuterJoin.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_outer_join/struct.EliminateOuterJoin.html" class="struct" title="struct datafusion::optimizer::eliminate_outer_join::EliminateOuterJoin">EliminateOuterJoin</a>

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_outer_join/struct.EliminateOuterJoin.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_outer_join/struct.EliminateOuterJoin.html#impl-Debug-for-EliminateOuterJoin" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_outer_join/struct.EliminateOuterJoin.html" class="struct" title="struct datafusion::optimizer::eliminate_outer_join::EliminateOuterJoin">EliminateOuterJoin</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_outer_join/struct.EliminateOuterJoin.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_outer_join/struct.EliminateOuterJoin.html#impl-Default-for-EliminateOuterJoin" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_outer_join/struct.EliminateOuterJoin.html" class="struct" title="struct datafusion::optimizer::eliminate_outer_join::EliminateOuterJoin">EliminateOuterJoin</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_outer_join/struct.EliminateOuterJoin.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_outer_join/struct.EliminateOuterJoin.html" class="struct" title="struct datafusion::optimizer::eliminate_outer_join::EliminateOuterJoin">EliminateOuterJoin</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_outer_join/struct.EliminateOuterJoin.html#impl-OptimizerRule-for-EliminateOuterJoin" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html" class="trait" title="trait datafusion::optimizer::OptimizerRule">OptimizerRule</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_outer_join/struct.EliminateOuterJoin.html" class="struct" title="struct datafusion::optimizer::eliminate_outer_join::EliminateOuterJoin">EliminateOuterJoin</a>

Attempt to eliminate outer joins.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_outer_join/struct.EliminateOuterJoin.html#method.name" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html#tymethod.name" class="fn">name</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

A human readable name for this optimizer rule

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_outer_join/struct.EliminateOuterJoin.html#method.apply_order" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html#method.apply_order" class="fn">apply_order</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/enum.ApplyOrder.html" class="enum" title="enum datafusion::optimizer::ApplyOrder">ApplyOrder</a>\>

How should the rule be applied by the optimizer? See comments on [`ApplyOrder`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/enum.ApplyOrder.html "enum datafusion::optimizer::ApplyOrder") for details. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html#method.apply_order)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_outer_join/struct.EliminateOuterJoin.html#method.supports_rewrite" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html#method.supports_rewrite" class="fn">supports_rewrite</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

👎Deprecated since 47.0.0: This method is no longer used

Does this rule support rewriting owned plans (rather than by reference)?

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_outer_join/struct.EliminateOuterJoin.html#method.rewrite" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html#method.rewrite" class="fn">rewrite</a>( &self, plan: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, \_config: &dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerConfig.html" class="trait" title="trait datafusion::optimizer::OptimizerConfig">OptimizerConfig</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Try to rewrite `plan` to an optimized form, returning `Transformed::yes` if the plan was rewritten and `Transformed::no` if it was not.

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_outer_join/struct.EliminateOuterJoin.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_outer_join/struct.EliminateOuterJoin.html#blanket-implementations" class="anchor">§</a>
