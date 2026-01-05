# Trait ByteArrayType Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/types.rs.html#1590" class="src">Source</a>

``` rust
pub trait ByteArrayType:
    'static
    + Send
    + Sync
    + ByteArrayTypeSealed {
    type Offset: OffsetSizeTrait;
    type Native: ByteArrayNativeType + AsRef<Self::Native> + AsRef<[u8]> + ?Sized;

    const PREFIX: &'static str;
    const DATA_TYPE: DataType;

    // Required method
    fn validate(
        offsets: &OffsetBuffer<Self::Offset>,
        values: &Buffer,
    ) -> Result<(), ArrowError>;
}
```

Expand description

A trait over the variable-size byte array types

See [Variable Size Binary Layout](https://arrow.apache.org/docs/format/Columnar.html#variable-size-binary-layout)

## Required Associated Constants<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.ByteArrayType.html#required-associated-consts" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/array/types/trait.ByteArrayType.html#associatedconstant.PREFIX" class="constant">PREFIX</a>: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

“Binary” or “String”, for use in error messages

#### const <a href="https://docs.rs/arrow/latest/arrow/array/types/trait.ByteArrayType.html#associatedconstant.DATA_TYPE" class="constant">DATA_TYPE</a>: <a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a>

Datatype of array elements

## Required Associated Types<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.ByteArrayType.html#required-associated-types" class="anchor">§</a>

#### type <a href="https://docs.rs/arrow/latest/arrow/array/types/trait.ByteArrayType.html#associatedtype.Offset" class="associatedtype">Offset</a>: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>

Type of offset i.e i32/i64

#### type <a href="https://docs.rs/arrow/latest/arrow/array/types/trait.ByteArrayType.html#associatedtype.Native" class="associatedtype">Native</a>: ByteArrayNativeType + <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<Self::<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteArrayType.html#associatedtype.Native" class="associatedtype" title="type arrow::datatypes::ByteArrayType::Native">Native</a>\> + <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>

Type for representing its equivalent rust type i.e Utf8Array will have native type has &str BinaryArray will have type as [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html "primitive u8")

## Required Methods<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.ByteArrayType.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/types/trait.ByteArrayType.html#tymethod.validate" class="fn">validate</a>( offsets: &<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.OffsetBuffer.html" class="struct" title="struct arrow::buffer::OffsetBuffer">OffsetBuffer</a>\<Self::<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteArrayType.html#associatedtype.Offset" class="associatedtype" title="type arrow::datatypes::ByteArrayType::Offset">Offset</a>\>, values: &<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Verifies that every consecutive pair of `offsets` denotes a valid slice of `values`

## Dyn Compatibility<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.ByteArrayType.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.ByteArrayType.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.ByteArrayType.html#impl-ByteArrayType-for-GenericBinaryType%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait arrow::datatypes::ByteArrayType">ByteArrayType</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.GenericBinaryType.html" class="struct" title="struct arrow::datatypes::GenericBinaryType">GenericBinaryType</a>\<O\>

where O: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.ByteArrayType.html#associatedconstant.PREFIX-1" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/array/types/trait.ByteArrayType.html#associatedconstant.PREFIX" class="constant">PREFIX</a>: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a> = "Binary"

<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.ByteArrayType.html#associatedconstant.DATA_TYPE-1" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/array/types/trait.ByteArrayType.html#associatedconstant.DATA_TYPE" class="constant">DATA_TYPE</a>: <a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a>

<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.ByteArrayType.html#associatedtype.Offset-1" class="anchor">§</a>

#### type <a href="https://docs.rs/arrow/latest/arrow/array/types/trait.ByteArrayType.html#associatedtype.Offset" class="associatedtype">Offset</a> = O

<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.ByteArrayType.html#associatedtype.Native-1" class="anchor">§</a>

#### type <a href="https://docs.rs/arrow/latest/arrow/array/types/trait.ByteArrayType.html#associatedtype.Native" class="associatedtype">Native</a> = \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]

<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.ByteArrayType.html#impl-ByteArrayType-for-GenericStringType%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait arrow::datatypes::ByteArrayType">ByteArrayType</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.GenericStringType.html" class="struct" title="struct arrow::datatypes::GenericStringType">GenericStringType</a>\<O\>

where O: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.ByteArrayType.html#associatedconstant.PREFIX-2" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/array/types/trait.ByteArrayType.html#associatedconstant.PREFIX" class="constant">PREFIX</a>: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a> = "String"

<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.ByteArrayType.html#associatedconstant.DATA_TYPE-2" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/array/types/trait.ByteArrayType.html#associatedconstant.DATA_TYPE" class="constant">DATA_TYPE</a>: <a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a>

<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.ByteArrayType.html#associatedtype.Offset-2" class="anchor">§</a>

#### type <a href="https://docs.rs/arrow/latest/arrow/array/types/trait.ByteArrayType.html#associatedtype.Offset" class="associatedtype">Offset</a> = O

<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.ByteArrayType.html#associatedtype.Native-2" class="anchor">§</a>

#### type <a href="https://docs.rs/arrow/latest/arrow/array/types/trait.ByteArrayType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>
