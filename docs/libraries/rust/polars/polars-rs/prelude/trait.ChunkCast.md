# Trait ChunkCast Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/mod.rs.html#186" class="src">Source</a>

``` rust
pub trait ChunkCast {
    // Required methods
    fn cast_with_options(
        &self,
        dtype: &DataType,
        options: CastOptions,
    ) -> Result<Series, PolarsError>;
    unsafe fn cast_unchecked(
        &self,
        dtype: &DataType,
    ) -> Result<Series, PolarsError>;

    // Provided method
    fn cast(&self, dtype: &DataType) -> Result<Series, PolarsError> { ... }
}
```

Expand description

Cast `ChunkedArray<T>` to `ChunkedArray<N>`

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCast.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCast.html#tymethod.cast_with_options" class="fn">cast_with_options</a>( &self, dtype: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>, options: <a href="https://docs.rs/polars/latest/polars/chunked_array/cast/enum.CastOptions.html" class="enum" title="enum polars::chunked_array::cast::CastOptions">CastOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Cast a [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") to [`DataType`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html "enum polars::prelude::DataType")

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCast.html#tymethod.cast_unchecked" class="fn">cast_unchecked</a>(&self, dtype: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Does not check if the cast is a valid one and may over/underflow

##### <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCast.html#safety" class="doc-anchor">§</a>Safety

- This doesn’t do utf8 validation checking when casting from binary
- This doesn’t do categorical bound checking when casting from UInt32

## Provided Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCast.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCast.html#method.cast" class="fn">cast</a>(&self, dtype: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Cast a [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") to [`DataType`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html "enum polars::prelude::DataType")

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCast.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCast.html#impl-ChunkCast-for-ChunkedArray%3CBinaryOffsetType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCast.html" class="trait" title="trait polars::prelude::ChunkCast">ChunkCast</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BinaryOffsetType.html" class="struct" title="struct polars::prelude::BinaryOffsetType">BinaryOffsetType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCast.html#impl-ChunkCast-for-ChunkedArray%3CBinaryType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCast.html" class="trait" title="trait polars::prelude::ChunkCast">ChunkCast</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BinaryType.html" class="struct" title="struct polars::prelude::BinaryType">BinaryType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCast.html#impl-ChunkCast-for-ChunkedArray%3CBooleanType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCast.html" class="trait" title="trait polars::prelude::ChunkCast">ChunkCast</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCast.html#impl-ChunkCast-for-ChunkedArray%3CFixedSizeListType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCast.html" class="trait" title="trait polars::prelude::ChunkCast">ChunkCast</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.FixedSizeListType.html" class="struct" title="struct polars::prelude::FixedSizeListType">FixedSizeListType</a>\>

We cannot cast anything to or from List/LargeList So this implementation casts the inner type

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCast.html#impl-ChunkCast-for-ChunkedArray%3CListType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCast.html" class="trait" title="trait polars::prelude::ChunkCast">ChunkCast</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListType.html" class="struct" title="struct polars::prelude::ListType">ListType</a>\>

We cannot cast anything to or from List/LargeList So this implementation casts the inner type

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCast.html#impl-ChunkCast-for-ChunkedArray%3CStringType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCast.html" class="trait" title="trait polars::prelude::ChunkCast">ChunkCast</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCast.html#impl-ChunkCast-for-ChunkedArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCast.html" class="trait" title="trait polars::prelude::ChunkCast">ChunkCast</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>,
