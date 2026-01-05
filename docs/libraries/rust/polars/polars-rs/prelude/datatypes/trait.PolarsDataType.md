# Trait PolarsDataType Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/datatypes/mod.rs.html#77" class="src">Source</a>

``` rust
pub unsafe trait PolarsDataType:
    Sized
    + Send
    + Sync
    + 'static {
    type Physical<'a>: Debug + Clone;
    type OwnedPhysical: Debug + Send + Sync + Clone + PartialEq;
    type ZeroablePhysical<'a>: Zeroable + From<Self::Physical<'a>>;
    type Array: for<'a> StaticArray<ValueT<'a> = Self::Physical<'a>, ZeroableValueT<'a> = Self::ZeroablePhysical<'a>>;
    type IsNested;
    type HasViews;
    type IsStruct;
    type IsObject;

    // Required method
    fn get_static_dtype() -> DataType
       where Self: Sized;
}
```

Expand description

## <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#safety" class="doc-anchor">§</a>Safety

The StaticArray and dtype return must be correct.

## Required Associated Types<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#required-associated-types" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical" class="associatedtype">Physical</a>\<'a\>: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> + <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical" class="associatedtype">OwnedPhysical</a>: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> + <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical" class="associatedtype">ZeroablePhysical</a>\<'a\>: <a href="https://docs.rs/bytemuck/1.23.1/x86_64-unknown-linux-gnu/bytemuck/zeroable/trait.Zeroable.html" class="trait" title="trait bytemuck::zeroable::Zeroable">Zeroable</a> + <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html#associatedtype.Physical" class="associatedtype" title="type polars::prelude::PolarsDataType::Physical">Physical</a>\<'a\>\>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array" class="associatedtype">Array</a>: for\<'a\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html" class="trait" title="trait polars::prelude::StaticArray">StaticArray</a>\<ValueT\<'a\> = Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html#associatedtype.Physical" class="associatedtype" title="type polars::prelude::PolarsDataType::Physical">Physical</a>\<'a\>, ZeroableValueT\<'a\> = Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html#associatedtype.ZeroablePhysical" class="associatedtype" title="type polars::prelude::PolarsDataType::ZeroablePhysical">ZeroablePhysical</a>\<'a\>\>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested" class="associatedtype">IsNested</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews" class="associatedtype">HasViews</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct" class="associatedtype">IsStruct</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject" class="associatedtype">IsObject</a>

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#tymethod.get_static_dtype" class="fn">get_static_dtype</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Returns the DataType variant associated with this PolarsDataType. Not implemented for types whose DataTypes have parameters.

