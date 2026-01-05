# Struct FilterPushdown Copy item path

<a href="https://docs.rs/datafusion-physical-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_optimizer/filter_pushdown.rs.html#384" class="src">Source</a>

``` rust
pub struct FilterPushdown { /* private fields */ }
```

Expand description

Attempts to recursively push given filters from the top of the tree into leaves.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/filter_pushdown/struct.FilterPushdown.html#default-implementation" class="doc-anchor">§</a>Default Implementation

The default implementation in [`ExecutionPlan::gather_filters_for_pushdown`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.gather_filters_for_pushdown "method datafusion::physical_plan::ExecutionPlan::gather_filters_for_pushdown") and [`ExecutionPlan::handle_child_pushdown_result`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.handle_child_pushdown_result "method datafusion::physical_plan::ExecutionPlan::handle_child_pushdown_result") assumes that:

- Parent filters can’t be passed onto children (determined by [`ExecutionPlan::gather_filters_for_pushdown`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.gather_filters_for_pushdown "method datafusion::physical_plan::ExecutionPlan::gather_filters_for_pushdown"))
- This node has no filters to contribute (determined by [`ExecutionPlan::gather_filters_for_pushdown`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.gather_filters_for_pushdown "method datafusion::physical_plan::ExecutionPlan::gather_filters_for_pushdown")).
- Any filters that could not be pushed down to the children are marked as unsupported (determined by [`ExecutionPlan::handle_child_pushdown_result`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.handle_child_pushdown_result "method datafusion::physical_plan::ExecutionPlan::handle_child_pushdown_result")).

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/filter_pushdown/struct.FilterPushdown.html#example-push-filter-into-a-datasourceexec" class="doc-anchor">§</a>Example: Push filter into a `DataSourceExec`

For example, consider the following plan:

``` text
┌──────────────────────┐
│ CoalesceBatchesExec  │
└──────────────────────┘
            │
            ▼
┌──────────────────────┐
│      FilterExec      │
│  filters = [ id=1]   │
└──────────────────────┘
            │
            ▼
┌──────────────────────┐
│    DataSourceExec    │
│    projection = *    │
└──────────────────────┘
```

Our goal is to move the `id = 1` filter from the [`FilterExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter/struct.FilterExec.html "struct datafusion::physical_plan::filter::FilterExec") node to the `DataSourceExec` node.

If this filter is selective pushing it into the scan can avoid massive amounts of data being read from the source (the projection is `*` so all matching columns are read).

The new plan looks like:

``` text
┌──────────────────────┐
│ CoalesceBatchesExec  │
└──────────────────────┘
          │
          ▼
┌──────────────────────┐
│    DataSourceExec    │
│    projection = *    │
│   filters = [ id=1]  │
└──────────────────────┘
```

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/filter_pushdown/struct.FilterPushdown.html#example-push-filters-with-projectionexec" class="doc-anchor">§</a>Example: Push filters with `ProjectionExec`

Let’s consider a more complex example involving a [`ProjectionExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/projection/struct.ProjectionExec.html "struct datafusion::physical_plan::projection::ProjectionExec") node in between the [`FilterExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter/struct.FilterExec.html "struct datafusion::physical_plan::filter::FilterExec") and `DataSourceExec` nodes that creates a new column that the filter depends on.

``` text
┌──────────────────────┐
│ CoalesceBatchesExec  │
└──────────────────────┘
            │
            ▼
┌──────────────────────┐
│      FilterExec      │
│    filters =         │
│     [cost>50,id=1]   │
└──────────────────────┘
            │
            ▼
┌──────────────────────┐
│    ProjectionExec    │
│ cost = price * 1.2   │
└──────────────────────┘
            │
            ▼
┌──────────────────────┐
│    DataSourceExec    │
│    projection = *    │
└──────────────────────┘
```

We want to push down the filters `[id=1]` to the `DataSourceExec` node, but can’t push down `cost>50` because it requires the [`ProjectionExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/projection/struct.ProjectionExec.html "struct datafusion::physical_plan::projection::ProjectionExec") node to be executed first. A simple thing to do would be to split up the filter into two separate filters and push down the first one:

