# Struct Buffer Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/types/buffer.rs.html#118" class="src">Source</a>

``` rust
pub struct Buffer(/* private fields */);
```

Expand description

Buffer is a wrapper of contiguous `Bytes` and non-contiguous `[Bytes]`.

We designed buffer to allow underlying storage to return non-contiguous bytes. For example, http based storage like s3 could generate non-contiguous bytes by stream.

### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#features" class="doc-anchor">Â§</a>Features

- [`Buffer`](https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html "struct opendal::Buffer") can be used as \[`Buf`\], [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator"), \[`Stream`\] directly.
- [`Buffer`](https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html "struct opendal::Buffer") is cheap to clone like \[`Bytes`\], only update reference count, no allocation.
- [`Buffer`](https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html "struct opendal::Buffer") is vectorized write friendly, you can convert it to [`IoSlice`](https://doc.rust-lang.org/nightly/std/io/struct.IoSlice.html "struct std::io::IoSlice") for vectored write.

### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#examples" class="doc-anchor">Â§</a>Examples

#### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#as-buf" class="doc-anchor">Â§</a>As `Buf`

`Buffer` implements `Buf` trait:

``` rust
use bytes::Buf;
use opendal::Buffer;
use serde_json;

fn test(mut buf: Buffer) -> Vec<String> {
    serde_json::from_reader(buf.reader()).unwrap()
}
```

#### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#as-bytes-iterator" class="doc-anchor">Â§</a>As Bytes `Iterator`

`Buffer` implements `Iterator<Item=Bytes>` trait:

``` rust
use bytes::Bytes;
use opendal::Buffer;

fn test(mut buf: Buffer) -> Vec<Bytes> {
    buf.into_iter().collect()
}
```

#### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#as-bytes-stream" class="doc-anchor">Â§</a>As Bytes `Stream`

`Buffer` implements `Stream<Item=Result<Bytes, Infallible>>` trait:

``` rust
use bytes::Bytes;
use futures::TryStreamExt;
use opendal::Buffer;

async fn test(mut buf: Buffer) -> Vec<Bytes> {
    buf.into_iter().try_collect().await.unwrap()
}
```

#### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#as-one-contiguous-bytes" class="doc-anchor">Â§</a>As one contiguous Bytes

`Buffer` can make contiguous by transform into `Bytes` or `Vec<u8>`. Please keep in mind that this operation involves new allocation and bytes copy, and we canâ€™t reuse the same memory region anymore.

``` rust
use bytes::Bytes;
use opendal::Buffer;

fn test_to_vec(buf: Buffer) -> Vec<u8> {
    buf.to_vec()
}

fn test_to_bytes(buf: Buffer) -> Bytes {
    buf.to_bytes()
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#impl-Buffer" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>

#### pub const fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.new" class="fn">new</a>() -\> Self

Create a new empty buffer.

This operation is const and no allocation will be performed.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Get the length of the buffer.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Check if buffer is empty.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.count" class="fn">count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Number of \[`Bytes`\] in [`Buffer`](https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html "struct opendal::Buffer").

For contiguous buffer, itâ€™s always 1. For non-contiguous buffer, itâ€™s number of bytes available for use.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.current" class="fn">current</a>(&self) -\> Bytes

Get current \[`Bytes`\].

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.truncate" class="fn">truncate</a>(&mut self, len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Shortens the buffer, keeping the first `len` bytes and dropping the rest.

If `len` is greater than the bufferâ€™s current length, this has no effect.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.slice" class="fn">slice</a>(&self, range: impl <a href="https://doc.rust-lang.org/nightly/core/ops/range/trait.RangeBounds.html" class="trait" title="trait core::ops::range::RangeBounds">RangeBounds</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>) -\> Self

Returns a slice of self for the provided range.

This will increment the reference count for the underlying memory and return a new Buffer handle set to the slice.

This operation is O(1).

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.to_bytes" class="fn">to_bytes</a>(&self) -\> Bytes

Combine all bytes together into one single \[`Bytes`\].

This operation is zero copy if the underlying bytes are contiguous. Otherwise, it will copy all bytes into one single \[`Bytes`\]. Please use API from \[`Buf`\], [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") or \[`Stream`\] whenever possible.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.to_vec" class="fn">to_vec</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#" class="tooltip" data-notable-ty="Vec&lt;u8&gt;">â“˜</a>

Combine all bytes together into one single [`Vec<u8>`](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html "struct alloc::vec::Vec").

This operation is not zero copy, it will copy all bytes into one single [`Vec<u8>`](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html "struct alloc::vec::Vec"). Please use API from \[`Buf`\], [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") or \[`Stream`\] whenever possible.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.to_io_slice" class="fn">to_io_slice</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/io/struct.IoSlice.html" class="struct" title="struct std::io::IoSlice">IoSlice</a>\<'\_\>\>

Convert buffer into a slice of [`IoSlice`](https://doc.rust-lang.org/nightly/std/io/struct.IoSlice.html "struct std::io::IoSlice") for vectored write.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.chunks" class="fn">chunks</a>(&self, chunk_size: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> BufferChunks

Split the buffer into an iterator of chunks, each with at most `chunk_size` bytes.

The chunks share the same underlying storage with the original buffer. The last chunk will be shorter if `self.len()` is not a multiple of `chunk_size`.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#panics" class="doc-anchor">Â§</a>Panics

Panics if `chunk_size` is zero.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#impl-Buf-for-Buffer" class="anchor">Â§</a>

### impl Buf for <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.remaining" class="anchor">Â§</a>

#### fn remaining(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the number of bytes between the current position and the end of the buffer. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.chunk" class="anchor">Â§</a>

#### fn chunk(&self) -\> &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\] <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#" class="tooltip" data-notable-ty="&amp;[u8]">â“˜</a>

Returns a slice starting at the current position and of length between 0 and `Buf::remaining()`. Note that this *can* return a shorter slice (this allows non-continuous internal representation). Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.chunks_vectored" class="anchor">Â§</a>

#### fn chunks_vectored\<'a\>(&'a self, dst: &mut \[<a href="https://doc.rust-lang.org/nightly/std/io/struct.IoSlice.html" class="struct" title="struct std::io::IoSlice">IoSlice</a>\<'a\>\]) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Fills `dst` with potentially multiple slices starting at `self`â€™s current position. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.advance" class="anchor">Â§</a>

#### fn advance(&mut self, cnt: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Advance the internal cursor of the Buf Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.has_remaining" class="anchor">Â§</a>

#### fn has_remaining(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if there are any more bytes to consume Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.copy_to_slice" class="anchor">Â§</a>

#### fn copy_to_slice(&mut self, dst: &mut \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\])

Copies bytes from `self` into `dst`. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.get_u8" class="anchor">Â§</a>

#### fn get_u8(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

Gets an unsigned 8 bit integer from `self`. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.get_i8" class="anchor">Â§</a>

#### fn get_i8(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>

Gets a signed 8 bit integer from `self`. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.get_u16" class="anchor">Â§</a>

#### fn get_u16(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

Gets an unsigned 16 bit integer from `self` in big-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.get_u16_le" class="anchor">Â§</a>

#### fn get_u16_le(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

Gets an unsigned 16 bit integer from `self` in little-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.get_u16_ne" class="anchor">Â§</a>

#### fn get_u16_ne(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

Gets an unsigned 16 bit integer from `self` in native-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.get_i16" class="anchor">Â§</a>

#### fn get_i16(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>

Gets a signed 16 bit integer from `self` in big-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.get_i16_le" class="anchor">Â§</a>

#### fn get_i16_le(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>

Gets a signed 16 bit integer from `self` in little-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.get_i16_ne" class="anchor">Â§</a>

#### fn get_i16_ne(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>

Gets a signed 16 bit integer from `self` in native-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.get_u32" class="anchor">Â§</a>

#### fn get_u32(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

Gets an unsigned 32 bit integer from `self` in the big-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.get_u32_le" class="anchor">Â§</a>

#### fn get_u32_le(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

Gets an unsigned 32 bit integer from `self` in the little-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.get_u32_ne" class="anchor">Â§</a>

#### fn get_u32_ne(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

Gets an unsigned 32 bit integer from `self` in native-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.get_i32" class="anchor">Â§</a>

#### fn get_i32(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

Gets a signed 32 bit integer from `self` in big-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.get_i32_le" class="anchor">Â§</a>

#### fn get_i32_le(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

Gets a signed 32 bit integer from `self` in little-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.get_i32_ne" class="anchor">Â§</a>

#### fn get_i32_ne(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

Gets a signed 32 bit integer from `self` in native-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.get_u64" class="anchor">Â§</a>

#### fn get_u64(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

Gets an unsigned 64 bit integer from `self` in big-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.get_u64_le" class="anchor">Â§</a>

#### fn get_u64_le(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

Gets an unsigned 64 bit integer from `self` in little-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.get_u64_ne" class="anchor">Â§</a>

#### fn get_u64_ne(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

Gets an unsigned 64 bit integer from `self` in native-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.get_i64" class="anchor">Â§</a>

#### fn get_i64(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

Gets a signed 64 bit integer from `self` in big-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.get_i64_le" class="anchor">Â§</a>

#### fn get_i64_le(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

Gets a signed 64 bit integer from `self` in little-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.get_i64_ne" class="anchor">Â§</a>

#### fn get_i64_ne(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

Gets a signed 64 bit integer from `self` in native-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.get_u128" class="anchor">Â§</a>

#### fn get_u128(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>

Gets an unsigned 128 bit integer from `self` in big-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.get_u128_le" class="anchor">Â§</a>

#### fn get_u128_le(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>

Gets an unsigned 128 bit integer from `self` in little-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.get_u128_ne" class="anchor">Â§</a>

#### fn get_u128_ne(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>

Gets an unsigned 128 bit integer from `self` in native-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.get_i128" class="anchor">Â§</a>

#### fn get_i128(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>

Gets a signed 128 bit integer from `self` in big-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.get_i128_le" class="anchor">Â§</a>

#### fn get_i128_le(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>

Gets a signed 128 bit integer from `self` in little-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.get_i128_ne" class="anchor">Â§</a>

#### fn get_i128_ne(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>

Gets a signed 128 bit integer from `self` in native-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.get_uint" class="anchor">Â§</a>

#### fn get_uint(&mut self, nbytes: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

Gets an unsigned n-byte integer from `self` in big-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.get_uint_le" class="anchor">Â§</a>

#### fn get_uint_le(&mut self, nbytes: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

Gets an unsigned n-byte integer from `self` in little-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.get_uint_ne" class="anchor">Â§</a>

#### fn get_uint_ne(&mut self, nbytes: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

Gets an unsigned n-byte integer from `self` in native-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.get_int" class="anchor">Â§</a>

#### fn get_int(&mut self, nbytes: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

Gets a signed n-byte integer from `self` in big-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.get_int_le" class="anchor">Â§</a>

#### fn get_int_le(&mut self, nbytes: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

Gets a signed n-byte integer from `self` in little-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.get_int_ne" class="anchor">Â§</a>

#### fn get_int_ne(&mut self, nbytes: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

Gets a signed n-byte integer from `self` in native-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.get_f32" class="anchor">Â§</a>

#### fn get_f32(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>

Gets an IEEE754 single-precision (4 bytes) floating point number from `self` in big-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.get_f32_le" class="anchor">Â§</a>

#### fn get_f32_le(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>

Gets an IEEE754 single-precision (4 bytes) floating point number from `self` in little-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.get_f32_ne" class="anchor">Â§</a>

#### fn get_f32_ne(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>

Gets an IEEE754 single-precision (4 bytes) floating point number from `self` in native-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.get_f64" class="anchor">Â§</a>

#### fn get_f64(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>

Gets an IEEE754 double-precision (8 bytes) floating point number from `self` in big-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.get_f64_le" class="anchor">Â§</a>

#### fn get_f64_le(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>

Gets an IEEE754 double-precision (8 bytes) floating point number from `self` in little-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.get_f64_ne" class="anchor">Â§</a>

#### fn get_f64_ne(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>

Gets an IEEE754 double-precision (8 bytes) floating point number from `self` in native-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.try_copy_to_slice" class="anchor">Â§</a>

#### fn try_copy_to_slice(&mut self, dst: &mut \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, TryGetError\>

Copies bytes from `self` into `dst`. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.try_get_u8" class="anchor">Â§</a>

#### fn try_get_u8(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>, TryGetError\>

Gets an unsigned 8 bit integer from `self`. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.try_get_i8" class="anchor">Â§</a>

#### fn try_get_i8(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>, TryGetError\>

Gets a signed 8 bit integer from `self`. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.try_get_u16" class="anchor">Â§</a>

#### fn try_get_u16(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>, TryGetError\>

Gets an unsigned 16 bit integer from `self` in big-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.try_get_u16_le" class="anchor">Â§</a>

#### fn try_get_u16_le(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>, TryGetError\>

Gets an unsigned 16 bit integer from `self` in little-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.try_get_u16_ne" class="anchor">Â§</a>

#### fn try_get_u16_ne(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>, TryGetError\>

Gets an unsigned 16 bit integer from `self` in native-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.try_get_i16" class="anchor">Â§</a>

#### fn try_get_i16(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>, TryGetError\>

Gets a signed 16 bit integer from `self` in big-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.try_get_i16_le" class="anchor">Â§</a>

#### fn try_get_i16_le(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>, TryGetError\>

Gets an signed 16 bit integer from `self` in little-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.try_get_i16_ne" class="anchor">Â§</a>

#### fn try_get_i16_ne(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>, TryGetError\>

Gets a signed 16 bit integer from `self` in native-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.try_get_u32" class="anchor">Â§</a>

#### fn try_get_u32(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, TryGetError\>

Gets an unsigned 32 bit integer from `self` in big-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.try_get_u32_le" class="anchor">Â§</a>

#### fn try_get_u32_le(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, TryGetError\>

Gets an unsigned 32 bit integer from `self` in little-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.try_get_u32_ne" class="anchor">Â§</a>

#### fn try_get_u32_ne(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, TryGetError\>

Gets an unsigned 32 bit integer from `self` in native-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.try_get_i32" class="anchor">Â§</a>

#### fn try_get_i32(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, TryGetError\>

Gets a signed 32 bit integer from `self` in big-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.try_get_i32_le" class="anchor">Â§</a>

#### fn try_get_i32_le(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, TryGetError\>

Gets a signed 32 bit integer from `self` in little-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.try_get_i32_ne" class="anchor">Â§</a>

#### fn try_get_i32_ne(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, TryGetError\>

Gets a signed 32 bit integer from `self` in native-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.try_get_u64" class="anchor">Â§</a>

#### fn try_get_u64(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>, TryGetError\>

Gets an unsigned 64 bit integer from `self` in big-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.try_get_u64_le" class="anchor">Â§</a>

#### fn try_get_u64_le(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>, TryGetError\>

Gets an unsigned 64 bit integer from `self` in little-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.try_get_u64_ne" class="anchor">Â§</a>

#### fn try_get_u64_ne(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>, TryGetError\>

Gets an unsigned 64 bit integer from `self` in native-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.try_get_i64" class="anchor">Â§</a>

#### fn try_get_i64(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, TryGetError\>

Gets a signed 64 bit integer from `self` in big-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.try_get_i64_le" class="anchor">Â§</a>

#### fn try_get_i64_le(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, TryGetError\>

Gets a signed 64 bit integer from `self` in little-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.try_get_i64_ne" class="anchor">Â§</a>

#### fn try_get_i64_ne(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, TryGetError\>

Gets a signed 64 bit integer from `self` in native-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.try_get_u128" class="anchor">Â§</a>

#### fn try_get_u128(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>, TryGetError\>

Gets an unsigned 128 bit integer from `self` in big-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.try_get_u128_le" class="anchor">Â§</a>

#### fn try_get_u128_le(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>, TryGetError\>

Gets an unsigned 128 bit integer from `self` in little-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.try_get_u128_ne" class="anchor">Â§</a>

#### fn try_get_u128_ne(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>, TryGetError\>

Gets an unsigned 128 bit integer from `self` in native-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.try_get_i128" class="anchor">Â§</a>

#### fn try_get_i128(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>, TryGetError\>

Gets a signed 128 bit integer from `self` in big-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.try_get_i128_le" class="anchor">Â§</a>

#### fn try_get_i128_le(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>, TryGetError\>

Gets a signed 128 bit integer from `self` in little-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.try_get_i128_ne" class="anchor">Â§</a>

#### fn try_get_i128_ne(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>, TryGetError\>

Gets a signed 128 bit integer from `self` in native-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.try_get_uint" class="anchor">Â§</a>

#### fn try_get_uint(&mut self, nbytes: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>, TryGetError\>

Gets an unsigned n-byte integer from `self` in big-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.try_get_uint_le" class="anchor">Â§</a>

#### fn try_get_uint_le(&mut self, nbytes: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>, TryGetError\>

Gets an unsigned n-byte integer from `self` in little-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.try_get_uint_ne" class="anchor">Â§</a>

#### fn try_get_uint_ne(&mut self, nbytes: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>, TryGetError\>

Gets an unsigned n-byte integer from `self` in native-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.try_get_int" class="anchor">Â§</a>

#### fn try_get_int(&mut self, nbytes: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, TryGetError\>

Gets a signed n-byte integer from `self` in big-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.try_get_int_le" class="anchor">Â§</a>

#### fn try_get_int_le(&mut self, nbytes: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, TryGetError\>

Gets a signed n-byte integer from `self` in little-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.try_get_int_ne" class="anchor">Â§</a>

#### fn try_get_int_ne(&mut self, nbytes: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, TryGetError\>

Gets a signed n-byte integer from `self` in native-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.try_get_f32" class="anchor">Â§</a>

#### fn try_get_f32(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>, TryGetError\>

Gets an IEEE754 single-precision (4 bytes) floating point number from `self` in big-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.try_get_f32_le" class="anchor">Â§</a>

#### fn try_get_f32_le(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>, TryGetError\>

Gets an IEEE754 single-precision (4 bytes) floating point number from `self` in little-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.try_get_f32_ne" class="anchor">Â§</a>

#### fn try_get_f32_ne(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>, TryGetError\>

Gets an IEEE754 single-precision (4 bytes) floating point number from `self` in native-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.try_get_f64" class="anchor">Â§</a>

#### fn try_get_f64(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>, TryGetError\>

Gets an IEEE754 double-precision (8 bytes) floating point number from `self` in big-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.try_get_f64_le" class="anchor">Â§</a>

#### fn try_get_f64_le(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>, TryGetError\>

Gets an IEEE754 double-precision (8 bytes) floating point number from `self` in little-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.try_get_f64_ne" class="anchor">Â§</a>

#### fn try_get_f64_ne(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>, TryGetError\>

Gets an IEEE754 double-precision (8 bytes) floating point number from `self` in native-endian byte order. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.copy_to_bytes" class="anchor">Â§</a>

#### fn copy_to_bytes(&mut self, len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Bytes

Consumes `len` bytes inside self and returns new instance of `Bytes` with this data. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.take" class="anchor">Â§</a>

#### fn take(self, limit: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Take\<Self\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Creates an adaptor which will read at most `limit` bytes from `self`. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.chain" class="anchor">Â§</a>

#### fn chain\<U\>(self, next: U) -\> Chain\<Self, U\>

where U: Buf, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Creates an adaptor which will chain this buffer with another. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.reader" class="anchor">Â§</a>

#### fn reader(self) -\> Reader\<Self\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Creates an adaptor which implements the `Read` trait for `self`. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#impl-BufRead-for-Buffer" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/std/io/trait.BufRead.html" class="trait" title="trait std::io::BufRead">BufRead</a> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.fill_buf" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.BufRead.html#tymethod.fill_buf" class="fn">fill_buf</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/std/io/error/type.Result.html" class="type" title="type std::io::error::Result">Result</a>\<&\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>

Returns the contents of the internal buffer, filling it with more data, via `Read` methods, if empty. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.BufRead.html#tymethod.fill_buf)

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.consume" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.BufRead.html#tymethod.consume" class="fn">consume</a>(&mut self, amt: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Marks the given `amount` of additional bytes from the internal buffer as having been read. Subsequent calls to `read` only return bytes that have not been marked as read. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.BufRead.html#tymethod.consume)

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.has_data_left" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.BufRead.html#method.has_data_left" class="fn">has_data_left</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

ðŸ”¬This is a nightly-only experimental API. (`buf_read_has_data_left`)

Checks if there is any data left to be `read`. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.BufRead.html#method.has_data_left)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#2454" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.read_until" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.BufRead.html#method.read_until" class="fn">read_until</a>(&mut self, byte: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>, buf: &mut <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

Reads all bytes into `buf` until the delimiter `byte` or EOF is reached. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.BufRead.html#method.read_until)

1.83.0 Â· <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#2519" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.skip_until" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.BufRead.html#method.skip_until" class="fn">skip_until</a>(&mut self, byte: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

Skips all bytes until the delimiter `byte` or EOF is reached. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.BufRead.html#method.skip_until)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#2587" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.read_line" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.BufRead.html#method.read_line" class="fn">read_line</a>(&mut self, buf: &mut <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

Reads all bytes until a newline (the `0xA` byte) is reached, and append them to the provided `String` buffer. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.BufRead.html#method.read_line)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#2625-2627" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.split" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.BufRead.html#method.split" class="fn">split</a>(self, byte: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/io/struct.Split.html" class="struct" title="struct std::io::Split">Split</a>\<Self\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Returns an iterator over the contents of this reader split on the byte `byte`. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.BufRead.html#method.split)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#2662-2664" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.lines" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.BufRead.html#method.lines" class="fn">lines</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/io/struct.Lines.html" class="struct" title="struct std::io::Lines">Lines</a>\<Self\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Returns an iterator over the lines of this reader. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.BufRead.html#method.lines)

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#impl-Clone-for-Buffer" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a> <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#" class="tooltip" data-notable-ty="Buffer">â“˜</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#impl-Debug-for-Buffer" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#impl-Default-for-Buffer" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> Self

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#impl-From%3C%26%5Bu8%5D%3E-for-Buffer" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&'static \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.from-3" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(s: &'static \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]) -\> Self

Converts to this type from the input type.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#impl-From%3C%26str%3E-for-Buffer" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.from-4" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(s: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Converts to this type from the input type.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#impl-From%3CArc%3C%5BBytes%5D%3E%3E-for-Buffer" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<\[Bytes\]\>\> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.from-7" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(bs: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<\[Bytes\]\>) -\> Self

Converts to this type from the input type.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#impl-From%3CBytes%3E-for-Buffer" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<Bytes\> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.from-1" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(bs: Bytes) -\> Self

Converts to this type from the input type.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#impl-From%3CString%3E-for-Buffer" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.from-2" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(s: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>) -\> Self

Converts to this type from the input type.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#impl-From%3CVec%3CBytes%3E%3E-for-Buffer" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<Bytes\>\> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.from-6" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(bs: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<Bytes\>) -\> Self

Converts to this type from the input type.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#impl-From%3CVec%3Cu8%3E%3E-for-Buffer" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(bs: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>) -\> Self

Converts to this type from the input type.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#impl-From%3CVecDeque%3CBytes%3E%3E-for-Buffer" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html" class="struct" title="struct alloc::collections::vec_deque::VecDeque">VecDeque</a>\<Bytes\>\> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.from-5" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(bs: <a href="https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html" class="struct" title="struct alloc::collections::vec_deque::VecDeque">VecDeque</a>\<Bytes\>) -\> Self

Converts to this type from the input type.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#impl-FromIterator%3CBytes%3E-for-Buffer" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<Bytes\> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.from_iter-1" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<T: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = Bytes\>\>(iter: T) -\> Self

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#impl-FromIterator%3Cu8%3E-for-Buffer" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.from_iter" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<T: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\>(iter: T) -\> Self

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#impl-IoBuf-for-Buffer" class="anchor">Â§</a>

### impl IoBuf for <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>

Available on **crate feature `services-compfs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.as_buf_ptr" class="anchor">Â§</a>

#### fn as_buf_ptr(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.pointer.html" class="primitive">*const</a> <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

Returns a raw pointer to the vectorâ€™s buffer. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.buf_len" class="anchor">Â§</a>

#### fn buf_len(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Number of initialized bytes. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.buf_capacity" class="anchor">Â§</a>

#### fn buf_capacity(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Total size of the buffer, including uninitialized memory, if any. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.as_slice" class="anchor">Â§</a>

#### fn as_slice(&self) -\> &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\] <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#" class="tooltip" data-notable-ty="&amp;[u8]">â“˜</a>

Get the initialized part of the buffer.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.as_io_slice" class="anchor">Â§</a>

#### unsafe fn as_io_slice(&self) -\> IoSlice

Create an \[`IoSlice`\] of this buffer. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.as_io_buffer" class="anchor">Â§</a>

#### unsafe fn as_io_buffer(&self) -\> IoBuffer

Create an \[`IoBuffer`\] of this buffer. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.slice-1" class="anchor">Â§</a>

#### fn slice(self, range: impl <a href="https://doc.rust-lang.org/nightly/core/ops/range/trait.RangeBounds.html" class="trait" title="trait core::ops::range::RangeBounds">RangeBounds</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>) -\> Slice\<Self\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Returns a view of the buffer with the specified range. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.uninit" class="anchor">Â§</a>

#### fn uninit(self) -\> Uninit\<Self\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Returns an \[`Uninit`\], which is a \[`Slice`\] that only exposes uninitialized bytes. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.filled" class="anchor">Â§</a>

#### fn filled(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Indicate whether the buffer has been filled (uninit portion is empty)

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#impl-Iterator-for-Buffer" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#associatedtype.Item" class="anchor">Â§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype">Item</a> = Bytes

The type of the elements being iterated over.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.next" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#tymethod.next" class="fn">next</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>\>

Advances the iterator and returns the next value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#tymethod.next)

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.size_hint" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.size_hint" class="fn">size_hint</a>(&self) -\> (<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>)

Returns the bounds on the remaining length of the iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.size_hint)

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.next_chunk" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.next_chunk" class="fn">next_chunk</a>\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>( &mut self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\[Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>; <a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">N</a>\], <a href="https://doc.rust-lang.org/nightly/core/array/iter/struct.IntoIter.html" class="struct" title="struct core::array::iter::IntoIter">IntoIter</a>\<Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>, N\>\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

ðŸ”¬This is a nightly-only experimental API. (`iter_next_chunk`)

Advances the iterator and returns an array containing the next `N` values. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.next_chunk)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#222-224" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.count-1" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.count" class="fn">count</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Consumes the iterator, counting the number of iterations and returning it. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.count)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#250-252" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.last" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.last" class="fn">last</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Consumes the iterator, returning the last element. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.last)

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.advance_by" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.advance_by" class="fn">advance_by</a>(&mut self, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html" class="struct" title="struct core::num::nonzero::NonZero">NonZero</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\>

ðŸ”¬This is a nightly-only experimental API. (`iter_advance_by`)

Advances the iterator by `n` elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.advance_by)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#374" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.nth" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.nth" class="fn">nth</a>(&mut self, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>\>

Returns the `n`th element of the iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.nth)

1.28.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#424-426" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.step_by" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.step_by" class="fn">step_by</a>(self, step: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/step_by/struct.StepBy.html" class="struct" title="struct core::iter::adapters::step_by::StepBy">StepBy</a>\<Self\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Creates an iterator starting at the same point, but stepping by the given amount at each iteration. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.step_by)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#495-498" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.chain-1" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.chain" class="fn">chain</a>\<U\>(self, other: U) -\> <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/chain/struct.Chain.html" class="struct" title="struct core::iter::adapters::chain::Chain">Chain</a>\<Self, \<U as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::IntoIter">IntoIter</a>\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, U: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>\>,

Takes two iterators and creates a new iterator over both in sequence. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.chain)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#613-616" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.zip" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.zip" class="fn">zip</a>\<U\>(self, other: U) -\> <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/zip/struct.Zip.html" class="struct" title="struct core::iter::adapters::zip::Zip">Zip</a>\<Self, \<U as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::IntoIter">IntoIter</a>\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, U: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>,

