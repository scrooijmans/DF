# Trait ByteViewType Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/types.rs.html#1697" class="src">Source</a>

``` rust
pub trait ByteViewType:
    Sealed
    + 'static
    + PartialEq
    + Send
    + Sync {
    type Native: ByteArrayNativeType + AsRef<Self::Native> + AsRef<[u8]> + ?Sized;
    type Owned: Debug + Clone + Sync + Send + AsRef<Self::Native>;

    const IS_UTF8: bool;
    const PREFIX: &'static str;
    const DATA_TYPE: DataType = _;

    // Required method
    fn validate(views: &[u128], buffers: &[Buffer]) -> Result<(), ArrowError>;
}
```

Expand description

A trait over the variable length bytes view array types

## Required Associated Constants<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html#required-associated-consts" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html#associatedconstant.IS_UTF8" class="constant">IS_UTF8</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

If element in array is utf8 encoded string.

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html#associatedconstant.PREFIX" class="constant">PREFIX</a>: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

“Binary” or “String”, for use in displayed or error messages

## Provided Associated Constants<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html#provided-associated-consts" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html#associatedconstant.DATA_TYPE" class="constant">DATA_TYPE</a>: <a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a> = \_

Datatype of array elements

## Required Associated Types<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html#required-associated-types" class="anchor">§</a>

#### type <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html#associatedtype.Native" class="associatedtype">Native</a>: ByteArrayNativeType + <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<Self::<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html#associatedtype.Native" class="associatedtype" title="type arrow::datatypes::ByteViewType::Native">Native</a>\> + <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>

Type for representing its equivalent rust type i.e Utf8Array will have native type has &str BinaryArray will have type as [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html "primitive u8")

#### type <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html#associatedtype.Owned" class="associatedtype">Owned</a>: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> + <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<Self::<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html#associatedtype.Native" class="associatedtype" title="type arrow::datatypes::ByteViewType::Native">Native</a>\>

Type for owned corresponding to `Native`

## Required Methods<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html#tymethod.validate" class="fn">validate</a>(views: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\], buffers: &\[<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Verifies that the provided buffers are valid for this array type

## Dyn Compatibility<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html#impl-ByteViewType-for-BinaryViewType" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html" class="trait" title="trait arrow::datatypes::ByteViewType">ByteViewType</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.BinaryViewType.html" class="struct" title="struct arrow::datatypes::BinaryViewType">BinaryViewType</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html#associatedconstant.IS_UTF8-1" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html#associatedconstant.IS_UTF8" class="constant">IS_UTF8</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a> = false

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html#associatedconstant.PREFIX-1" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html#associatedconstant.PREFIX" class="constant">PREFIX</a>: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a> = "Binary"

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html#associatedtype.Native-1" class="anchor">§</a>

#### type <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html#associatedtype.Native" class="associatedtype">Native</a> = \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html#associatedtype.Owned-1" class="anchor">§</a>

#### type <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html#associatedtype.Owned" class="associatedtype">Owned</a> = <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html#impl-ByteViewType-for-StringViewType" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html" class="trait" title="trait arrow::datatypes::ByteViewType">ByteViewType</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.StringViewType.html" class="struct" title="struct arrow::datatypes::StringViewType">StringViewType</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html#associatedconstant.IS_UTF8-2" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html#associatedconstant.IS_UTF8" class="constant">IS_UTF8</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a> = true

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html#associatedconstant.PREFIX-2" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html#associatedconstant.PREFIX" class="constant">PREFIX</a>: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a> = "String"

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html#associatedtype.Native-2" class="anchor">§</a>

#### type <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html#associatedtype.Owned-2" class="anchor">§</a>

#### type <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html#associatedtype.Owned" class="associatedtype">Owned</a> = <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>
