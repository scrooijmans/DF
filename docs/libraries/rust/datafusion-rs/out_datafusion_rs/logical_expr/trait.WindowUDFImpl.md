# Trait WindowUDFImpl Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/udwf.rs.html#312" class="src">Source</a>

``` rust
pub trait WindowUDFImpl:
    Debug
    + DynEq
    + DynHash
    + Send
    + Sync {
    // Required methods
    fn as_any(&self) -> &(dyn Any + 'static);
    fn name(&self) -> &str;
    fn signature(&self) -> &Signature;
    fn partition_evaluator(
        &self,
        partition_evaluator_args: PartitionEvaluatorArgs<'_>,
    ) -> Result<Box<dyn PartitionEvaluator>, DataFusionError>;
    fn field(
        &self,
        field_args: WindowUDFFieldArgs<'_>,
    ) -> Result<Arc<Field>, DataFusionError>;

    // Provided methods
    fn aliases(&self) -> &[String] { ... }
    fn expressions(
        &self,
        expr_args: ExpressionArgs<'_>,
    ) -> Vec<Arc<dyn PhysicalExpr>> { ... }
    fn simplify(
        &self,
    ) -> Option<Box<dyn Fn(WindowFunction, &dyn SimplifyInfo) -> Result<Expr, DataFusionError>>> { ... }
    fn sort_options(&self) -> Option<SortOptions> { ... }
    fn coerce_types(
        &self,
        _arg_types: &[DataType],
    ) -> Result<Vec<DataType>, DataFusionError> { ... }
    fn reverse_expr(&self) -> ReversedUDWF { ... }
    fn documentation(&self) -> Option<&Documentation> { ... }
}
```

Expand description

Trait for implementing [`WindowUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowUDF.html "struct datafusion::logical_expr::WindowUDF").

This trait exposes the full API for implementing user defined window functions and can be used to implement any function.

While the trait depends on [`DynEq`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/dyn_eq/trait.DynEq.html "trait datafusion::logical_expr_common::dyn_eq::DynEq") and [`DynHash`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/dyn_eq/trait.DynHash.html "trait datafusion::logical_expr_common::dyn_eq::DynHash") traits, these should not be implemented directly. Instead, implement [`Eq`](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") and [`Hash`](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") and leverage the blanket implementations of [`DynEq`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/dyn_eq/trait.DynEq.html "trait datafusion::logical_expr_common::dyn_eq::DynEq") and [`DynHash`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/dyn_eq/trait.DynHash.html "trait datafusion::logical_expr_common::dyn_eq::DynHash").

See [`advanced_udwf.rs`](https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/advanced_udwf.rs) for a full example with complete implementation and [`WindowUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowUDF.html "struct datafusion::logical_expr::WindowUDF") for other available options.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#basic-example" class="doc-anchor">§</a>Basic Example