â€˜Zips upâ€™ two iterators into a single iterator of pairs. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.zip)

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.intersperse" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.intersperse" class="fn">intersperse</a>(self, separator: Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/intersperse/struct.Intersperse.html" class="struct" title="struct core::iter::adapters::intersperse::Intersperse">Intersperse</a>\<Self\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>,

ðŸ”¬This is a nightly-only experimental API. (`iter_intersperse`)

Creates a new iterator which places a copy of `separator` between adjacent items of the original iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.intersperse)

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.intersperse_with" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.intersperse_with" class="fn">intersperse_with</a>\<G\>(self, separator: G) -\> <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/intersperse/struct.IntersperseWith.html" class="struct" title="struct core::iter::adapters::intersperse::IntersperseWith">IntersperseWith</a>\<Self, G\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, G: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>() -\> Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>,

ðŸ”¬This is a nightly-only experimental API. (`iter_intersperse`)

Creates a new iterator which places an item generated by `separator` between adjacent items of the original iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.intersperse_with)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#773-776" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.map" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.map" class="fn">map</a>\<B, F\>(self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/map/struct.Map.html" class="struct" title="struct core::iter::adapters::map::Map">Map</a>\<Self, F\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> B,

Takes a closure and creates an iterator which calls that closure on each element. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.map)

