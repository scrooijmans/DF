# Struct WindowUDFExpr Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/windows/mod.rs.html#210" class="src">Source</a>

``` rust
pub struct WindowUDFExpr { /* private fields */ }
```

Expand description

Implements [`StandardWindowFunctionExpr`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/window/trait.StandardWindowFunctionExpr.html "trait datafusion::physical_expr::window::StandardWindowFunctionExpr") for [`WindowUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowUDF.html "struct datafusion::logical_expr::WindowUDF")

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/windows/struct.WindowUDFExpr.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/windows/struct.WindowUDFExpr.html#impl-WindowUDFExpr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/windows/struct.WindowUDFExpr.html" class="struct" title="struct datafusion::physical_plan::windows::WindowUDFExpr">WindowUDFExpr</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/windows/struct.WindowUDFExpr.html#method.fun" class="fn">fun</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowUDF.html" class="struct" title="struct datafusion::logical_expr::WindowUDF">WindowUDF</a>\>

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/windows/struct.WindowUDFExpr.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/windows/struct.WindowUDFExpr.html#impl-Clone-for-WindowUDFExpr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/windows/struct.WindowUDFExpr.html" class="struct" title="struct datafusion::physical_plan::windows::WindowUDFExpr">WindowUDFExpr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/windows/struct.WindowUDFExpr.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/windows/struct.WindowUDFExpr.html" class="struct" title="struct datafusion::physical_plan::windows::WindowUDFExpr">WindowUDFExpr</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/windows/struct.WindowUDFExpr.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/windows/struct.WindowUDFExpr.html#impl-Debug-for-WindowUDFExpr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/windows/struct.WindowUDFExpr.html" class="struct" title="struct datafusion::physical_plan::windows::WindowUDFExpr">WindowUDFExpr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/windows/struct.WindowUDFExpr.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/windows/struct.WindowUDFExpr.html#impl-StandardWindowFunctionExpr-for-WindowUDFExpr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/window/trait.StandardWindowFunctionExpr.html" class="trait" title="trait datafusion::physical_expr::window::StandardWindowFunctionExpr">StandardWindowFunctionExpr</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/windows/struct.WindowUDFExpr.html" class="struct" title="struct datafusion::physical_plan::windows::WindowUDFExpr">WindowUDFExpr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/windows/struct.WindowUDFExpr.html#method.as_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/window/trait.StandardWindowFunctionExpr.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the aggregate expression as [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") so that it can be downcast to a specific implementation.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/windows/struct.WindowUDFExpr.html#method.field" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/window/trait.StandardWindowFunctionExpr.html#tymethod.field" class="fn">field</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

The field of the final result of evaluating this window function.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/windows/struct.WindowUDFExpr.html#method.expressions" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/window/trait.StandardWindowFunctionExpr.html#tymethod.expressions" class="fn">expressions</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>\>

Expressions that are passed to the [`PartitionEvaluator`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.PartitionEvaluator.html "trait datafusion::logical_expr::PartitionEvaluator").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/windows/struct.WindowUDFExpr.html#method.create_evaluator" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/window/trait.StandardWindowFunctionExpr.html#tymethod.create_evaluator" class="fn">create_evaluator</a>( &self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.PartitionEvaluator.html" class="trait" title="trait datafusion::logical_expr::PartitionEvaluator">PartitionEvaluator</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Create a [`PartitionEvaluator`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.PartitionEvaluator.html "trait datafusion::logical_expr::PartitionEvaluator") for evaluating the function on a particular partition.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/windows/struct.WindowUDFExpr.html#method.name" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/window/trait.StandardWindowFunctionExpr.html#method.name" class="fn">name</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Human readable name such as `"MIN(c2)"` or `"RANK()"`. The default implementation returns placeholder text.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/windows/struct.WindowUDFExpr.html#method.reverse_expr" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/window/trait.StandardWindowFunctionExpr.html#method.reverse_expr" class="fn">reverse_expr</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/window/trait.StandardWindowFunctionExpr.html" class="trait" title="trait datafusion::physical_expr::window::StandardWindowFunctionExpr">StandardWindowFunctionExpr</a>\>\>

Construct a new [`StandardWindowFunctionExpr`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/window/trait.StandardWindowFunctionExpr.html "trait datafusion::physical_expr::window::StandardWindowFunctionExpr") that produces the same result as this function on a window with reverse order. The return value of this function is used by the DataFusion optimizer to avoid re-sorting the data when possible. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/window/trait.StandardWindowFunctionExpr.html#method.reverse_expr)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/windows/struct.WindowUDFExpr.html#method.get_result_ordering" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/window/trait.StandardWindowFunctionExpr.html#method.get_result_ordering" class="fn">get_result_ordering</a>(&self, schema: &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortExpr.html" class="struct" title="struct datafusion::physical_expr::PhysicalSortExpr">PhysicalSortExpr</a>\>

Returns the ordering introduced by the window function, if applicable. Most window functions don’t introduce an ordering, hence the default value is `None`. Note that this information is used to update ordering equivalences.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/windows/struct.WindowUDFExpr.html#method.evaluate_args" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/window/trait.StandardWindowFunctionExpr.html#method.evaluate_args" class="fn">evaluate_args</a>( &self, batch: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Evaluate window function’s arguments against the input window batch and return an [`ArrayRef`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.ArrayRef.html "type datafusion::common::arrow::array::ArrayRef"). [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/window/trait.StandardWindowFunctionExpr.html#method.evaluate_args)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/windows/struct.WindowUDFExpr.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/windows/struct.WindowUDFExpr.html#blanket-implementations" class="anchor">§</a>
