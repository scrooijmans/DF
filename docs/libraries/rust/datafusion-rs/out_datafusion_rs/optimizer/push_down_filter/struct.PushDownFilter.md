# Struct PushDownFilter Copy item path

<a href="https://docs.rs/datafusion-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_optimizer/push_down_filter.rs.html#136" class="src">Source</a>

``` rust
pub struct PushDownFilter {}
```

Expand description

Optimizer rule for pushing (moving) filter expressions down in a plan so they are applied as early as possible.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/push_down_filter/struct.PushDownFilter.html#introduction" class="doc-anchor">§</a>Introduction

The goal of this rule is to improve query performance by eliminating redundant work.

For example, given a plan that sorts all values where `a > 10`:

``` text
 Filter (a > 10)
   Sort (a, b)
```

A better plan is to filter the data *before* the Sort, which sorts fewer rows and therefore does less work overall:

``` text
 Sort (a, b)
   Filter (a > 10)  <-- Filter is moved before the sort
```

However it is not always possible to push filters down. For example, given a plan that finds the top 3 values and then keeps only those that are greater than 10, if the filter is pushed below the limit it would produce a different result.

``` text
 Filter (a > 10)   <-- can not move this Filter before the limit
   Limit (fetch=3)
     Sort (a, b)
```

More formally, a filter-commutative operation is an operation `op` that satisfies `filter(op(data)) = op(filter(data))`.

The filter-commutative property is plan and column-specific. A filter on `a` can be pushed through a `Aggregate(group_by = [a], agg=[sum(b))`. However, a filter on `sum(b)` can not be pushed through the same aggregate.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/push_down_filter/struct.PushDownFilter.html#handling-conjunctions" class="doc-anchor">§</a>Handling Conjunctions

It is possible to only push down **part** of a filter expression if is connected with `AND`s (more formally if it is a “conjunction”).

For example, given the following plan:

``` text
Filter(a > 10 AND sum(b) < 5)
  Aggregate(group_by = [a], agg = [sum(b))
```

The `a > 10` is commutative with the `Aggregate` but `sum(b) < 5` is not. Therefore it is possible to only push part of the expression, resulting in:

``` text
Filter(sum(b) < 5)
  Aggregate(group_by = [a], agg = [sum(b))
    Filter(a > 10)
```

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/push_down_filter/struct.PushDownFilter.html#handling-column-aliases" class="doc-anchor">§</a>Handling Column Aliases

This optimizer must sometimes handle re-writing filter expressions when they pushed, for example if there is a projection that aliases `a+1` to `"b"`:

``` text
Filter (b > 10)
    Projection: [a+1 AS "b"]  <-- changes the name of `a+1` to `b`
```

To apply the filter prior to the `Projection`, all references to `b` must be rewritten to `a+1`:

``` text
Projection: a AS "b"
    Filter: (a + 1 > 10)  <--- changed from b to a + 1
```

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/push_down_filter/struct.PushDownFilter.html#implementation-notes" class="doc-anchor">§</a>Implementation Notes

This implementation performs a single pass through the plan, “pushing” down filters. When it passes through a filter, it stores that filter, and when it reaches a plan node that does not commute with that filter, it adds the filter to that place. When it passes through a projection, it re-writes the filter’s expression taking into account that projection.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/push_down_filter/struct.PushDownFilter.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/push_down_filter/struct.PushDownFilter.html#impl-PushDownFilter" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/push_down_filter/struct.PushDownFilter.html" class="struct" title="struct datafusion::optimizer::push_down_filter::PushDownFilter">PushDownFilter</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/push_down_filter/struct.PushDownFilter.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/push_down_filter/struct.PushDownFilter.html" class="struct" title="struct datafusion::optimizer::push_down_filter::PushDownFilter">PushDownFilter</a>

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/push_down_filter/struct.PushDownFilter.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/push_down_filter/struct.PushDownFilter.html#impl-Debug-for-PushDownFilter" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/push_down_filter/struct.PushDownFilter.html" class="struct" title="struct datafusion::optimizer::push_down_filter::PushDownFilter">PushDownFilter</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/push_down_filter/struct.PushDownFilter.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/push_down_filter/struct.PushDownFilter.html#impl-Default-for-PushDownFilter" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/push_down_filter/struct.PushDownFilter.html" class="struct" title="struct datafusion::optimizer::push_down_filter::PushDownFilter">PushDownFilter</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/push_down_filter/struct.PushDownFilter.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/push_down_filter/struct.PushDownFilter.html" class="struct" title="struct datafusion::optimizer::push_down_filter::PushDownFilter">PushDownFilter</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/push_down_filter/struct.PushDownFilter.html#impl-OptimizerRule-for-PushDownFilter" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html" class="trait" title="trait datafusion::optimizer::OptimizerRule">OptimizerRule</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/push_down_filter/struct.PushDownFilter.html" class="struct" title="struct datafusion::optimizer::push_down_filter::PushDownFilter">PushDownFilter</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/push_down_filter/struct.PushDownFilter.html#method.name" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html#tymethod.name" class="fn">name</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

A human readable name for this optimizer rule

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/push_down_filter/struct.PushDownFilter.html#method.apply_order" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html#method.apply_order" class="fn">apply_order</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/enum.ApplyOrder.html" class="enum" title="enum datafusion::optimizer::ApplyOrder">ApplyOrder</a>\>

How should the rule be applied by the optimizer? See comments on [`ApplyOrder`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/enum.ApplyOrder.html "enum datafusion::optimizer::ApplyOrder") for details. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html#method.apply_order)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/push_down_filter/struct.PushDownFilter.html#method.supports_rewrite" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html#method.supports_rewrite" class="fn">supports_rewrite</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

👎Deprecated since 47.0.0: This method is no longer used

Does this rule support rewriting owned plans (rather than by reference)?

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/push_down_filter/struct.PushDownFilter.html#method.rewrite" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html#method.rewrite" class="fn">rewrite</a>( &self, plan: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, \_config: &dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerConfig.html" class="trait" title="trait datafusion::optimizer::OptimizerConfig">OptimizerConfig</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Try to rewrite `plan` to an optimized form, returning `Transformed::yes` if the plan was rewritten and `Transformed::no` if it was not.

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/push_down_filter/struct.PushDownFilter.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/push_down_filter/struct.PushDownFilter.html#blanket-implementations" class="anchor">§</a>