1.21.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#818-821" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.for_each" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.for_each" class="fn">for_each</a>\<F\>(self, f: F)

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>),

Calls a closure on each element of an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.for_each)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#893-896" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.filter" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.filter" class="fn">filter</a>\<P\>(self, predicate: P) -\> <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/filter/struct.Filter.html" class="struct" title="struct core::iter::adapters::filter::Filter">Filter</a>\<Self, P\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, P: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(&Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

Creates an iterator which uses a closure to determine if an element should be yielded. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.filter)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#938-941" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.filter_map" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.filter_map" class="fn">filter_map</a>\<B, F\>(self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/filter_map/struct.FilterMap.html" class="struct" title="struct core::iter::adapters::filter_map::FilterMap">FilterMap</a>\<Self, F\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<B\>,

Creates an iterator that both filters and maps. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.filter_map)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#985-987" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.enumerate" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.enumerate" class="fn">enumerate</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/enumerate/struct.Enumerate.html" class="struct" title="struct core::iter::adapters::enumerate::Enumerate">Enumerate</a>\<Self\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Creates an iterator which gives the current iteration count as well as the next value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.enumerate)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#1056-1058" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.peekable" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.peekable" class="fn">peekable</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/peekable/struct.Peekable.html" class="struct" title="struct core::iter::adapters::peekable::Peekable">Peekable</a>\<Self\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Creates an iterator which can use the [`peek`](https://doc.rust-lang.org/nightly/core/iter/adapters/peekable/struct.Peekable.html#method.peek "method core::iter::adapters::peekable::Peekable::peek") and [`peek_mut`](https://doc.rust-lang.org/nightly/core/iter/adapters/peekable/struct.Peekable.html#method.peek_mut "method core::iter::adapters::peekable::Peekable::peek_mut") methods to look at the next element of the iterator without consuming it. See their documentation for more information. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.peekable)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#1121-1124" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.skip_while" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.skip_while" class="fn">skip_while</a>\<P\>(self, predicate: P) -\> <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/skip_while/struct.SkipWhile.html" class="struct" title="struct core::iter::adapters::skip_while::SkipWhile">SkipWhile</a>\<Self, P\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, P: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(&Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

