# Trait LogicalType Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/types/logical.rs.html#78" class="src">Source</a>

``` rust
pub trait LogicalType: Sync + Send {
    // Required methods
    fn native(&self) -> &NativeType;
    fn signature(&self) -> TypeSignature<'_>;

    // Provided method
    fn default_cast_for(
        &self,
        origin: &DataType,
    ) -> Result<DataType, DataFusionError> { ... }
}
```

Expand description

Representation of a logical type with its signature and its native backing type.

The logical type is meant to be used during the DataFusion logical planning phase in order to reason about logical types without worrying about their underlying physical implementation.

#### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/trait.LogicalType.html#extension-types" class="doc-anchor">§</a>Extension types

[`LogicalType`](https://docs.rs/datafusion/50.2.0/datafusion/common/types/trait.LogicalType.html "trait datafusion::common::types::LogicalType") is a trait in order to allow the possibility of declaring extension types:

``` rust
use datafusion_common::types::{LogicalType, NativeType, TypeSignature};

struct JSON {}

impl LogicalType for JSON {
    fn native(&self) -> &NativeType {
        &NativeType::String
    }

   fn signature(&self) -> TypeSignature<'_> {
       TypeSignature::Extension {
           name: "JSON",
           parameters: &[],
       }
   }
}
```

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/trait.LogicalType.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/trait.LogicalType.html#tymethod.native" class="fn">native</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/enum.NativeType.html" class="enum" title="enum datafusion::common::types::NativeType">NativeType</a>

Get the native backing type of this logical type.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/trait.LogicalType.html#tymethod.signature" class="fn">signature</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/enum.TypeSignature.html" class="enum" title="enum datafusion::common::types::TypeSignature">TypeSignature</a>\<'\_\>

Get the unique type signature for this logical type. Logical types with identical signatures are considered equal.

## Provided Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/trait.LogicalType.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/trait.LogicalType.html#method.default_cast_for" class="fn">default_cast_for</a>( &self, origin: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Get the default physical type to cast `origin` to in order to obtain a physical type that is logically compatible with this logical type.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/trait.LogicalType.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/trait.LogicalType.html#impl-Debug-for-dyn+LogicalType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/trait.LogicalType.html" class="trait" title="trait datafusion::common::types::LogicalType">LogicalType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/trait.LogicalType.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/trait.LogicalType.html#impl-Display-for-dyn+LogicalType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/trait.LogicalType.html" class="trait" title="trait datafusion::common::types::LogicalType">LogicalType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/trait.LogicalType.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/trait.LogicalType.html#impl-Hash-for-dyn+LogicalType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/trait.LogicalType.html" class="trait" title="trait datafusion::common::types::LogicalType">LogicalType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/trait.LogicalType.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/trait.LogicalType.html#impl-Ord-for-dyn+LogicalType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a> for dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/trait.LogicalType.html" class="trait" title="trait datafusion::common::types::LogicalType">LogicalType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/trait.LogicalType.html#method.cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp" class="fn">cmp</a>(&self, other: &(dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/trait.LogicalType.html" class="trait" title="trait datafusion::common::types::LogicalType">LogicalType</a> + 'static)) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>

This method returns an [`Ordering`](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering") between `self` and `other`. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/trait.LogicalType.html#impl-PartialEq-for-dyn+LogicalType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/trait.LogicalType.html" class="trait" title="trait datafusion::common::types::LogicalType">LogicalType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/trait.LogicalType.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &(dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/trait.LogicalType.html" class="trait" title="trait datafusion::common::types::LogicalType">LogicalType</a> + 'static)) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/trait.LogicalType.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/trait.LogicalType.html#impl-PartialOrd-for-dyn+LogicalType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a> for dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/trait.LogicalType.html" class="trait" title="trait datafusion::common::types::LogicalType">LogicalType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/trait.LogicalType.html#method.partial_cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp" class="fn">partial_cmp</a>(&self, other: &(dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/trait.LogicalType.html" class="trait" title="trait datafusion::common::types::LogicalType">LogicalType</a> + 'static)) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1398" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/trait.LogicalType.html#method.lt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt" class="fn">lt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1416" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/trait.LogicalType.html#method.le" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le" class="fn">le</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1434" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/trait.LogicalType.html#method.gt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt" class="fn">gt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1452" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/trait.LogicalType.html#method.ge" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge" class="fn">ge</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/trait.LogicalType.html#impl-Eq-for-dyn+LogicalType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/trait.LogicalType.html" class="trait" title="trait datafusion::common::types::LogicalType">LogicalType</a>

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/trait.LogicalType.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/trait.LogicalType.html#impl-LogicalType-for-NativeType" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/trait.LogicalType.html" class="trait" title="trait datafusion::common::types::LogicalType">LogicalType</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/enum.NativeType.html" class="enum" title="enum datafusion::common::types::NativeType">NativeType</a>
