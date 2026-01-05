# Enum TypeSignatureClass Copy item path

<a href="https://docs.rs/datafusion-expr-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr_common/signature.rs.html#260" class="src">Source</a>

``` rust
pub enum TypeSignatureClass {
    Timestamp,
    Time,
    Interval,
    Duration,
    Native(Arc<dyn LogicalType>),
    Integer,
}
```

Expand description

Represents the class of types that can be used in a function signature.

This is used to specify what types are valid for function arguments in a more flexible way than just listing specific DataTypes. For example, TypeSignatureClass::Timestamp matches any timestamp type regardless of timezone or precision.

Used primarily with [`TypeSignature::Coercible`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#variant.Coercible "variant datafusion::logical_expr::TypeSignature::Coercible") to define function signatures that can accept arguments that can be coerced to a particular class of types.

## Variants<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/signature/enum.TypeSignatureClass.html#variants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/signature/enum.TypeSignatureClass.html#variant.Timestamp" class="anchor">§</a>

### Timestamp

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/signature/enum.TypeSignatureClass.html#variant.Time" class="anchor">§</a>

### Time

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/signature/enum.TypeSignatureClass.html#variant.Interval" class="anchor">§</a>

### Interval

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/signature/enum.TypeSignatureClass.html#variant.Duration" class="anchor">§</a>

### Duration

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/signature/enum.TypeSignatureClass.html#variant.Native" class="anchor">§</a>

### Native(<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/trait.LogicalType.html" class="trait" title="trait datafusion::common::types::LogicalType">LogicalType</a>\>)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/signature/enum.TypeSignatureClass.html#variant.Integer" class="anchor">§</a>

### Integer

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/signature/enum.TypeSignatureClass.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/signature/enum.TypeSignatureClass.html#impl-TypeSignatureClass" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignatureClass.html" class="enum" title="enum datafusion::logical_expr::TypeSignatureClass">TypeSignatureClass</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/signature/enum.TypeSignatureClass.html#method.matches_native_type" class="fn">matches_native_type</a>(&self, logical_type: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/enum.NativeType.html" class="enum" title="enum datafusion::common::types::NativeType">NativeType</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Does the specified `NativeType` match this type signature class?

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/signature/enum.TypeSignatureClass.html#method.default_casted_type" class="fn">default_casted_type</a>( &self, native_type: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/enum.NativeType.html" class="enum" title="enum datafusion::common::types::NativeType">NativeType</a>, origin_type: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

What type would `origin_type` be casted to when casting to the specified native type?

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/signature/enum.TypeSignatureClass.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/signature/enum.TypeSignatureClass.html#impl-Clone-for-TypeSignatureClass" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignatureClass.html" class="enum" title="enum datafusion::logical_expr::TypeSignatureClass">TypeSignatureClass</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/signature/enum.TypeSignatureClass.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignatureClass.html" class="enum" title="enum datafusion::logical_expr::TypeSignatureClass">TypeSignatureClass</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/signature/enum.TypeSignatureClass.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/signature/enum.TypeSignatureClass.html#impl-Debug-for-TypeSignatureClass" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignatureClass.html" class="enum" title="enum datafusion::logical_expr::TypeSignatureClass">TypeSignatureClass</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/signature/enum.TypeSignatureClass.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/signature/enum.TypeSignatureClass.html#impl-Display-for-TypeSignatureClass" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignatureClass.html" class="enum" title="enum datafusion::logical_expr::TypeSignatureClass">TypeSignatureClass</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/signature/enum.TypeSignatureClass.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/signature/enum.TypeSignatureClass.html#impl-Hash-for-TypeSignatureClass" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignatureClass.html" class="enum" title="enum datafusion::logical_expr::TypeSignatureClass">TypeSignatureClass</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/signature/enum.TypeSignatureClass.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/signature/enum.TypeSignatureClass.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/signature/enum.TypeSignatureClass.html#impl-PartialEq-for-TypeSignatureClass" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignatureClass.html" class="enum" title="enum datafusion::logical_expr::TypeSignatureClass">TypeSignatureClass</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/signature/enum.TypeSignatureClass.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignatureClass.html" class="enum" title="enum datafusion::logical_expr::TypeSignatureClass">TypeSignatureClass</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/signature/enum.TypeSignatureClass.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/signature/enum.TypeSignatureClass.html#impl-PartialOrd-for-TypeSignatureClass" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignatureClass.html" class="enum" title="enum datafusion::logical_expr::TypeSignatureClass">TypeSignatureClass</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/signature/enum.TypeSignatureClass.html#method.partial_cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp" class="fn">partial_cmp</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignatureClass.html" class="enum" title="enum datafusion::logical_expr::TypeSignatureClass">TypeSignatureClass</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1398" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/signature/enum.TypeSignatureClass.html#method.lt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt" class="fn">lt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1416" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/signature/enum.TypeSignatureClass.html#method.le" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le" class="fn">le</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1434" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/signature/enum.TypeSignatureClass.html#method.gt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt" class="fn">gt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1452" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/signature/enum.TypeSignatureClass.html#method.ge" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge" class="fn">ge</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/signature/enum.TypeSignatureClass.html#impl-Eq-for-TypeSignatureClass" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignatureClass.html" class="enum" title="enum datafusion::logical_expr::TypeSignatureClass">TypeSignatureClass</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/signature/enum.TypeSignatureClass.html#impl-StructuralPartialEq-for-TypeSignatureClass" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignatureClass.html" class="enum" title="enum datafusion::logical_expr::TypeSignatureClass">TypeSignatureClass</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/signature/enum.TypeSignatureClass.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/signature/enum.TypeSignatureClass.html#blanket-implementations" class="anchor">§</a>