Creates an iterator that [`skip`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.skip "method core::iter::traits::iterator::Iterator::skip")s elements based on a predicate. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.skip_while)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#1199-1202" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.take_while" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.take_while" class="fn">take_while</a>\<P\>(self, predicate: P) -\> <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/take_while/struct.TakeWhile.html" class="struct" title="struct core::iter::adapters::take_while::TakeWhile">TakeWhile</a>\<Self, P\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, P: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(&Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

Creates an iterator that yields elements based on a predicate. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.take_while)

1.57.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#1287-1290" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.map_while" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.map_while" class="fn">map_while</a>\<B, P\>(self, predicate: P) -\> <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/map_while/struct.MapWhile.html" class="struct" title="struct core::iter::adapters::map_while::MapWhile">MapWhile</a>\<Self, P\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, P: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<B\>,

Creates an iterator that both yields elements based on a predicate and maps. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.map_while)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#1316-1318" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.skip" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.skip" class="fn">skip</a>(self, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/skip/struct.Skip.html" class="struct" title="struct core::iter::adapters::skip::Skip">Skip</a>\<Self\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Creates an iterator that skips the first `n` elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.skip)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#1388-1390" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.take-1" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.take" class="fn">take</a>(self, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/take/struct.Take.html" class="struct" title="struct core::iter::adapters::take::Take">Take</a>\<Self\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Creates an iterator that yields the first `n` elements, or fewer if the underlying iterator ends sooner. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.take)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#1435-1438" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.scan" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.scan" class="fn">scan</a>\<St, B, F\>(self, initial_state: St, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/scan/struct.Scan.html" class="struct" title="struct core::iter::adapters::scan::Scan">Scan</a>\<Self, St, F\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut St</a>, Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<B\>,

