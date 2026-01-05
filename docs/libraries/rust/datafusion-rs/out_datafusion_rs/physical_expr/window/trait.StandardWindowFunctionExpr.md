# Trait StandardWindowFunctionExpr Copy item path

<a href="https://docs.rs/datafusion-physical-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr/window/standard_window_function_expr.rs.html#38" class="src">Source</a>

``` rust
pub trait StandardWindowFunctionExpr:
    Send
    + Sync
    + Debug {
    // Required methods
    fn as_any(&self) -> &(dyn Any + 'static);
    fn field(&self) -> Result<Arc<Field>, DataFusionError>;
    fn expressions(&self) -> Vec<Arc<dyn PhysicalExpr>>;
    fn create_evaluator(
        &self,
    ) -> Result<Box<dyn PartitionEvaluator>, DataFusionError>;

    // Provided methods
    fn name(&self) -> &str { ... }
    fn evaluate_args(
        &self,
        batch: &RecordBatch,
    ) -> Result<Vec<Arc<dyn Array>>, DataFusionError> { ... }
    fn reverse_expr(&self) -> Option<Arc<dyn StandardWindowFunctionExpr>> { ... }
    fn get_result_ordering(
        &self,
        _schema: &Arc<Schema>,
    ) -> Option<PhysicalSortExpr> { ... }
}
```

Expand description

Evaluates a window function by instantiating a `[PartitionEvaluator]` for calculating the function’s output in that partition.

Note that unlike aggregation based window functions, some window functions such as `rank` ignore the values in the window frame, but others such as `first_value`, `last_value`, and `nth_value` need the value.

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/window/trait.StandardWindowFunctionExpr.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/window/trait.StandardWindowFunctionExpr.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the aggregate expression as [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") so that it can be downcast to a specific implementation.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/window/trait.StandardWindowFunctionExpr.html#tymethod.field" class="fn">field</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

The field of the final result of evaluating this window function.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/window/trait.StandardWindowFunctionExpr.html#tymethod.expressions" class="fn">expressions</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>\>

Expressions that are passed to the [`PartitionEvaluator`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.PartitionEvaluator.html "trait datafusion::logical_expr::PartitionEvaluator").

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/window/trait.StandardWindowFunctionExpr.html#tymethod.create_evaluator" class="fn">create_evaluator</a>( &self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.PartitionEvaluator.html" class="trait" title="trait datafusion::logical_expr::PartitionEvaluator">PartitionEvaluator</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Create a [`PartitionEvaluator`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.PartitionEvaluator.html "trait datafusion::logical_expr::PartitionEvaluator") for evaluating the function on a particular partition.

## Provided Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/window/trait.StandardWindowFunctionExpr.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/window/trait.StandardWindowFunctionExpr.html#method.name" class="fn">name</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Human readable name such as `"MIN(c2)"` or `"RANK()"`. The default implementation returns placeholder text.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/window/trait.StandardWindowFunctionExpr.html#method.evaluate_args" class="fn">evaluate_args</a>( &self, batch: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Evaluate window function’s arguments against the input window batch and return an [`ArrayRef`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.ArrayRef.html "type datafusion::common::arrow::array::ArrayRef").

Typically, the resulting vector is a single element vector.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/window/trait.StandardWindowFunctionExpr.html#method.reverse_expr" class="fn">reverse_expr</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/window/trait.StandardWindowFunctionExpr.html" class="trait" title="trait datafusion::physical_expr::window::StandardWindowFunctionExpr">StandardWindowFunctionExpr</a>\>\>

Construct a new [`StandardWindowFunctionExpr`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/window/trait.StandardWindowFunctionExpr.html "trait datafusion::physical_expr::window::StandardWindowFunctionExpr") that produces the same result as this function on a window with reverse order. The return value of this function is used by the DataFusion optimizer to avoid re-sorting the data when possible.

Returns `None` (the default) if no reverse is known (or possible).

For example, the reverse of `lead(10)` is `lag(10)`.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/window/trait.StandardWindowFunctionExpr.html#method.get_result_ordering" class="fn">get_result_ordering</a>(&self, \_schema: &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortExpr.html" class="struct" title="struct datafusion::physical_expr::PhysicalSortExpr">PhysicalSortExpr</a>\>

Returns the ordering introduced by the window function, if applicable. Most window functions don’t introduce an ordering, hence the default value is `None`. Note that this information is used to update ordering equivalences.

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/window/trait.StandardWindowFunctionExpr.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/window/trait.StandardWindowFunctionExpr.html#impl-StandardWindowFunctionExpr-for-WindowUDFExpr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/window/trait.StandardWindowFunctionExpr.html" class="trait" title="trait datafusion::physical_expr::window::StandardWindowFunctionExpr">StandardWindowFunctionExpr</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/windows/struct.WindowUDFExpr.html" class="struct" title="struct datafusion::physical_plan::windows::WindowUDFExpr">WindowUDFExpr</a>
