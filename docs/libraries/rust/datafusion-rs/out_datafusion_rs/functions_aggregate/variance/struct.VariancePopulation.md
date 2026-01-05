# Struct VariancePopulation Copy item path

<a href="https://docs.rs/datafusion-functions-aggregate/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_aggregate/variance.rs.html#158" class="src">Source</a>

``` rust
pub struct VariancePopulation { /* private fields */ }
```

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html#impl-VariancePopulation" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html" class="struct" title="struct datafusion::functions_aggregate::variance::VariancePopulation">VariancePopulation</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html" class="struct" title="struct datafusion::functions_aggregate::variance::VariancePopulation">VariancePopulation</a>

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html#impl-AggregateUDFImpl-for-VariancePopulation" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html" class="trait" title="trait datafusion::logical_expr::AggregateUDFImpl">AggregateUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html" class="struct" title="struct datafusion::functions_aggregate::variance::VariancePopulation">VariancePopulation</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html#method.as_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns this object as an [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") trait object

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html#method.name" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#tymethod.name" class="fn">name</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Returns this function’s name

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html#method.signature" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#tymethod.signature" class="fn">signature</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html" class="struct" title="struct datafusion::logical_expr::Signature">Signature</a>

Returns the function’s [`Signature`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html "struct datafusion::logical_expr::Signature") for information about what input types are accepted and the function’s Volatility.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html#method.return_type" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#tymethod.return_type" class="fn">return_type</a>( &self, arg_types: &\[<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

What [`DataType`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html "enum datafusion::common::arrow::datatypes::DataType") will be returned by this function, given the types of the arguments

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html#method.state_fields" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.state_fields" class="fn">state_fields</a>( &self, args: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.StateFieldsArgs.html" class="struct" title="struct datafusion::logical_expr::function::StateFieldsArgs">StateFieldsArgs</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Return the fields used to store the intermediate state of this accumulator. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.state_fields)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html#method.accumulator" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#tymethod.accumulator" class="fn">accumulator</a>( &self, acc_args: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.AccumulatorArgs.html" class="struct" title="struct datafusion::logical_expr::function::AccumulatorArgs">AccumulatorArgs</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html" class="trait" title="trait datafusion::logical_expr::Accumulator">Accumulator</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Return a new [`Accumulator`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html "trait datafusion::logical_expr::Accumulator") that aggregates values for a specific group during query execution. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#tymethod.accumulator)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html#method.aliases" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.aliases" class="fn">aliases</a>(&self) -\> &\[<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\]

Returns any aliases (alternate names) for this function. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.aliases)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html#method.groups_accumulator_supported" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.groups_accumulator_supported" class="fn">groups_accumulator_supported</a>(&self, acc_args: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.AccumulatorArgs.html" class="struct" title="struct datafusion::logical_expr::function::AccumulatorArgs">AccumulatorArgs</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

If the aggregate expression has a specialized [`GroupsAccumulator`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html "trait datafusion::logical_expr::GroupsAccumulator") implementation. If this returns true, `[Self::create_groups_accumulator]` will be called. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.groups_accumulator_supported)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html#method.create_groups_accumulator" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.create_groups_accumulator" class="fn">create_groups_accumulator</a>( &self, \_args: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.AccumulatorArgs.html" class="struct" title="struct datafusion::logical_expr::function::AccumulatorArgs">AccumulatorArgs</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html" class="trait" title="trait datafusion::logical_expr::GroupsAccumulator">GroupsAccumulator</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Return a specialized [`GroupsAccumulator`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html "trait datafusion::logical_expr::GroupsAccumulator") that manages state for all groups. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.create_groups_accumulator)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html#method.documentation" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.documentation" class="fn">documentation</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Documentation.html" class="struct" title="struct datafusion::logical_expr::Documentation">Documentation</a>\>

Returns the documentation for this Aggregate UDF. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.documentation)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html#method.schema_name" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.schema_name" class="fn">schema_name</a>( &self, params: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.AggregateFunctionParams.html" class="struct" title="struct datafusion::logical_expr::expr::AggregateFunctionParams">AggregateFunctionParams</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns the name of the column this expression would create [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.schema_name)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html#method.human_display" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.human_display" class="fn">human_display</a>( &self, params: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.AggregateFunctionParams.html" class="struct" title="struct datafusion::logical_expr::expr::AggregateFunctionParams">AggregateFunctionParams</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns a human readable expression. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.human_display)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html#method.window_function_schema_name" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.window_function_schema_name" class="fn">window_function_schema_name</a>( &self, params: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WindowFunctionParams.html" class="struct" title="struct datafusion::logical_expr::expr::WindowFunctionParams">WindowFunctionParams</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns the name of the column this expression would create [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.window_function_schema_name)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html#method.display_name" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.display_name" class="fn">display_name</a>( &self, params: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.AggregateFunctionParams.html" class="struct" title="struct datafusion::logical_expr::expr::AggregateFunctionParams">AggregateFunctionParams</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns the user-defined display name of function, given the arguments [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.display_name)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html#method.window_function_display_name" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.window_function_display_name" class="fn">window_function_display_name</a>( &self, params: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WindowFunctionParams.html" class="struct" title="struct datafusion::logical_expr::expr::WindowFunctionParams">WindowFunctionParams</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns the user-defined display name of function, given the arguments [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.window_function_display_name)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html#method.return_field" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.return_field" class="fn">return_field</a>( &self, arg_fields: &\[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