``` text
┌──────────────────────┐
│ CoalesceBatchesExec  │
└──────────────────────┘
            │
            ▼
┌──────────────────────┐
│      FilterExec      │
│    filters =         │
│     [cost>50]        │
└──────────────────────┘
            │
            ▼
┌──────────────────────┐
│    ProjectionExec    │
│ cost = price * 1.2   │
└──────────────────────┘
            │
            ▼
┌──────────────────────┐
│    DataSourceExec    │
│    projection = *    │
│   filters = [ id=1]  │
└──────────────────────┘
```

We can actually however do better by pushing down `price * 1.2 > 50` instead of `cost > 50`:

``` text
┌──────────────────────┐
│ CoalesceBatchesExec  │
└──────────────────────┘
           │
           ▼
┌──────────────────────┐
│    ProjectionExec    │
│ cost = price * 1.2   │
└──────────────────────┘
           │
           ▼
┌──────────────────────┐
│    DataSourceExec    │
│    projection = *    │
│   filters = [id=1,   │
│   price * 1.2 > 50]  │
└──────────────────────┘
```

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/filter_pushdown/struct.FilterPushdown.html#example-push-filters-within-a-subtree" class="doc-anchor">§</a>Example: Push filters within a subtree

There are also cases where we may be able to push down filters within a subtree but not the entire tree. A good example of this is aggregation nodes:

``` text
┌──────────────────────┐
│ ProjectionExec       │
│ projection = *       │
└──────────────────────┘
          │
          ▼
┌──────────────────────┐
│ FilterExec           │
│ filters = [sum > 10] │
└──────────────────────┘
          │
          ▼
┌───────────────────────┐
│     AggregateExec     │
│    group by = [id]    │
│    aggregate =        │
│      [sum(price)]     │
└───────────────────────┘
          │
          ▼
┌──────────────────────┐
│ FilterExec           │
│ filters = [id=1]     │
└──────────────────────┘
         │
         ▼
┌──────────────────────┐
│ DataSourceExec       │
│ projection = *       │
└──────────────────────┘
```

The transformation here is to push down the `id=1` filter to the `DataSourceExec` node:

``` text
┌──────────────────────┐
│ ProjectionExec       │
│ projection = *       │
└──────────────────────┘
          │
          ▼
┌──────────────────────┐
│ FilterExec           │
│ filters = [sum > 10] │
└──────────────────────┘
          │
          ▼
┌───────────────────────┐
│     AggregateExec     │
│    group by = [id]    │
│    aggregate =        │
│      [sum(price)]     │
└───────────────────────┘
          │
          ▼
┌──────────────────────┐
│ DataSourceExec       │
│ projection = *       │
│ filters = [id=1]     │
└──────────────────────┘
```

The point here is that:

1.  We cannot push down `sum > 10` through the [`AggregateExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/aggregates/struct.AggregateExec.html "struct datafusion::physical_plan::aggregates::AggregateExec") node into the `DataSourceExec` node. Any filters above the [`AggregateExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/aggregates/struct.AggregateExec.html "struct datafusion::physical_plan::aggregates::AggregateExec") node are not pushed down. This is determined by calling [`ExecutionPlan::gather_filters_for_pushdown`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.gather_filters_for_pushdown "method datafusion::physical_plan::ExecutionPlan::gather_filters_for_pushdown") on the [`AggregateExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/aggregates/struct.AggregateExec.html "struct datafusion::physical_plan::aggregates::AggregateExec") node.
2.  We need to keep recursing into the tree so that we can discover the other [`FilterExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter/struct.FilterExec.html "struct datafusion::physical_plan::filter::FilterExec") node and push down the `id=1` filter.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/filter_pushdown/struct.FilterPushdown.html#example-push-filters-through-joins" class="doc-anchor">§</a>Example: Push filters through Joins

