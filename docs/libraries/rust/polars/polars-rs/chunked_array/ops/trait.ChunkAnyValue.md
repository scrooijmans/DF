# Trait ChunkAnyValue Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/mod.rs.html#72" class="src">Source</a>

``` rust
pub trait ChunkAnyValue {
    // Required methods
    unsafe fn get_any_value_unchecked(&self, index: usize) -> AnyValue<'_>;
    fn get_any_value(&self, index: usize) -> Result<AnyValue<'_>, PolarsError>;
}
```

## Required Methods<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkAnyValue.html#required-methods" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkAnyValue.html#tymethod.get_any_value_unchecked" class="fn">get_any_value_unchecked</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>

Get a single value. Beware this is slow. If you need to use this slightly performant, cast Categorical to UInt32

##### <a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkAnyValue.html#safety" class="doc-anchor">§</a>Safety

Does not do any bounds checking.

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkAnyValue.html#tymethod.get_any_value" class="fn">get_any_value</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get a single value. Beware this is slow.

## Implementors<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkAnyValue.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkAnyValue.html#impl-ChunkAnyValue-for-ChunkedArray%3CBinaryOffsetType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkAnyValue.html" class="trait" title="trait polars::prelude::ChunkAnyValue">ChunkAnyValue</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BinaryOffsetType.html" class="struct" title="struct polars::prelude::BinaryOffsetType">BinaryOffsetType</a>\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkAnyValue.html#impl-ChunkAnyValue-for-ChunkedArray%3CBinaryType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkAnyValue.html" class="trait" title="trait polars::prelude::ChunkAnyValue">ChunkAnyValue</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BinaryType.html" class="struct" title="struct polars::prelude::BinaryType">BinaryType</a>\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkAnyValue.html#impl-ChunkAnyValue-for-ChunkedArray%3CBooleanType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkAnyValue.html" class="trait" title="trait polars::prelude::ChunkAnyValue">ChunkAnyValue</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkAnyValue.html#impl-ChunkAnyValue-for-ChunkedArray%3CFixedSizeListType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkAnyValue.html" class="trait" title="trait polars::prelude::ChunkAnyValue">ChunkAnyValue</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.FixedSizeListType.html" class="struct" title="struct polars::prelude::FixedSizeListType">FixedSizeListType</a>\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkAnyValue.html#impl-ChunkAnyValue-for-ChunkedArray%3CListType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkAnyValue.html" class="trait" title="trait polars::prelude::ChunkAnyValue">ChunkAnyValue</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListType.html" class="struct" title="struct polars::prelude::ListType">ListType</a>\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkAnyValue.html#impl-ChunkAnyValue-for-ChunkedArray%3CStringType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkAnyValue.html" class="trait" title="trait polars::prelude::ChunkAnyValue">ChunkAnyValue</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkAnyValue.html#impl-ChunkAnyValue-for-ChunkedArray%3CStructType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkAnyValue.html" class="trait" title="trait polars::prelude::ChunkAnyValue">ChunkAnyValue</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructType.html" class="struct" title="struct polars::prelude::StructType">StructType</a>\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkAnyValue.html#impl-ChunkAnyValue-for-NullChunked" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkAnyValue.html" class="trait" title="trait polars::prelude::ChunkAnyValue">ChunkAnyValue</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html" class="struct" title="struct polars::prelude::NullChunked">NullChunked</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkAnyValue.html#impl-ChunkAnyValue-for-ChunkedArray%3CObjectType%3CT%3E%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkAnyValue.html" class="trait" title="trait polars::prelude::ChunkAnyValue">ChunkAnyValue</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ObjectType.html" class="struct" title="struct polars::prelude::ObjectType">ObjectType</a>\<T\>\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsObject.html" class="trait" title="trait polars::prelude::PolarsObject">PolarsObject</a>,

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkAnyValue.html#impl-ChunkAnyValue-for-ChunkedArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkAnyValue.html" class="trait" title="trait polars::prelude::ChunkAnyValue">ChunkAnyValue</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>,