An iterator adapter which, like [`fold`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.fold "method core::iter::traits::iterator::Iterator::fold"), holds internal state, but unlike [`fold`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.fold "method core::iter::traits::iterator::Iterator::fold"), produces a new iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.scan)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#1473-1477" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.flat_map" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.flat_map" class="fn">flat_map</a>\<U, F\>(self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/flatten/struct.FlatMap.html" class="struct" title="struct core::iter::adapters::flatten::FlatMap">FlatMap</a>\<Self, U, F\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, U: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> U,

Creates an iterator that works like map, but flattens nested structure. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.flat_map)

1.29.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#1557-1560" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.flatten" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.flatten" class="fn">flatten</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/flatten/struct.Flatten.html" class="struct" title="struct core::iter::adapters::flatten::Flatten">Flatten</a>\<Self\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>,

Creates an iterator that flattens nested structure. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.flatten)

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.map_windows" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.map_windows" class="fn">map_windows</a>\<F, R, const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>(self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/map_windows/struct.MapWindows.html" class="struct" title="struct core::iter::adapters::map_windows::MapWindows">MapWindows</a>\<Self, F, N\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(&\[Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>; <a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">N</a>\]) -\> R,

ðŸ”¬This is a nightly-only experimental API. (`iter_map_windows`)

Calls the given function `f` for each contiguous window of size `N` over `self` and returns an iterator over the outputs of `f`. Like [`slice::windows()`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.windows "method slice::windows"), the windows during mapping overlap as well. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.map_windows)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#1775-1777" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.fuse" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.fuse" class="fn">fuse</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/fuse/struct.Fuse.html" class="struct" title="struct core::iter::adapters::fuse::Fuse">Fuse</a>\<Self\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Creates an iterator which ends after the first [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None"). [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.fuse)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#1859-1862" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.inspect" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.inspect" class="fn">inspect</a>\<F\>(self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/inspect/struct.Inspect.html" class="struct" title="struct core::iter::adapters::inspect::Inspect">Inspect</a>\<Self, F\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(&Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>),

Does something with each element of an iterator, passing the value on. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.inspect)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#1896-1898" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.by_ref" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.by_ref" class="fn">by_ref</a>(&mut self) -\> &mut Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Creates a â€œby referenceâ€? adapter for this instance of `Iterator`. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.by_ref)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#2015-2017" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.collect" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.collect" class="fn">collect</a>\<B\>(self) -\> B

where B: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>\>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Transforms an iterator into a collection. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.collect)

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.try_collect" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.try_collect" class="fn">try_collect</a>\<B\>( &mut self, ) -\> \<\<Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html" class="trait" title="trait core::ops::try_trait::Try">Try</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html#associatedtype.Residual" class="associatedtype" title="type core::ops::try_trait::Try::Residual">Residual</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Residual.html" class="trait" title="trait core::ops::try_trait::Residual">Residual</a>\<B\>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Residual.html#associatedtype.TryType" class="associatedtype" title="type core::ops::try_trait::Residual::TryType">TryType</a>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>: <a href="https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html" class="trait" title="trait core::ops::try_trait::Try">Try</a>, \<Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html" class="trait" title="trait core::ops::try_trait::Try">Try</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html#associatedtype.Residual" class="associatedtype" title="type core::ops::try_trait::Try::Residual">Residual</a>: <a href="https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Residual.html" class="trait" title="trait core::ops::try_trait::Residual">Residual</a>\<B\>, B: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<\<Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html" class="trait" title="trait core::ops::try_trait::Try">Try</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html#associatedtype.Output" class="associatedtype" title="type core::ops::try_trait::Try::Output">Output</a>\>,