It is also possible to push down filters through joins and filters that originate from joins. For example, a hash join where we build a hash table of the left side and probe the right side (ignoring why we would choose this order, typically it depends on the size of each table, etc.).

``` text
             ┌─────────────────────┐
             │     FilterExec      │
             │ filters =           │
             │  [d.size > 100]     │
             └─────────────────────┘
                        │
                        │
             ┌──────────▼──────────┐
             │                     │
             │    HashJoinExec     │
             │ [u.dept@hash(d.id)] │
             │                     │
             └─────────────────────┘
                        │
           ┌────────────┴────────────┐
┌──────────▼──────────┐   ┌──────────▼──────────┐
│   DataSourceExec    │   │   DataSourceExec    │
│  alias [users as u] │   │  alias [dept as d]  │
│                     │   │                     │
└─────────────────────┘   └─────────────────────┘
```

There are two pushdowns we can do here:

1.  Push down the `d.size > 100` filter through the `HashJoinExec` node to the `DataSourceExec` node for the `departments` table.
2.  Push down the hash table state from the `HashJoinExec` node to the `DataSourceExec` node to avoid reading rows from the `users` table that will be eliminated by the join. This can be done via a bloom filter or similar and is not (yet) supported in DataFusion. See <https://github.com/apache/datafusion/issues/7955>.

``` text
             ┌─────────────────────┐
             │                     │
             │    HashJoinExec     │
             │ [u.dept@hash(d.id)] │
             │                     │
             └─────────────────────┘
                        │
           ┌────────────┴────────────┐
┌──────────▼──────────┐   ┌──────────▼──────────┐
│   DataSourceExec    │   │   DataSourceExec    │
│  alias [users as u] │   │  alias [dept as d]  │
│ filters =           │   │  filters =          │
│   [depg@hash(d.id)] │   │    [ d.size > 100]  │
└─────────────────────┘   └─────────────────────┘
```

You may notice in this case that the filter is *dynamic*: the hash table is built *after* the `departments` table is read and at runtime. We don’t have a concrete `InList` filter or similar to push down at optimization time. These sorts of dynamic filters are handled by building a specialized [`PhysicalExpr`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html "trait datafusion::physical_expr::PhysicalExpr") that can be evaluated at runtime and internally maintains a reference to the hash table or other state.

