# Trait WindowExpr Copy item path

<a href="https://docs.rs/datafusion-physical-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr/window/window_expr.rs.html#70" class="src">Source</a>

``` rust
pub trait WindowExpr:
    Send
    + Sync
    + Debug {
Show 16 methods    // Required methods
    fn as_any(&self) -> &(dyn Any + 'static);
    fn field(&self) -> Result<Arc<Field>, DataFusionError>;
    fn expressions(&self) -> Vec<Arc<dyn PhysicalExpr>>;
    fn evaluate(
        &self,
        batch: &RecordBatch,
    ) -> Result<Arc<dyn Array>, DataFusionError>;
    fn partition_by(&self) -> &[Arc<dyn PhysicalExpr>];
    fn order_by(&self) -> &[PhysicalSortExpr];
    fn get_window_frame(&self) -> &Arc<WindowFrame>;
    fn uses_bounded_memory(&self) -> bool;
    fn get_reverse_expr(&self) -> Option<Arc<dyn WindowExpr>>;
    fn create_window_fn(&self) -> Result<WindowFn, DataFusionError>;

    // Provided methods
    fn name(&self) -> &str { ... }
    fn evaluate_args(
        &self,
        batch: &RecordBatch,
    ) -> Result<Vec<Arc<dyn Array>>, DataFusionError> { ... }
    fn evaluate_stateful(
        &self,
        _partition_batches: &IndexMap<Vec<ScalarValue>, PartitionBatchState>,
        _window_agg_state: &mut IndexMap<Vec<ScalarValue>, WindowState>,
    ) -> Result<(), DataFusionError> { ... }
    fn order_by_columns(
        &self,
        batch: &RecordBatch,
    ) -> Result<Vec<SortColumn>, DataFusionError> { ... }
    fn all_expressions(&self) -> WindowPhysicalExpressions { ... }
    fn with_new_expressions(
        &self,
        _args: Vec<Arc<dyn PhysicalExpr>>,
        _partition_bys: Vec<Arc<dyn PhysicalExpr>>,
        _order_by_exprs: Vec<Arc<dyn PhysicalExpr>>,
    ) -> Option<Arc<dyn WindowExpr>> { ... }
}
```

Expand description

