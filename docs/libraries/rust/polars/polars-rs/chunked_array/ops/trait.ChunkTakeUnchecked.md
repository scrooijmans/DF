# Trait ChunkTakeUnchecked Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/mod.rs.html#118" class="src">Source</a>

``` rust
pub trait ChunkTakeUnchecked<Idx>where
    Idx: ?Sized,{
    // Required method
    unsafe fn take_unchecked(&self, indices: &Idx) -> Self;
}
```

## Required Methods<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkTakeUnchecked.html#required-methods" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkTakeUnchecked.html#tymethod.take_unchecked" class="fn">take_unchecked</a>(&self, indices: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Idx</a>) -\> Self

Gather values from ChunkedArray by index.

##### <a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkTakeUnchecked.html#safety" class="doc-anchor">§</a>Safety

The non-null indices must be valid.

## Dyn Compatibility<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkTakeUnchecked.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkTakeUnchecked.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkTakeUnchecked.html#impl-ChunkTakeUnchecked%3CChunkedArray%3CUInt32Type%3E%3E-for-ChunkedArray%3CBinaryType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkTakeUnchecked.html" class="trait" title="trait polars::prelude::ChunkTakeUnchecked">ChunkTakeUnchecked</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt32Type.html" class="struct" title="struct polars::prelude::UInt32Type">UInt32Type</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BinaryType.html" class="struct" title="struct polars::prelude::BinaryType">BinaryType</a>\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkTakeUnchecked.html#impl-ChunkTakeUnchecked%3CChunkedArray%3CUInt32Type%3E%3E-for-ChunkedArray%3CFixedSizeListType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkTakeUnchecked.html" class="trait" title="trait polars::prelude::ChunkTakeUnchecked">ChunkTakeUnchecked</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt32Type.html" class="struct" title="struct polars::prelude::UInt32Type">UInt32Type</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.FixedSizeListType.html" class="struct" title="struct polars::prelude::FixedSizeListType">FixedSizeListType</a>\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkTakeUnchecked.html#impl-ChunkTakeUnchecked%3CChunkedArray%3CUInt32Type%3E%3E-for-ChunkedArray%3CListType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkTakeUnchecked.html" class="trait" title="trait polars::prelude::ChunkTakeUnchecked">ChunkTakeUnchecked</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt32Type.html" class="struct" title="struct polars::prelude::UInt32Type">UInt32Type</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListType.html" class="struct" title="struct polars::prelude::ListType">ListType</a>\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkTakeUnchecked.html#impl-ChunkTakeUnchecked%3CChunkedArray%3CUInt32Type%3E%3E-for-ChunkedArray%3CStringType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkTakeUnchecked.html" class="trait" title="trait polars::prelude::ChunkTakeUnchecked">ChunkTakeUnchecked</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt32Type.html" class="struct" title="struct polars::prelude::UInt32Type">UInt32Type</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkTakeUnchecked.html#impl-ChunkTakeUnchecked%3CChunkedArray%3CUInt32Type%3E%3E-for-ChunkedArray%3CStructType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkTakeUnchecked.html" class="trait" title="trait polars::prelude::ChunkTakeUnchecked">ChunkTakeUnchecked</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt32Type.html" class="struct" title="struct polars::prelude::UInt32Type">UInt32Type</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructType.html" class="struct" title="struct polars::prelude::StructType">StructType</a>\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkTakeUnchecked.html#impl-ChunkTakeUnchecked%3CI%3E-for-ChunkedArray%3CBinaryType%3E" class="anchor">§</a>

### impl\<I\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkTakeUnchecked.html" class="trait" title="trait polars::prelude::ChunkTakeUnchecked">ChunkTakeUnchecked</a>\<I\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BinaryType.html" class="struct" title="struct polars::prelude::BinaryType">BinaryType</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\]\> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkTakeUnchecked.html#impl-ChunkTakeUnchecked%3CI%3E-for-ChunkedArray%3CFixedSizeListType%3E" class="anchor">§</a>

### impl\<I\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkTakeUnchecked.html" class="trait" title="trait polars::prelude::ChunkTakeUnchecked">ChunkTakeUnchecked</a>\<I\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.FixedSizeListType.html" class="struct" title="struct polars::prelude::FixedSizeListType">FixedSizeListType</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\]\> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkTakeUnchecked.html#impl-ChunkTakeUnchecked%3CI%3E-for-ChunkedArray%3CListType%3E" class="anchor">§</a>

### impl\<I\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkTakeUnchecked.html" class="trait" title="trait polars::prelude::ChunkTakeUnchecked">ChunkTakeUnchecked</a>\<I\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListType.html" class="struct" title="struct polars::prelude::ListType">ListType</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\]\> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkTakeUnchecked.html#impl-ChunkTakeUnchecked%3CI%3E-for-ChunkedArray%3CStringType%3E" class="anchor">§</a>

### impl\<I\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkTakeUnchecked.html" class="trait" title="trait polars::prelude::ChunkTakeUnchecked">ChunkTakeUnchecked</a>\<I\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\]\> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkTakeUnchecked.html#impl-ChunkTakeUnchecked%3CI%3E-for-ChunkedArray%3CStructType%3E" class="anchor">§</a>

### impl\<I\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkTakeUnchecked.html" class="trait" title="trait polars::prelude::ChunkTakeUnchecked">ChunkTakeUnchecked</a>\<I\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructType.html" class="struct" title="struct polars::prelude::StructType">StructType</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\]\> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkTakeUnchecked.html#impl-ChunkTakeUnchecked%3CChunkedArray%3CUInt32Type%3E%3E-for-ChunkedArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkTakeUnchecked.html" class="trait" title="trait polars::prelude::ChunkTakeUnchecked">ChunkTakeUnchecked</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt32Type.html" class="struct" title="struct polars::prelude::UInt32Type">UInt32Type</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a>\<HasViews = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>, IsStruct = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>, IsNested = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>\> + <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a>,

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkTakeUnchecked.html#impl-ChunkTakeUnchecked%3CI%3E-for-ChunkedArray%3CT%3E" class="anchor">§</a>

### impl\<T, I\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkTakeUnchecked.html" class="trait" title="trait polars::prelude::ChunkTakeUnchecked">ChunkTakeUnchecked</a>\<I\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>

where I: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\]\> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a>\<HasViews = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>, IsStruct = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>, IsNested = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>\> + <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a>,