## Dyn Compatibility<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#impl-PolarsDataType-for-BinaryOffsetType" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.BinaryOffsetType.html" class="struct" title="struct polars::prelude::BinaryOffsetType">BinaryOffsetType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical-1" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical" class="associatedtype">Physical</a>\<'a\> = &'a \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical-1" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical" class="associatedtype">OwnedPhysical</a> = <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical-1" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical" class="associatedtype">ZeroablePhysical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array-1" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array" class="associatedtype">Array</a> = <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/binary/struct.BinaryArray.html" class="struct" title="struct polars_arrow::array::binary::BinaryArray">BinaryArray</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested-1" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested" class="associatedtype">IsNested</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews-1" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews" class="associatedtype">HasViews</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct-1" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct" class="associatedtype">IsStruct</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject-1" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject" class="associatedtype">IsObject</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#impl-PolarsDataType-for-BinaryType" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.BinaryType.html" class="struct" title="struct polars::prelude::BinaryType">BinaryType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical-2" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical" class="associatedtype">Physical</a>\<'a\> = &'a \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical-2" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical" class="associatedtype">OwnedPhysical</a> = <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical-2" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical" class="associatedtype">ZeroablePhysical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array-2" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array" class="associatedtype">Array</a> = <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/binview/struct.BinaryViewArrayGeneric.html" class="struct" title="struct polars_arrow::array::binview::BinaryViewArrayGeneric">BinaryViewArrayGeneric</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested-2" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested" class="associatedtype">IsNested</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews-2" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews" class="associatedtype">HasViews</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.TrueT.html" class="struct" title="struct polars::prelude::TrueT">TrueT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct-2" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct" class="associatedtype">IsStruct</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject-2" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject" class="associatedtype">IsObject</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#impl-PolarsDataType-for-BooleanType" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical-3" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical" class="associatedtype">Physical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical-3" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical" class="associatedtype">OwnedPhysical</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical-3" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical" class="associatedtype">ZeroablePhysical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array-3" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array" class="associatedtype">Array</a> = <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/boolean/struct.BooleanArray.html" class="struct" title="struct polars_arrow::array::boolean::BooleanArray">BooleanArray</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested-3" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested" class="associatedtype">IsNested</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews-3" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews" class="associatedtype">HasViews</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct-3" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct" class="associatedtype">IsStruct</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject-3" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject" class="associatedtype">IsObject</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#impl-PolarsDataType-for-Categorical8Type" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Categorical8Type.html" class="struct" title="struct polars::prelude::Categorical8Type">Categorical8Type</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical-4" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical" class="associatedtype">Physical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical-4" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical" class="associatedtype">OwnedPhysical</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical-4" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical" class="associatedtype">ZeroablePhysical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array-4" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array" class="associatedtype">Array</a> = <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/primitive/struct.PrimitiveArray.html" class="struct" title="struct polars_arrow::array::primitive::PrimitiveArray">PrimitiveArray</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested-4" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested" class="associatedtype">IsNested</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews-4" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews" class="associatedtype">HasViews</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct-4" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct" class="associatedtype">IsStruct</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject-4" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject" class="associatedtype">IsObject</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#impl-PolarsDataType-for-Categorical16Type" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Categorical16Type.html" class="struct" title="struct polars::prelude::Categorical16Type">Categorical16Type</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical-5" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical" class="associatedtype">Physical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical-5" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical" class="associatedtype">OwnedPhysical</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical-5" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical" class="associatedtype">ZeroablePhysical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array-5" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array" class="associatedtype">Array</a> = <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/primitive/struct.PrimitiveArray.html" class="struct" title="struct polars_arrow::array::primitive::PrimitiveArray">PrimitiveArray</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested-5" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested" class="associatedtype">IsNested</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews-5" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews" class="associatedtype">HasViews</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct-5" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct" class="associatedtype">IsStruct</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject-5" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject" class="associatedtype">IsObject</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#impl-PolarsDataType-for-Categorical32Type" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Categorical32Type.html" class="struct" title="struct polars::prelude::Categorical32Type">Categorical32Type</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical-6" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical" class="associatedtype">Physical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical-6" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical" class="associatedtype">OwnedPhysical</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical-6" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical" class="associatedtype">ZeroablePhysical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array-6" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array" class="associatedtype">Array</a> = <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/primitive/struct.PrimitiveArray.html" class="struct" title="struct polars_arrow::array::primitive::PrimitiveArray">PrimitiveArray</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested-6" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested" class="associatedtype">IsNested</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews-6" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews" class="associatedtype">HasViews</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct-6" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct" class="associatedtype">IsStruct</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject-6" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject" class="associatedtype">IsObject</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#impl-PolarsDataType-for-CategoricalType" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.CategoricalType.html" class="struct" title="struct polars::prelude::CategoricalType">CategoricalType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical-7" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical" class="associatedtype">Physical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical-7" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical" class="associatedtype">OwnedPhysical</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical-7" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical" class="associatedtype">ZeroablePhysical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array-7" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array" class="associatedtype">Array</a> = <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/primitive/struct.PrimitiveArray.html" class="struct" title="struct polars_arrow::array::primitive::PrimitiveArray">PrimitiveArray</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested-7" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested" class="associatedtype">IsNested</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews-7" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews" class="associatedtype">HasViews</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct-7" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct" class="associatedtype">IsStruct</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject-7" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject" class="associatedtype">IsObject</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#impl-PolarsDataType-for-DateType" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DateType.html" class="struct" title="struct polars::prelude::DateType">DateType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical-8" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical" class="associatedtype">Physical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical-8" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical" class="associatedtype">OwnedPhysical</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical-8" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical" class="associatedtype">ZeroablePhysical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array-8" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array" class="associatedtype">Array</a> = <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/primitive/struct.PrimitiveArray.html" class="struct" title="struct polars_arrow::array::primitive::PrimitiveArray">PrimitiveArray</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested-8" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested" class="associatedtype">IsNested</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews-8" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews" class="associatedtype">HasViews</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct-8" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct" class="associatedtype">IsStruct</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject-8" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject" class="associatedtype">IsObject</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#impl-PolarsDataType-for-DatetimeType" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeType.html" class="struct" title="struct polars::prelude::DatetimeType">DatetimeType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical-9" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical" class="associatedtype">Physical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical-9" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical" class="associatedtype">OwnedPhysical</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical-9" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical" class="associatedtype">ZeroablePhysical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array-9" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array" class="associatedtype">Array</a> = <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/primitive/struct.PrimitiveArray.html" class="struct" title="struct polars_arrow::array::primitive::PrimitiveArray">PrimitiveArray</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested-9" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested" class="associatedtype">IsNested</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews-9" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews" class="associatedtype">HasViews</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct-9" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct" class="associatedtype">IsStruct</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject-9" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject" class="associatedtype">IsObject</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#impl-PolarsDataType-for-DecimalType" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DecimalType.html" class="struct" title="struct polars::prelude::DecimalType">DecimalType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical-10" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical" class="associatedtype">Physical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical-10" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical" class="associatedtype">OwnedPhysical</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical-10" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical" class="associatedtype">ZeroablePhysical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array-10" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array" class="associatedtype">Array</a> = <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/primitive/struct.PrimitiveArray.html" class="struct" title="struct polars_arrow::array::primitive::PrimitiveArray">PrimitiveArray</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested-10" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested" class="associatedtype">IsNested</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews-10" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews" class="associatedtype">HasViews</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct-10" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct" class="associatedtype">IsStruct</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject-10" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject" class="associatedtype">IsObject</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#impl-PolarsDataType-for-DurationType" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationType.html" class="struct" title="struct polars::prelude::DurationType">DurationType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical-11" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical" class="associatedtype">Physical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical-11" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical" class="associatedtype">OwnedPhysical</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical-11" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical" class="associatedtype">ZeroablePhysical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array-11" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array" class="associatedtype">Array</a> = <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/primitive/struct.PrimitiveArray.html" class="struct" title="struct polars_arrow::array::primitive::PrimitiveArray">PrimitiveArray</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested-11" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested" class="associatedtype">IsNested</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews-11" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews" class="associatedtype">HasViews</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct-11" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct" class="associatedtype">IsStruct</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject-11" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject" class="associatedtype">IsObject</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#impl-PolarsDataType-for-FixedSizeListType" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.FixedSizeListType.html" class="struct" title="struct polars::prelude::FixedSizeListType">FixedSizeListType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical-12" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical" class="associatedtype">Physical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical-12" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical" class="associatedtype">OwnedPhysical</a> = <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical-12" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical" class="associatedtype">ZeroablePhysical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array-12" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array" class="associatedtype">Array</a> = <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/fixed_size_list/struct.FixedSizeListArray.html" class="struct" title="struct polars_arrow::array::fixed_size_list::FixedSizeListArray">FixedSizeListArray</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested-12" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested" class="associatedtype">IsNested</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.TrueT.html" class="struct" title="struct polars::prelude::TrueT">TrueT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews-12" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews" class="associatedtype">HasViews</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct-12" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct" class="associatedtype">IsStruct</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject-12" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject" class="associatedtype">IsObject</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#impl-PolarsDataType-for-Float32Type" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Float32Type.html" class="struct" title="struct polars::prelude::Float32Type">Float32Type</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical-13" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical" class="associatedtype">Physical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical-13" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical" class="associatedtype">OwnedPhysical</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical-13" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical" class="associatedtype">ZeroablePhysical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array-13" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array" class="associatedtype">Array</a> = <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/primitive/struct.PrimitiveArray.html" class="struct" title="struct polars_arrow::array::primitive::PrimitiveArray">PrimitiveArray</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested-13" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested" class="associatedtype">IsNested</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews-13" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews" class="associatedtype">HasViews</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct-13" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct" class="associatedtype">IsStruct</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject-13" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject" class="associatedtype">IsObject</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#impl-PolarsDataType-for-Float64Type" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Float64Type.html" class="struct" title="struct polars::prelude::Float64Type">Float64Type</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical-14" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical" class="associatedtype">Physical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical-14" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical" class="associatedtype">OwnedPhysical</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical-14" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical" class="associatedtype">ZeroablePhysical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array-14" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array" class="associatedtype">Array</a> = <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/primitive/struct.PrimitiveArray.html" class="struct" title="struct polars_arrow::array::primitive::PrimitiveArray">PrimitiveArray</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested-14" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested" class="associatedtype">IsNested</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews-14" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews" class="associatedtype">HasViews</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct-14" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct" class="associatedtype">IsStruct</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject-14" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject" class="associatedtype">IsObject</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#impl-PolarsDataType-for-Int8Type" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical-15" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical" class="associatedtype">Physical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical-15" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical" class="associatedtype">OwnedPhysical</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical-15" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical" class="associatedtype">ZeroablePhysical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array-15" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array" class="associatedtype">Array</a> = <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/primitive/struct.PrimitiveArray.html" class="struct" title="struct polars_arrow::array::primitive::PrimitiveArray">PrimitiveArray</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested-15" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested" class="associatedtype">IsNested</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews-15" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews" class="associatedtype">HasViews</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct-15" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct" class="associatedtype">IsStruct</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject-15" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject" class="associatedtype">IsObject</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#impl-PolarsDataType-for-Int16Type" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int16Type.html" class="struct" title="struct polars::prelude::Int16Type">Int16Type</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical-16" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical" class="associatedtype">Physical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical-16" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical" class="associatedtype">OwnedPhysical</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical-16" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical" class="associatedtype">ZeroablePhysical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array-16" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array" class="associatedtype">Array</a> = <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/primitive/struct.PrimitiveArray.html" class="struct" title="struct polars_arrow::array::primitive::PrimitiveArray">PrimitiveArray</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested-16" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested" class="associatedtype">IsNested</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews-16" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews" class="associatedtype">HasViews</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct-16" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct" class="associatedtype">IsStruct</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject-16" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject" class="associatedtype">IsObject</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#impl-PolarsDataType-for-Int32Type" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical-17" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical" class="associatedtype">Physical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical-17" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical" class="associatedtype">OwnedPhysical</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical-17" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical" class="associatedtype">ZeroablePhysical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array-17" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array" class="associatedtype">Array</a> = <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/primitive/struct.PrimitiveArray.html" class="struct" title="struct polars_arrow::array::primitive::PrimitiveArray">PrimitiveArray</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested-17" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested" class="associatedtype">IsNested</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews-17" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews" class="associatedtype">HasViews</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct-17" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct" class="associatedtype">IsStruct</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject-17" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject" class="associatedtype">IsObject</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#impl-PolarsDataType-for-Int64Type" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical-18" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical" class="associatedtype">Physical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical-18" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical" class="associatedtype">OwnedPhysical</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical-18" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical" class="associatedtype">ZeroablePhysical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array-18" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array" class="associatedtype">Array</a> = <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/primitive/struct.PrimitiveArray.html" class="struct" title="struct polars_arrow::array::primitive::PrimitiveArray">PrimitiveArray</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested-18" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested" class="associatedtype">IsNested</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews-18" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews" class="associatedtype">HasViews</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct-18" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct" class="associatedtype">IsStruct</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject-18" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject" class="associatedtype">IsObject</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#impl-PolarsDataType-for-Int128Type" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int128Type.html" class="struct" title="struct polars::prelude::Int128Type">Int128Type</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical-19" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical" class="associatedtype">Physical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical-19" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical" class="associatedtype">OwnedPhysical</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical-19" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical" class="associatedtype">ZeroablePhysical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array-19" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array" class="associatedtype">Array</a> = <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/primitive/struct.PrimitiveArray.html" class="struct" title="struct polars_arrow::array::primitive::PrimitiveArray">PrimitiveArray</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested-19" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested" class="associatedtype">IsNested</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews-19" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews" class="associatedtype">HasViews</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct-19" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct" class="associatedtype">IsStruct</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject-19" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject" class="associatedtype">IsObject</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#impl-PolarsDataType-for-ListType" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ListType.html" class="struct" title="struct polars::prelude::ListType">ListType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical-20" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical" class="associatedtype">Physical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical-20" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical" class="associatedtype">OwnedPhysical</a> = <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical-20" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical" class="associatedtype">ZeroablePhysical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array-20" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array" class="associatedtype">Array</a> = <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/list/struct.ListArray.html" class="struct" title="struct polars_arrow::array::list::ListArray">ListArray</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested-20" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested" class="associatedtype">IsNested</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.TrueT.html" class="struct" title="struct polars::prelude::TrueT">TrueT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews-20" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews" class="associatedtype">HasViews</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct-20" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct" class="associatedtype">IsStruct</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject-20" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject" class="associatedtype">IsObject</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#impl-PolarsDataType-for-StringType" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical-21" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical" class="associatedtype">Physical</a>\<'a\> = &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical-21" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical" class="associatedtype">OwnedPhysical</a> = <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical-21" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical" class="associatedtype">ZeroablePhysical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array-21" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array" class="associatedtype">Array</a> = <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/binview/struct.BinaryViewArrayGeneric.html" class="struct" title="struct polars_arrow::array::binview::BinaryViewArrayGeneric">BinaryViewArrayGeneric</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested-21" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested" class="associatedtype">IsNested</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews-21" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews" class="associatedtype">HasViews</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.TrueT.html" class="struct" title="struct polars::prelude::TrueT">TrueT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct-21" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct" class="associatedtype">IsStruct</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject-21" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject" class="associatedtype">IsObject</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#impl-PolarsDataType-for-StructType" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructType.html" class="struct" title="struct polars::prelude::StructType">StructType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical-22" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical" class="associatedtype">Physical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical-22" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical" class="associatedtype">OwnedPhysical</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical-22" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical" class="associatedtype">ZeroablePhysical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array-22" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array" class="associatedtype">Array</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested-22" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested" class="associatedtype">IsNested</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.TrueT.html" class="struct" title="struct polars::prelude::TrueT">TrueT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews-22" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews" class="associatedtype">HasViews</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct-22" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct" class="associatedtype">IsStruct</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.TrueT.html" class="struct" title="struct polars::prelude::TrueT">TrueT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject-22" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject" class="associatedtype">IsObject</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#impl-PolarsDataType-for-TimeType" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeType.html" class="struct" title="struct polars::prelude::TimeType">TimeType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical-23" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical" class="associatedtype">Physical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical-23" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical" class="associatedtype">OwnedPhysical</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical-23" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical" class="associatedtype">ZeroablePhysical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array-23" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array" class="associatedtype">Array</a> = <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/primitive/struct.PrimitiveArray.html" class="struct" title="struct polars_arrow::array::primitive::PrimitiveArray">PrimitiveArray</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested-23" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested" class="associatedtype">IsNested</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews-23" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews" class="associatedtype">HasViews</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct-23" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct" class="associatedtype">IsStruct</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject-23" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject" class="associatedtype">IsObject</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#impl-PolarsDataType-for-UInt8Type" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt8Type.html" class="struct" title="struct polars::prelude::UInt8Type">UInt8Type</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical-24" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical" class="associatedtype">Physical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical-24" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical" class="associatedtype">OwnedPhysical</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical-24" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical" class="associatedtype">ZeroablePhysical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array-24" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array" class="associatedtype">Array</a> = <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/primitive/struct.PrimitiveArray.html" class="struct" title="struct polars_arrow::array::primitive::PrimitiveArray">PrimitiveArray</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested-24" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested" class="associatedtype">IsNested</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews-24" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews" class="associatedtype">HasViews</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct-24" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct" class="associatedtype">IsStruct</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject-24" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject" class="associatedtype">IsObject</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#impl-PolarsDataType-for-UInt16Type" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt16Type.html" class="struct" title="struct polars::prelude::UInt16Type">UInt16Type</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical-25" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical" class="associatedtype">Physical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical-25" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical" class="associatedtype">OwnedPhysical</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical-25" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical" class="associatedtype">ZeroablePhysical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array-25" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array" class="associatedtype">Array</a> = <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/primitive/struct.PrimitiveArray.html" class="struct" title="struct polars_arrow::array::primitive::PrimitiveArray">PrimitiveArray</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested-25" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested" class="associatedtype">IsNested</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews-25" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews" class="associatedtype">HasViews</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct-25" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct" class="associatedtype">IsStruct</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject-25" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject" class="associatedtype">IsObject</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#impl-PolarsDataType-for-UInt32Type" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt32Type.html" class="struct" title="struct polars::prelude::UInt32Type">UInt32Type</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical-26" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical" class="associatedtype">Physical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical-26" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical" class="associatedtype">OwnedPhysical</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical-26" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical" class="associatedtype">ZeroablePhysical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array-26" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array" class="associatedtype">Array</a> = <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/primitive/struct.PrimitiveArray.html" class="struct" title="struct polars_arrow::array::primitive::PrimitiveArray">PrimitiveArray</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested-26" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested" class="associatedtype">IsNested</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews-26" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews" class="associatedtype">HasViews</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct-26" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct" class="associatedtype">IsStruct</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject-26" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject" class="associatedtype">IsObject</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#impl-PolarsDataType-for-UInt64Type" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt64Type.html" class="struct" title="struct polars::prelude::UInt64Type">UInt64Type</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical-27" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical" class="associatedtype">Physical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical-27" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical" class="associatedtype">OwnedPhysical</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical-27" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical" class="associatedtype">ZeroablePhysical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array-27" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array" class="associatedtype">Array</a> = <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/primitive/struct.PrimitiveArray.html" class="struct" title="struct polars_arrow::array::primitive::PrimitiveArray">PrimitiveArray</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested-27" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested" class="associatedtype">IsNested</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews-27" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews" class="associatedtype">HasViews</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct-27" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct" class="associatedtype">IsStruct</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject-27" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject" class="associatedtype">IsObject</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#impl-PolarsDataType-for-ObjectType%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ObjectType.html" class="struct" title="struct polars::prelude::ObjectType">ObjectType</a>\<T\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsObject.html" class="trait" title="trait polars::prelude::PolarsObject">PolarsObject</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical-28" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Physical" class="associatedtype">Physical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical-28" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.OwnedPhysical" class="associatedtype">OwnedPhysical</a> = T

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical-28" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.ZeroablePhysical" class="associatedtype">ZeroablePhysical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array-28" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.Array" class="associatedtype">Array</a> = <a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html" class="struct" title="struct polars::chunked_array::object::ObjectArray">ObjectArray</a>\<T\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested-28" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsNested" class="associatedtype">IsNested</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews-28" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.HasViews" class="associatedtype">HasViews</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct-28" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsStruct" class="associatedtype">IsStruct</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject-28" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html#associatedtype.IsObject" class="associatedtype">IsObject</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.TrueT.html" class="struct" title="struct polars::prelude::TrueT">TrueT</a>
