# Struct CastExpr Copy item path

<a href="https://docs.rs/datafusion-physical-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr/expressions/cast.rs.html#46" class="src">Source</a>

``` rust
pub struct CastExpr {
    pub expr: Arc<dyn PhysicalExpr>,
    /* private fields */
}
```

Expand description

CAST expression casts an expression to a specific data type and returns a runtime error on invalid cast

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html#structfield.expr" class="anchor field">§</a>`expr: `<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc"><code>Arc</code></a>`<dyn `<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr"><code>PhysicalExpr</code></a>`>`

The expression to cast

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html#impl-CastExpr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html" class="struct" title="struct datafusion::physical_expr::expressions::CastExpr">CastExpr</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html#method.new" class="fn">new</a>( expr: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>, cast_type: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, cast_options: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/struct.CastOptions.html" class="struct" title="struct datafusion::common::arrow::compute::CastOptions">CastOptions</a>\<'static\>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html" class="struct" title="struct datafusion::physical_expr::expressions::CastExpr">CastExpr</a>

Create a new CastExpr

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html#method.expr" class="fn">expr</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>

The expression to cast

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html#method.cast_type" class="fn">cast_type</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

The data type to cast to

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html#method.cast_options" class="fn">cast_options</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/struct.CastOptions.html" class="struct" title="struct datafusion::common::arrow::compute::CastOptions">CastOptions</a>\<'static\>

The cast options

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html#method.is_bigger_cast" class="fn">is_bigger_cast</a>(&self, src: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Check if the cast is a widening cast (e.g. from `Int8` to `Int16`).

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html#impl-Clone-for-CastExpr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html" class="struct" title="struct datafusion::physical_expr::expressions::CastExpr">CastExpr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html" class="struct" title="struct datafusion::physical_expr::expressions::CastExpr">CastExpr</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html#impl-Debug-for-CastExpr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html" class="struct" title="struct datafusion::physical_expr::expressions::CastExpr">CastExpr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html#impl-Display-for-CastExpr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html" class="struct" title="struct datafusion::physical_expr::expressions::CastExpr">CastExpr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html#impl-Hash-for-CastExpr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html" class="struct" title="struct datafusion::physical_expr::expressions::CastExpr">CastExpr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html#impl-PartialEq-for-CastExpr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html" class="struct" title="struct datafusion::physical_expr::expressions::CastExpr">CastExpr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html" class="struct" title="struct datafusion::physical_expr::expressions::CastExpr">CastExpr</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html#impl-PhysicalExpr-for-CastExpr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html" class="struct" title="struct datafusion::physical_expr::expressions::CastExpr">CastExpr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html#method.as_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Return a reference to Any that can be used for downcasting

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html#method.get_properties" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#method.get_properties" class="fn">get_properties</a>( &self, children: &\[<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/struct.ExprProperties.html" class="struct" title="struct datafusion::logical_expr::sort_properties::ExprProperties">ExprProperties</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/struct.ExprProperties.html" class="struct" title="struct datafusion::logical_expr::sort_properties::ExprProperties">ExprProperties</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

A [`CastExpr`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html "struct datafusion::physical_expr::expressions::CastExpr") preserves the ordering of its child if the cast is done under the same datatype family.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html#method.data_type" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#method.data_type" class="fn">data_type</a>(&self, \_input_schema: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Get the data type of this expression, given the schema of the input

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html#method.nullable" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#method.nullable" class="fn">nullable</a>(&self, input_schema: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Determine whether this expression is nullable, given the schema of the input

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html#method.evaluate" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#tymethod.evaluate" class="fn">evaluate</a>( &self, batch: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html" class="enum" title="enum datafusion::logical_expr::ColumnarValue">ColumnarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Evaluate an expression against a RecordBatch

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html#method.return_field" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#method.return_field" class="fn">return_field</a>( &self, input_schema: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

The output field associated with this expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html#method.children" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#tymethod.children" class="fn">children</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<&<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>\>

