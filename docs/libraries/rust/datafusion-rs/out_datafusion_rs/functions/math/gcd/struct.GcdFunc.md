# Struct GcdFunc Copy item path

<a href="https://docs.rs/datafusion-functions/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions/math/gcd.rs.html#41" class="src">Source</a>

``` rust
pub struct GcdFunc { /* private fields */ }
```

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/gcd/struct.GcdFunc.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/gcd/struct.GcdFunc.html#impl-GcdFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/gcd/struct.GcdFunc.html" class="struct" title="struct datafusion::functions::math::gcd::GcdFunc">GcdFunc</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/gcd/struct.GcdFunc.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/gcd/struct.GcdFunc.html" class="struct" title="struct datafusion::functions::math::gcd::GcdFunc">GcdFunc</a>

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/gcd/struct.GcdFunc.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/gcd/struct.GcdFunc.html#impl-Debug-for-GcdFunc" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/gcd/struct.GcdFunc.html" class="struct" title="struct datafusion::functions::math::gcd::GcdFunc">GcdFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/gcd/struct.GcdFunc.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/gcd/struct.GcdFunc.html#impl-Default-for-GcdFunc" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/gcd/struct.GcdFunc.html" class="struct" title="struct datafusion::functions::math::gcd::GcdFunc">GcdFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/gcd/struct.GcdFunc.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/gcd/struct.GcdFunc.html" class="struct" title="struct datafusion::functions::math::gcd::GcdFunc">GcdFunc</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/gcd/struct.GcdFunc.html#impl-Hash-for-GcdFunc" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/gcd/struct.GcdFunc.html" class="struct" title="struct datafusion::functions::math::gcd::GcdFunc">GcdFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/gcd/struct.GcdFunc.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/gcd/struct.GcdFunc.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/gcd/struct.GcdFunc.html#impl-PartialEq-for-GcdFunc" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/gcd/struct.GcdFunc.html" class="struct" title="struct datafusion::functions::math::gcd::GcdFunc">GcdFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/gcd/struct.GcdFunc.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/gcd/struct.GcdFunc.html" class="struct" title="struct datafusion::functions::math::gcd::GcdFunc">GcdFunc</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/gcd/struct.GcdFunc.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/gcd/struct.GcdFunc.html#impl-ScalarUDFImpl-for-GcdFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html" class="trait" title="trait datafusion::logical_expr::ScalarUDFImpl">ScalarUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/gcd/struct.GcdFunc.html" class="struct" title="struct datafusion::functions::math::gcd::GcdFunc">GcdFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/gcd/struct.GcdFunc.html#method.as_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns this object as an [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") trait object

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/gcd/struct.GcdFunc.html#method.name" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#tymethod.name" class="fn">name</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Returns this function’s name

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/gcd/struct.GcdFunc.html#method.signature" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#tymethod.signature" class="fn">signature</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html" class="struct" title="struct datafusion::logical_expr::Signature">Signature</a>

