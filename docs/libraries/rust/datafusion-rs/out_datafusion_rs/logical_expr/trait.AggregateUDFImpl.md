# Trait AggregateUDFImpl Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/udaf.rs.html#430" class="src">Source</a>

``` rust
pub trait AggregateUDFImpl:
    Debug
    + DynEq
    + DynHash
    + Send
    + Sync {
Show 29 methods    // Required methods
    fn as_any(&self) -> &(dyn Any + 'static);
    fn name(&self) -> &str;
    fn signature(&self) -> &Signature;
    fn return_type(
        &self,
        arg_types: &[DataType],
    ) -> Result<DataType, DataFusionError>;
    fn accumulator(
        &self,
        acc_args: AccumulatorArgs<'_>,
    ) -> Result<Box<dyn Accumulator>, DataFusionError>;

    // Provided methods
    fn aliases(&self) -> &[String] { ... }
    fn schema_name(
        &self,
        params: &AggregateFunctionParams,
    ) -> Result<String, DataFusionError> { ... }
    fn human_display(
        &self,
        params: &AggregateFunctionParams,
    ) -> Result<String, DataFusionError> { ... }
    fn window_function_schema_name(
        &self,
        params: &WindowFunctionParams,
    ) -> Result<String, DataFusionError> { ... }
    fn display_name(
        &self,
        params: &AggregateFunctionParams,
    ) -> Result<String, DataFusionError> { ... }
    fn window_function_display_name(
        &self,
        params: &WindowFunctionParams,
    ) -> Result<String, DataFusionError> { ... }
    fn return_field(
        &self,
        arg_fields: &[Arc<Field>],
    ) -> Result<Arc<Field>, DataFusionError> { ... }
    fn is_nullable(&self) -> bool { ... }
    fn state_fields(
        &self,
        args: StateFieldsArgs<'_>,
    ) -> Result<Vec<Arc<Field>>, DataFusionError> { ... }
    fn groups_accumulator_supported(&self, _args: AccumulatorArgs<'_>) -> bool { ... }
    fn create_groups_accumulator(
        &self,
        _args: AccumulatorArgs<'_>,
    ) -> Result<Box<dyn GroupsAccumulator>, DataFusionError> { ... }
    fn create_sliding_accumulator(
        &self,
        args: AccumulatorArgs<'_>,
    ) -> Result<Box<dyn Accumulator>, DataFusionError> { ... }
    fn with_beneficial_ordering(
        self: Arc<Self>,
        _beneficial_ordering: bool,
    ) -> Result<Option<Arc<dyn AggregateUDFImpl>>, DataFusionError> { ... }
    fn order_sensitivity(&self) -> AggregateOrderSensitivity { ... }
    fn simplify(
        &self,
    ) -> Option<Box<dyn Fn(AggregateFunction, &dyn SimplifyInfo) -> Result<Expr, DataFusionError>>> { ... }
    fn reverse_expr(&self) -> ReversedUDAF { ... }
    fn coerce_types(
        &self,
        _arg_types: &[DataType],
    ) -> Result<Vec<DataType>, DataFusionError> { ... }
    fn is_descending(&self) -> Option<bool> { ... }
    fn value_from_stats(
        &self,
        _statistics_args: &StatisticsArgs<'_>,
    ) -> Option<ScalarValue> { ... }
    fn default_value(
        &self,
        data_type: &DataType,
    ) -> Result<ScalarValue, DataFusionError> { ... }
    fn supports_null_handling_clause(&self) -> bool { ... }
    fn is_ordered_set_aggregate(&self) -> bool { ... }
    fn documentation(&self) -> Option<&Documentation> { ... }
    fn set_monotonicity(&self, _data_type: &DataType) -> SetMonotonicity { ... }
}
```

Expand description