Common trait for [window function](https://en.wikipedia.org/wiki/Window_function_(SQL)) implementations

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.WindowExpr.html#aggregate-window-expressions" class="doc-anchor">§</a>Aggregate Window Expressions

These expressions take the form

``` text
OVER({ROWS | RANGE| GROUPS} BETWEEN UNBOUNDED PRECEDING AND ...)
```

For example, cumulative window frames uses `PlainAggregateWindowExpr`.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.WindowExpr.html#non-aggregate-window-expressions" class="doc-anchor">§</a>Non Aggregate Window Expressions

The expressions have the form

``` text
OVER({ROWS | RANGE| GROUPS} BETWEEN M {PRECEDING| FOLLOWING} AND ...)
```

For example, sliding window frames use [`SlidingAggregateWindowExpr`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/window/struct.SlidingAggregateWindowExpr.html "struct datafusion::physical_expr::window::SlidingAggregateWindowExpr").

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.WindowExpr.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.WindowExpr.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the window expression as [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") so that it can be downcast to a specific implementation.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.WindowExpr.html#tymethod.field" class="fn">field</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

The field of the final result of this window function.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.WindowExpr.html#tymethod.expressions" class="fn">expressions</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>\>

Expressions that are passed to the WindowAccumulator. Functions which take a single input argument, such as `sum`, return a single [`datafusion_expr::expr::Expr`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr"), others (e.g. `cov`) return many.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.WindowExpr.html#tymethod.evaluate" class="fn">evaluate</a>( &self, batch: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Evaluate the window function values against the batch

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.WindowExpr.html#tymethod.partition_by" class="fn">partition_by</a>(&self) -\> &\[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>\]

Expressions that’s from the window function’s partition by clause, empty if absent

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.WindowExpr.html#tymethod.order_by" class="fn">order_by</a>(&self) -\> &\[<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortExpr.html" class="struct" title="struct datafusion::physical_expr::PhysicalSortExpr">PhysicalSortExpr</a>\]

Expressions that’s from the window function’s order by clause, empty if absent

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.WindowExpr.html#tymethod.get_window_frame" class="fn">get_window_frame</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html" class="struct" title="struct datafusion::logical_expr::WindowFrame">WindowFrame</a>\>

Get the window frame of this [WindowExpr](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.WindowExpr.html "trait datafusion::physical_plan::WindowExpr").

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.WindowExpr.html#tymethod.uses_bounded_memory" class="fn">uses_bounded_memory</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Return a flag indicating whether this [WindowExpr](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.WindowExpr.html "trait datafusion::physical_plan::WindowExpr") can run with bounded memory.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.WindowExpr.html#tymethod.get_reverse_expr" class="fn">get_reverse_expr</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.WindowExpr.html" class="trait" title="trait datafusion::physical_plan::WindowExpr">WindowExpr</a>\>\>

Get the reverse expression of this [WindowExpr](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.WindowExpr.html "trait datafusion::physical_plan::WindowExpr").

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.WindowExpr.html#tymethod.create_window_fn" class="fn">create_window_fn</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<WindowFn, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Creates a new instance of the window function evaluator.

Returns `WindowFn::Builtin` for built-in window functions (e.g., ROW_NUMBER, RANK) or `WindowFn::Aggregate` for aggregate window functions (e.g., SUM, AVG).

## Provided Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.WindowExpr.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.WindowExpr.html#method.name" class="fn">name</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Human readable name such as `"MIN(c2)"` or `"RANK()"`. The default implementation returns placeholder text.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.WindowExpr.html#method.evaluate_args" class="fn">evaluate_args</a>( &self, batch: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Evaluate the window function arguments against the batch and return array ref, normally the resulting `Vec` is a single element one.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.WindowExpr.html#method.evaluate_stateful" class="fn">evaluate_stateful</a>( &self, \_partition_batches: &<a href="https://docs.rs/indexmap/2.11.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html" class="struct" title="struct indexmap::map::IndexMap">IndexMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/window_state/struct.PartitionBatchState.html" class="struct" title="struct datafusion::logical_expr::window_state::PartitionBatchState">PartitionBatchState</a>\>, \_window_agg_state: &mut <a href="https://docs.rs/indexmap/2.11.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html" class="struct" title="struct indexmap::map::IndexMap">IndexMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/window/struct.WindowState.html" class="struct" title="struct datafusion::physical_expr::window::WindowState">WindowState</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Evaluate the window function against the batch. This function facilitates stateful, bounded-memory implementations.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.WindowExpr.html#method.order_by_columns" class="fn">order_by_columns</a>( &self, batch: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/struct.SortColumn.html" class="struct" title="struct datafusion::common::arrow::compute::SortColumn">SortColumn</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Get order by columns, empty if absent

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.WindowExpr.html#method.all_expressions" class="fn">all_expressions</a>(&self) -\> WindowPhysicalExpressions

Returns all expressions used in the [`WindowExpr`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.WindowExpr.html "trait datafusion::physical_plan::WindowExpr"). These expressions are (1) function arguments, (2) partition by expressions, (3) order by expressions.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.WindowExpr.html#method.with_new_expressions" class="fn">with_new_expressions</a>( &self, \_args: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>\>, \_partition_bys: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>\>, \_order_by_exprs: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.WindowExpr.html" class="trait" title="trait datafusion::physical_plan::WindowExpr">WindowExpr</a>\>\>

Rewrites [`WindowExpr`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.WindowExpr.html "trait datafusion::physical_plan::WindowExpr"), with new expressions given. The argument should be consistent with the return value of the [`WindowExpr::all_expressions`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.WindowExpr.html#method.all_expressions "method datafusion::physical_plan::WindowExpr::all_expressions") method. Returns `Some(Arc<dyn WindowExpr>)` if re-write is supported, otherwise returns `None`.

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.WindowExpr.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.WindowExpr.html#impl-WindowExpr-for-PlainAggregateWindowExpr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.WindowExpr.html" class="trait" title="trait datafusion::physical_plan::WindowExpr">WindowExpr</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/window/struct.PlainAggregateWindowExpr.html" class="struct" title="struct datafusion::physical_expr::window::PlainAggregateWindowExpr">PlainAggregateWindowExpr</a>

peer based evaluation based on the fact that batch is pre-sorted given the sort columns and then per partition point we’ll evaluate the peer group (e.g. SUM or MAX gives the same results for peers) and concatenate the results.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.WindowExpr.html#impl-WindowExpr-for-SlidingAggregateWindowExpr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.WindowExpr.html" class="trait" title="trait datafusion::physical_plan::WindowExpr">WindowExpr</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/window/struct.SlidingAggregateWindowExpr.html" class="struct" title="struct datafusion::physical_expr::window::SlidingAggregateWindowExpr">SlidingAggregateWindowExpr</a>

Incrementally update window function using the fact that batch is pre-sorted given the sort columns and then per partition point.

Evaluates the peer group (e.g. `SUM` or `MAX` gives the same results for peers) and concatenate the results.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.WindowExpr.html#impl-WindowExpr-for-StandardWindowExpr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.WindowExpr.html" class="trait" title="trait datafusion::physical_plan::WindowExpr">WindowExpr</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/window/struct.StandardWindowExpr.html" class="struct" title="struct datafusion::physical_expr::window::StandardWindowExpr">StandardWindowExpr</a>