To make working with these sorts of dynamic filters more tractable we have the method [`PhysicalExpr::snapshot`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#method.snapshot "method datafusion::physical_expr::PhysicalExpr::snapshot") which attempts to simplify a dynamic filter into a “basic” non-dynamic filter. For a join this could mean converting it to an `InList` filter or a min/max filter for example. See `datafusion/physical-plan/src/dynamic_filters.rs` for more details.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/filter_pushdown/struct.FilterPushdown.html#example-push-topk-filters-into-scans" class="doc-anchor">§</a>Example: Push TopK filters into Scans

Another form of dynamic filter is pushing down the state of a `TopK` operator for queries like `SELECT * FROM t ORDER BY id LIMIT 10`:

``` text
┌──────────────────────┐
│       TopK           │
│     limit = 10       │
│   order by = [id]    │
└──────────────────────┘
           │
           ▼
┌──────────────────────┐
│    DataSourceExec    │
│    projection = *    │
└──────────────────────┘
```

We can avoid large amounts of data processing by transforming this into:

``` text
┌──────────────────────┐
│       TopK           │
│     limit = 10       │
│   order by = [id]    │
└──────────────────────┘
           │
           ▼
┌──────────────────────┐
│    DataSourceExec    │
│    projection = *    │
│ filters =            │
│    [id < @ TopKHeap] │
└──────────────────────┘
```

Now as we fill our `TopK` heap we can push down the state of the heap to the `DataSourceExec` node to avoid reading files / row groups / pages / rows that could not possibly be in the top 10.

This is not yet implemented in DataFusion. See <https://github.com/apache/datafusion/issues/15037>

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/filter_pushdown/struct.FilterPushdown.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/filter_pushdown/struct.FilterPushdown.html#impl-FilterPushdown" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/filter_pushdown/struct.FilterPushdown.html" class="struct" title="struct datafusion::physical_optimizer::filter_pushdown::FilterPushdown">FilterPushdown</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/filter_pushdown/struct.FilterPushdown.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/filter_pushdown/struct.FilterPushdown.html" class="struct" title="struct datafusion::physical_optimizer::filter_pushdown::FilterPushdown">FilterPushdown</a>

Create a new [`FilterPushdown`](https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/filter_pushdown/struct.FilterPushdown.html "struct datafusion::physical_optimizer::filter_pushdown::FilterPushdown") optimizer rule that runs in the pre-optimization phase. See [`FilterPushdownPhase`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/enum.FilterPushdownPhase.html "enum datafusion::physical_plan::filter_pushdown::FilterPushdownPhase") for more details.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/filter_pushdown/struct.FilterPushdown.html#method.new_post_optimization" class="fn">new_post_optimization</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/filter_pushdown/struct.FilterPushdown.html" class="struct" title="struct datafusion::physical_optimizer::filter_pushdown::FilterPushdown">FilterPushdown</a>

Create a new [`FilterPushdown`](https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/filter_pushdown/struct.FilterPushdown.html "struct datafusion::physical_optimizer::filter_pushdown::FilterPushdown") optimizer rule that runs in the post-optimization phase. See [`FilterPushdownPhase`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/enum.FilterPushdownPhase.html "enum datafusion::physical_plan::filter_pushdown::FilterPushdownPhase") for more details.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/filter_pushdown/struct.FilterPushdown.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/filter_pushdown/struct.FilterPushdown.html#impl-Debug-for-FilterPushdown" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/filter_pushdown/struct.FilterPushdown.html" class="struct" title="struct datafusion::physical_optimizer::filter_pushdown::FilterPushdown">FilterPushdown</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/filter_pushdown/struct.FilterPushdown.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/filter_pushdown/struct.FilterPushdown.html#impl-Default-for-FilterPushdown" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/filter_pushdown/struct.FilterPushdown.html" class="struct" title="struct datafusion::physical_optimizer::filter_pushdown::FilterPushdown">FilterPushdown</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/filter_pushdown/struct.FilterPushdown.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/filter_pushdown/struct.FilterPushdown.html" class="struct" title="struct datafusion::physical_optimizer::filter_pushdown::FilterPushdown">FilterPushdown</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/filter_pushdown/struct.FilterPushdown.html#impl-PhysicalOptimizerRule-for-FilterPushdown" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html" class="trait" title="trait datafusion::physical_optimizer::PhysicalOptimizerRule">PhysicalOptimizerRule</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/filter_pushdown/struct.FilterPushdown.html" class="struct" title="struct datafusion::physical_optimizer::filter_pushdown::FilterPushdown">FilterPushdown</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/filter_pushdown/struct.FilterPushdown.html#method.optimize" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html#tymethod.optimize" class="fn">optimize</a>( &self, plan: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>, config: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Rewrite `plan` to an optimized form

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/filter_pushdown/struct.FilterPushdown.html#method.name" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html#tymethod.name" class="fn">name</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

A human readable name for this optimizer rule

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/filter_pushdown/struct.FilterPushdown.html#method.schema_check" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html#tymethod.schema_check" class="fn">schema_check</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

A flag to indicate whether the physical planner should validate that the rule will not change the schema of the plan after the rewriting. Some of the optimization rules might change the nullable properties of the schema and should disable the schema check.

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/filter_pushdown/struct.FilterPushdown.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/filter_pushdown/struct.FilterPushdown.html#blanket-implementations" class="anchor">§</a>