Get a list of child PhysicalExpr that provide the input for this expr.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html#method.with_new_children" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#tymethod.with_new_children" class="fn">with_new_children</a>( self: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html" class="struct" title="struct datafusion::physical_expr::expressions::CastExpr">CastExpr</a>\>, children: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns a new PhysicalExpr where all children were replaced by new exprs.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html#method.evaluate_bounds" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#method.evaluate_bounds" class="fn">evaluate_bounds</a>( &self, children: &\[&<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/struct.Interval.html" class="struct" title="struct datafusion::logical_expr::interval_arithmetic::Interval">Interval</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/struct.Interval.html" class="struct" title="struct datafusion::logical_expr::interval_arithmetic::Interval">Interval</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Computes the output interval for the expression, given the input intervals. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#method.evaluate_bounds)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html#method.propagate_constraints" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#method.propagate_constraints" class="fn">propagate_constraints</a>( &self, interval: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/struct.Interval.html" class="struct" title="struct datafusion::logical_expr::interval_arithmetic::Interval">Interval</a>, children: &\[&<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/struct.Interval.html" class="struct" title="struct datafusion::logical_expr::interval_arithmetic::Interval">Interval</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/struct.Interval.html" class="struct" title="struct datafusion::logical_expr::interval_arithmetic::Interval">Interval</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Updates bounds for child expressions, given a known interval for this expression. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#method.propagate_constraints)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html#method.fmt_sql" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#tymethod.fmt_sql" class="fn">fmt_sql</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Format this `PhysicalExpr` in nice human readable “SQL” format [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#tymethod.fmt_sql)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html#method.evaluate_selection" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#method.evaluate_selection" class="fn">evaluate_selection</a>( &self, batch: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>, selection: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.BooleanArray.html" class="struct" title="struct datafusion::common::arrow::array::BooleanArray">BooleanArray</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html" class="enum" title="enum datafusion::logical_expr::ColumnarValue">ColumnarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Evaluate an expression against a RecordBatch after first applying a validity array

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html#method.evaluate_statistics" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#method.evaluate_statistics" class="fn">evaluate_statistics</a>( &self, children: &\[&<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html" class="enum" title="enum datafusion::logical_expr::statistics::Distribution">Distribution</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html" class="enum" title="enum datafusion::logical_expr::statistics::Distribution">Distribution</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Computes the output statistics for the expression, given the input statistics. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#method.evaluate_statistics)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html#method.propagate_statistics" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#method.propagate_statistics" class="fn">propagate_statistics</a>( &self, parent: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html" class="enum" title="enum datafusion::logical_expr::statistics::Distribution">Distribution</a>, children: &\[&<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html" class="enum" title="enum datafusion::logical_expr::statistics::Distribution">Distribution</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html" class="enum" title="enum datafusion::logical_expr::statistics::Distribution">Distribution</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Updates children statistics using the given parent statistic for this expression. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#method.propagate_statistics)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html#method.snapshot" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#method.snapshot" class="fn">snapshot</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Take a snapshot of this `PhysicalExpr`, if it is dynamic. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#method.snapshot)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html#method.snapshot_generation" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#method.snapshot_generation" class="fn">snapshot_generation</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

Returns the generation of this `PhysicalExpr` for snapshotting purposes. The generation is an arbitrary u64 that can be used to track changes in the state of the `PhysicalExpr` over time without having to do an exhaustive comparison. This is useful to avoid unnecessary computation or serialization if there are no changes to the expression. In particular, dynamic expressions that may change over time; this allows cheap checks for changes. Static expressions that do not change over time should return 0, as does the default implementation. You should not call this method directly as it does not handle recursion. Instead use [`snapshot_generation`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/physical_expr/fn.snapshot_generation.html "fn datafusion::physical_expr_common::physical_expr::snapshot_generation") to handle recursion and capture the full state of the `PhysicalExpr`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html#method.is_volatile_node" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#method.is_volatile_node" class="fn">is_volatile_node</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the expression node is volatile, i.e. whether it can return different results when evaluated multiple times with the same input. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#method.is_volatile_node)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html#impl-Eq-for-CastExpr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html" class="struct" title="struct datafusion::physical_expr::expressions::CastExpr">CastExpr</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html#blanket-implementations" class="anchor">§</a>
