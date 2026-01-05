# Trait ScalarUDFImpl Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/udf.rs.html#442" class="src">Source</a>

``` rust
pub trait ScalarUDFImpl:
    Debug
    + DynEq
    + DynHash
    + Send
    + Sync {
Show 18 methods    // Required methods
    fn as_any(&self) -> &(dyn Any + 'static);
    fn name(&self) -> &str;
    fn signature(&self) -> &Signature;
    fn return_type(
        &self,
        arg_types: &[DataType],
    ) -> Result<DataType, DataFusionError>;
    fn invoke_with_args(
        &self,
        args: ScalarFunctionArgs,
    ) -> Result<ColumnarValue, DataFusionError>;

    // Provided methods
    fn aliases(&self) -> &[String] { ... }
    fn display_name(&self, args: &[Expr]) -> Result<String, DataFusionError> { ... }
    fn schema_name(&self, args: &[Expr]) -> Result<String, DataFusionError> { ... }
    fn return_field_from_args(
        &self,
        args: ReturnFieldArgs<'_>,
    ) -> Result<Arc<Field>, DataFusionError> { ... }
    fn is_nullable(&self, _args: &[Expr], _schema: &dyn ExprSchema) -> bool { ... }
    fn simplify(
        &self,
        args: Vec<Expr>,
        _info: &dyn SimplifyInfo,
    ) -> Result<ExprSimplifyResult, DataFusionError> { ... }
    fn short_circuits(&self) -> bool { ... }
    fn evaluate_bounds(
        &self,
        _input: &[&Interval],
    ) -> Result<Interval, DataFusionError> { ... }
    fn propagate_constraints(
        &self,
        _interval: &Interval,
        _inputs: &[&Interval],
    ) -> Result<Option<Vec<Interval>>, DataFusionError> { ... }
    fn output_ordering(
        &self,
        inputs: &[ExprProperties],
    ) -> Result<SortProperties, DataFusionError> { ... }
    fn preserves_lex_ordering(
        &self,
        _inputs: &[ExprProperties],
    ) -> Result<bool, DataFusionError> { ... }
    fn coerce_types(
        &self,
        _arg_types: &[DataType],
    ) -> Result<Vec<DataType>, DataFusionError> { ... }
    fn documentation(&self) -> Option<&Documentation> { ... }
}
```

Expand description

Trait for implementing user defined scalar functions.

This trait exposes the full API for implementing user defined functions and can be used to implement any function.

