# Trait BinaryNameSpaceImpl Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/chunked_array/binary/namespace.rs.html#17" class="src">Source</a>

``` rust
pub trait BinaryNameSpaceImpl: AsBinary {
Show 13 methods    // Provided methods
    fn contains(&self, lit: &[u8]) -> ChunkedArray<BooleanType> { ... }
    fn contains_chunked(
        &self,
        lit: &ChunkedArray<BinaryType>,
    ) -> Result<ChunkedArray<BooleanType>, PolarsError> { ... }
    fn ends_with(&self, sub: &[u8]) -> ChunkedArray<BooleanType> { ... }
    fn starts_with(&self, sub: &[u8]) -> ChunkedArray<BooleanType> { ... }
    fn starts_with_chunked(
        &self,
        prefix: &ChunkedArray<BinaryType>,
    ) -> Result<ChunkedArray<BooleanType>, PolarsError> { ... }
    fn ends_with_chunked(
        &self,
        suffix: &ChunkedArray<BinaryType>,
    ) -> Result<ChunkedArray<BooleanType>, PolarsError> { ... }
    fn size_bytes(&self) -> ChunkedArray<UInt32Type> { ... }
    fn hex_decode(
        &self,
        strict: bool,
    ) -> Result<ChunkedArray<BinaryType>, PolarsError> { ... }
    fn hex_encode(&self) -> Series { ... }
    fn base64_decode(
        &self,
        strict: bool,
    ) -> Result<ChunkedArray<BinaryType>, PolarsError> { ... }
    fn base64_encode(&self) -> Series { ... }
    fn reinterpret(
        &self,
        dtype: &DataType,
        is_little_endian: bool,
    ) -> Result<Series, PolarsError> { ... }
    fn _reinterpret_inner(
        &self,
        dtype: &DataType,
        is_little_endian: bool,
    ) -> Result<Vec<Box<dyn Array>>, PolarsError> { ... }
}
```

Available on **crate feature `polars-ops`** only.

## Provided Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.BinaryNameSpaceImpl.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.BinaryNameSpaceImpl.html#method.contains" class="fn">contains</a>(&self, lit: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

Check if binary contains given literal

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.BinaryNameSpaceImpl.html#method.contains_chunked" class="fn">contains_chunked</a>( &self, lit: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BinaryType.html" class="struct" title="struct polars::prelude::BinaryType">BinaryType</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.BinaryNameSpaceImpl.html#method.ends_with" class="fn">ends_with</a>(&self, sub: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

Check if strings ends with a substring

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.BinaryNameSpaceImpl.html#method.starts_with" class="fn">starts_with</a>(&self, sub: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

Check if strings starts with a substring

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.BinaryNameSpaceImpl.html#method.starts_with_chunked" class="fn">starts_with_chunked</a>( &self, prefix: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BinaryType.html" class="struct" title="struct polars::prelude::BinaryType">BinaryType</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.BinaryNameSpaceImpl.html#method.ends_with_chunked" class="fn">ends_with_chunked</a>( &self, suffix: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BinaryType.html" class="struct" title="struct polars::prelude::BinaryType">BinaryType</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.BinaryNameSpaceImpl.html#method.size_bytes" class="fn">size_bytes</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt32Type.html" class="struct" title="struct polars::prelude::UInt32Type">UInt32Type</a>\>

Get the size of the binary values in bytes.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.BinaryNameSpaceImpl.html#method.hex_decode" class="fn">hex_decode</a>( &self, strict: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BinaryType.html" class="struct" title="struct polars::prelude::BinaryType">BinaryType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.BinaryNameSpaceImpl.html#method.hex_encode" class="fn">hex_encode</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.BinaryNameSpaceImpl.html#method.base64_decode" class="fn">base64_decode</a>( &self, strict: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BinaryType.html" class="struct" title="struct polars::prelude::BinaryType">BinaryType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.BinaryNameSpaceImpl.html#method.base64_encode" class="fn">base64_encode</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.BinaryNameSpaceImpl.html#method.reinterpret" class="fn">reinterpret</a>( &self, dtype: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>, is_little_endian: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.BinaryNameSpaceImpl.html#method._reinterpret_inner" class="fn">_reinterpret_inner</a>( &self, dtype: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>, is_little_endian: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.BinaryNameSpaceImpl.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.BinaryNameSpaceImpl.html#impl-BinaryNameSpaceImpl-for-ChunkedArray%3CBinaryType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.BinaryNameSpaceImpl.html" class="trait" title="trait polars::prelude::BinaryNameSpaceImpl">BinaryNameSpaceImpl</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BinaryType.html" class="struct" title="struct polars::prelude::BinaryType">BinaryType</a>\>