Trait for implementing [`AggregateUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html "struct datafusion::logical_expr::AggregateUDF").

This trait exposes the full API for implementing user defined aggregate functions and can be used to implement any function.

See [`advanced_udaf.rs`](https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/advanced_udaf.rs) for a full example with complete implementation and [`AggregateUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html "struct datafusion::logical_expr::AggregateUDF") for other available options.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#basic-example" class="doc-anchor">§</a>Basic Example

``` rust

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct GeoMeanUdf {
  signature: Signature,
}

impl GeoMeanUdf {
  fn new() -> Self {
    Self {
      signature: Signature::uniform(1, vec![DataType::Float64], Volatility::Immutable),
     }
  }
}

static DOCUMENTATION: LazyLock<Documentation> = LazyLock::new(|| {
        Documentation::builder(DOC_SECTION_AGGREGATE, "calculates a geometric mean", "geo_mean(2.0)")
            .with_argument("arg1", "The Float64 number for the geometric mean")
            .build()
    });

fn get_doc() -> &'static Documentation {
    &DOCUMENTATION
}

/// Implement the AggregateUDFImpl trait for GeoMeanUdf
impl AggregateUDFImpl for GeoMeanUdf {
   fn as_any(&self) -> &dyn Any { self }
   fn name(&self) -> &str { "geo_mean" }
   fn signature(&self) -> &Signature { &self.signature }
   fn return_type(&self, args: &[DataType]) -> Result<DataType> {
     if !matches!(args.get(0), Some(&DataType::Float64)) {
       return plan_err!("geo_mean only accepts Float64 arguments");
     }
     Ok(DataType::Float64)
   }
   // This is the accumulator factory; DataFusion uses it to create new accumulators.
   fn accumulator(&self, _acc_args: AccumulatorArgs) -> Result<Box<dyn Accumulator>> { unimplemented!() }
   fn state_fields(&self, args: StateFieldsArgs) -> Result<Vec<FieldRef>> {
       Ok(vec![
            Arc::new(args.return_field.as_ref().clone().with_name("value")),
            Arc::new(Field::new("ordering", DataType::UInt32, true))
       ])
   }
   fn documentation(&self) -> Option<&Documentation> {
       Some(get_doc())
   }
}

// Create a new AggregateUDF from the implementation
let geometric_mean = AggregateUDF::from(GeoMeanUdf::new());

// Call the function `geo_mean(col)`
let expr = geometric_mean.call(vec![col("a")]);
```

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns this object as an [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") trait object

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#tymethod.name" class="fn">name</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Returns this function’s name

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#tymethod.signature" class="fn">signature</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html" class="struct" title="struct datafusion::logical_expr::Signature">Signature</a>

Returns the function’s [`Signature`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html "struct datafusion::logical_expr::Signature") for information about what input types are accepted and the function’s Volatility.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#tymethod.return_type" class="fn">return_type</a>( &self, arg_types: &\[<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

What [`DataType`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html "enum datafusion::common::arrow::datatypes::DataType") will be returned by this function, given the types of the arguments

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#tymethod.accumulator" class="fn">accumulator</a>( &self, acc_args: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.AccumulatorArgs.html" class="struct" title="struct datafusion::logical_expr::function::AccumulatorArgs">AccumulatorArgs</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html" class="trait" title="trait datafusion::logical_expr::Accumulator">Accumulator</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Return a new [`Accumulator`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html "trait datafusion::logical_expr::Accumulator") that aggregates values for a specific group during query execution.

acc_args: [`AccumulatorArgs`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.AccumulatorArgs.html "struct datafusion::logical_expr::function::AccumulatorArgs") contains information about how the aggregate function was called.

## Provided Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.aliases" class="fn">aliases</a>(&self) -\> &\[<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\]

Returns any aliases (alternate names) for this function.

Note: `aliases` should only include names other than [`Self::name`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#tymethod.name "method datafusion_expr::udaf::AggregateUDFImpl::name::name"). Defaults to `[]` (no aliases)

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.schema_name" class="fn">schema_name</a>( &self, params: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.AggregateFunctionParams.html" class="struct" title="struct datafusion::logical_expr::expr::AggregateFunctionParams">AggregateFunctionParams</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns the name of the column this expression would create

See [`Expr::schema_name`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.schema_name "method datafusion::prelude::Expr::schema_name") for details

Example of schema_name: count(DISTINCT column1) FILTER (WHERE column2 \> 10) ORDER BY \[..\]

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.human_display" class="fn">human_display</a>( &self, params: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.AggregateFunctionParams.html" class="struct" title="struct datafusion::logical_expr::expr::AggregateFunctionParams">AggregateFunctionParams</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns a human readable expression.

See [`Expr::human_display`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.human_display "method datafusion::prelude::Expr::human_display") for details.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.window_function_schema_name" class="fn">window_function_schema_name</a>( &self, params: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WindowFunctionParams.html" class="struct" title="struct datafusion::logical_expr::expr::WindowFunctionParams">WindowFunctionParams</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns the name of the column this expression would create

See [`Expr::schema_name`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.schema_name "method datafusion::prelude::Expr::schema_name") for details

Different from `schema_name` in that it is used for window aggregate function

Example of schema_name: count(DISTINCT column1) FILTER (WHERE column2 \> 10) \[PARTITION BY \[..\]\] \[ORDER BY \[..\]\]

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.display_name" class="fn">display_name</a>( &self, params: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.AggregateFunctionParams.html" class="struct" title="struct datafusion::logical_expr::expr::AggregateFunctionParams">AggregateFunctionParams</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns the user-defined display name of function, given the arguments

This can be used to customize the output column name generated by this function.

Defaults to `function_name([DISTINCT] column1, column2, ..) [null_treatment] [filter] [order_by [..]]`

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.window_function_display_name" class="fn">window_function_display_name</a>( &self, params: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WindowFunctionParams.html" class="struct" title="struct datafusion::logical_expr::expr::WindowFunctionParams">WindowFunctionParams</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns the user-defined display name of function, given the arguments

This can be used to customize the output column name generated by this function.

Different from `display_name` in that it is used for window aggregate function

Defaults to `function_name([DISTINCT] column1, column2, ..) [null_treatment] [partition by [..]] [order_by [..]]`

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.return_field" class="fn">return_field</a>( &self, arg_fields: &\[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

What type will be returned by this function, given the arguments?

By default, this function calls [`Self::return_type`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#tymethod.return_type "method datafusion_expr::udaf::AggregateUDFImpl::return_type::return_type") with the types of each argument.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#notes" class="doc-anchor">§</a>Notes

Most UDFs should implement [`Self::return_type`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#tymethod.return_type "method datafusion_expr::udaf::AggregateUDFImpl::return_type::return_type") and not this function as the output type for most functions only depends on the types of their inputs (e.g. `sum(f64)` is always `f64`).

This function can be used for more advanced cases such as:

1.  specifying nullability
2.  return types based on the **values** of the arguments (rather than their **types**.
3.  return types based on metadata within the fields of the inputs

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.is_nullable" class="fn">is_nullable</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Whether the aggregate function is nullable.

Nullable means that the function could return `null` for any inputs. For example, aggregate functions like `COUNT` always return a non null value but others like `MIN` will return `NULL` if there is nullable input. Note that if the function is declared as *not* nullable, make sure the [`AggregateUDFImpl::default_value`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.default_value "method datafusion::logical_expr::AggregateUDFImpl::default_value") is `non-null`

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.state_fields" class="fn">state_fields</a>( &self, args: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.StateFieldsArgs.html" class="struct" title="struct datafusion::logical_expr::function::StateFieldsArgs">StateFieldsArgs</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Return the fields used to store the intermediate state of this accumulator.

See [`Accumulator::state`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html#tymethod.state "method datafusion::logical_expr::Accumulator::state") for background information.

args: [`StateFieldsArgs`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.StateFieldsArgs.html "struct datafusion::logical_expr::function::StateFieldsArgs") contains arguments passed to the aggregate function’s accumulator.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#notes-1" class="doc-anchor">§</a>Notes:

The default implementation returns a single state field named `name` with the same type as `value_type`. This is suitable for aggregates such as `SUM` or `MIN` where partial state can be combined by applying the same aggregate.

For aggregates such as `AVG` where the partial state is more complex (e.g. a COUNT and a SUM), this method is used to define the additional fields.

The name of the fields must be unique within the query and thus should be derived from `name`. See [`format_state_name`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/fn.format_state_name.html "fn datafusion::logical_expr::utils::format_state_name") for a utility function to generate a unique name.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.groups_accumulator_supported" class="fn">groups_accumulator_supported</a>(&self, \_args: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.AccumulatorArgs.html" class="struct" title="struct datafusion::logical_expr::function::AccumulatorArgs">AccumulatorArgs</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

If the aggregate expression has a specialized [`GroupsAccumulator`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html "trait datafusion::logical_expr::GroupsAccumulator") implementation. If this returns true, `[Self::create_groups_accumulator]` will be called.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#notes-2" class="doc-anchor">§</a>Notes

Even if this function returns true, DataFusion will still use [`Self::accumulator`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#tymethod.accumulator "method datafusion_expr::udaf::AggregateUDFImpl::accumulator::accumulator") for certain queries, such as when this aggregate is used as a window function or when there no GROUP BY columns in the query.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.create_groups_accumulator" class="fn">create_groups_accumulator</a>( &self, \_args: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.AccumulatorArgs.html" class="struct" title="struct datafusion::logical_expr::function::AccumulatorArgs">AccumulatorArgs</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html" class="trait" title="trait datafusion::logical_expr::GroupsAccumulator">GroupsAccumulator</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Return a specialized [`GroupsAccumulator`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html "trait datafusion::logical_expr::GroupsAccumulator") that manages state for all groups.

For maximum performance, a [`GroupsAccumulator`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html "trait datafusion::logical_expr::GroupsAccumulator") should be implemented in addition to [`Accumulator`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html "trait datafusion::logical_expr::Accumulator").

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.create_sliding_accumulator" class="fn">create_sliding_accumulator</a>( &self, args: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.AccumulatorArgs.html" class="struct" title="struct datafusion::logical_expr::function::AccumulatorArgs">AccumulatorArgs</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html" class="trait" title="trait datafusion::logical_expr::Accumulator">Accumulator</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Sliding accumulator is an alternative accumulator that can be used for window functions. It has retract method to revert the previous update.

See [retract_batch](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html#method.retract_batch "method datafusion::logical_expr::Accumulator::retract_batch") for more details.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.with_beneficial_ordering" class="fn">with_beneficial_ordering</a>( self: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<Self\>, \_beneficial_ordering: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html" class="trait" title="trait datafusion::logical_expr::AggregateUDFImpl">AggregateUDFImpl</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Sets the indicator whether ordering requirements of the AggregateUDFImpl is satisfied by its input. If this is not the case, UDFs with order sensitivity `AggregateOrderSensitivity::Beneficial` can still produce the correct result with possibly more work internally.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#returns" class="doc-anchor">§</a>Returns

Returns `Ok(Some(updated_udf))` if the process completes successfully. If the expression can benefit from existing input ordering, but does not implement the method, returns an error. Order insensitive and hard requirement aggregators return `Ok(None)`.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.order_sensitivity" class="fn">order_sensitivity</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/enum.AggregateOrderSensitivity.html" class="enum" title="enum datafusion::logical_expr::utils::AggregateOrderSensitivity">AggregateOrderSensitivity</a>

Gets the order sensitivity of the UDF. See [`AggregateOrderSensitivity`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/enum.AggregateOrderSensitivity.html "enum datafusion::logical_expr::utils::AggregateOrderSensitivity") for possible options.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.simplify" class="fn">simplify</a>( &self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.AggregateFunction.html" class="struct" title="struct datafusion::logical_expr::expr::AggregateFunction">AggregateFunction</a>, &dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/simplify/trait.SimplifyInfo.html" class="trait" title="trait datafusion::logical_expr::simplify::SimplifyInfo">SimplifyInfo</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\>\>

Optionally apply per-UDaF simplification / rewrite rules.

This can be used to apply function specific simplification rules during optimization (e.g. `arrow_cast` –\> `Expr::Cast`). The default implementation does nothing.

Note that DataFusion handles simplifying arguments and “constant folding” (replacing a function call with constant arguments such as `my_add(1,2) --> 3` ). Thus, there is no need to implement such optimizations manually for specific UDFs.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#returns-1" class="doc-anchor">§</a>Returns

[None](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if simplify is not defined or,

Or, a closure with two arguments:

- ‘aggregate_function’: [crate::expr::AggregateFunction](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.AggregateFunction.html "struct datafusion::logical_expr::expr::AggregateFunction") for which simplified has been invoked
- ‘info’: [crate::simplify::SimplifyInfo](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/simplify/trait.SimplifyInfo.html "trait datafusion::logical_expr::simplify::SimplifyInfo")

closure returns simplified [Expr](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr") or an error.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.reverse_expr" class="fn">reverse_expr</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ReversedUDAF.html" class="enum" title="enum datafusion::logical_expr::ReversedUDAF">ReversedUDAF</a>

Returns the reverse expression of the aggregate function.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.coerce_types" class="fn">coerce_types</a>( &self, \_arg_types: &\[<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Coerce arguments of a function call to types that the function can evaluate.

This function is only called if [`AggregateUDFImpl::signature`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#tymethod.signature "method datafusion::logical_expr::AggregateUDFImpl::signature") returns [`crate::TypeSignature::UserDefined`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#variant.UserDefined "variant datafusion::logical_expr::TypeSignature::UserDefined"). Most UDAFs should return one of the other variants of `TypeSignature` which handle common cases

See the [type coercion module](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/type_coercion/index.html "mod datafusion::logical_expr::type_coercion") documentation for more details on type coercion

For example, if your function requires a floating point arguments, but the user calls it like `my_func(1::int)` (aka with `1` as an integer), coerce_types could return `[DataType::Float64]` to ensure the argument was cast to `1::double`

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#parameters" class="doc-anchor">§</a>Parameters

- `arg_types`: The argument types of the arguments this function with

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#return-value" class="doc-anchor">§</a>Return value

A Vec the same length as `arg_types`. DataFusion will `CAST` the function call arguments to these specific types.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.is_descending" class="fn">is_descending</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>

If this function is max, return true If the function is min, return false Otherwise return None (the default)

Note: this is used to use special aggregate implementations in certain conditions

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.value_from_stats" class="fn">value_from_stats</a>( &self, \_statistics_args: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.StatisticsArgs.html" class="struct" title="struct datafusion::logical_expr::StatisticsArgs">StatisticsArgs</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>\>

Return the value of this aggregate function if it can be determined entirely from statistics and arguments.

Using a [`ScalarValue`](https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html "enum datafusion::scalar::ScalarValue") rather than a runtime computation can significantly improving query performance.

For example, if the minimum value of column `x` is known to be `42` from statistics, then the aggregate `MIN(x)` should return `Some(ScalarValue(42))`

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.default_value" class="fn">default_value</a>( &self, data_type: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns default value of the function given the input is all `null`.

Most of the aggregate function return Null if input is Null, while `count` returns 0 if input is Null

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.supports_null_handling_clause" class="fn">supports_null_handling_clause</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

If this function supports `[IGNORE NULLS | RESPECT NULLS]` clause, return true If the function does not, return false

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.is_ordered_set_aggregate" class="fn">is_ordered_set_aggregate</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

If this function is ordered-set aggregate function, return true If the function is not, return false

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.documentation" class="fn">documentation</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Documentation.html" class="struct" title="struct datafusion::logical_expr::Documentation">Documentation</a>\>

Returns the documentation for this Aggregate UDF.

Documentation can be accessed programmatically as well as generating publicly facing documentation.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.set_monotonicity" class="fn">set_monotonicity</a>(&self, \_data_type: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.SetMonotonicity.html" class="enum" title="enum datafusion::logical_expr::SetMonotonicity">SetMonotonicity</a>

Indicates whether the aggregation function is monotonic as a set function. See [`SetMonotonicity`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.SetMonotonicity.html "enum datafusion::logical_expr::SetMonotonicity") for details.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#impl-PartialEq-for-dyn+AggregateUDFImpl" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html" class="trait" title="trait datafusion::logical_expr::AggregateUDFImpl">AggregateUDFImpl</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &(dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html" class="trait" title="trait datafusion::logical_expr::AggregateUDFImpl">AggregateUDFImpl</a> + 'static)) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#impl-PartialOrd-for-dyn+AggregateUDFImpl" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a> for dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html" class="trait" title="trait datafusion::logical_expr::AggregateUDFImpl">AggregateUDFImpl</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.partial_cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp" class="fn">partial_cmp</a>( &self, other: &(dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html" class="trait" title="trait datafusion::logical_expr::AggregateUDFImpl">AggregateUDFImpl</a> + 'static), ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1398" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.lt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt" class="fn">lt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1416" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.le" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le" class="fn">le</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1434" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.gt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt" class="fn">gt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1452" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.ge" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge" class="fn">ge</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#impl-AggregateUDFImpl-for-ApproxDistinct" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html" class="trait" title="trait datafusion::logical_expr::AggregateUDFImpl">AggregateUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/approx_distinct/struct.ApproxDistinct.html" class="struct" title="struct datafusion::functions_aggregate::approx_distinct::ApproxDistinct">ApproxDistinct</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#impl-AggregateUDFImpl-for-ApproxMedian" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html" class="trait" title="trait datafusion::logical_expr::AggregateUDFImpl">AggregateUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/approx_median/struct.ApproxMedian.html" class="struct" title="struct datafusion::functions_aggregate::approx_median::ApproxMedian">ApproxMedian</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#impl-AggregateUDFImpl-for-ApproxPercentileCont" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html" class="trait" title="trait datafusion::logical_expr::AggregateUDFImpl">AggregateUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/approx_percentile_cont/struct.ApproxPercentileCont.html" class="struct" title="struct datafusion::functions_aggregate::approx_percentile_cont::ApproxPercentileCont">ApproxPercentileCont</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#impl-AggregateUDFImpl-for-ApproxPercentileContWithWeight" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html" class="trait" title="trait datafusion::logical_expr::AggregateUDFImpl">AggregateUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/approx_percentile_cont_with_weight/struct.ApproxPercentileContWithWeight.html" class="struct" title="struct datafusion::functions_aggregate::approx_percentile_cont_with_weight::ApproxPercentileContWithWeight">ApproxPercentileContWithWeight</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#impl-AggregateUDFImpl-for-ArrayAgg" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html" class="trait" title="trait datafusion::logical_expr::AggregateUDFImpl">AggregateUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/array_agg/struct.ArrayAgg.html" class="struct" title="struct datafusion::functions_aggregate::array_agg::ArrayAgg">ArrayAgg</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#impl-AggregateUDFImpl-for-Avg" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html" class="trait" title="trait datafusion::logical_expr::AggregateUDFImpl">AggregateUDFImpl</a> for datafusion::functions_aggregate::average::<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/average/struct.Avg.html" class="struct" title="struct datafusion::functions_aggregate::average::Avg">Avg</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#impl-AggregateUDFImpl-for-BoolAnd" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html" class="trait" title="trait datafusion::logical_expr::AggregateUDFImpl">AggregateUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/bool_and_or/struct.BoolAnd.html" class="struct" title="struct datafusion::functions_aggregate::bool_and_or::BoolAnd">BoolAnd</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#impl-AggregateUDFImpl-for-BoolOr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html" class="trait" title="trait datafusion::logical_expr::AggregateUDFImpl">AggregateUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/bool_and_or/struct.BoolOr.html" class="struct" title="struct datafusion::functions_aggregate::bool_and_or::BoolOr">BoolOr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#impl-AggregateUDFImpl-for-Correlation" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html" class="trait" title="trait datafusion::logical_expr::AggregateUDFImpl">AggregateUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/correlation/struct.Correlation.html" class="struct" title="struct datafusion::functions_aggregate::correlation::Correlation">Correlation</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#impl-AggregateUDFImpl-for-Count" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html" class="trait" title="trait datafusion::logical_expr::AggregateUDFImpl">AggregateUDFImpl</a> for datafusion::functions_aggregate::count::<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/count/struct.Count.html" class="struct" title="struct datafusion::functions_aggregate::count::Count">Count</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#impl-AggregateUDFImpl-for-CovariancePopulation" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html" class="trait" title="trait datafusion::logical_expr::AggregateUDFImpl">AggregateUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/covariance/struct.CovariancePopulation.html" class="struct" title="struct datafusion::functions_aggregate::covariance::CovariancePopulation">CovariancePopulation</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#impl-AggregateUDFImpl-for-CovarianceSample" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html" class="trait" title="trait datafusion::logical_expr::AggregateUDFImpl">AggregateUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/covariance/struct.CovarianceSample.html" class="struct" title="struct datafusion::functions_aggregate::covariance::CovarianceSample">CovarianceSample</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#impl-AggregateUDFImpl-for-FirstValue" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html" class="trait" title="trait datafusion::logical_expr::AggregateUDFImpl">AggregateUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/first_last/struct.FirstValue.html" class="struct" title="struct datafusion::functions_aggregate::first_last::FirstValue">FirstValue</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#impl-AggregateUDFImpl-for-LastValue" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html" class="trait" title="trait datafusion::logical_expr::AggregateUDFImpl">AggregateUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/first_last/struct.LastValue.html" class="struct" title="struct datafusion::functions_aggregate::first_last::LastValue">LastValue</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#impl-AggregateUDFImpl-for-Grouping" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html" class="trait" title="trait datafusion::logical_expr::AggregateUDFImpl">AggregateUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/grouping/struct.Grouping.html" class="struct" title="struct datafusion::functions_aggregate::grouping::Grouping">Grouping</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#impl-AggregateUDFImpl-for-Median" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html" class="trait" title="trait datafusion::logical_expr::AggregateUDFImpl">AggregateUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/median/struct.Median.html" class="struct" title="struct datafusion::functions_aggregate::median::Median">Median</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#impl-AggregateUDFImpl-for-Max" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html" class="trait" title="trait datafusion::logical_expr::AggregateUDFImpl">AggregateUDFImpl</a> for datafusion::functions_aggregate::min_max::<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/struct.Max.html" class="struct" title="struct datafusion::functions_aggregate::min_max::Max">Max</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#impl-AggregateUDFImpl-for-Min" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html" class="trait" title="trait datafusion::logical_expr::AggregateUDFImpl">AggregateUDFImpl</a> for datafusion::functions_aggregate::min_max::<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/struct.Min.html" class="struct" title="struct datafusion::functions_aggregate::min_max::Min">Min</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#impl-AggregateUDFImpl-for-NthValueAgg" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html" class="trait" title="trait datafusion::logical_expr::AggregateUDFImpl">AggregateUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/nth_value/struct.NthValueAgg.html" class="struct" title="struct datafusion::functions_aggregate::nth_value::NthValueAgg">NthValueAgg</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#impl-AggregateUDFImpl-for-Regr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html" class="trait" title="trait datafusion::logical_expr::AggregateUDFImpl">AggregateUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/struct.Regr.html" class="struct" title="struct datafusion::functions_aggregate::regr::Regr">Regr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#impl-AggregateUDFImpl-for-Stddev" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html" class="trait" title="trait datafusion::logical_expr::AggregateUDFImpl">AggregateUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/stddev/struct.Stddev.html" class="struct" title="struct datafusion::functions_aggregate::stddev::Stddev">Stddev</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#impl-AggregateUDFImpl-for-StddevPop" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html" class="trait" title="trait datafusion::logical_expr::AggregateUDFImpl">AggregateUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/stddev/struct.StddevPop.html" class="struct" title="struct datafusion::functions_aggregate::stddev::StddevPop">StddevPop</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#impl-AggregateUDFImpl-for-StringAgg" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html" class="trait" title="trait datafusion::logical_expr::AggregateUDFImpl">AggregateUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/string_agg/struct.StringAgg.html" class="struct" title="struct datafusion::functions_aggregate::string_agg::StringAgg">StringAgg</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#impl-AggregateUDFImpl-for-Sum" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html" class="trait" title="trait datafusion::logical_expr::AggregateUDFImpl">AggregateUDFImpl</a> for datafusion::functions_aggregate::sum::<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/sum/struct.Sum.html" class="struct" title="struct datafusion::functions_aggregate::sum::Sum">Sum</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#impl-AggregateUDFImpl-for-VariancePopulation" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html" class="trait" title="trait datafusion::logical_expr::AggregateUDFImpl">AggregateUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VariancePopulation.html" class="struct" title="struct datafusion::functions_aggregate::variance::VariancePopulation">VariancePopulation</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#impl-AggregateUDFImpl-for-VarianceSample" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html" class="trait" title="trait datafusion::logical_expr::AggregateUDFImpl">AggregateUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VarianceSample.html" class="struct" title="struct datafusion::functions_aggregate::variance::VarianceSample">VarianceSample</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#impl-AggregateUDFImpl-for-SimpleAggregateUDF" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html" class="trait" title="trait datafusion::logical_expr::AggregateUDFImpl">AggregateUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SimpleAggregateUDF.html" class="struct" title="struct datafusion::prelude::SimpleAggregateUDF">SimpleAggregateUDF</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#impl-AggregateUDFImpl-for-Avg-1" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html" class="trait" title="trait datafusion::logical_expr::AggregateUDFImpl">AggregateUDFImpl</a> for datafusion::logical_expr::test::function_stub::<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/test/function_stub/struct.Avg.html" class="struct" title="struct datafusion::logical_expr::test::function_stub::Avg">Avg</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#impl-AggregateUDFImpl-for-Count-1" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html" class="trait" title="trait datafusion::logical_expr::AggregateUDFImpl">AggregateUDFImpl</a> for datafusion::logical_expr::test::function_stub::<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/test/function_stub/struct.Count.html" class="struct" title="struct datafusion::logical_expr::test::function_stub::Count">Count</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#impl-AggregateUDFImpl-for-Max-1" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html" class="trait" title="trait datafusion::logical_expr::AggregateUDFImpl">AggregateUDFImpl</a> for datafusion::logical_expr::test::function_stub::<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/test/function_stub/struct.Max.html" class="struct" title="struct datafusion::logical_expr::test::function_stub::Max">Max</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#impl-AggregateUDFImpl-for-Min-1" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html" class="trait" title="trait datafusion::logical_expr::AggregateUDFImpl">AggregateUDFImpl</a> for datafusion::logical_expr::test::function_stub::<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/test/function_stub/struct.Min.html" class="struct" title="struct datafusion::logical_expr::test::function_stub::Min">Min</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#impl-AggregateUDFImpl-for-Sum-1" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html" class="trait" title="trait datafusion::logical_expr::AggregateUDFImpl">AggregateUDFImpl</a> for datafusion::logical_expr::test::function_stub::<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/test/function_stub/struct.Sum.html" class="struct" title="struct datafusion::logical_expr::test::function_stub::Sum">Sum</a>