``` rust

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SmoothIt {
  signature: Signature,
}

impl SmoothIt {
  fn new() -> Self {
    Self {
      signature: Signature::uniform(1, vec![DataType::Int32], Volatility::Immutable),
     }
  }
}

static DOCUMENTATION: LazyLock<Documentation> = LazyLock::new(|| {
    Documentation::builder(DOC_SECTION_ANALYTICAL, "smooths the windows", "smooth_it(2)")
        .with_argument("arg1", "The int32 number to smooth by")
        .build()
});

fn get_doc() -> &'static Documentation {
    &DOCUMENTATION
}

/// Implement the WindowUDFImpl trait for SmoothIt
impl WindowUDFImpl for SmoothIt {
   fn as_any(&self) -> &dyn Any { self }
   fn name(&self) -> &str { "smooth_it" }
   fn signature(&self) -> &Signature { &self.signature }
   // The actual implementation would smooth the window
   fn partition_evaluator(
       &self,
       _partition_evaluator_args: PartitionEvaluatorArgs,
   ) -> Result<Box<dyn PartitionEvaluator>> {
       unimplemented!()
   }
   fn field(&self, field_args: WindowUDFFieldArgs) -> Result<FieldRef> {
     if let Some(DataType::Int32) = field_args.get_input_field(0).map(|f| f.data_type().clone()) {
       Ok(Field::new(field_args.name(), DataType::Int32, false).into())
     } else {
       plan_err!("smooth_it only accepts Int32 arguments")
     }
   }
   fn documentation(&self) -> Option<&Documentation> {
     Some(get_doc())
   }
}

// Create a new WindowUDF from the implementation
let smooth_it = WindowUDF::from(SmoothIt::new());

// Call the function `add_one(col)`
// smooth_it(speed) OVER (PARTITION BY car ORDER BY time ASC)
let expr = smooth_it.call(vec![col("speed")])
    .partition_by(vec![col("car")])
    .order_by(vec![col("time").sort(true, true)])
    .window_frame(WindowFrame::new(None))
    .build()
    .unwrap();
```

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns this object as an [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") trait object

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#tymethod.name" class="fn">name</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Returns this function’s name

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#tymethod.signature" class="fn">signature</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html" class="struct" title="struct datafusion::logical_expr::Signature">Signature</a>

Returns the function’s [`Signature`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html "struct datafusion::logical_expr::Signature") for information about what input types are accepted and the function’s Volatility.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#tymethod.partition_evaluator" class="fn">partition_evaluator</a>( &self, partition_evaluator_args: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.PartitionEvaluatorArgs.html" class="struct" title="struct datafusion::logical_expr::function::PartitionEvaluatorArgs">PartitionEvaluatorArgs</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.PartitionEvaluator.html" class="trait" title="trait datafusion::logical_expr::PartitionEvaluator">PartitionEvaluator</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Invoke the function, returning the [`PartitionEvaluator`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.PartitionEvaluator.html "trait datafusion::logical_expr::PartitionEvaluator") instance

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#tymethod.field" class="fn">field</a>( &self, field_args: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.WindowUDFFieldArgs.html" class="struct" title="struct datafusion::logical_expr::function::WindowUDFFieldArgs">WindowUDFFieldArgs</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

The [`FieldRef`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/type.FieldRef.html "type datafusion::common::arrow::datatypes::FieldRef") of the final result of evaluating this window function.

Call `field_args.name()` to get the fully qualified name for defining the [`FieldRef`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/type.FieldRef.html "type datafusion::common::arrow::datatypes::FieldRef"). For a complete example see the implementation in the [Basic Example](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#basic-example "trait datafusion::logical_expr::WindowUDFImpl") section.

## Provided Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#method.aliases" class="fn">aliases</a>(&self) -\> &\[<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\]

Returns any aliases (alternate names) for this function.

Note: `aliases` should only include names other than [`Self::name`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#tymethod.name "method datafusion_expr::udwf::WindowUDFImpl::name::name"). Defaults to `[]` (no aliases)

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#method.expressions" class="fn">expressions</a>( &self, expr_args: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.ExpressionArgs.html" class="struct" title="struct datafusion::logical_expr::function::ExpressionArgs">ExpressionArgs</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>\>

Returns the expressions that are passed to the [`PartitionEvaluator`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.PartitionEvaluator.html "trait datafusion::logical_expr::PartitionEvaluator").

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#method.simplify" class="fn">simplify</a>( &self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WindowFunction.html" class="struct" title="struct datafusion::logical_expr::expr::WindowFunction">WindowFunction</a>, &dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/simplify/trait.SimplifyInfo.html" class="trait" title="trait datafusion::logical_expr::simplify::SimplifyInfo">SimplifyInfo</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\>\>

Optionally apply per-UDWF simplification / rewrite rules.

This can be used to apply function specific simplification rules during optimization. The default implementation does nothing.

Note that DataFusion handles simplifying arguments and “constant folding” (replacing a function call with constant arguments such as `my_add(1,2) --> 3` ). Thus, there is no need to implement such optimizations manually for specific UDFs.

Example: \[`advanced_udwf.rs`\]: <https://github.com/apache/arrow-datafusion/blob/main/datafusion-examples/examples/advanced_udwf.rs>

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#returns" class="doc-anchor">§</a>Returns

[None](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if simplify is not defined or,

Or, a closure with two arguments:

- ‘window_function’: [crate::expr::WindowFunction](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WindowFunction.html "struct datafusion::logical_expr::expr::WindowFunction") for which simplified has been invoked
- ‘info’: [crate::simplify::SimplifyInfo](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/simplify/trait.SimplifyInfo.html "trait datafusion::logical_expr::simplify::SimplifyInfo")

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#method.sort_options" class="fn">sort_options</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/struct.SortOptions.html" class="struct" title="struct datafusion::common::arrow::compute::SortOptions">SortOptions</a>\>

Allows the window UDF to define a custom result ordering.

By default, a window UDF doesn’t introduce an ordering. But when specified by a window UDF this is used to update ordering equivalences.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#method.coerce_types" class="fn">coerce_types</a>( &self, \_arg_types: &\[<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Coerce arguments of a function call to types that the function can evaluate.

This function is only called if [`WindowUDFImpl::signature`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#tymethod.signature "method datafusion::logical_expr::WindowUDFImpl::signature") returns [`crate::TypeSignature::UserDefined`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#variant.UserDefined "variant datafusion::logical_expr::TypeSignature::UserDefined"). Most UDWFs should return one of the other variants of `TypeSignature` which handle common cases

See the [type coercion module](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/type_coercion/index.html "mod datafusion::logical_expr::type_coercion") documentation for more details on type coercion

For example, if your function requires a floating point arguments, but the user calls it like `my_func(1::int)` (aka with `1` as an integer), coerce_types could return `[DataType::Float64]` to ensure the argument was cast to `1::double`

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#parameters" class="doc-anchor">§</a>Parameters

- `arg_types`: The argument types of the arguments this function with

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#return-value" class="doc-anchor">§</a>Return value

A Vec the same length as `arg_types`. DataFusion will `CAST` the function call arguments to these specific types.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#method.reverse_expr" class="fn">reverse_expr</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ReversedUDWF.html" class="enum" title="enum datafusion::logical_expr::ReversedUDWF">ReversedUDWF</a>

Allows customizing the behavior of the user-defined window function when it is evaluated in reverse order.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#method.documentation" class="fn">documentation</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Documentation.html" class="struct" title="struct datafusion::logical_expr::Documentation">Documentation</a>\>

Returns the documentation for this Window UDF.

Documentation can be accessed programmatically as well as generating publicly facing documentation.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#impl-PartialEq-for-dyn+WindowUDFImpl" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html" class="trait" title="trait datafusion::logical_expr::WindowUDFImpl">WindowUDFImpl</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &(dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html" class="trait" title="trait datafusion::logical_expr::WindowUDFImpl">WindowUDFImpl</a> + 'static)) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#impl-PartialOrd-for-dyn+WindowUDFImpl" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a> for dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html" class="trait" title="trait datafusion::logical_expr::WindowUDFImpl">WindowUDFImpl</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#method.partial_cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp" class="fn">partial_cmp</a>(&self, other: &(dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html" class="trait" title="trait datafusion::logical_expr::WindowUDFImpl">WindowUDFImpl</a> + 'static)) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1398" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#method.lt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt" class="fn">lt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1416" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#method.le" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le" class="fn">le</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1434" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#method.gt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt" class="fn">gt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1452" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#method.ge" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge" class="fn">ge</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#impl-WindowUDFImpl-for-CumeDist" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html" class="trait" title="trait datafusion::logical_expr::WindowUDFImpl">WindowUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/cume_dist/struct.CumeDist.html" class="struct" title="struct datafusion::functions_window::cume_dist::CumeDist">CumeDist</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#impl-WindowUDFImpl-for-WindowShift" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html" class="trait" title="trait datafusion::logical_expr::WindowUDFImpl">WindowUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/lead_lag/struct.WindowShift.html" class="struct" title="struct datafusion::functions_window::lead_lag::WindowShift">WindowShift</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#impl-WindowUDFImpl-for-NthValue" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html" class="trait" title="trait datafusion::logical_expr::WindowUDFImpl">WindowUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/nth_value/struct.NthValue.html" class="struct" title="struct datafusion::functions_window::nth_value::NthValue">NthValue</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#impl-WindowUDFImpl-for-Ntile" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html" class="trait" title="trait datafusion::logical_expr::WindowUDFImpl">WindowUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/ntile/struct.Ntile.html" class="struct" title="struct datafusion::functions_window::ntile::Ntile">Ntile</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#impl-WindowUDFImpl-for-Rank" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html" class="trait" title="trait datafusion::logical_expr::WindowUDFImpl">WindowUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/rank/struct.Rank.html" class="struct" title="struct datafusion::functions_window::rank::Rank">Rank</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#impl-WindowUDFImpl-for-RowNumber" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html" class="trait" title="trait datafusion::logical_expr::WindowUDFImpl">WindowUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/row_number/struct.RowNumber.html" class="struct" title="struct datafusion::functions_window::row_number::RowNumber">RowNumber</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#impl-WindowUDFImpl-for-SimpleWindowUDF" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html" class="trait" title="trait datafusion::logical_expr::WindowUDFImpl">WindowUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SimpleWindowUDF.html" class="struct" title="struct datafusion::prelude::SimpleWindowUDF">SimpleWindowUDF</a>
