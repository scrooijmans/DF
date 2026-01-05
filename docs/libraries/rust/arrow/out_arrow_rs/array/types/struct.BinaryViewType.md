# Struct BinaryViewType Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/types.rs.html#1741" class="src">Source</a>

``` rust
pub struct BinaryViewType {}
```

Expand description

[`BinaryViewType`](https://docs.rs/arrow/latest/arrow/datatypes/struct.BinaryViewType.html "struct arrow::datatypes::BinaryViewType") for string arrays

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.BinaryViewType.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.BinaryViewType.html#impl-ByteViewType-for-BinaryViewType" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html" class="trait" title="trait arrow::datatypes::ByteViewType">ByteViewType</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.BinaryViewType.html" class="struct" title="struct arrow::datatypes::BinaryViewType">BinaryViewType</a>

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.BinaryViewType.html#associatedconstant.IS_UTF8" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html#associatedconstant.IS_UTF8" class="constant">IS_UTF8</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a> = false

If element in array is utf8 encoded string.

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.BinaryViewType.html#associatedconstant.PREFIX" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html#associatedconstant.PREFIX" class="constant">PREFIX</a>: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a> = "Binary"

“Binary” or “String”, for use in displayed or error messages

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.BinaryViewType.html#associatedtype.Native" class="anchor">§</a>

#### type <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html#associatedtype.Native" class="associatedtype">Native</a> = \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]

Type for representing its equivalent rust type i.e Utf8Array will have native type has &str BinaryArray will have type as [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html "primitive u8")

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.BinaryViewType.html#associatedtype.Owned" class="anchor">§</a>

#### type <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html#associatedtype.Owned" class="associatedtype">Owned</a> = <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>

Type for owned corresponding to `Native`

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.BinaryViewType.html#method.validate" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html#tymethod.validate" class="fn">validate</a>(views: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\], buffers: &\[<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Verifies that the provided buffers are valid for this array type

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.BinaryViewType.html#associatedconstant.DATA_TYPE" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html#associatedconstant.DATA_TYPE" class="constant">DATA_TYPE</a>: <a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a> = \_

Datatype of array elements

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.BinaryViewType.html#impl-PartialEq-for-BinaryViewType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.BinaryViewType.html" class="struct" title="struct arrow::datatypes::BinaryViewType">BinaryViewType</a>

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.BinaryViewType.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.BinaryViewType.html" class="struct" title="struct arrow::datatypes::BinaryViewType">BinaryViewType</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/array/types/struct.BinaryViewType.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.BinaryViewType.html#impl-StructuralPartialEq-for-BinaryViewType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.BinaryViewType.html" class="struct" title="struct arrow::datatypes::BinaryViewType">BinaryViewType</a>

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.BinaryViewType.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.BinaryViewType.html#blanket-implementations" class="anchor">§</a>
