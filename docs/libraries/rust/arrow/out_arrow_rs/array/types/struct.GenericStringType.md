# Struct GenericStringType Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/types.rs.html#1609" class="src">Source</a>

``` rust
pub struct GenericStringType<O>where
    O: OffsetSizeTrait,{ /* private fields */ }
```

Expand description

[`ByteArrayType`](https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteArrayType.html "trait arrow::datatypes::ByteArrayType") for string arrays

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.GenericStringType.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.GenericStringType.html#impl-ByteArrayType-for-GenericStringType%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait arrow::datatypes::ByteArrayType">ByteArrayType</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.GenericStringType.html" class="struct" title="struct arrow::datatypes::GenericStringType">GenericStringType</a>\<O\>

where O: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.GenericStringType.html#associatedconstant.PREFIX" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteArrayType.html#associatedconstant.PREFIX" class="constant">PREFIX</a>: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a> = "String"

“Binary” or “String”, for use in error messages

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.GenericStringType.html#associatedconstant.DATA_TYPE" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteArrayType.html#associatedconstant.DATA_TYPE" class="constant">DATA_TYPE</a>: <a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a>

Datatype of array elements

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.GenericStringType.html#associatedtype.Offset" class="anchor">§</a>

#### type <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteArrayType.html#associatedtype.Offset" class="associatedtype">Offset</a> = O

Type of offset i.e i32/i64

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.GenericStringType.html#associatedtype.Native" class="anchor">§</a>

#### type <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteArrayType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Type for representing its equivalent rust type i.e Utf8Array will have native type has &str BinaryArray will have type as [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html "primitive u8")

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.GenericStringType.html#method.validate" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteArrayType.html#tymethod.validate" class="fn">validate</a>( offsets: &<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.OffsetBuffer.html" class="struct" title="struct arrow::buffer::OffsetBuffer">OffsetBuffer</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.GenericStringType.html" class="struct" title="struct arrow::datatypes::GenericStringType">GenericStringType</a>\<O\> as <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait arrow::datatypes::ByteArrayType">ByteArrayType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteArrayType.html#associatedtype.Offset" class="associatedtype" title="type arrow::datatypes::ByteArrayType::Offset">Offset</a>\>, values: &<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Verifies that every consecutive pair of `offsets` denotes a valid slice of `values`

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.GenericStringType.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.GenericStringType.html#blanket-implementations" class="anchor">§</a>