What type will be returned by this function, given the arguments? [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.return_field)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html#method.is_nullable" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.is_nullable" class="fn">is_nullable</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Whether the aggregate function is nullable. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.is_nullable)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html#method.create_sliding_accumulator" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.create_sliding_accumulator" class="fn">create_sliding_accumulator</a>( &self, args: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.AccumulatorArgs.html" class="struct" title="struct datafusion::logical_expr::function::AccumulatorArgs">AccumulatorArgs</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html" class="trait" title="trait datafusion::logical_expr::Accumulator">Accumulator</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Sliding accumulator is an alternative accumulator that can be used for window functions. It has retract method to revert the previous update. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.create_sliding_accumulator)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html#method.with_beneficial_ordering" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.with_beneficial_ordering" class="fn">with_beneficial_ordering</a>( self: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<Self\>, \_beneficial_ordering: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html" class="trait" title="trait datafusion::logical_expr::AggregateUDFImpl">AggregateUDFImpl</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Sets the indicator whether ordering requirements of the AggregateUDFImpl is satisfied by its input. If this is not the case, UDFs with order sensitivity `AggregateOrderSensitivity::Beneficial` can still produce the correct result with possibly more work internally. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.with_beneficial_ordering)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html#method.order_sensitivity" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.order_sensitivity" class="fn">order_sensitivity</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/enum.AggregateOrderSensitivity.html" class="enum" title="enum datafusion::logical_expr::utils::AggregateOrderSensitivity">AggregateOrderSensitivity</a>

Gets the order sensitivity of the UDF. See [`AggregateOrderSensitivity`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/enum.AggregateOrderSensitivity.html "enum datafusion::logical_expr::utils::AggregateOrderSensitivity") for possible options.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html#method.simplify" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.simplify" class="fn">simplify</a>( &self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.AggregateFunction.html" class="struct" title="struct datafusion::logical_expr::expr::AggregateFunction">AggregateFunction</a>, &dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/simplify/trait.SimplifyInfo.html" class="trait" title="trait datafusion::logical_expr::simplify::SimplifyInfo">SimplifyInfo</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\>\>

Optionally apply per-UDaF simplification / rewrite rules. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.simplify)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html#method.reverse_expr" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.reverse_expr" class="fn">reverse_expr</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ReversedUDAF.html" class="enum" title="enum datafusion::logical_expr::ReversedUDAF">ReversedUDAF</a>

Returns the reverse expression of the aggregate function.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html#method.coerce_types" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.coerce_types" class="fn">coerce_types</a>( &self, \_arg_types: &\[<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Coerce arguments of a function call to types that the function can evaluate. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.coerce_types)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html#method.is_descending" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.is_descending" class="fn">is_descending</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>

If this function is max, return true If the function is min, return false Otherwise return None (the default) [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.is_descending)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html#method.value_from_stats" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.value_from_stats" class="fn">value_from_stats</a>( &self, \_statistics_args: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.StatisticsArgs.html" class="struct" title="struct datafusion::logical_expr::StatisticsArgs">StatisticsArgs</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>\>

Return the value of this aggregate function if it can be determined entirely from statistics and arguments. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.value_from_stats)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html#method.default_value" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.default_value" class="fn">default_value</a>( &self, data_type: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns default value of the function given the input is all `null`. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.default_value)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html#method.supports_null_handling_clause" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.supports_null_handling_clause" class="fn">supports_null_handling_clause</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

If this function supports `[IGNORE NULLS | RESPECT NULLS]` clause, return true If the function does not, return false

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html#method.is_ordered_set_aggregate" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.is_ordered_set_aggregate" class="fn">is_ordered_set_aggregate</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

If this function is ordered-set aggregate function, return true If the function is not, return false

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html#method.set_monotonicity" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.set_monotonicity" class="fn">set_monotonicity</a>(&self, \_data_type: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.SetMonotonicity.html" class="enum" title="enum datafusion::logical_expr::SetMonotonicity">SetMonotonicity</a>

Indicates whether the aggregation function is monotonic as a set function. See [`SetMonotonicity`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.SetMonotonicity.html "enum datafusion::logical_expr::SetMonotonicity") for details.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html#impl-Debug-for-VariancePopulation" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html" class="struct" title="struct datafusion::functions_aggregate::variance::VariancePopulation">VariancePopulation</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html#impl-Default-for-VariancePopulation" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html" class="struct" title="struct datafusion::functions_aggregate::variance::VariancePopulation">VariancePopulation</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html" class="struct" title="struct datafusion::functions_aggregate::variance::VariancePopulation">VariancePopulation</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html#impl-Hash-for-VariancePopulation" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html" class="struct" title="struct datafusion::functions_aggregate::variance::VariancePopulation">VariancePopulation</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html#impl-PartialEq-for-VariancePopulation" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html" class="struct" title="struct datafusion::functions_aggregate::variance::VariancePopulation">VariancePopulation</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html" class="struct" title="struct datafusion::functions_aggregate::variance::VariancePopulation">VariancePopulation</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html#impl-Eq-for-VariancePopulation" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html" class="struct" title="struct datafusion::functions_aggregate::variance::VariancePopulation">VariancePopulation</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html#impl-StructuralPartialEq-for-VariancePopulation" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html" class="struct" title="struct datafusion::functions_aggregate::variance::VariancePopulation">VariancePopulation</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html#blanket-implementations" class="anchor">§</a>
