# Struct AggregateUDF Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/udaf.rs.html#80" class="src">Source</a>

``` rust
pub struct AggregateUDF { /* private fields */ }
```

Expand description

Logical representation of a user-defined [aggregate function](https://en.wikipedia.org/wiki/Aggregate_function) (UDAF).

An aggregate function combines the values from multiple input rows into a single output “aggregate” (summary) row. It is different from a scalar function because it is stateful across batches. User defined aggregate functions can be used as normal SQL aggregate functions (`GROUP BY` clause) as well as window functions (`OVER` clause).

`AggregateUDF` provides DataFusion the information needed to plan and call aggregate functions, including name, type information, and a factory function to create an [`Accumulator`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html "trait datafusion::logical_expr::Accumulator") instance, to perform the actual aggregation.

For more information, please see [the examples](https://github.com/apache/datafusion/tree/main/datafusion-examples#single-process):

1.  For simple use cases, use [`create_udaf`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.create_udaf.html "fn datafusion::prelude::create_udaf") (examples in [`simple_udaf.rs`](https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/simple_udaf.rs)).

2.  For advanced use cases, use [`AggregateUDFImpl`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html "trait datafusion::logical_expr::AggregateUDFImpl") which provides full API access (examples in [`advanced_udaf.rs`](https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/advanced_udaf.rs)).

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#api-note" class="doc-anchor">§</a>API Note

This is a separate struct from `AggregateUDFImpl` to maintain backwards compatibility with the older API.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#impl-AggregateUDF" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html" class="struct" title="struct datafusion::logical_expr::AggregateUDF">AggregateUDF</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#method.new_from_impl" class="fn">new_from_impl</a>\<F\>(fun: F) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html" class="struct" title="struct datafusion::logical_expr::AggregateUDF">AggregateUDF</a>

where F: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html" class="trait" title="trait datafusion::logical_expr::AggregateUDFImpl">AggregateUDFImpl</a> + 'static,

Create a new `AggregateUDF` from a `[AggregateUDFImpl]` trait object

Note this is the same as using the `From` impl (`AggregateUDF::from`)

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#method.new_from_shared_impl" class="fn">new_from_shared_impl</a>(fun: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html" class="trait" title="trait datafusion::logical_expr::AggregateUDFImpl">AggregateUDFImpl</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html" class="struct" title="struct datafusion::logical_expr::AggregateUDF">AggregateUDF</a>

Create a new `AggregateUDF` from a `[AggregateUDFImpl]` trait object

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#method.inner" class="fn">inner</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html" class="trait" title="trait datafusion::logical_expr::AggregateUDFImpl">AggregateUDFImpl</a>\>

Return the underlying [`AggregateUDFImpl`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html "trait datafusion::logical_expr::AggregateUDFImpl") trait object for this function

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#method.with_aliases" class="fn">with_aliases</a>( self, aliases: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html" class="struct" title="struct datafusion::logical_expr::AggregateUDF">AggregateUDF</a>

Adds additional names that can be used to invoke this function, in addition to `name`

If you implement [`AggregateUDFImpl`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html "trait datafusion::logical_expr::AggregateUDFImpl") directly you should return aliases directly.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#method.call" class="fn">call</a>(&self, args: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Creates an [`Expr`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr") that calls the aggregate function.

This utility allows using the UDAF without requiring access to the registry, such as with the DataFrame API.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#method.name" class="fn">name</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Returns this function’s name

See [`AggregateUDFImpl::name`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#tymethod.name "method datafusion::logical_expr::AggregateUDFImpl::name") for more details.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#method.aliases" class="fn">aliases</a>(&self) -\> &\[<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\]

Returns the aliases for this function.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#method.schema_name" class="fn">schema_name</a>( &self, params: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.AggregateFunctionParams.html" class="struct" title="struct datafusion::logical_expr::expr::AggregateFunctionParams">AggregateFunctionParams</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

See [`AggregateUDFImpl::schema_name`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.schema_name "method datafusion::logical_expr::AggregateUDFImpl::schema_name") for more details.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#method.human_display" class="fn">human_display</a>( &self, params: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.AggregateFunctionParams.html" class="struct" title="struct datafusion::logical_expr::expr::AggregateFunctionParams">AggregateFunctionParams</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns a human readable expression.

See [`Expr::human_display`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.human_display "method datafusion::prelude::Expr::human_display") for details.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#method.window_function_schema_name" class="fn">window_function_schema_name</a>( &self, params: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WindowFunctionParams.html" class="struct" title="struct datafusion::logical_expr::expr::WindowFunctionParams">WindowFunctionParams</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#method.display_name" class="fn">display_name</a>( &self, params: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.AggregateFunctionParams.html" class="struct" title="struct datafusion::logical_expr::expr::AggregateFunctionParams">AggregateFunctionParams</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

See [`AggregateUDFImpl::display_name`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.display_name "method datafusion::logical_expr::AggregateUDFImpl::display_name") for more details.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#method.window_function_display_name" class="fn">window_function_display_name</a>( &self, params: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WindowFunctionParams.html" class="struct" title="struct datafusion::logical_expr::expr::WindowFunctionParams">WindowFunctionParams</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#method.is_nullable" class="fn">is_nullable</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#method.signature" class="fn">signature</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html" class="struct" title="struct datafusion::logical_expr::Signature">Signature</a>

Returns this function’s signature (what input types are accepted)

See [`AggregateUDFImpl::signature`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#tymethod.signature "method datafusion::logical_expr::AggregateUDFImpl::signature") for more details.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#method.return_type" class="fn">return_type</a>( &self, args: &\[<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Return the type of the function given its input types

See [`AggregateUDFImpl::return_type`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#tymethod.return_type "method datafusion::logical_expr::AggregateUDFImpl::return_type") for more details.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#method.return_field" class="fn">return_field</a>( &self, args: &\[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Return the field of the function given its input fields

See [`AggregateUDFImpl::return_field`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.return_field "method datafusion::logical_expr::AggregateUDFImpl::return_field") for more details.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#method.accumulator" class="fn">accumulator</a>( &self, acc_args: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.AccumulatorArgs.html" class="struct" title="struct datafusion::logical_expr::function::AccumulatorArgs">AccumulatorArgs</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html" class="trait" title="trait datafusion::logical_expr::Accumulator">Accumulator</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Return an accumulator the given aggregate, given its return datatype

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#method.state_fields" class="fn">state_fields</a>( &self, args: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.StateFieldsArgs.html" class="struct" title="struct datafusion::logical_expr::function::StateFieldsArgs">StateFieldsArgs</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Return the fields used to store the intermediate state for this aggregator, given the name of the aggregate, value type and ordering fields. See [`AggregateUDFImpl::state_fields`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.state_fields "method datafusion::logical_expr::AggregateUDFImpl::state_fields") for more details.

This is used to support multi-phase aggregations

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#method.groups_accumulator_supported" class="fn">groups_accumulator_supported</a>(&self, args: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.AccumulatorArgs.html" class="struct" title="struct datafusion::logical_expr::function::AccumulatorArgs">AccumulatorArgs</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

See [`AggregateUDFImpl::groups_accumulator_supported`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.groups_accumulator_supported "method datafusion::logical_expr::AggregateUDFImpl::groups_accumulator_supported") for more details.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#method.create_groups_accumulator" class="fn">create_groups_accumulator</a>( &self, args: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.AccumulatorArgs.html" class="struct" title="struct datafusion::logical_expr::function::AccumulatorArgs">AccumulatorArgs</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html" class="trait" title="trait datafusion::logical_expr::GroupsAccumulator">GroupsAccumulator</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

See [`AggregateUDFImpl::create_groups_accumulator`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.create_groups_accumulator "method datafusion::logical_expr::AggregateUDFImpl::create_groups_accumulator") for more details.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#method.create_sliding_accumulator" class="fn">create_sliding_accumulator</a>( &self, args: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.AccumulatorArgs.html" class="struct" title="struct datafusion::logical_expr::function::AccumulatorArgs">AccumulatorArgs</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html" class="trait" title="trait datafusion::logical_expr::Accumulator">Accumulator</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#method.coerce_types" class="fn">coerce_types</a>( &self, arg_types: &\[<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#method.with_beneficial_ordering" class="fn">with_beneficial_ordering</a>( self, beneficial_ordering: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html" class="struct" title="struct datafusion::logical_expr::AggregateUDF">AggregateUDF</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

See [`AggregateUDFImpl::with_beneficial_ordering`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.with_beneficial_ordering "method datafusion::logical_expr::AggregateUDFImpl::with_beneficial_ordering") for more details.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#method.order_sensitivity" class="fn">order_sensitivity</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/enum.AggregateOrderSensitivity.html" class="enum" title="enum datafusion::logical_expr::utils::AggregateOrderSensitivity">AggregateOrderSensitivity</a>

Gets the order sensitivity of the UDF. See [`AggregateOrderSensitivity`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/enum.AggregateOrderSensitivity.html "enum datafusion::logical_expr::utils::AggregateOrderSensitivity") for possible options.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#method.reverse_udf" class="fn">reverse_udf</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ReversedUDAF.html" class="enum" title="enum datafusion::logical_expr::ReversedUDAF">ReversedUDAF</a>

Reserves the `AggregateUDF` (e.g. returns the `AggregateUDF` that will generate same result with this `AggregateUDF` when iterated in reverse order, and `None` if there is no such `AggregateUDF`).

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#method.simplify" class="fn">simplify</a>( &self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.AggregateFunction.html" class="struct" title="struct datafusion::logical_expr::expr::AggregateFunction">AggregateFunction</a>, &dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/simplify/trait.SimplifyInfo.html" class="trait" title="trait datafusion::logical_expr::simplify::SimplifyInfo">SimplifyInfo</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\>\>

Do the function rewrite

See [`AggregateUDFImpl::simplify`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.simplify "method datafusion::logical_expr::AggregateUDFImpl::simplify") for more details.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#method.is_descending" class="fn">is_descending</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>

Returns true if the function is max, false if the function is min None in all other cases, used in certain optimizations for or aggregate

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#method.value_from_stats" class="fn">value_from_stats</a>( &self, statistics_args: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.StatisticsArgs.html" class="struct" title="struct datafusion::logical_expr::StatisticsArgs">StatisticsArgs</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>\>

Return the value of this aggregate function if it can be determined entirely from statistics and arguments.

See [`AggregateUDFImpl::value_from_stats`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.value_from_stats "method datafusion::logical_expr::AggregateUDFImpl::value_from_stats") for more details.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#method.default_value" class="fn">default_value</a>( &self, data_type: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

See [`AggregateUDFImpl::default_value`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.default_value "method datafusion::logical_expr::AggregateUDFImpl::default_value") for more details.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#method.supports_null_handling_clause" class="fn">supports_null_handling_clause</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

See [`AggregateUDFImpl::supports_null_handling_clause`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.supports_null_handling_clause "method datafusion::logical_expr::AggregateUDFImpl::supports_null_handling_clause") for more details.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#method.is_ordered_set_aggregate" class="fn">is_ordered_set_aggregate</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

See [`AggregateUDFImpl::is_ordered_set_aggregate`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.is_ordered_set_aggregate "method datafusion::logical_expr::AggregateUDFImpl::is_ordered_set_aggregate") for more details.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#method.documentation" class="fn">documentation</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Documentation.html" class="struct" title="struct datafusion::logical_expr::Documentation">Documentation</a>\>

Returns the documentation for this Aggregate UDF.

Documentation can be accessed programmatically as well as generating publicly facing documentation.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#impl-Clone-for-AggregateUDF" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html" class="struct" title="struct datafusion::logical_expr::AggregateUDF">AggregateUDF</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html" class="struct" title="struct datafusion::logical_expr::AggregateUDF">AggregateUDF</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#impl-Debug-for-AggregateUDF" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html" class="struct" title="struct datafusion::logical_expr::AggregateUDF">AggregateUDF</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#impl-Display-for-AggregateUDF" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html" class="struct" title="struct datafusion::logical_expr::AggregateUDF">AggregateUDF</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#impl-From%3CF%3E-for-AggregateUDF" class="anchor">§</a>

### impl\<F\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<F\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html" class="struct" title="struct datafusion::logical_expr::AggregateUDF">AggregateUDF</a>

where F: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html" class="trait" title="trait datafusion::logical_expr::AggregateUDFImpl">AggregateUDFImpl</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + 'static,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(fun: F) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html" class="struct" title="struct datafusion::logical_expr::AggregateUDF">AggregateUDF</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#impl-Hash-for-AggregateUDF" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html" class="struct" title="struct datafusion::logical_expr::AggregateUDF">AggregateUDF</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#impl-PartialEq-for-AggregateUDF" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html" class="struct" title="struct datafusion::logical_expr::AggregateUDF">AggregateUDF</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html" class="struct" title="struct datafusion::logical_expr::AggregateUDF">AggregateUDF</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#impl-PartialOrd-for-AggregateUDF" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html" class="struct" title="struct datafusion::logical_expr::AggregateUDF">AggregateUDF</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#method.partial_cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp" class="fn">partial_cmp</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html" class="struct" title="struct datafusion::logical_expr::AggregateUDF">AggregateUDF</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1398" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#method.lt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt" class="fn">lt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1416" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#method.le" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le" class="fn">le</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1434" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#method.gt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt" class="fn">gt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1452" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#method.ge" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge" class="fn">ge</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#impl-Eq-for-AggregateUDF" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html" class="struct" title="struct datafusion::logical_expr::AggregateUDF">AggregateUDF</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html#blanket-implementations" class="anchor">§</a>