See [`advanced_udf.rs`](https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/advanced_udf.rs) for a full example with complete implementation and [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") for other available options.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#basic-example" class="doc-anchor">§</a>Basic Example

``` rust
/// This struct for a simple UDF that adds one to an int32
#[derive(Debug, PartialEq, Eq, Hash)]
struct AddOne {
  signature: Signature,
}

impl AddOne {
  fn new() -> Self {
    Self {
      signature: Signature::uniform(1, vec![DataType::Int32], Volatility::Immutable),
     }
  }
}

static DOCUMENTATION: LazyLock<Documentation> = LazyLock::new(|| {
        Documentation::builder(DOC_SECTION_MATH, "Add one to an int32", "add_one(2)")
            .with_argument("arg1", "The int32 number to add one to")
            .build()
    });

fn get_doc() -> &'static Documentation {
    &DOCUMENTATION
}

/// Implement the ScalarUDFImpl trait for AddOne
impl ScalarUDFImpl for AddOne {
   fn as_any(&self) -> &dyn Any { self }
   fn name(&self) -> &str { "add_one" }
   fn signature(&self) -> &Signature { &self.signature }
   fn return_type(&self, args: &[DataType]) -> Result<DataType> {
     if !matches!(args.get(0), Some(&DataType::Int32)) {
       return plan_err!("add_one only accepts Int32 arguments");
     }
     Ok(DataType::Int32)
   }
   // The actual implementation would add one to the argument
   fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue> {
        unimplemented!()
   }
   fn documentation(&self) -> Option<&Documentation> {
        Some(get_doc())
    }
}

// Create a new ScalarUDF from the implementation
let add_one = ScalarUDF::from(AddOne::new());

// Call the function `add_one(col)`
let expr = add_one.call(vec![col("a")]);
```

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns this object as an [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") trait object

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#tymethod.name" class="fn">name</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Returns this function’s name

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#tymethod.signature" class="fn">signature</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html" class="struct" title="struct datafusion::logical_expr::Signature">Signature</a>

Returns a [`Signature`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html "struct datafusion::logical_expr::Signature") describing the argument types for which this function has an implementation, and the function’s [`Volatility`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Volatility.html "enum datafusion::logical_expr::Volatility").

See [`Signature`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html "struct datafusion::logical_expr::Signature") for more details on argument type handling and [`Self::return_type`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#tymethod.return_type "method datafusion_expr::udf::ScalarUDFImpl::return_type::return_type") for computing the return type.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#tymethod.return_type" class="fn">return_type</a>( &self, arg_types: &\[<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

[`DataType`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html "enum datafusion::common::arrow::datatypes::DataType") returned by this function, given the types of the arguments.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#arguments" class="doc-anchor">§</a>Arguments

`arg_types` Data types of the arguments. The implementation of `return_type` can assume that some other part of the code has coerced the actual argument types to match [`Self::signature`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#tymethod.signature "method datafusion_expr::udf::ScalarUDFImpl::signature::signature").

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#notes" class="doc-anchor">§</a>Notes

If you provide an implementation for [`Self::return_field_from_args`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#method.return_field_from_args "method datafusion_expr::udf::ScalarUDFImpl::return_field_from_args::return_field_from_args"), DataFusion will not call `return_type` (this function). While it is valid to to put [`unimplemented!()`](https://doc.rust-lang.org/nightly/core/macro.unimplemented.html "macro core::unimplemented") or [`unreachable!()`](https://doc.rust-lang.org/nightly/core/macro.unreachable.html "macro core::unreachable"), it is recommended to return [`DataFusionError::Internal`](https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html#variant.Internal "variant datafusion::error::DataFusionError::Internal") instead, which reduces the severity of symptoms if bugs occur (an error rather than a panic).

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#tymethod.invoke_with_args" class="fn">invoke_with_args</a>( &self, args: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarFunctionArgs.html" class="struct" title="struct datafusion::logical_expr::ScalarFunctionArgs">ScalarFunctionArgs</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html" class="enum" title="enum datafusion::logical_expr::ColumnarValue">ColumnarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Invoke the function returning the appropriate result.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#performance" class="doc-anchor">§</a>Performance

For the best performance, the implementations should handle the common case when one or more of their arguments are constant values (aka [`ColumnarValue::Scalar`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html#variant.Scalar "variant datafusion::logical_expr::ColumnarValue::Scalar")).

[`ColumnarValue::values_to_arrays`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html#method.values_to_arrays "associated function datafusion::logical_expr::ColumnarValue::values_to_arrays") can be used to convert the arguments to arrays, which will likely be simpler code, but be slower.

## Provided Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#method.aliases" class="fn">aliases</a>(&self) -\> &\[<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\]

Returns any aliases (alternate names) for this function.

Aliases can be used to invoke the same function using different names. For example in some databases `now()` and `current_timestamp()` are aliases for the same function. This behavior can be obtained by returning `current_timestamp` as an alias for the `now` function.

Note: `aliases` should only include names other than [`Self::name`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#tymethod.name "method datafusion_expr::udf::ScalarUDFImpl::name::name"). Defaults to `[]` (no aliases)

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#method.display_name" class="fn">display_name</a>(&self, args: &\[<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

👎Deprecated since 50.0.0: This method is unused and will be removed in a future release

Returns the user-defined display name of function, given the arguments

This can be used to customize the output column name generated by this function.

Defaults to `name(args[0], args[1], ...)`

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#method.schema_name" class="fn">schema_name</a>(&self, args: &\[<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns the name of the column this expression would create

See [`Expr::schema_name`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.schema_name "method datafusion::prelude::Expr::schema_name") for details

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#method.return_field_from_args" class="fn">return_field_from_args</a>( &self, args: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ReturnFieldArgs.html" class="struct" title="struct datafusion::logical_expr::ReturnFieldArgs">ReturnFieldArgs</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

What type will be returned by this function, given the arguments?

By default, this function calls [`Self::return_type`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#tymethod.return_type "method datafusion_expr::udf::ScalarUDFImpl::return_type::return_type") with the types of each argument.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#notes-1" class="doc-anchor">§</a>Notes

For the majority of UDFs, implementing [`Self::return_type`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#tymethod.return_type "method datafusion_expr::udf::ScalarUDFImpl::return_type::return_type") is sufficient, as the result type is typically a deterministic function of the input types (e.g., `sqrt(f32)` consistently yields `f32`). Implementing this method directly is generally unnecessary unless the return type depends on runtime values.

This function can be used for more advanced cases such as:

1.  specifying nullability
2.  return types based on the **values** of the arguments (rather than their **types**.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#example-creating-field" class="doc-anchor">§</a>Example creating `Field`

Note the name of the [`Field`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html "struct datafusion::common::arrow::datatypes::Field") is ignored, except for structured types such as `DataType::Struct`.

``` rust
fn return_field_from_args(&self, args: ReturnFieldArgs) -> Result<FieldRef> {
  // report output is only nullable if any one of the arguments are nullable
  let nullable = args.arg_fields.iter().any(|f| f.is_nullable());
  let field = Arc::new(Field::new("ignored_name", DataType::Int32, true));
  Ok(field)
}
```

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#output-type-based-on-values" class="doc-anchor">§</a>Output Type based on Values

For example, the following two function calls get the same argument types (something and a `Utf8` string) but return different types based on the value of the second argument:

- `arrow_cast(x, 'Int16')` –\> `Int16`
- `arrow_cast(x, 'Float32')` –\> `Float32`

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#requirements" class="doc-anchor">§</a>Requirements

This function **must** consistently return the same type for the same logical input even if the input is simplified (e.g. it must return the same value for `('foo' | 'bar')` as it does for (‘foobar’).

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#method.is_nullable" class="fn">is_nullable</a>(&self, \_args: &\[<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\], \_schema: &dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ExprSchema.html" class="trait" title="trait datafusion::common::ExprSchema">ExprSchema</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

👎Deprecated since 45.0.0: Use `return_field_from_args` instead. if you use `is_nullable` that returns non-nullable with `return_type`, you would need to switch to `return_field_from_args`, you might have error

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#method.simplify" class="fn">simplify</a>( &self, args: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>, \_info: &dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/simplify/trait.SimplifyInfo.html" class="trait" title="trait datafusion::logical_expr::simplify::SimplifyInfo">SimplifyInfo</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/simplify/enum.ExprSimplifyResult.html" class="enum" title="enum datafusion::logical_expr::simplify::ExprSimplifyResult">ExprSimplifyResult</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Optionally apply per-UDF simplification / rewrite rules.

This can be used to apply function specific simplification rules during optimization (e.g. `arrow_cast` –\> `Expr::Cast`). The default implementation does nothing.

Note that DataFusion handles simplifying arguments and “constant folding” (replacing a function call with constant arguments such as `my_add(1,2) --> 3` ). Thus, there is no need to implement such optimizations manually for specific UDFs.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#arguments-1" class="doc-anchor">§</a>Arguments

- `args`: The arguments of the function
- `info`: The necessary information for simplification

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#returns" class="doc-anchor">§</a>Returns

[`ExprSimplifyResult`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/simplify/enum.ExprSimplifyResult.html "enum datafusion::logical_expr::simplify::ExprSimplifyResult") indicating the result of the simplification NOTE if the function cannot be simplified, the arguments *MUST* be returned unmodified

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#method.short_circuits" class="fn">short_circuits</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if some of this `exprs` subexpressions may not be evaluated and thus any side effects (like divide by zero) may not be encountered.

Setting this to true prevents certain optimizations such as common subexpression elimination

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#method.evaluate_bounds" class="fn">evaluate_bounds</a>( &self, \_input: &\[&<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/struct.Interval.html" class="struct" title="struct datafusion::logical_expr::interval_arithmetic::Interval">Interval</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/struct.Interval.html" class="struct" title="struct datafusion::logical_expr::interval_arithmetic::Interval">Interval</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Computes the output [`Interval`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/struct.Interval.html "struct datafusion::logical_expr::interval_arithmetic::Interval") for a [`ScalarUDFImpl`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html "trait datafusion::logical_expr::ScalarUDFImpl"), given the input intervals.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#parameters" class="doc-anchor">§</a>Parameters

- `children` are the intervals for the children (inputs) of this function.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#example" class="doc-anchor">§</a>Example

If the function is `ABS(a)`, and the input interval is `a: [-3, 2]`, then the output interval would be `[0, 3]`.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#method.propagate_constraints" class="fn">propagate_constraints</a>( &self, \_interval: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/struct.Interval.html" class="struct" title="struct datafusion::logical_expr::interval_arithmetic::Interval">Interval</a>, \_inputs: &\[&<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/struct.Interval.html" class="struct" title="struct datafusion::logical_expr::interval_arithmetic::Interval">Interval</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/struct.Interval.html" class="struct" title="struct datafusion::logical_expr::interval_arithmetic::Interval">Interval</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Updates bounds for child expressions, given a known [`Interval`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/struct.Interval.html "struct datafusion::logical_expr::interval_arithmetic::Interval")s for this function.

This function is used to propagate constraints down through an expression tree.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#parameters-1" class="doc-anchor">§</a>Parameters

- `interval` is the currently known interval for this function.
- `inputs` are the current intervals for the inputs (children) of this function.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#returns-1" class="doc-anchor">§</a>Returns

A `Vec` of new intervals for the children, in order.

If constraint propagation reveals an infeasibility for any child, returns [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None"). If none of the children intervals change as a result of propagation, may return an empty vector instead of cloning `children`. This is the default (and conservative) return value.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#example-1" class="doc-anchor">§</a>Example

If the function is `ABS(a)`, the current `interval` is `[4, 5]` and the input `a` is given as `[-7, 3]`, then propagation would return `[-5, 3]`.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#method.output_ordering" class="fn">output_ordering</a>( &self, inputs: &\[<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/struct.ExprProperties.html" class="struct" title="struct datafusion::logical_expr::sort_properties::ExprProperties">ExprProperties</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/enum.SortProperties.html" class="enum" title="enum datafusion::logical_expr::sort_properties::SortProperties">SortProperties</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Calculates the [`SortProperties`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/enum.SortProperties.html "enum datafusion::logical_expr::sort_properties::SortProperties") of this function based on its children’s properties.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#method.preserves_lex_ordering" class="fn">preserves_lex_ordering</a>( &self, \_inputs: &\[<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/struct.ExprProperties.html" class="struct" title="struct datafusion::logical_expr::sort_properties::ExprProperties">ExprProperties</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns true if the function preserves lexicographical ordering based on the input ordering.

For example, `concat(a || b)` preserves lexicographical ordering, but `abs(a)` does not.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#method.coerce_types" class="fn">coerce_types</a>( &self, \_arg_types: &\[<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Coerce arguments of a function call to types that the function can evaluate.

This function is only called if [`ScalarUDFImpl::signature`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#tymethod.signature "method datafusion::logical_expr::ScalarUDFImpl::signature") returns [`crate::TypeSignature::UserDefined`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#variant.UserDefined "variant datafusion::logical_expr::TypeSignature::UserDefined"). Most UDFs should return one of the other variants of [`TypeSignature`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html "enum datafusion::logical_expr::TypeSignature") which handle common cases.

See the [type coercion module](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/type_coercion/index.html "mod datafusion::logical_expr::type_coercion") documentation for more details on type coercion

For example, if your function requires a floating point arguments, but the user calls it like `my_func(1::int)` (i.e. with `1` as an integer), coerce_types can return `[DataType::Float64]` to ensure the argument is converted to `1::double`

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#parameters-2" class="doc-anchor">§</a>Parameters

- `arg_types`: The argument types of the arguments this function with

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#return-value" class="doc-anchor">§</a>Return value

A Vec the same length as `arg_types`. DataFusion will `CAST` the function call arguments to these specific types.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#method.documentation" class="fn">documentation</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Documentation.html" class="struct" title="struct datafusion::logical_expr::Documentation">Documentation</a>\>

Returns the documentation for this Scalar UDF.

Documentation can be accessed programmatically as well as generating publicly facing documentation.

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-ArrowCastFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/arrow_cast/struct.ArrowCastFunc.html" class="struct" title="struct datafusion::functions::core::arrow_cast::ArrowCastFunc">ArrowCastFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-ArrowTypeOfFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/arrowtypeof/struct.ArrowTypeOfFunc.html" class="struct" title="struct datafusion::functions::core::arrowtypeof::ArrowTypeOfFunc">ArrowTypeOfFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-CoalesceFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/coalesce/struct.CoalesceFunc.html" class="struct" title="struct datafusion::functions::core::coalesce::CoalesceFunc">CoalesceFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-GetFieldFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/getfield/struct.GetFieldFunc.html" class="struct" title="struct datafusion::functions::core::getfield::GetFieldFunc">GetFieldFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-GreatestFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/greatest/struct.GreatestFunc.html" class="struct" title="struct datafusion::functions::core::greatest::GreatestFunc">GreatestFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-LeastFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/least/struct.LeastFunc.html" class="struct" title="struct datafusion::functions::core::least::LeastFunc">LeastFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-NamedStructFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/named_struct/struct.NamedStructFunc.html" class="struct" title="struct datafusion::functions::core::named_struct::NamedStructFunc">NamedStructFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-NullIfFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/nullif/struct.NullIfFunc.html" class="struct" title="struct datafusion::functions::core::nullif::NullIfFunc">NullIfFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-NVL2Func" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/nvl2/struct.NVL2Func.html" class="struct" title="struct datafusion::functions::core::nvl2::NVL2Func">NVL2Func</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-NVLFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/nvl/struct.NVLFunc.html" class="struct" title="struct datafusion::functions::core::nvl::NVLFunc">NVLFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-StructFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/struct/struct.StructFunc.html" class="struct" title="struct datafusion::functions::core::struct::StructFunc">StructFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-UnionExtractFun" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/union_extract/struct.UnionExtractFun.html" class="struct" title="struct datafusion::functions::core::union_extract::UnionExtractFun">UnionExtractFun</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-UnionTagFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/union_tag/struct.UnionTagFunc.html" class="struct" title="struct datafusion::functions::core::union_tag::UnionTagFunc">UnionTagFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-VersionFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/version/struct.VersionFunc.html" class="struct" title="struct datafusion::functions::core::version::VersionFunc">VersionFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-DigestFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/crypto/digest/struct.DigestFunc.html" class="struct" title="struct datafusion::functions::crypto::digest::DigestFunc">DigestFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-Md5Func" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/crypto/md5/struct.Md5Func.html" class="struct" title="struct datafusion::functions::crypto::md5::Md5Func">Md5Func</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-SHA224Func" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/crypto/sha224/struct.SHA224Func.html" class="struct" title="struct datafusion::functions::crypto::sha224::SHA224Func">SHA224Func</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-SHA256Func" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/crypto/sha256/struct.SHA256Func.html" class="struct" title="struct datafusion::functions::crypto::sha256::SHA256Func">SHA256Func</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-SHA384Func" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/crypto/sha384/struct.SHA384Func.html" class="struct" title="struct datafusion::functions::crypto::sha384::SHA384Func">SHA384Func</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-SHA512Func" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/crypto/sha512/struct.SHA512Func.html" class="struct" title="struct datafusion::functions::crypto::sha512::SHA512Func">SHA512Func</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-CurrentDateFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/current_date/struct.CurrentDateFunc.html" class="struct" title="struct datafusion::functions::datetime::current_date::CurrentDateFunc">CurrentDateFunc</a>

Create an implementation of `current_date()` that always returns the specified current date.

The semantics of `current_date()` require it to return the same value wherever it appears within a single statement. This value is chosen during planning time.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-CurrentTimeFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/current_time/struct.CurrentTimeFunc.html" class="struct" title="struct datafusion::functions::datetime::current_time::CurrentTimeFunc">CurrentTimeFunc</a>

Create an implementation of `current_time()` that always returns the specified current time.

The semantics of `current_time()` require it to return the same value wherever it appears within a single statement. This value is chosen during planning time.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-DateBinFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/date_bin/struct.DateBinFunc.html" class="struct" title="struct datafusion::functions::datetime::date_bin::DateBinFunc">DateBinFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-DatePartFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/date_part/struct.DatePartFunc.html" class="struct" title="struct datafusion::functions::datetime::date_part::DatePartFunc">DatePartFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-DateTruncFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/date_trunc/struct.DateTruncFunc.html" class="struct" title="struct datafusion::functions::datetime::date_trunc::DateTruncFunc">DateTruncFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-FromUnixtimeFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/from_unixtime/struct.FromUnixtimeFunc.html" class="struct" title="struct datafusion::functions::datetime::from_unixtime::FromUnixtimeFunc">FromUnixtimeFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-MakeDateFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/make_date/struct.MakeDateFunc.html" class="struct" title="struct datafusion::functions::datetime::make_date::MakeDateFunc">MakeDateFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-NowFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/now/struct.NowFunc.html" class="struct" title="struct datafusion::functions::datetime::now::NowFunc">NowFunc</a>

Create an implementation of `now()` that always returns the specified timestamp.

The semantics of `now()` require it to return the same value wherever it appears within a single statement. This value is chosen during planning time.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-ToCharFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/to_char/struct.ToCharFunc.html" class="struct" title="struct datafusion::functions::datetime::to_char::ToCharFunc">ToCharFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-ToDateFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/to_date/struct.ToDateFunc.html" class="struct" title="struct datafusion::functions::datetime::to_date::ToDateFunc">ToDateFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-ToLocalTimeFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/to_local_time/struct.ToLocalTimeFunc.html" class="struct" title="struct datafusion::functions::datetime::to_local_time::ToLocalTimeFunc">ToLocalTimeFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-ToTimestampFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/to_timestamp/struct.ToTimestampFunc.html" class="struct" title="struct datafusion::functions::datetime::to_timestamp::ToTimestampFunc">ToTimestampFunc</a>

to_timestamp SQL function

Note: `to_timestamp` returns `Timestamp(Nanosecond)` though its arguments are interpreted as **seconds**. The supported range for integer input is between `-9223372037` and `9223372036`. Supported range for string input is between `1677-09-21T00:12:44.0` and `2262-04-11T23:47:16.0`. Please use `to_timestamp_seconds` for the input outside of supported bounds.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-ToTimestampMicrosFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/to_timestamp/struct.ToTimestampMicrosFunc.html" class="struct" title="struct datafusion::functions::datetime::to_timestamp::ToTimestampMicrosFunc">ToTimestampMicrosFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-ToTimestampMillisFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/to_timestamp/struct.ToTimestampMillisFunc.html" class="struct" title="struct datafusion::functions::datetime::to_timestamp::ToTimestampMillisFunc">ToTimestampMillisFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-ToTimestampNanosFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/to_timestamp/struct.ToTimestampNanosFunc.html" class="struct" title="struct datafusion::functions::datetime::to_timestamp::ToTimestampNanosFunc">ToTimestampNanosFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-ToTimestampSecondsFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/to_timestamp/struct.ToTimestampSecondsFunc.html" class="struct" title="struct datafusion::functions::datetime::to_timestamp::ToTimestampSecondsFunc">ToTimestampSecondsFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-ToUnixtimeFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/to_unixtime/struct.ToUnixtimeFunc.html" class="struct" title="struct datafusion::functions::datetime::to_unixtime::ToUnixtimeFunc">ToUnixtimeFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-DecodeFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/encoding/inner/struct.DecodeFunc.html" class="struct" title="struct datafusion::functions::encoding::inner::DecodeFunc">DecodeFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-EncodeFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/encoding/inner/struct.EncodeFunc.html" class="struct" title="struct datafusion::functions::encoding::inner::EncodeFunc">EncodeFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-AbsFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/abs/struct.AbsFunc.html" class="struct" title="struct datafusion::functions::math::abs::AbsFunc">AbsFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-CotFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/cot/struct.CotFunc.html" class="struct" title="struct datafusion::functions::math::cot::CotFunc">CotFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-FactorialFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/factorial/struct.FactorialFunc.html" class="struct" title="struct datafusion::functions::math::factorial::FactorialFunc">FactorialFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-GcdFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/gcd/struct.GcdFunc.html" class="struct" title="struct datafusion::functions::math::gcd::GcdFunc">GcdFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-IsZeroFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/iszero/struct.IsZeroFunc.html" class="struct" title="struct datafusion::functions::math::iszero::IsZeroFunc">IsZeroFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-LcmFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/lcm/struct.LcmFunc.html" class="struct" title="struct datafusion::functions::math::lcm::LcmFunc">LcmFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-LogFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/log/struct.LogFunc.html" class="struct" title="struct datafusion::functions::math::log::LogFunc">LogFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-IsNanFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/nans/struct.IsNanFunc.html" class="struct" title="struct datafusion::functions::math::nans::IsNanFunc">IsNanFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-NanvlFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/nanvl/struct.NanvlFunc.html" class="struct" title="struct datafusion::functions::math::nanvl::NanvlFunc">NanvlFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-PiFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/pi/struct.PiFunc.html" class="struct" title="struct datafusion::functions::math::pi::PiFunc">PiFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-PowerFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/power/struct.PowerFunc.html" class="struct" title="struct datafusion::functions::math::power::PowerFunc">PowerFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-RandomFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/random/struct.RandomFunc.html" class="struct" title="struct datafusion::functions::math::random::RandomFunc">RandomFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-RoundFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/round/struct.RoundFunc.html" class="struct" title="struct datafusion::functions::math::round::RoundFunc">RoundFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-SignumFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/signum/struct.SignumFunc.html" class="struct" title="struct datafusion::functions::math::signum::SignumFunc">SignumFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-TruncFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/trunc/struct.TruncFunc.html" class="struct" title="struct datafusion::functions::math::trunc::TruncFunc">TruncFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-RegexpCountFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/regex/regexpcount/struct.RegexpCountFunc.html" class="struct" title="struct datafusion::functions::regex::regexpcount::RegexpCountFunc">RegexpCountFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-RegexpInstrFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/regex/regexpinstr/struct.RegexpInstrFunc.html" class="struct" title="struct datafusion::functions::regex::regexpinstr::RegexpInstrFunc">RegexpInstrFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-RegexpLikeFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/regex/regexplike/struct.RegexpLikeFunc.html" class="struct" title="struct datafusion::functions::regex::regexplike::RegexpLikeFunc">RegexpLikeFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-RegexpMatchFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/regex/regexpmatch/struct.RegexpMatchFunc.html" class="struct" title="struct datafusion::functions::regex::regexpmatch::RegexpMatchFunc">RegexpMatchFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-RegexpReplaceFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/regex/regexpreplace/struct.RegexpReplaceFunc.html" class="struct" title="struct datafusion::functions::regex::regexpreplace::RegexpReplaceFunc">RegexpReplaceFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-AsciiFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/string/ascii/struct.AsciiFunc.html" class="struct" title="struct datafusion::functions::string::ascii::AsciiFunc">AsciiFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-BitLengthFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/string/bit_length/struct.BitLengthFunc.html" class="struct" title="struct datafusion::functions::string::bit_length::BitLengthFunc">BitLengthFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-BTrimFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/string/btrim/struct.BTrimFunc.html" class="struct" title="struct datafusion::functions::string::btrim::BTrimFunc">BTrimFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-ChrFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/string/chr/struct.ChrFunc.html" class="struct" title="struct datafusion::functions::string::chr::ChrFunc">ChrFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-ConcatFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/string/concat/struct.ConcatFunc.html" class="struct" title="struct datafusion::functions::string::concat::ConcatFunc">ConcatFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-ConcatWsFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/string/concat_ws/struct.ConcatWsFunc.html" class="struct" title="struct datafusion::functions::string::concat_ws::ConcatWsFunc">ConcatWsFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-ContainsFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/string/contains/struct.ContainsFunc.html" class="struct" title="struct datafusion::functions::string::contains::ContainsFunc">ContainsFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-EndsWithFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/string/ends_with/struct.EndsWithFunc.html" class="struct" title="struct datafusion::functions::string::ends_with::EndsWithFunc">EndsWithFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-LevenshteinFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/string/levenshtein/struct.LevenshteinFunc.html" class="struct" title="struct datafusion::functions::string::levenshtein::LevenshteinFunc">LevenshteinFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-LowerFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/string/lower/struct.LowerFunc.html" class="struct" title="struct datafusion::functions::string::lower::LowerFunc">LowerFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-LtrimFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/string/ltrim/struct.LtrimFunc.html" class="struct" title="struct datafusion::functions::string::ltrim::LtrimFunc">LtrimFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-OctetLengthFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/string/octet_length/struct.OctetLengthFunc.html" class="struct" title="struct datafusion::functions::string::octet_length::OctetLengthFunc">OctetLengthFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-OverlayFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/string/overlay/struct.OverlayFunc.html" class="struct" title="struct datafusion::functions::string::overlay::OverlayFunc">OverlayFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-RepeatFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/string/repeat/struct.RepeatFunc.html" class="struct" title="struct datafusion::functions::string::repeat::RepeatFunc">RepeatFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-ReplaceFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/string/replace/struct.ReplaceFunc.html" class="struct" title="struct datafusion::functions::string::replace::ReplaceFunc">ReplaceFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-RtrimFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/string/rtrim/struct.RtrimFunc.html" class="struct" title="struct datafusion::functions::string::rtrim::RtrimFunc">RtrimFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-SplitPartFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/string/split_part/struct.SplitPartFunc.html" class="struct" title="struct datafusion::functions::string::split_part::SplitPartFunc">SplitPartFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-StartsWithFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/string/starts_with/struct.StartsWithFunc.html" class="struct" title="struct datafusion::functions::string::starts_with::StartsWithFunc">StartsWithFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-ToHexFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/string/to_hex/struct.ToHexFunc.html" class="struct" title="struct datafusion::functions::string::to_hex::ToHexFunc">ToHexFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-UpperFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/string/upper/struct.UpperFunc.html" class="struct" title="struct datafusion::functions::string::upper::UpperFunc">UpperFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-UuidFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/string/uuid/struct.UuidFunc.html" class="struct" title="struct datafusion::functions::string::uuid::UuidFunc">UuidFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-CharacterLengthFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/unicode/character_length/struct.CharacterLengthFunc.html" class="struct" title="struct datafusion::functions::unicode::character_length::CharacterLengthFunc">CharacterLengthFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-FindInSetFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/unicode/find_in_set/struct.FindInSetFunc.html" class="struct" title="struct datafusion::functions::unicode::find_in_set::FindInSetFunc">FindInSetFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-InitcapFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/unicode/initcap/struct.InitcapFunc.html" class="struct" title="struct datafusion::functions::unicode::initcap::InitcapFunc">InitcapFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-LeftFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/unicode/left/struct.LeftFunc.html" class="struct" title="struct datafusion::functions::unicode::left::LeftFunc">LeftFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-LPadFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/unicode/lpad/struct.LPadFunc.html" class="struct" title="struct datafusion::functions::unicode::lpad::LPadFunc">LPadFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-ReverseFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/unicode/reverse/struct.ReverseFunc.html" class="struct" title="struct datafusion::functions::unicode::reverse::ReverseFunc">ReverseFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-RightFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/unicode/right/struct.RightFunc.html" class="struct" title="struct datafusion::functions::unicode::right::RightFunc">RightFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-RPadFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/unicode/rpad/struct.RPadFunc.html" class="struct" title="struct datafusion::functions::unicode::rpad::RPadFunc">RPadFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-StrposFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/unicode/strpos/struct.StrposFunc.html" class="struct" title="struct datafusion::functions::unicode::strpos::StrposFunc">StrposFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-SubstrFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/unicode/substr/struct.SubstrFunc.html" class="struct" title="struct datafusion::functions::unicode::substr::SubstrFunc">SubstrFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-SubstrIndexFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/unicode/substrindex/struct.SubstrIndexFunc.html" class="struct" title="struct datafusion::functions::unicode::substrindex::SubstrIndexFunc">SubstrIndexFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-TranslateFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/unicode/translate/struct.TranslateFunc.html" class="struct" title="struct datafusion::functions::unicode::translate::TranslateFunc">TranslateFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-ArrayHas" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_nested/array_has/struct.ArrayHas.html" class="struct" title="struct datafusion::functions_nested::array_has::ArrayHas">ArrayHas</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-ArrayHasAll" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_nested/array_has/struct.ArrayHasAll.html" class="struct" title="struct datafusion::functions_nested::array_has::ArrayHasAll">ArrayHasAll</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-ArrayHasAny" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_nested/array_has/struct.ArrayHasAny.html" class="struct" title="struct datafusion::functions_nested::array_has::ArrayHasAny">ArrayHasAny</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-Cardinality" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_nested/cardinality/struct.Cardinality.html" class="struct" title="struct datafusion::functions_nested::cardinality::Cardinality">Cardinality</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-ArrayAppend" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_nested/concat/struct.ArrayAppend.html" class="struct" title="struct datafusion::functions_nested::concat::ArrayAppend">ArrayAppend</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-ArrayConcat" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_nested/concat/struct.ArrayConcat.html" class="struct" title="struct datafusion::functions_nested::concat::ArrayConcat">ArrayConcat</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-ArrayPrepend" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_nested/concat/struct.ArrayPrepend.html" class="struct" title="struct datafusion::functions_nested::concat::ArrayPrepend">ArrayPrepend</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-ArrayDims" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_nested/dimension/struct.ArrayDims.html" class="struct" title="struct datafusion::functions_nested::dimension::ArrayDims">ArrayDims</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-ArrayDistance" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_nested/distance/struct.ArrayDistance.html" class="struct" title="struct datafusion::functions_nested::distance::ArrayDistance">ArrayDistance</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-ArrayEmpty" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_nested/empty/struct.ArrayEmpty.html" class="struct" title="struct datafusion::functions_nested::empty::ArrayEmpty">ArrayEmpty</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-ArrayExcept" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_nested/except/struct.ArrayExcept.html" class="struct" title="struct datafusion::functions_nested::except::ArrayExcept">ArrayExcept</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-ArrayElement" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_nested/extract/struct.ArrayElement.html" class="struct" title="struct datafusion::functions_nested::extract::ArrayElement">ArrayElement</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-Flatten" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_nested/flatten/struct.Flatten.html" class="struct" title="struct datafusion::functions_nested::flatten::Flatten">Flatten</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-ArrayLength" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_nested/length/struct.ArrayLength.html" class="struct" title="struct datafusion::functions_nested::length::ArrayLength">ArrayLength</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-MakeArray" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_nested/make_array/struct.MakeArray.html" class="struct" title="struct datafusion::functions_nested::make_array::MakeArray">MakeArray</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-MapFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_nested/map/struct.MapFunc.html" class="struct" title="struct datafusion::functions_nested::map::MapFunc">MapFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-MapEntriesFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_nested/map_entries/struct.MapEntriesFunc.html" class="struct" title="struct datafusion::functions_nested::map_entries::MapEntriesFunc">MapEntriesFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-MapExtract" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_nested/map_extract/struct.MapExtract.html" class="struct" title="struct datafusion::functions_nested::map_extract::MapExtract">MapExtract</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-MapKeysFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_nested/map_keys/struct.MapKeysFunc.html" class="struct" title="struct datafusion::functions_nested::map_keys::MapKeysFunc">MapKeysFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-ArrayMax" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_nested/min_max/struct.ArrayMax.html" class="struct" title="struct datafusion::functions_nested::min_max::ArrayMax">ArrayMax</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-ArrayPosition" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_nested/position/struct.ArrayPosition.html" class="struct" title="struct datafusion::functions_nested::position::ArrayPosition">ArrayPosition</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-Range" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_nested/range/struct.Range.html" class="struct" title="struct datafusion::functions_nested::range::Range">Range</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-ArrayRemove" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_nested/remove/struct.ArrayRemove.html" class="struct" title="struct datafusion::functions_nested::remove::ArrayRemove">ArrayRemove</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-ArrayRepeat" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_nested/repeat/struct.ArrayRepeat.html" class="struct" title="struct datafusion::functions_nested::repeat::ArrayRepeat">ArrayRepeat</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-ArrayReplace" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_nested/replace/struct.ArrayReplace.html" class="struct" title="struct datafusion::functions_nested::replace::ArrayReplace">ArrayReplace</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-ArrayResize" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_nested/resize/struct.ArrayResize.html" class="struct" title="struct datafusion::functions_nested::resize::ArrayResize">ArrayResize</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-ArrayReverse" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_nested/reverse/struct.ArrayReverse.html" class="struct" title="struct datafusion::functions_nested::reverse::ArrayReverse">ArrayReverse</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-ArrayUnion" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_nested/set_ops/struct.ArrayUnion.html" class="struct" title="struct datafusion::functions_nested::set_ops::ArrayUnion">ArrayUnion</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-ArraySort" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_nested/sort/struct.ArraySort.html" class="struct" title="struct datafusion::functions_nested::sort::ArraySort">ArraySort</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-ArrayToString" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_nested/string/struct.ArrayToString.html" class="struct" title="struct datafusion::functions_nested::string::ArrayToString">ArrayToString</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-SimpleScalarUDF" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SimpleScalarUDF.html" class="struct" title="struct datafusion::prelude::SimpleScalarUDF">SimpleScalarUDF</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#impl-ScalarUDFImpl-for-AsyncScalarUDF" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/async_udf/struct.AsyncScalarUDF.html" class="struct" title="struct datafusion::logical_expr::async_udf::AsyncScalarUDF">AsyncScalarUDF</a>