ðŸ”¬This is a nightly-only experimental API. (`iterator_try_collect`)

Fallibly transforms an iterator into a collection, short circuiting if a failure is encountered. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.try_collect)

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.collect_into" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.collect_into" class="fn">collect_into</a>\<E\>(self, collection: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut E</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut E</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html" class="trait" title="trait core::iter::traits::collect::Extend">Extend</a>\<Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>\>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

ðŸ”¬This is a nightly-only experimental API. (`iter_collect_into`)

Collects all the items from an iterator into a collection. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.collect_into)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#2206-2210" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.partition" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.partition" class="fn">partition</a>\<B, F\>(self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.tuple.html" class="primitive">(B, B)</a>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, B: <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> + <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html" class="trait" title="trait core::iter::traits::collect::Extend">Extend</a>\<Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>\>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(&Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

Consumes an iterator, creating two collections from it. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.partition)

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.is_partitioned" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.is_partitioned" class="fn">is_partitioned</a>\<P\>(self, predicate: P) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, P: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

ðŸ”¬This is a nightly-only experimental API. (`iter_is_partitioned`)

Checks if the elements of this iterator are partitioned according to the given predicate, such that all those that return `true` precede all those that return `false`. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.is_partitioned)

1.27.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#2419-2423" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.try_fold" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.try_fold" class="fn">try_fold</a>\<B, F, R\>(&mut self, init: B, f: F) -\> R

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(B, Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> R, R: <a href="https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html" class="trait" title="trait core::ops::try_trait::Try">Try</a>\<Output = B\>,

An iterator method that applies a function as long as it returns successfully, producing a single, final value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.try_fold)

1.27.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#2477-2481" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.try_for_each" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.try_for_each" class="fn">try_for_each</a>\<F, R\>(&mut self, f: F) -\> R

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> R, R: <a href="https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html" class="trait" title="trait core::ops::try_trait::Try">Try</a>\<Output = <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>,

An iterator method that applies a fallible function to each item in the iterator, stopping at the first error and returning that error. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.try_for_each)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#2596-2599" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.fold" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.fold" class="fn">fold</a>\<B, F\>(self, init: B, f: F) -\> B

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(B, Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> B,

Folds every element into an accumulator by applying an operation, returning the final result. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.fold)

1.51.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#2633-2636" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.reduce" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.reduce" class="fn">reduce</a>\<F\>(self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>, Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>,

Reduces the elements to a single one, by repeatedly applying a reducing operation. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.reduce)

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.try_reduce" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.try_reduce" class="fn">try_reduce</a>\<R\>( &mut self, f: impl <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>, Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> R, ) -\> \<\<R as <a href="https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html" class="trait" title="trait core::ops::try_trait::Try">Try</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html#associatedtype.Residual" class="associatedtype" title="type core::ops::try_trait::Try::Residual">Residual</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Residual.html" class="trait" title="trait core::ops::try_trait::Residual">Residual</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<R as <a href="https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html" class="trait" title="trait core::ops::try_trait::Try">Try</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html#associatedtype.Output" class="associatedtype" title="type core::ops::try_trait::Try::Output">Output</a>\>\>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Residual.html#associatedtype.TryType" class="associatedtype" title="type core::ops::try_trait::Residual::TryType">TryType</a>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, R: <a href="https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html" class="trait" title="trait core::ops::try_trait::Try">Try</a>\<Output = Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>\>, \<R as <a href="https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html" class="trait" title="trait core::ops::try_trait::Try">Try</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html#associatedtype.Residual" class="associatedtype" title="type core::ops::try_trait::Try::Residual">Residual</a>: <a href="https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Residual.html" class="trait" title="trait core::ops::try_trait::Residual">Residual</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>\>\>,

ðŸ”¬This is a nightly-only experimental API. (`iterator_try_reduce`)

Reduces the elements to a single one by repeatedly applying a reducing operation. If the closure returns a failure, the failure is propagated back to the caller immediately. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.try_reduce)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#2762-2765" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.all" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.all" class="fn">all</a>\<F\>(&mut self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

Tests if every element of the iterator matches a predicate. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.all)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#2815-2818" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.any" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.any" class="fn">any</a>\<F\>(&mut self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

Tests if any element of the iterator matches a predicate. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.any)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#2877-2880" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.find" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.find" class="fn">find</a>\<P\>(&mut self, predicate: P) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, P: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(&Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

Searches for an element of an iterator that satisfies a predicate. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.find)

1.30.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#2908-2911" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.find_map" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.find_map" class="fn">find_map</a>\<B, F\>(&mut self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<B\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<B\>,

Applies function to the elements of iterator and returns the first non-none result. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.find_map)

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.try_find" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.try_find" class="fn">try_find</a>\<R\>( &mut self, f: impl <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(&Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> R, ) -\> \<\<R as <a href="https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html" class="trait" title="trait core::ops::try_trait::Try">Try</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html#associatedtype.Residual" class="associatedtype" title="type core::ops::try_trait::Try::Residual">Residual</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Residual.html" class="trait" title="trait core::ops::try_trait::Residual">Residual</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>\>\>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Residual.html#associatedtype.TryType" class="associatedtype" title="type core::ops::try_trait::Residual::TryType">TryType</a>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, R: <a href="https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html" class="trait" title="trait core::ops::try_trait::Try">Try</a>\<Output = <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>, \<R as <a href="https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html" class="trait" title="trait core::ops::try_trait::Try">Try</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html#associatedtype.Residual" class="associatedtype" title="type core::ops::try_trait::Try::Residual">Residual</a>: <a href="https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Residual.html" class="trait" title="trait core::ops::try_trait::Residual">Residual</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>\>\>,

ðŸ”¬This is a nightly-only experimental API. (`try_find`)

Applies function to the elements of iterator and returns the first true result or the first error. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.try_find)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3049-3052" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.position" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.position" class="fn">position</a>\<P\>(&mut self, predicate: P) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, P: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

Searches for an element in an iterator, returning its index. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.position)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3163-3166" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.max" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.max" class="fn">max</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a>,

Returns the maximum element of an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.max)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3199-3202" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.min" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.min" class="fn">min</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a>,

Returns the minimum element of an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.min)

1.6.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3221-3224" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.max_by_key" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.max_by_key" class="fn">max_by_key</a>\<B, F\>(self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>\>

where B: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(&Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> B,

Returns the element that gives the maximum value from the specified function. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.max_by_key)

1.15.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3254-3257" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.max_by" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.max_by" class="fn">max_by</a>\<F\>(self, compare: F) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(&Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>, &Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>,

Returns the element that gives the maximum value with respect to the specified comparison function. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.max_by)

1.6.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3281-3284" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.min_by_key" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.min_by_key" class="fn">min_by_key</a>\<B, F\>(self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>\>

where B: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(&Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> B,

Returns the element that gives the minimum value from the specified function. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.min_by_key)

1.15.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3314-3317" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.min_by" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.min_by" class="fn">min_by</a>\<F\>(self, compare: F) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(&Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>, &Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>,