Returns a [`Signature`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html "struct datafusion::logical_expr::Signature") describing the argument types for which this function has an implementation, and the function’s [`Volatility`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Volatility.html "enum datafusion::logical_expr::Volatility"). [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#tymethod.signature)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/gcd/struct.GcdFunc.html#method.return_type" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#tymethod.return_type" class="fn">return_type</a>( &self, \_arg_types: &\[<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

[`DataType`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html "enum datafusion::common::arrow::datatypes::DataType") returned by this function, given the types of the arguments. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#tymethod.return_type)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/gcd/struct.GcdFunc.html#method.invoke_with_args" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#tymethod.invoke_with_args" class="fn">invoke_with_args</a>( &self, args: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarFunctionArgs.html" class="struct" title="struct datafusion::logical_expr::ScalarFunctionArgs">ScalarFunctionArgs</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html" class="enum" title="enum datafusion::logical_expr::ColumnarValue">ColumnarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Invoke the function returning the appropriate result. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#tymethod.invoke_with_args)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/gcd/struct.GcdFunc.html#method.documentation" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#method.documentation" class="fn">documentation</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Documentation.html" class="struct" title="struct datafusion::logical_expr::Documentation">Documentation</a>\>

Returns the documentation for this Scalar UDF. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#method.documentation)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/gcd/struct.GcdFunc.html#method.aliases" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#method.aliases" class="fn">aliases</a>(&self) -\> &\[<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\]

Returns any aliases (alternate names) for this function. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#method.aliases)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/gcd/struct.GcdFunc.html#method.display_name" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#method.display_name" class="fn">display_name</a>(&self, args: &\[<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

👎Deprecated since 50.0.0: This method is unused and will be removed in a future release

Returns the user-defined display name of function, given the arguments [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#method.display_name)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/gcd/struct.GcdFunc.html#method.schema_name" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#method.schema_name" class="fn">schema_name</a>(&self, args: &\[<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns the name of the column this expression would create [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#method.schema_name)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/gcd/struct.GcdFunc.html#method.return_field_from_args" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#method.return_field_from_args" class="fn">return_field_from_args</a>( &self, args: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ReturnFieldArgs.html" class="struct" title="struct datafusion::logical_expr::ReturnFieldArgs">ReturnFieldArgs</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

What type will be returned by this function, given the arguments? [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#method.return_field_from_args)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/gcd/struct.GcdFunc.html#method.is_nullable" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#method.is_nullable" class="fn">is_nullable</a>(&self, \_args: &\[<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\], \_schema: &dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ExprSchema.html" class="trait" title="trait datafusion::common::ExprSchema">ExprSchema</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

👎Deprecated since 45.0.0: Use `return_field_from_args` instead. if you use `is_nullable` that returns non-nullable with `return_type`, you would need to switch to `return_field_from_args`, you might have error

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/gcd/struct.GcdFunc.html#method.simplify" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#method.simplify" class="fn">simplify</a>( &self, args: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>, \_info: &dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/simplify/trait.SimplifyInfo.html" class="trait" title="trait datafusion::logical_expr::simplify::SimplifyInfo">SimplifyInfo</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/simplify/enum.ExprSimplifyResult.html" class="enum" title="enum datafusion::logical_expr::simplify::ExprSimplifyResult">ExprSimplifyResult</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Optionally apply per-UDF simplification / rewrite rules. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#method.simplify)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/gcd/struct.GcdFunc.html#method.short_circuits" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#method.short_circuits" class="fn">short_circuits</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if some of this `exprs` subexpressions may not be evaluated and thus any side effects (like divide by zero) may not be encountered. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#method.short_circuits)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/gcd/struct.GcdFunc.html#method.evaluate_bounds" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#method.evaluate_bounds" class="fn">evaluate_bounds</a>( &self, \_input: &\[&<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/struct.Interval.html" class="struct" title="struct datafusion::logical_expr::interval_arithmetic::Interval">Interval</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/struct.Interval.html" class="struct" title="struct datafusion::logical_expr::interval_arithmetic::Interval">Interval</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Computes the output [`Interval`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/struct.Interval.html "struct datafusion::logical_expr::interval_arithmetic::Interval") for a [`ScalarUDFImpl`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html "trait datafusion::logical_expr::ScalarUDFImpl"), given the input intervals. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#method.evaluate_bounds)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/gcd/struct.GcdFunc.html#method.propagate_constraints" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#method.propagate_constraints" class="fn">propagate_constraints</a>( &self, \_interval: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/struct.Interval.html" class="struct" title="struct datafusion::logical_expr::interval_arithmetic::Interval">Interval</a>, \_inputs: &\[&<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/struct.Interval.html" class="struct" title="struct datafusion::logical_expr::interval_arithmetic::Interval">Interval</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/struct.Interval.html" class="struct" title="struct datafusion::logical_expr::interval_arithmetic::Interval">Interval</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Updates bounds for child expressions, given a known [`Interval`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/struct.Interval.html "struct datafusion::logical_expr::interval_arithmetic::Interval")s for this function. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#method.propagate_constraints)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/gcd/struct.GcdFunc.html#method.output_ordering" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#method.output_ordering" class="fn">output_ordering</a>( &self, inputs: &\[<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/struct.ExprProperties.html" class="struct" title="struct datafusion::logical_expr::sort_properties::ExprProperties">ExprProperties</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/enum.SortProperties.html" class="enum" title="enum datafusion::logical_expr::sort_properties::SortProperties">SortProperties</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Calculates the [`SortProperties`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/enum.SortProperties.html "enum datafusion::logical_expr::sort_properties::SortProperties") of this function based on its children’s properties.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/gcd/struct.GcdFunc.html#method.preserves_lex_ordering" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#method.preserves_lex_ordering" class="fn">preserves_lex_ordering</a>( &self, \_inputs: &\[<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/struct.ExprProperties.html" class="struct" title="struct datafusion::logical_expr::sort_properties::ExprProperties">ExprProperties</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns true if the function preserves lexicographical ordering based on the input ordering. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#method.preserves_lex_ordering)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/gcd/struct.GcdFunc.html#method.coerce_types" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#method.coerce_types" class="fn">coerce_types</a>( &self, \_arg_types: &\[<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Coerce arguments of a function call to types that the function can evaluate. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#method.coerce_types)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/gcd/struct.GcdFunc.html#impl-Eq-for-GcdFunc" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/gcd/struct.GcdFunc.html" class="struct" title="struct datafusion::functions::math::gcd::GcdFunc">GcdFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/gcd/struct.GcdFunc.html#impl-StructuralPartialEq-for-GcdFunc" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/gcd/struct.GcdFunc.html" class="struct" title="struct datafusion::functions::math::gcd::GcdFunc">GcdFunc</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/gcd/struct.GcdFunc.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/gcd/struct.GcdFunc.html#blanket-implementations" class="anchor">§</a>
