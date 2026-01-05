# Struct CumeDist Copy item path

<a href="https://docs.rs/datafusion-functions-window/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_window/cume_dist.rs.html#67" class="src">Source</a>

``` rust
pub struct CumeDist { /* private fields */ }
```

Expand description

CumeDist calculates the cume_dist in the window function with order by

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/cume_dist/struct.CumeDist.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/cume_dist/struct.CumeDist.html#impl-CumeDist" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/cume_dist/struct.CumeDist.html" class="struct" title="struct datafusion::functions_window::cume_dist::CumeDist">CumeDist</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/cume_dist/struct.CumeDist.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/cume_dist/struct.CumeDist.html" class="struct" title="struct datafusion::functions_window::cume_dist::CumeDist">CumeDist</a>

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/cume_dist/struct.CumeDist.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/cume_dist/struct.CumeDist.html#impl-Debug-for-CumeDist" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/cume_dist/struct.CumeDist.html" class="struct" title="struct datafusion::functions_window::cume_dist::CumeDist">CumeDist</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/cume_dist/struct.CumeDist.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/cume_dist/struct.CumeDist.html#impl-Default-for-CumeDist" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/cume_dist/struct.CumeDist.html" class="struct" title="struct datafusion::functions_window::cume_dist::CumeDist">CumeDist</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/cume_dist/struct.CumeDist.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/cume_dist/struct.CumeDist.html" class="struct" title="struct datafusion::functions_window::cume_dist::CumeDist">CumeDist</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/cume_dist/struct.CumeDist.html#impl-Hash-for-CumeDist" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/cume_dist/struct.CumeDist.html" class="struct" title="struct datafusion::functions_window::cume_dist::CumeDist">CumeDist</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/cume_dist/struct.CumeDist.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/cume_dist/struct.CumeDist.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/cume_dist/struct.CumeDist.html#impl-PartialEq-for-CumeDist" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/cume_dist/struct.CumeDist.html" class="struct" title="struct datafusion::functions_window::cume_dist::CumeDist">CumeDist</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/cume_dist/struct.CumeDist.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/cume_dist/struct.CumeDist.html" class="struct" title="struct datafusion::functions_window::cume_dist::CumeDist">CumeDist</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/cume_dist/struct.CumeDist.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/cume_dist/struct.CumeDist.html#impl-WindowUDFImpl-for-CumeDist" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html" class="trait" title="trait datafusion::logical_expr::WindowUDFImpl">WindowUDFImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/cume_dist/struct.CumeDist.html" class="struct" title="struct datafusion::functions_window::cume_dist::CumeDist">CumeDist</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/cume_dist/struct.CumeDist.html#method.as_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Return a reference to Any that can be used for downcasting

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/cume_dist/struct.CumeDist.html#method.name" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#tymethod.name" class="fn">name</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Returns this function’s name

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/cume_dist/struct.CumeDist.html#method.signature" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#tymethod.signature" class="fn">signature</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html" class="struct" title="struct datafusion::logical_expr::Signature">Signature</a>

Returns the function’s [`Signature`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html "struct datafusion::logical_expr::Signature") for information about what input types are accepted and the function’s Volatility.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/cume_dist/struct.CumeDist.html#method.partition_evaluator" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#tymethod.partition_evaluator" class="fn">partition_evaluator</a>( &self, \_partition_evaluator_args: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.PartitionEvaluatorArgs.html" class="struct" title="struct datafusion::logical_expr::function::PartitionEvaluatorArgs">PartitionEvaluatorArgs</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.PartitionEvaluator.html" class="trait" title="trait datafusion::logical_expr::PartitionEvaluator">PartitionEvaluator</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Invoke the function, returning the [`PartitionEvaluator`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.PartitionEvaluator.html "trait datafusion::logical_expr::PartitionEvaluator") instance

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/cume_dist/struct.CumeDist.html#method.field" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#tymethod.field" class="fn">field</a>( &self, field_args: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.WindowUDFFieldArgs.html" class="struct" title="struct datafusion::logical_expr::function::WindowUDFFieldArgs">WindowUDFFieldArgs</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

The [`FieldRef`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/type.FieldRef.html "type datafusion::common::arrow::datatypes::FieldRef") of the final result of evaluating this window function. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#tymethod.field)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/cume_dist/struct.CumeDist.html#method.documentation" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#method.documentation" class="fn">documentation</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Documentation.html" class="struct" title="struct datafusion::logical_expr::Documentation">Documentation</a>\>

Returns the documentation for this Window UDF. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#method.documentation)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/cume_dist/struct.CumeDist.html#method.aliases" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#method.aliases" class="fn">aliases</a>(&self) -\> &\[<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\]

Returns any aliases (alternate names) for this function. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#method.aliases)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/cume_dist/struct.CumeDist.html#method.expressions" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#method.expressions" class="fn">expressions</a>( &self, expr_args: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.ExpressionArgs.html" class="struct" title="struct datafusion::logical_expr::function::ExpressionArgs">ExpressionArgs</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>\>

Returns the expressions that are passed to the [`PartitionEvaluator`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.PartitionEvaluator.html "trait datafusion::logical_expr::PartitionEvaluator").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/cume_dist/struct.CumeDist.html#method.simplify" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#method.simplify" class="fn">simplify</a>( &self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WindowFunction.html" class="struct" title="struct datafusion::logical_expr::expr::WindowFunction">WindowFunction</a>, &dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/simplify/trait.SimplifyInfo.html" class="trait" title="trait datafusion::logical_expr::simplify::SimplifyInfo">SimplifyInfo</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\>\>

Optionally apply per-UDWF simplification / rewrite rules. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#method.simplify)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/cume_dist/struct.CumeDist.html#method.sort_options" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#method.sort_options" class="fn">sort_options</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/struct.SortOptions.html" class="struct" title="struct datafusion::common::arrow::compute::SortOptions">SortOptions</a>\>

Allows the window UDF to define a custom result ordering. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#method.sort_options)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/cume_dist/struct.CumeDist.html#method.coerce_types" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#method.coerce_types" class="fn">coerce_types</a>( &self, \_arg_types: &\[<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Coerce arguments of a function call to types that the function can evaluate. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#method.coerce_types)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/cume_dist/struct.CumeDist.html#method.reverse_expr" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#method.reverse_expr" class="fn">reverse_expr</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ReversedUDWF.html" class="enum" title="enum datafusion::logical_expr::ReversedUDWF">ReversedUDWF</a>

Allows customizing the behavior of the user-defined window function when it is evaluated in reverse order.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/cume_dist/struct.CumeDist.html#impl-Eq-for-CumeDist" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/cume_dist/struct.CumeDist.html" class="struct" title="struct datafusion::functions_window::cume_dist::CumeDist">CumeDist</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/cume_dist/struct.CumeDist.html#impl-StructuralPartialEq-for-CumeDist" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/cume_dist/struct.CumeDist.html" class="struct" title="struct datafusion::functions_window::cume_dist::CumeDist">CumeDist</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/cume_dist/struct.CumeDist.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/cume_dist/struct.CumeDist.html#blanket-implementations" class="anchor">§</a>