Returns the element that gives the minimum value with respect to the specified comparison function. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.min_by)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3387-3391" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.unzip" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.unzip" class="fn">unzip</a>\<A, B, FromA, FromB\>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.tuple.html" class="primitive">(FromA, FromB)</a>

where FromA: <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> + <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html" class="trait" title="trait core::iter::traits::collect::Extend">Extend</a>\<A\>, FromB: <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> + <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html" class="trait" title="trait core::iter::traits::collect::Extend">Extend</a>\<B\>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a> + <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.tuple.html" class="primitive">(A, B)</a>\>,

Converts an iterator of pairs into a pair of containers. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.unzip)

1.36.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3418-3421" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.copied" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.copied" class="fn">copied</a>\<'a, T\>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/copied/struct.Copied.html" class="struct" title="struct core::iter::adapters::copied::Copied">Copied</a>\<Self\>

where T: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> + 'a, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a> + <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>\>,

Creates an iterator which copies all of its elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.copied)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3466-3469" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.cloned" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.cloned" class="fn">cloned</a>\<'a, T\>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/cloned/struct.Cloned.html" class="struct" title="struct core::iter::adapters::cloned::Cloned">Cloned</a>\<Self\>

where T: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> + 'a, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a> + <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>\>,

Creates an iterator which [`clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone "method core::clone::Clone::clone")s all of its elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.cloned)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3497-3499" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.cycle" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.cycle" class="fn">cycle</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/cycle/struct.Cycle.html" class="struct" title="struct core::iter::adapters::cycle::Cycle">Cycle</a>\<Self\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a> + <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>,

Repeats an iterator endlessly. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.cycle)

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.array_chunks" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.array_chunks" class="fn">array_chunks</a>\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/array_chunks/struct.ArrayChunks.html" class="struct" title="struct core::iter::adapters::array_chunks::ArrayChunks">ArrayChunks</a>\<Self, N\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

ðŸ”¬This is a nightly-only experimental API. (`iter_array_chunks`)

Returns an iterator over `N` elements of the iterator at a time. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.array_chunks)

1.11.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3576-3579" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.sum" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.sum" class="fn">sum</a>\<S\>(self) -\> S

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, S: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html" class="trait" title="trait core::iter::traits::accum::Sum">Sum</a>\<Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>\>,

Sums the elements of an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.sum)

1.11.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3608-3611" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.product" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.product" class="fn">product</a>\<P\>(self) -\> P

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, P: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html" class="trait" title="trait core::iter::traits::accum::Product">Product</a>\<Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>\>,

Iterates over the entire iterator, multiplying all the elements [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.product)

1.5.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3629-3633" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.cmp" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.cmp" class="fn">cmp</a>\<I\>(self, other: I) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>\>, Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

[Lexicographically](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#lexicographical-comparison "trait core::cmp::Ord") compares the elements of this [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") with those of another. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.cmp)

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.cmp_by" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.cmp_by" class="fn">cmp_by</a>\<I, F\>(self, other: I, cmp: F) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>, \<I as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>,

ðŸ”¬This is a nightly-only experimental API. (`iter_order_by`)

[Lexicographically](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#lexicographical-comparison "trait core::cmp::Ord") compares the elements of this [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") with those of another with respect to the specified comparison function. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.cmp_by)

1.5.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3712-3716" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.partial_cmp" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.partial_cmp" class="fn">partial_cmp</a>\<I\>(self, other: I) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>, Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a>\<\<I as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::Item">Item</a>\>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

[Lexicographically](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#lexicographical-comparison "trait core::cmp::Ord") compares the [`PartialOrd`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd") elements of this [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") with those of another. The comparison works like short-circuit evaluation, returning a result without comparing the remaining elements. As soon as an order can be determined, the evaluation stops and a result is returned. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.partial_cmp)

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.partial_cmp_by" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.partial_cmp_by" class="fn">partial_cmp_by</a>\<I, F\>(self, other: I, partial_cmp: F) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>, \<I as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>,

ðŸ”¬This is a nightly-only experimental API. (`iter_order_by`)

[Lexicographically](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#lexicographical-comparison "trait core::cmp::Ord") compares the elements of this [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") with those of another with respect to the specified comparison function. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.partial_cmp_by)

1.5.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3781-3785" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.eq" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.eq" class="fn">eq</a>\<I\>(self, other: I) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>, Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>\<\<I as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::Item">Item</a>\>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Determines if the elements of this [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") are equal to those of another. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.eq)

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.eq_by" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.eq_by" class="fn">eq_by</a>\<I, F\>(self, other: I, eq: F) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>, \<I as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

ðŸ”¬This is a nightly-only experimental API. (`iter_order_by`)

Determines if the elements of this [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") are equal to those of another with respect to the specified equality function. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.eq_by)

1.5.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3833-3837" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.ne" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.ne" class="fn">ne</a>\<I\>(self, other: I) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>, Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>\<\<I as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::Item">Item</a>\>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Determines if the elements of this [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") are not equal to those of another. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.ne)

1.5.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3854-3858" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.lt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.lt" class="fn">lt</a>\<I\>(self, other: I) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>, Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a>\<\<I as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::Item">Item</a>\>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Determines if the elements of this [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") are [lexicographically](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#lexicographical-comparison "trait core::cmp::Ord") less than those of another. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.lt)

1.5.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3875-3879" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.le" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.le" class="fn">le</a>\<I\>(self, other: I) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>, Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a>\<\<I as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::Item">Item</a>\>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Determines if the elements of this [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") are [lexicographically](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#lexicographical-comparison "trait core::cmp::Ord") less or equal to those of another. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.le)

1.5.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3896-3900" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.gt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.gt" class="fn">gt</a>\<I\>(self, other: I) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>, Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a>\<\<I as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::Item">Item</a>\>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Determines if the elements of this [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") are [lexicographically](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#lexicographical-comparison "trait core::cmp::Ord") greater than those of another. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.gt)

1.5.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3917-3921" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.ge" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.ge" class="fn">ge</a>\<I\>(self, other: I) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>, Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a>\<\<I as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::Item">Item</a>\>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Determines if the elements of this [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") are [lexicographically](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#lexicographical-comparison "trait core::cmp::Ord") greater than or equal to those of another. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.ge)

1.82.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3946-3949" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.is_sorted" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.is_sorted" class="fn">is_sorted</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a>,

Checks if the elements of this iterator are sorted. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.is_sorted)

1.82.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3972-3975" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.is_sorted_by" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.is_sorted_by" class="fn">is_sorted_by</a>\<F\>(self, compare: F) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(&Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>, &Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

Checks if the elements of this iterator are sorted using the given comparator function. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.is_sorted_by)

1.82.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#4016-4020" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.is_sorted_by_key" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.is_sorted_by_key" class="fn">is_sorted_by_key</a>\<F, K\>(self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> K, K: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a>,

Checks if the elements of this iterator are sorted using the given key extraction function. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.is_sorted_by_key)

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#impl-Read-for-Buffer" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/std/io/trait.Read.html" class="trait" title="trait std::io::Read">Read</a> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.read" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Read.html#tymethod.read" class="fn">read</a>(&mut self, buf: &mut \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/std/io/error/type.Result.html" class="type" title="type std::io::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

Pull some bytes from this source into the specified buffer, returning how many bytes were read. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#tymethod.read)

1.36.0 Â· <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#825" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.read_vectored" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_vectored" class="fn">read_vectored</a>(&mut self, bufs: &mut \[<a href="https://doc.rust-lang.org/nightly/std/io/struct.IoSliceMut.html" class="struct" title="struct std::io::IoSliceMut">IoSliceMut</a>\<'\_\>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

Like `read`, except that it reads into a slice of buffers. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_vectored)

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.is_read_vectored" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.is_read_vectored" class="fn">is_read_vectored</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

ðŸ”¬This is a nightly-only experimental API. (`can_vector`)

Determines if this `Read`er has an efficient `read_vectored` implementation. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.is_read_vectored)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#935" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.read_to_end" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_to_end" class="fn">read_to_end</a>(&mut self, buf: &mut <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

Reads all bytes until EOF in this source, placing them into `buf`. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_to_end)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#991" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.read_to_string" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_to_string" class="fn">read_to_string</a>(&mut self, buf: &mut <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

Reads all bytes until EOF in this source, appending them to `buf`. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_to_string)

1.6.0 Â· <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1044" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.read_exact" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_exact" class="fn">read_exact</a>(&mut self, buf: &mut \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

Reads the exact number of bytes required to fill `buf`. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_exact)

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.read_buf" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_buf" class="fn">read_buf</a>(&mut self, buf: <a href="https://doc.rust-lang.org/nightly/core/io/borrowed_buf/struct.BorrowedCursor.html" class="struct" title="struct core::io::borrowed_buf::BorrowedCursor">BorrowedCursor</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

ðŸ”¬This is a nightly-only experimental API. (`read_buf`)

Pull some bytes from this source into the specified buffer. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_buf)

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.read_buf_exact" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_buf_exact" class="fn">read_buf_exact</a>(&mut self, cursor: <a href="https://doc.rust-lang.org/nightly/core/io/borrowed_buf/struct.BorrowedCursor.html" class="struct" title="struct core::io::borrowed_buf::BorrowedCursor">BorrowedCursor</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

ðŸ”¬This is a nightly-only experimental API. (`read_buf`)

Reads the exact number of bytes required to fill `cursor`. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_buf_exact)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1119-1121" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.by_ref-1" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.by_ref" class="fn">by_ref</a>(&mut self) -\> &mut Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Creates a â€œby referenceâ€? adapter for this instance of `Read`. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.by_ref)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1162-1164" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.bytes" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.bytes" class="fn">bytes</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/io/struct.Bytes.html" class="struct" title="struct std::io::Bytes">Bytes</a>\<Self\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Transforms this `Read` instance to an [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") over its bytes. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.bytes)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1200-1202" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.chain-2" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.chain" class="fn">chain</a>\<R\>(self, next: R) -\> <a href="https://doc.rust-lang.org/nightly/std/io/struct.Chain.html" class="struct" title="struct std::io::Chain">Chain</a>\<Self, R\>

where R: <a href="https://doc.rust-lang.org/nightly/std/io/trait.Read.html" class="trait" title="trait std::io::Read">Read</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Creates an adapter which will chain this stream with another. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.chain)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1239-1241" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.take-2" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.take" class="fn">take</a>(self, limit: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/io/struct.Take.html" class="struct" title="struct std::io::Take">Take</a>\<Self\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Creates an adapter which will read at most `limit` bytes from it. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.take)

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#impl-Read-for-Buffer-1" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html" class="trait" title="trait opendal::raw::oio::Read">Read</a> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.read-1" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html#tymethod.read" class="fn">read</a>(&mut self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>\>

Read at the given offset with the given size.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.read_all" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html#method.read_all" class="fn">read_all</a>(&mut self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

Read all data from the reader.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#impl-Seek-for-Buffer" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/std/io/trait.Seek.html" class="trait" title="trait std::io::Seek">Seek</a> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.seek" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Seek.html#tymethod.seek" class="fn">seek</a>(&mut self, pos: <a href="https://doc.rust-lang.org/nightly/std/io/enum.SeekFrom.html" class="enum" title="enum std::io::SeekFrom">SeekFrom</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/io/error/type.Result.html" class="type" title="type std::io::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>

Seek to an offset, in bytes, in a stream. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Seek.html#tymethod.seek)

1.55.0 Â· <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#2064" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.rewind" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Seek.html#method.rewind" class="fn">rewind</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

Rewind to the beginning of a stream. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Seek.html#method.rewind)

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.stream_len" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Seek.html#method.stream_len" class="fn">stream_len</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

ðŸ”¬This is a nightly-only experimental API. (`seek_stream_len`)

Returns the length of this stream (in bytes). [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Seek.html#method.stream_len)

1.51.0 Â· <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#2132" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.stream_position" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Seek.html#method.stream_position" class="fn">stream_position</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

Returns the current seek position from the start of the stream. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Seek.html#method.stream_position)

1.80.0 Â· <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#2160" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.seek_relative" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Seek.html#method.seek_relative" class="fn">seek_relative</a>(&mut self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

Seeks relative to the current position. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Seek.html#method.seek_relative)

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#impl-Sink%3CBuffer%3E-for-BufferSink" class="anchor">Â§</a>

### impl Sink\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>\> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.BufferSink.html" class="struct" title="struct opendal::BufferSink">BufferSink</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#associatedtype.Error" class="anchor">Â§</a>

#### type Error = <a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html" class="struct" title="struct opendal::Error">Error</a>

The type of value produced by the sink when an error occurs.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.poll_ready" class="anchor">Â§</a>

#### fn poll_ready(self: <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<&mut Self\>, cx: &mut <a href="https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html" class="struct" title="struct core::task::wake::Context">Context</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html" class="enum" title="enum core::task::poll::Poll">Poll</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\>

Attempts to prepare the `Sink` to receive a value. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.start_send" class="anchor">Â§</a>

#### fn start_send(self: <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<&mut Self\>, item: <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Begin the process of sending a value to the sink. Each call to this function must be preceded by a successful call to `poll_ready` which returned `Poll::Ready(Ok(()))`. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.poll_flush" class="anchor">Â§</a>

#### fn poll_flush(self: <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<&mut Self\>, cx: &mut <a href="https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html" class="struct" title="struct core::task::wake::Context">Context</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html" class="enum" title="enum core::task::poll::Poll">Poll</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\>

Flush any remaining output from this sink. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.poll_close" class="anchor">Â§</a>

#### fn poll_close(self: <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<&mut Self\>, cx: &mut <a href="https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html" class="struct" title="struct core::task::wake::Context">Context</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html" class="enum" title="enum core::task::poll::Poll">Poll</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\>

Flush any remaining output and close this sink, if necessary. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#impl-Stream-for-Buffer" class="anchor">Â§</a>

### impl Stream for <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#associatedtype.Item-1" class="anchor">Â§</a>

#### type Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Bytes, <a href="https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html" class="enum" title="enum core::convert::Infallible">Infallible</a>\>

Values yielded by the stream.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.poll_next" class="anchor">Â§</a>

#### fn poll_next( self: <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<&mut Self\>, \_: &mut <a href="https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html" class="struct" title="struct core::task::wake::Context">Context</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html" class="enum" title="enum core::task::poll::Poll">Poll</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::Item\>\>

Attempt to pull out the next value of this stream, registering the current task for wakeup if the value is not yet available, and returning `None` if the stream is exhausted. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#method.size_hint-1" class="anchor">Â§</a>

#### fn size_hint(&self) -\> (<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>)

Returns the bounds on the remaining length of the stream. Read more

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html#blanket-implementations" class="anchor">Â§</a>
