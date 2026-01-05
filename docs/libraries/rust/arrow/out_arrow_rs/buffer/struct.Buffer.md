# Struct Buffer Copy item path

<a href="https://docs.rs/arrow-buffer/56.2.0/x86_64-unknown-linux-gnu/src/arrow_buffer/buffer/immutable.rs.html#72" class="src">Source</a>

``` rust
pub struct Buffer { /* private fields */ }
```

Expand description

A contiguous memory region that can be shared with other buffers and across thread boundaries that stores Arrow data.

`Buffer`s can be sliced and cloned without copying the underlying data and can be created from memory allocated by non-Rust sources such as C/C++.

## <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#example-create-a-buffer-from-a-vec-without-copying" class="doc-anchor">§</a>Example: Create a `Buffer` from a `Vec` (without copying)

``` rust
let vec: Vec<u32> = vec![1, 2, 3];
let buffer = Buffer::from(vec);
```

## <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#example-convert-a-buffer-to-a-vec-without-copying" class="doc-anchor">§</a>Example: Convert a `Buffer` to a `Vec` (without copying)

Use [`Self::into_vec`](https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.into_vec "method arrow::buffer::Buffer::into_vec") to convert a `Buffer` back into a `Vec` if there are no other references and the types are aligned correctly.

``` rust
// convert the buffer back into a Vec of u32
// note this will fail if the buffer is shared or not aligned correctly
let vec: Vec<u32> = buffer.into_vec().unwrap();
```

## <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#example-create-a-buffer-from-a-bytesbytes-without-copying" class="doc-anchor">§</a>Example: Create a `Buffer` from a [`bytes::Bytes`](https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html "struct bytes::bytes::Bytes") (without copying)

[`bytes::Bytes`](https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html "struct bytes::bytes::Bytes") is a common type in the Rust ecosystem for shared memory regions. You can create a buffer from a `Bytes` instance using the `From` implementation, also without copying.

``` rust
let bytes = bytes::Bytes::from("hello");
let buffer = Buffer::from(bytes);
```

## Implementations<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#impl-Buffer" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.from_bytes" class="fn">from_bytes</a>(bytes: Bytes) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

👎Deprecated since 54.1.0: Use Buffer::from instead

Create a new Buffer from a (internal) `Bytes`

NOTE despite the same name, `Bytes` is an internal struct in arrow-rs and is different than [`bytes::Bytes`](https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html "struct bytes::bytes::Bytes").

See examples on [`Buffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html "struct arrow::buffer::Buffer") for ways to create a buffer from a [`bytes::Bytes`](https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html "struct bytes::bytes::Bytes").

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.ptr_offset" class="fn">ptr_offset</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the offset, in bytes, of `Self::ptr` to `Self::data`

self.ptr and self.data can be different after slicing or advancing the buffer.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.data_ptr" class="fn">data_ptr</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/ptr/non_null/struct.NonNull.html" class="struct" title="struct core::ptr::non_null::NonNull">NonNull</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>

Returns the pointer to the start of the buffer without the offset.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.strong_count" class="fn">strong_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the number of strong references to the buffer.

This method is safe but if the buffer is shared across multiple threads the underlying value could change between calling this method and using the result.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.from_vec" class="fn">from_vec</a>\<T\>(vec: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<T\>) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html" class="trait" title="trait arrow::datatypes::ArrowNativeType">ArrowNativeType</a>,

Create a [`Buffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html "struct arrow::buffer::Buffer") from the provided [`Vec`](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html "struct alloc::vec::Vec") without copying

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.from_slice_ref" class="fn">from_slice_ref</a>\<U, T\>(items: T) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

where U: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html" class="trait" title="trait arrow::datatypes::ArrowNativeType">ArrowNativeType</a>, T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[U]</a>\>,

Initializes a [Buffer](https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html "struct arrow::buffer::Buffer") from a slice of items.

#### pub unsafe fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.from_custom_allocation" class="fn">from_custom_allocation</a>( ptr: <a href="https://doc.rust-lang.org/nightly/core/ptr/non_null/struct.NonNull.html" class="struct" title="struct core::ptr::non_null::NonNull">NonNull</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>, len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, owner: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/alloc/trait.Allocation.html" class="trait" title="trait arrow::alloc::Allocation">Allocation</a>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

Creates a buffer from an existing memory region.

Ownership of the memory is tracked via reference counting and the memory will be freed using the `drop` method of [crate::alloc::Allocation](https://docs.rs/arrow/latest/arrow/alloc/trait.Allocation.html "trait arrow::alloc::Allocation") when the reference count reaches zero.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#arguments" class="doc-anchor">§</a>Arguments

- `ptr` - Pointer to raw parts
- `len` - Length of raw parts in **bytes**
- `owner` - A [crate::alloc::Allocation](https://docs.rs/arrow/latest/arrow/alloc/trait.Allocation.html "trait arrow::alloc::Allocation") which is responsible for freeing that data

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#safety" class="doc-anchor">§</a>Safety

This function is unsafe as there is no guarantee that the given pointer is valid for `len` bytes

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the number of bytes in the buffer

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.capacity" class="fn">capacity</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the capacity of this buffer. For externally owned buffers, this returns zero

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.shrink_to_fit" class="fn">shrink_to_fit</a>(&mut self)

Tries to shrink the capacity of the buffer as much as possible, freeing unused memory.

If the buffer is shared, this is a no-op.

If the memory was allocated with a custom allocator, this is a no-op.

If the capacity is already less than or equal to the desired capacity, this is a no-op.

The memory region will be reallocated using `std::alloc::realloc`.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the buffer is empty.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.as_slice" class="fn">as_slice</a>(&self) -\> &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\] <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#" class="tooltip" data-notable-ty="&amp;[u8]">ⓘ</a>

Returns the byte slice stored in this buffer

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.slice" class="fn">slice</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

Returns a new [Buffer](https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html "struct arrow::buffer::Buffer") that is a slice of this buffer starting at `offset`.

This function is `O(1)` and does not copy any data, allowing the same memory region to be shared between buffers.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#panics" class="doc-anchor">§</a>Panics

Panics iff `offset` is larger than `len`.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.advance" class="fn">advance</a>(&mut self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Increases the offset of this buffer by `offset`

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#panics-1" class="doc-anchor">§</a>Panics

Panics iff `offset` is larger than `len`.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.slice_with_length" class="fn">slice_with_length</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

Returns a new [Buffer](https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html "struct arrow::buffer::Buffer") that is a slice of this buffer starting at `offset`, with `length` bytes.

This function is `O(1)` and does not copy any data, allowing the same memory region to be shared between buffers.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#panics-2" class="doc-anchor">§</a>Panics

Panics iff `(offset + length)` is larger than the existing length.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.as_ptr" class="fn">as_ptr</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.pointer.html" class="primitive">*const</a> <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

Returns a pointer to the start of this buffer.

Note that this should be used cautiously, and the returned pointer should not be stored anywhere, to avoid dangling pointers.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.typed_data" class="fn">typed_data</a>\<T\>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html" class="trait" title="trait arrow::datatypes::ArrowNativeType">ArrowNativeType</a>,

View buffer as a slice of a specific type.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#panics-3" class="doc-anchor">§</a>Panics

This function panics if the underlying buffer is not aligned correctly for type `T`.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.bit_slice" class="fn">bit_slice</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

Returns a slice of this buffer starting at a certain bit offset. If the offset is byte-aligned the returned buffer is a shallow clone, otherwise a new buffer is allocated and filled with a copy of the bits in the range.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.bit_chunks" class="fn">bit_chunks</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/util/bit_chunk_iterator/struct.BitChunks.html" class="struct" title="struct arrow::util::bit_chunk_iterator::BitChunks">BitChunks</a>\<'\_\>

Returns a `BitChunks` instance which can be used to iterate over this buffers bits in larger chunks and starting at arbitrary bit offsets. Note that both `offset` and `length` are measured in bits.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.count_set_bits_offset" class="fn">count_set_bits_offset</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the number of 1-bits in this buffer, starting from `offset` with `length` bits inspected. Note that both `offset` and `length` are measured in bits.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.into_mutable" class="fn">into_mutable</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html" class="struct" title="struct arrow::buffer::MutableBuffer">MutableBuffer</a>, <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>\>

Returns `MutableBuffer` for mutating the buffer if this buffer is not shared. Returns `Err` if this is shared or its allocation is from an external source or it is not allocated with alignment [`ALIGNMENT`](https://docs.rs/arrow/latest/arrow/alloc/constant.ALIGNMENT.html "constant arrow::alloc::ALIGNMENT")

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.into_vec" class="fn">into_vec</a>\<T\>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<T\>, <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>\>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html" class="trait" title="trait arrow::datatypes::ArrowNativeType">ArrowNativeType</a>,

Converts self into a `Vec`, if possible.

This can be used to reuse / mutate the underlying data.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#errors" class="doc-anchor">§</a>Errors

Returns `Err(self)` if

1.  this buffer does not have the same [`Layout`](https://doc.rust-lang.org/nightly/core/alloc/layout/struct.Layout.html "struct core::alloc::layout::Layout") as the destination Vec
2.  contains a non-zero offset
3.  The buffer is shared

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.ptr_eq" class="fn">ptr_eq</a>(&self, other: &<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if this [`Buffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html "struct arrow::buffer::Buffer") is equal to `other`, using pointer comparisons to determine buffer equality. This is cheaper than `PartialEq::eq` but may return false when the arrays are logically equal

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#impl-Buffer-1" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

#### pub unsafe fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.from_trusted_len_iter" class="fn">from_trusted_len_iter</a>\<T, I\>(iterator: I) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html" class="trait" title="trait arrow::datatypes::ArrowNativeType">ArrowNativeType</a>, I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = T\>,

Creates a [`Buffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html "struct arrow::buffer::Buffer") from an [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") with a trusted (upper) length.

Prefer this to `collect` whenever possible, as it is ~60% faster.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#example" class="doc-anchor">§</a>Example

``` rust
let v = vec![1u32];
let iter = v.iter().map(|x| x * 2);
let buffer = unsafe { Buffer::from_trusted_len_iter(iter) };
assert_eq!(buffer.len(), 4) // u32 has 4 bytes
```

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#safety-1" class="doc-anchor">§</a>Safety

This method assumes that the iterator’s size is correct and is undefined behavior to use it on an iterator that reports an incorrect length.

#### pub unsafe fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.try_from_trusted_len_iter" class="fn">try_from_trusted_len_iter</a>\<E, T, I\>( iterator: I, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>, E\>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html" class="trait" title="trait arrow::datatypes::ArrowNativeType">ArrowNativeType</a>, I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<T, E\>\>,

Creates a [`Buffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html "struct arrow::buffer::Buffer") from an [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") with a trusted (upper) length or errors if any of the items of the iterator is an error. Prefer this to `collect` whenever possible, as it is ~60% faster.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#safety-2" class="doc-anchor">§</a>Safety

This method assumes that the iterator’s size is correct and is undefined behavior to use it on an iterator that reports an incorrect length.

## Methods from <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html" class="trait" title="trait core::ops::deref::Deref">Deref</a>\<Target = \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\><a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#deref-methods-%5Bu8%5D" class="anchor">§</a>

1.23.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/ascii.rs.html#16" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.is_ascii" class="fn">is_ascii</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Checks if all bytes in this slice are within the ASCII range.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.as_ascii" class="fn">as_ascii</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&\[<a href="https://doc.rust-lang.org/nightly/core/ascii/ascii_char/enum.AsciiChar.html" class="enum" title="enum core::ascii::ascii_char::AsciiChar">AsciiChar</a>\]\>

🔬This is a nightly-only experimental API. (`ascii_char`)

If this slice [`is_ascii`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.is_ascii "method slice::is_ascii"), returns it as a slice of [ASCII characters](https://doc.rust-lang.org/nightly/core/ascii/ascii_char/enum.AsciiChar.html "enum core::ascii::ascii_char::AsciiChar"), otherwise returns `None`.

#### pub unsafe fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.as_ascii_unchecked" class="fn">as_ascii_unchecked</a>(&self) -\> &\[<a href="https://doc.rust-lang.org/nightly/core/ascii/ascii_char/enum.AsciiChar.html" class="enum" title="enum core::ascii::ascii_char::AsciiChar">AsciiChar</a>\]

🔬This is a nightly-only experimental API. (`ascii_char`)

Converts this slice of bytes into a slice of ASCII characters, without checking whether they’re valid.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#safety-3" class="doc-anchor">§</a>Safety

Every byte in the slice must be in `0..=127`, or else this is UB.

1.23.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/ascii.rs.html#58" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.eq_ignore_ascii_case" class="fn">eq_ignore_ascii_case</a>(&self, other: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Checks that two slices are an ASCII case-insensitive match.

Same as `to_ascii_lowercase(a) == to_ascii_lowercase(b)`, but without allocating and copying temporaries.

1.60.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/ascii.rs.html#138" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.escape_ascii" class="fn">escape_ascii</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/slice/ascii/struct.EscapeAscii.html" class="struct" title="struct core::slice::ascii::EscapeAscii">EscapeAscii</a>\<'\_\>

Returns an iterator that produces an escaped version of this slice, treating it as an ASCII string.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples" class="doc-anchor">§</a>Examples

``` rust
let s = b"0\t\r\n'\"\\\x9d";
let escaped = s.escape_ascii().to_string();
assert_eq!(escaped, "0\\t\\r\\n\\'\\\"\\\\\\x9d");
```

1.80.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/ascii.rs.html#157" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.trim_ascii_start" class="fn">trim_ascii_start</a>(&self) -\> &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\] <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#" class="tooltip" data-notable-ty="&amp;[u8]">ⓘ</a>

Returns a byte slice with leading ASCII whitespace bytes removed.

‘Whitespace’ refers to the definition used by [`u8::is_ascii_whitespace`](https://doc.rust-lang.org/nightly/std/primitive.u8.html#method.is_ascii_whitespace "method u8::is_ascii_whitespace").

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-1" class="doc-anchor">§</a>Examples

``` rust
assert_eq!(b" \t hello world\n".trim_ascii_start(), b"hello world\n");
assert_eq!(b"  ".trim_ascii_start(), b"");
assert_eq!(b"".trim_ascii_start(), b"");
```

1.80.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/ascii.rs.html#186" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.trim_ascii_end" class="fn">trim_ascii_end</a>(&self) -\> &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\] <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#" class="tooltip" data-notable-ty="&amp;[u8]">ⓘ</a>

Returns a byte slice with trailing ASCII whitespace bytes removed.

‘Whitespace’ refers to the definition used by [`u8::is_ascii_whitespace`](https://doc.rust-lang.org/nightly/std/primitive.u8.html#method.is_ascii_whitespace "method u8::is_ascii_whitespace").

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-2" class="doc-anchor">§</a>Examples

``` rust
assert_eq!(b"\r hello world\n ".trim_ascii_end(), b"\r hello world");
assert_eq!(b"  ".trim_ascii_end(), b"");
assert_eq!(b"".trim_ascii_end(), b"");
```

1.80.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/ascii.rs.html#216" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.trim_ascii" class="fn">trim_ascii</a>(&self) -\> &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\] <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#" class="tooltip" data-notable-ty="&amp;[u8]">ⓘ</a>

Returns a byte slice with leading and trailing ASCII whitespace bytes removed.

‘Whitespace’ refers to the definition used by [`u8::is_ascii_whitespace`](https://doc.rust-lang.org/nightly/std/primitive.u8.html#method.is_ascii_whitespace "method u8::is_ascii_whitespace").

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-3" class="doc-anchor">§</a>Examples

``` rust
assert_eq!(b"\r hello world\n ".trim_ascii(), b"hello world");
assert_eq!(b"  ".trim_ascii(), b"");
assert_eq!(b"".trim_ascii(), b"");
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#114" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.len-1" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the number of elements in the slice.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-4" class="doc-anchor">§</a>Examples

``` rust
let a = [1, 2, 3];
assert_eq!(a.len(), 3);
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#134" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.is_empty-1" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns `true` if the slice has a length of 0.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-5" class="doc-anchor">§</a>Examples

``` rust
let a = [1, 2, 3];
assert!(!a.is_empty());

let b: &[i32] = &[];
assert!(b.is_empty());
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#153" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.first" class="fn">first</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>\>

Returns the first element of the slice, or `None` if it is empty.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-6" class="doc-anchor">§</a>Examples

``` rust
let v = [10, 40, 30];
assert_eq!(Some(&10), v.first());

let w: &[i32] = &[];
assert_eq!(None, w.first());
```

1.5.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#196" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.split_first" class="fn">split_first</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>, &<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>)\>

Returns the first and all the rest of the elements of the slice, or `None` if it is empty.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-7" class="doc-anchor">§</a>Examples

``` rust
let x = &[0, 1, 2];

if let Some((first, elements)) = x.split_first() {
    assert_eq!(first, &0);
    assert_eq!(elements, &[1, 2]);
}
```

1.5.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#238" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.split_last" class="fn">split_last</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>, &<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>)\>

Returns the last and all the rest of the elements of the slice, or `None` if it is empty.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-8" class="doc-anchor">§</a>Examples

``` rust
let x = &[0, 1, 2];

if let Some((last, elements)) = x.split_last() {
    assert_eq!(last, &2);
    assert_eq!(elements, &[0, 1]);
}
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#279" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.last" class="fn">last</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>\>

Returns the last element of the slice, or `None` if it is empty.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-9" class="doc-anchor">§</a>Examples

``` rust
let v = [10, 40, 30];
assert_eq!(Some(&30), v.last());

let w: &[i32] = &[];
assert_eq!(None, w.last());
```

1.77.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#325" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.first_chunk" class="fn">first_chunk</a>\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">[T; N]</a>\>

Returns an array reference to the first `N` items in the slice.

If the slice is not at least `N` in length, this will return `None`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-10" class="doc-anchor">§</a>Examples

``` rust
let u = [10, 40, 30];
assert_eq!(Some(&[10, 40]), u.first_chunk::<2>());

let v: &[i32] = &[10];
assert_eq!(None, v.first_chunk::<2>());

let w: &[i32] = &[];
assert_eq!(Some(&[]), w.first_chunk::<0>());
```

1.77.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#385" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.split_first_chunk" class="fn">split_first_chunk</a>\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(&<a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">[T; N]</a>, &<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>)\>

Returns an array reference to the first `N` items in the slice and the remaining slice.

If the slice is not at least `N` in length, this will return `None`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-11" class="doc-anchor">§</a>Examples

``` rust
let x = &[0, 1, 2];

if let Some((first, elements)) = x.split_first_chunk::<2>() {
    assert_eq!(first, &[0, 1]);
    assert_eq!(elements, &[2]);
}

assert_eq!(None, x.split_first_chunk::<4>());
```

1.77.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#445" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.split_last_chunk" class="fn">split_last_chunk</a>\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(&<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>, &<a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">[T; N]</a>)\>

Returns an array reference to the last `N` items in the slice and the remaining slice.

If the slice is not at least `N` in length, this will return `None`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-12" class="doc-anchor">§</a>Examples

``` rust
let x = &[0, 1, 2];

if let Some((elements, last)) = x.split_last_chunk::<2>() {
    assert_eq!(elements, &[0]);
    assert_eq!(last, &[1, 2]);
}

assert_eq!(None, x.split_last_chunk::<4>());
```

1.77.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#507" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.last_chunk" class="fn">last_chunk</a>\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">[T; N]</a>\>

Returns an array reference to the last `N` items in the slice.

If the slice is not at least `N` in length, this will return `None`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-13" class="doc-anchor">§</a>Examples

``` rust
let u = [10, 40, 30];
assert_eq!(Some(&[40, 30]), u.last_chunk::<2>());

let v: &[i32] = &[10];
assert_eq!(None, v.last_chunk::<2>());

let w: &[i32] = &[];
assert_eq!(Some(&[]), w.last_chunk::<0>());
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#570-572" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.get" class="fn">get</a>\<I\>(&self, index: I) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&\<I as <a href="https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html" class="trait" title="trait core::slice::index::SliceIndex">SliceIndex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>\>\>::<a href="https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html#associatedtype.Output" class="associatedtype" title="type core::slice::index::SliceIndex::Output">Output</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html" class="trait" title="trait core::slice::index::SliceIndex">SliceIndex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>\>,

Returns a reference to an element or subslice depending on the type of index.

- If given a position, returns a reference to the element at that position or `None` if out of bounds.
- If given a range, returns the subslice corresponding to that range, or `None` if out of bounds.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-14" class="doc-anchor">§</a>Examples

``` rust
let v = [10, 40, 30];
assert_eq!(Some(&40), v.get(1));
assert_eq!(Some(&[10, 40][..]), v.get(0..2));
assert_eq!(None, v.get(3));
assert_eq!(None, v.get(0..4));
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#637-639" class="src">Source</a>

#### pub unsafe fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.get_unchecked" class="fn">get_unchecked</a>\<I\>( &self, index: I, ) -\> &\<I as <a href="https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html" class="trait" title="trait core::slice::index::SliceIndex">SliceIndex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>\>\>::<a href="https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html#associatedtype.Output" class="associatedtype" title="type core::slice::index::SliceIndex::Output">Output</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html" class="trait" title="trait core::slice::index::SliceIndex">SliceIndex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>\>,

Returns a reference to an element or subslice, without doing bounds checking.

For a safe alternative see [`get`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.get "method slice::get").

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#safety-4" class="doc-anchor">§</a>Safety

Calling this method with an out-of-bounds index is *[undefined behavior](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)* even if the resulting reference is not used.

You can think of this like `.get(index).unwrap_unchecked()`. It’s UB to call `.get_unchecked(len)`, even if you immediately convert to a pointer. And it’s UB to call `.get_unchecked(..len + 1)`, `.get_unchecked(..=len)`, or similar.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-15" class="doc-anchor">§</a>Examples

``` rust
let x = &[1, 2, 4];

unsafe {
    assert_eq!(x.get_unchecked(1), &2);
}
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#724" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.as_ptr-1" class="fn">as_ptr</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.pointer.html" class="primitive">*const T</a>

Returns a raw pointer to the slice’s buffer.

The caller must ensure that the slice outlives the pointer this function returns, or else it will end up dangling.

The caller must also ensure that the memory the pointer (non-transitively) points to is never written to (except inside an `UnsafeCell`) using this pointer or any pointer derived from it. If you need to mutate the contents of the slice, use [`as_mut_ptr`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_mut_ptr "method slice::as_mut_ptr").

Modifying the container referenced by this slice may cause its buffer to be reallocated, which would also make any pointers to it invalid.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-16" class="doc-anchor">§</a>Examples

``` rust
let x = &[1, 2, 4];
let x_ptr = x.as_ptr();

unsafe {
    for i in 0..x.len() {
        assert_eq!(x.get_unchecked(i), &*x_ptr.add(i));
    }
}
```

1.48.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#791" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.as_ptr_range" class="fn">as_ptr_range</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.pointer.html" class="primitive">*const T</a>\>

Returns the two raw pointers spanning the slice.

The returned range is half-open, which means that the end pointer points *one past* the last element of the slice. This way, an empty slice is represented by two equal pointers, and the difference between the two pointers represents the size of the slice.

See [`as_ptr`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_ptr "method slice::as_ptr") for warnings on using these pointers. The end pointer requires extra caution, as it does not point to a valid element in the slice.

This function is useful for interacting with foreign interfaces which use two pointers to refer to a range of elements in memory, as is common in C++.

It can also be useful to check if a pointer to an element refers to an element of this slice:

``` rust
let a = [1, 2, 3];
let x = &a[1] as *const _;
let y = &5 as *const _;

assert!(a.as_ptr_range().contains(&x));
assert!(!a.as_ptr_range().contains(&y));
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.as_array" class="fn">as_array</a>\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">[T; N]</a>\>

🔬This is a nightly-only experimental API. (`slice_as_array`)

Gets a reference to the underlying array.

If `N` is not exactly equal to the length of `self`, then this method returns `None`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1036" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.iter" class="fn">iter</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/slice/iter/struct.Iter.html" class="struct" title="struct core::slice::iter::Iter">Iter</a>\<'\_, T\>

Returns an iterator over the slice.

The iterator yields all items from start to end.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-17" class="doc-anchor">§</a>Examples

``` rust
let x = &[1, 2, 4];
let mut iterator = x.iter();

assert_eq!(iterator.next(), Some(&1));
assert_eq!(iterator.next(), Some(&2));
assert_eq!(iterator.next(), Some(&4));
assert_eq!(iterator.next(), None);
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1111" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.windows" class="fn">windows</a>(&self, size: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/slice/iter/struct.Windows.html" class="struct" title="struct core::slice::iter::Windows">Windows</a>\<'\_, T\>

Returns an iterator over all contiguous windows of length `size`. The windows overlap. If the slice is shorter than `size`, the iterator returns no values.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#panics-4" class="doc-anchor">§</a>Panics

Panics if `size` is zero.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-18" class="doc-anchor">§</a>Examples

``` rust
let slice = ['l', 'o', 'r', 'e', 'm'];
let mut iter = slice.windows(3);
assert_eq!(iter.next().unwrap(), &['l', 'o', 'r']);
assert_eq!(iter.next().unwrap(), &['o', 'r', 'e']);
assert_eq!(iter.next().unwrap(), &['r', 'e', 'm']);
assert!(iter.next().is_none());
```

If the slice is shorter than `size`:

``` rust
let slice = ['f', 'o', 'o'];
let mut iter = slice.windows(4);
assert!(iter.next().is_none());
```

Because the [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") trait cannot represent the required lifetimes, there is no `windows_mut` analog to `windows`; `[0,1,2].windows_mut(2).collect()` would violate [the rules of references](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#the-rules-of-references) (though a [LendingIterator](https://blog.rust-lang.org/2022/10/28/gats-stabilization.html) analog is possible). You can sometimes use [`Cell::as_slice_of_cells`](https://doc.rust-lang.org/nightly/core/cell/struct.Cell.html#method.as_slice_of_cells "method core::cell::Cell::as_slice_of_cells") in conjunction with `windows` instead:

``` rust
use std::cell::Cell;

let mut array = ['R', 'u', 's', 't', ' ', '2', '0', '1', '5'];
let slice = &mut array[..];
let slice_of_cells: &[Cell<char>] = Cell::from_mut(slice).as_slice_of_cells();
for w in slice_of_cells.windows(3) {
    Cell::swap(&w[0], &w[2]);
}
assert_eq!(array, ['s', 't', ' ', '2', '0', '1', '5', 'u', 'R']);
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1151" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.chunks" class="fn">chunks</a>(&self, chunk_size: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/slice/iter/struct.Chunks.html" class="struct" title="struct core::slice::iter::Chunks">Chunks</a>\<'\_, T\>

Returns an iterator over `chunk_size` elements of the slice at a time, starting at the beginning of the slice.

The chunks are slices and do not overlap. If `chunk_size` does not divide the length of the slice, then the last chunk will not have length `chunk_size`.

See [`chunks_exact`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.chunks_exact "method slice::chunks_exact") for a variant of this iterator that returns chunks of always exactly `chunk_size` elements, and [`rchunks`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.rchunks "method slice::rchunks") for the same iterator but starting at the end of the slice.

If your `chunk_size` is a constant, consider using [`as_chunks`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_chunks "method slice::as_chunks") instead, which will give references to arrays of exactly that length, rather than slices.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#panics-5" class="doc-anchor">§</a>Panics

Panics if `chunk_size` is zero.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-19" class="doc-anchor">§</a>Examples

``` rust
let slice = ['l', 'o', 'r', 'e', 'm'];
let mut iter = slice.chunks(2);
assert_eq!(iter.next().unwrap(), &['l', 'o']);
assert_eq!(iter.next().unwrap(), &['r', 'e']);
assert_eq!(iter.next().unwrap(), &['m']);
assert!(iter.next().is_none());
```

1.31.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1238" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.chunks_exact" class="fn">chunks_exact</a>(&self, chunk_size: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/slice/iter/struct.ChunksExact.html" class="struct" title="struct core::slice::iter::ChunksExact">ChunksExact</a>\<'\_, T\>

Returns an iterator over `chunk_size` elements of the slice at a time, starting at the beginning of the slice.

The chunks are slices and do not overlap. If `chunk_size` does not divide the length of the slice, then the last up to `chunk_size-1` elements will be omitted and can be retrieved from the `remainder` function of the iterator.

Due to each chunk having exactly `chunk_size` elements, the compiler can often optimize the resulting code better than in the case of [`chunks`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.chunks "method slice::chunks").

See [`chunks`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.chunks "method slice::chunks") for a variant of this iterator that also returns the remainder as a smaller chunk, and [`rchunks_exact`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.rchunks_exact "method slice::rchunks_exact") for the same iterator but starting at the end of the slice.

If your `chunk_size` is a constant, consider using [`as_chunks`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_chunks "method slice::as_chunks") instead, which will give references to arrays of exactly that length, rather than slices.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#panics-6" class="doc-anchor">§</a>Panics

Panics if `chunk_size` is zero.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-20" class="doc-anchor">§</a>Examples

``` rust
let slice = ['l', 'o', 'r', 'e', 'm'];
let mut iter = slice.chunks_exact(2);
assert_eq!(iter.next().unwrap(), &['l', 'o']);
assert_eq!(iter.next().unwrap(), &['r', 'e']);
assert!(iter.next().is_none());
assert_eq!(iter.remainder(), &['m']);
```

1.88.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1334" class="src">Source</a>

#### pub unsafe fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.as_chunks_unchecked" class="fn">as_chunks_unchecked</a>\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>(&self) -\> &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">[T; N]</a>\]

Splits the slice into a slice of `N`-element arrays, assuming that there’s no remainder.

This is the inverse operation to [`as_flattened`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_flattened "method slice::as_flattened").

As this is `unsafe`, consider whether you could use [`as_chunks`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_chunks "method slice::as_chunks") or [`as_rchunks`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_rchunks "method slice::as_rchunks") instead, perhaps via something like `if let (chunks, []) = slice.as_chunks()` or `let (chunks, []) = slice.as_chunks() else { unreachable!() };`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#safety-5" class="doc-anchor">§</a>Safety

This may only be called when

- The slice splits exactly into `N`-element chunks (aka `self.len() % N == 0`).
- `N != 0`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-21" class="doc-anchor">§</a>Examples

``` rust
let slice: &[char] = &['l', 'o', 'r', 'e', 'm', '!'];
let chunks: &[[char; 1]] =
    // SAFETY: 1-element chunks never have remainder
    unsafe { slice.as_chunks_unchecked() };
assert_eq!(chunks, &[['l'], ['o'], ['r'], ['e'], ['m'], ['!']]);
let chunks: &[[char; 3]] =
    // SAFETY: The slice length (6) is a multiple of 3
    unsafe { slice.as_chunks_unchecked() };
assert_eq!(chunks, &[['l', 'o', 'r'], ['e', 'm', '!']]);

// These would be unsound:
// let chunks: &[[_; 5]] = slice.as_chunks_unchecked() // The slice length is not a multiple of 5
// let chunks: &[[_; 0]] = slice.as_chunks_unchecked() // Zero-length chunks are never allowed
```

1.88.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1392" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.as_chunks" class="fn">as_chunks</a>\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>(&self) -\> (&\[<a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">[T; N]</a>\], &<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>)

Splits the slice into a slice of `N`-element arrays, starting at the beginning of the slice, and a remainder slice with length strictly less than `N`.

The remainder is meaningful in the division sense. Given `let (chunks, remainder) = slice.as_chunks()`, then:

- `chunks.len()` equals `slice.len() / N`,
- `remainder.len()` equals `slice.len() % N`, and
- `slice.len()` equals `chunks.len() * N + remainder.len()`.

You can flatten the chunks back into a slice-of-`T` with [`as_flattened`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_flattened "method slice::as_flattened").

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#panics-7" class="doc-anchor">§</a>Panics

Panics if `N` is zero.

Note that this check is against a const generic parameter, not a runtime value, and thus a particular monomorphization will either always panic or it will never panic.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-22" class="doc-anchor">§</a>Examples

``` rust
let slice = ['l', 'o', 'r', 'e', 'm'];
let (chunks, remainder) = slice.as_chunks();
assert_eq!(chunks, &[['l', 'o'], ['r', 'e']]);
assert_eq!(remainder, &['m']);
```

If you expect the slice to be an exact multiple, you can combine `let`-`else` with an empty slice pattern:

``` rust
let slice = ['R', 'u', 's', 't'];
let (chunks, []) = slice.as_chunks::<2>() else {
    panic!("slice didn't have even length")
};
assert_eq!(chunks, &[['R', 'u'], ['s', 't']]);
```

1.88.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1439" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.as_rchunks" class="fn">as_rchunks</a>\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>(&self) -\> (&<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>, &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">[T; N]</a>\])

Splits the slice into a slice of `N`-element arrays, starting at the end of the slice, and a remainder slice with length strictly less than `N`.

The remainder is meaningful in the division sense. Given `let (remainder, chunks) = slice.as_rchunks()`, then:

- `remainder.len()` equals `slice.len() % N`,
- `chunks.len()` equals `slice.len() / N`, and
- `slice.len()` equals `chunks.len() * N + remainder.len()`.

You can flatten the chunks back into a slice-of-`T` with [`as_flattened`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_flattened "method slice::as_flattened").

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#panics-8" class="doc-anchor">§</a>Panics

Panics if `N` is zero.

Note that this check is against a const generic parameter, not a runtime value, and thus a particular monomorphization will either always panic or it will never panic.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-23" class="doc-anchor">§</a>Examples

``` rust
let slice = ['l', 'o', 'r', 'e', 'm'];
let (remainder, chunks) = slice.as_rchunks();
assert_eq!(remainder, &['l']);
assert_eq!(chunks, &[['o', 'r'], ['e', 'm']]);
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.array_windows" class="fn">array_windows</a>\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/slice/iter/struct.ArrayWindows.html" class="struct" title="struct core::slice::iter::ArrayWindows">ArrayWindows</a>\<'\_, T, N\>

🔬This is a nightly-only experimental API. (`array_windows`)

Returns an iterator over overlapping windows of `N` elements of a slice, starting at the beginning of the slice.

This is the const generic equivalent of [`windows`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.windows "method slice::windows").

If `N` is greater than the size of the slice, it will return no windows.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#panics-9" class="doc-anchor">§</a>Panics

Panics if `N` is zero. This check will most probably get changed to a compile time error before this method gets stabilized.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-24" class="doc-anchor">§</a>Examples

``` rust
#![feature(array_windows)]
let slice = [0, 1, 2, 3];
let mut iter = slice.array_windows();
assert_eq!(iter.next().unwrap(), &[0, 1]);
assert_eq!(iter.next().unwrap(), &[1, 2]);
assert_eq!(iter.next().unwrap(), &[2, 3]);
assert!(iter.next().is_none());
```

1.31.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1680" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.rchunks" class="fn">rchunks</a>(&self, chunk_size: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/slice/iter/struct.RChunks.html" class="struct" title="struct core::slice::iter::RChunks">RChunks</a>\<'\_, T\>

Returns an iterator over `chunk_size` elements of the slice at a time, starting at the end of the slice.

The chunks are slices and do not overlap. If `chunk_size` does not divide the length of the slice, then the last chunk will not have length `chunk_size`.

See [`rchunks_exact`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.rchunks_exact "method slice::rchunks_exact") for a variant of this iterator that returns chunks of always exactly `chunk_size` elements, and [`chunks`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.chunks "method slice::chunks") for the same iterator but starting at the beginning of the slice.

If your `chunk_size` is a constant, consider using [`as_rchunks`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_rchunks "method slice::as_rchunks") instead, which will give references to arrays of exactly that length, rather than slices.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#panics-10" class="doc-anchor">§</a>Panics

Panics if `chunk_size` is zero.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-25" class="doc-anchor">§</a>Examples

``` rust
let slice = ['l', 'o', 'r', 'e', 'm'];
let mut iter = slice.rchunks(2);
assert_eq!(iter.next().unwrap(), &['e', 'm']);
assert_eq!(iter.next().unwrap(), &['o', 'r']);
assert_eq!(iter.next().unwrap(), &['l']);
assert!(iter.next().is_none());
```

1.31.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1769" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.rchunks_exact" class="fn">rchunks_exact</a>(&self, chunk_size: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/slice/iter/struct.RChunksExact.html" class="struct" title="struct core::slice::iter::RChunksExact">RChunksExact</a>\<'\_, T\>

Returns an iterator over `chunk_size` elements of the slice at a time, starting at the end of the slice.

The chunks are slices and do not overlap. If `chunk_size` does not divide the length of the slice, then the last up to `chunk_size-1` elements will be omitted and can be retrieved from the `remainder` function of the iterator.

Due to each chunk having exactly `chunk_size` elements, the compiler can often optimize the resulting code better than in the case of [`rchunks`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.rchunks "method slice::rchunks").

See [`rchunks`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.rchunks "method slice::rchunks") for a variant of this iterator that also returns the remainder as a smaller chunk, and [`chunks_exact`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.chunks_exact "method slice::chunks_exact") for the same iterator but starting at the beginning of the slice.

If your `chunk_size` is a constant, consider using [`as_rchunks`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_rchunks "method slice::as_rchunks") instead, which will give references to arrays of exactly that length, rather than slices.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#panics-11" class="doc-anchor">§</a>Panics

Panics if `chunk_size` is zero.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-26" class="doc-anchor">§</a>Examples

``` rust
let slice = ['l', 'o', 'r', 'e', 'm'];
let mut iter = slice.rchunks_exact(2);
assert_eq!(iter.next().unwrap(), &['e', 'm']);
assert_eq!(iter.next().unwrap(), &['o', 'r']);
assert!(iter.next().is_none());
assert_eq!(iter.remainder(), &['l']);
```

1.77.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1858-1860" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.chunk_by" class="fn">chunk_by</a>\<F\>(&self, pred: F) -\> <a href="https://doc.rust-lang.org/nightly/core/slice/iter/struct.ChunkBy.html" class="struct" title="struct core::slice::iter::ChunkBy">ChunkBy</a>\<'\_, T, F\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

Returns an iterator over the slice producing non-overlapping runs of elements using the predicate to separate them.

The predicate is called for every pair of consecutive elements, meaning that it is called on `slice[0]` and `slice[1]`, followed by `slice[1]` and `slice[2]`, and so on.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-27" class="doc-anchor">§</a>Examples

``` rust
let slice = &[1, 1, 1, 3, 3, 2, 2, 2];

let mut iter = slice.chunk_by(|a, b| a == b);

assert_eq!(iter.next(), Some(&[1, 1, 1][..]));
assert_eq!(iter.next(), Some(&[3, 3][..]));
assert_eq!(iter.next(), Some(&[2, 2, 2][..]));
assert_eq!(iter.next(), None);
```

This method can be used to extract the sorted subslices:

``` rust
let slice = &[1, 1, 2, 3, 2, 3, 2, 3, 4];

let mut iter = slice.chunk_by(|a, b| a <= b);

assert_eq!(iter.next(), Some(&[1, 1, 2, 3][..]));
assert_eq!(iter.next(), Some(&[2, 3][..]));
assert_eq!(iter.next(), Some(&[2, 3, 4][..]));
assert_eq!(iter.next(), None);
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1946" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.split_at" class="fn">split_at</a>(&self, mid: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> (&<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>, &<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>)

Divides one slice into two at an index.

The first will contain all indices from `[0, mid)` (excluding the index `mid` itself) and the second will contain all indices from `[mid, len)` (excluding the index `len` itself).

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#panics-12" class="doc-anchor">§</a>Panics

Panics if `mid > len`. For a non-panicking alternative see [`split_at_checked`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.split_at_checked "method slice::split_at_checked").

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-28" class="doc-anchor">§</a>Examples

``` rust
let v = ['a', 'b', 'c'];

{
   let (left, right) = v.split_at(0);
   assert_eq!(left, []);
   assert_eq!(right, ['a', 'b', 'c']);
}

{
    let (left, right) = v.split_at(2);
    assert_eq!(left, ['a', 'b']);
    assert_eq!(right, ['c']);
}

{
    let (left, right) = v.split_at(3);
    assert_eq!(left, ['a', 'b', 'c']);
    assert_eq!(right, []);
}
```

1.79.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2032" class="src">Source</a>

#### pub unsafe fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.split_at_unchecked" class="fn">split_at_unchecked</a>(&self, mid: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> (&<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>, &<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>)

Divides one slice into two at an index, without doing bounds checking.

The first will contain all indices from `[0, mid)` (excluding the index `mid` itself) and the second will contain all indices from `[mid, len)` (excluding the index `len` itself).

For a safe alternative see [`split_at`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.split_at "method slice::split_at").

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#safety-6" class="doc-anchor">§</a>Safety

Calling this method with an out-of-bounds index is *[undefined behavior](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)* even if the resulting reference is not used. The caller has to ensure that `0 <= mid <= self.len()`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-29" class="doc-anchor">§</a>Examples

``` rust
let v = ['a', 'b', 'c'];

unsafe {
   let (left, right) = v.split_at_unchecked(0);
   assert_eq!(left, []);
   assert_eq!(right, ['a', 'b', 'c']);
}

unsafe {
    let (left, right) = v.split_at_unchecked(2);
    assert_eq!(left, ['a', 'b']);
    assert_eq!(right, ['c']);
}

unsafe {
    let (left, right) = v.split_at_unchecked(3);
    assert_eq!(left, ['a', 'b', 'c']);
    assert_eq!(right, []);
}
```

1.80.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2147" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.split_at_checked" class="fn">split_at_checked</a>(&self, mid: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(&<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>, &<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>)\>

Divides one slice into two at an index, returning `None` if the slice is too short.

If `mid ≤ len` returns a pair of slices where the first will contain all indices from `[0, mid)` (excluding the index `mid` itself) and the second will contain all indices from `[mid, len)` (excluding the index `len` itself).

Otherwise, if `mid > len`, returns `None`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-30" class="doc-anchor">§</a>Examples

``` rust
let v = [1, -2, 3, -4, 5, -6];

{
   let (left, right) = v.split_at_checked(0).unwrap();
   assert_eq!(left, []);
   assert_eq!(right, [1, -2, 3, -4, 5, -6]);
}

{
    let (left, right) = v.split_at_checked(2).unwrap();
    assert_eq!(left, [1, -2]);
    assert_eq!(right, [3, -4, 5, -6]);
}

{
    let (left, right) = v.split_at_checked(6).unwrap();
    assert_eq!(left, [1, -2, 3, -4, 5, -6]);
    assert_eq!(right, []);
}

assert_eq!(None, v.split_at_checked(7));
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2238-2240" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.split" class="fn">split</a>\<F\>(&self, pred: F) -\> <a href="https://doc.rust-lang.org/nightly/core/slice/iter/struct.Split.html" class="struct" title="struct core::slice::iter::Split">Split</a>\<'\_, T, F\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

Returns an iterator over subslices separated by elements that match `pred`. The matched element is not contained in the subslices.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-31" class="doc-anchor">§</a>Examples

``` rust
let slice = [10, 40, 33, 20];
let mut iter = slice.split(|num| num % 3 == 0);

assert_eq!(iter.next().unwrap(), &[10, 40]);
assert_eq!(iter.next().unwrap(), &[20]);
assert!(iter.next().is_none());
```

If the first element is matched, an empty slice will be the first item returned by the iterator. Similarly, if the last element in the slice is matched, an empty slice will be the last item returned by the iterator:

``` rust
let slice = [10, 40, 33];
let mut iter = slice.split(|num| num % 3 == 0);

assert_eq!(iter.next().unwrap(), &[10, 40]);
assert_eq!(iter.next().unwrap(), &[]);
assert!(iter.next().is_none());
```

If two matched elements are directly adjacent, an empty slice will be present between them:

``` rust
let slice = [10, 6, 33, 20];
let mut iter = slice.split(|num| num % 3 == 0);

assert_eq!(iter.next().unwrap(), &[10]);
assert_eq!(iter.next().unwrap(), &[]);
assert_eq!(iter.next().unwrap(), &[20]);
assert!(iter.next().is_none());
```

1.51.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2296-2298" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.split_inclusive" class="fn">split_inclusive</a>\<F\>(&self, pred: F) -\> <a href="https://doc.rust-lang.org/nightly/core/slice/iter/struct.SplitInclusive.html" class="struct" title="struct core::slice::iter::SplitInclusive">SplitInclusive</a>\<'\_, T, F\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

Returns an iterator over subslices separated by elements that match `pred`. The matched element is contained in the end of the previous subslice as a terminator.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-32" class="doc-anchor">§</a>Examples

``` rust
let slice = [10, 40, 33, 20];
let mut iter = slice.split_inclusive(|num| num % 3 == 0);

assert_eq!(iter.next().unwrap(), &[10, 40, 33]);
assert_eq!(iter.next().unwrap(), &[20]);
assert!(iter.next().is_none());
```

If the last element of the slice is matched, that element will be considered the terminator of the preceding slice. That slice will be the last item returned by the iterator.

``` rust
let slice = [3, 10, 40, 33];
let mut iter = slice.split_inclusive(|num| num % 3 == 0);

assert_eq!(iter.next().unwrap(), &[3]);
assert_eq!(iter.next().unwrap(), &[10, 40, 33]);
assert!(iter.next().is_none());
```

1.27.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2356-2358" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.rsplit" class="fn">rsplit</a>\<F\>(&self, pred: F) -\> <a href="https://doc.rust-lang.org/nightly/core/slice/iter/struct.RSplit.html" class="struct" title="struct core::slice::iter::RSplit">RSplit</a>\<'\_, T, F\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

Returns an iterator over subslices separated by elements that match `pred`, starting at the end of the slice and working backwards. The matched element is not contained in the subslices.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-33" class="doc-anchor">§</a>Examples

``` rust
let slice = [11, 22, 33, 0, 44, 55];
let mut iter = slice.rsplit(|num| *num == 0);

assert_eq!(iter.next().unwrap(), &[44, 55]);
assert_eq!(iter.next().unwrap(), &[11, 22, 33]);
assert_eq!(iter.next(), None);
```

As with `split()`, if the first or last element is matched, an empty slice will be the first (or last) item returned by the iterator.

``` rust
let v = &[0, 1, 1, 2, 3, 5, 8];
let mut it = v.rsplit(|n| *n % 2 == 0);
assert_eq!(it.next().unwrap(), &[]);
assert_eq!(it.next().unwrap(), &[3, 5]);
assert_eq!(it.next().unwrap(), &[1, 1]);
assert_eq!(it.next().unwrap(), &[]);
assert_eq!(it.next(), None);
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2410-2412" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.splitn" class="fn">splitn</a>\<F\>(&self, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, pred: F) -\> <a href="https://doc.rust-lang.org/nightly/core/slice/iter/struct.SplitN.html" class="struct" title="struct core::slice::iter::SplitN">SplitN</a>\<'\_, T, F\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

Returns an iterator over subslices separated by elements that match `pred`, limited to returning at most `n` items. The matched element is not contained in the subslices.

The last element returned, if any, will contain the remainder of the slice.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-34" class="doc-anchor">§</a>Examples

Print the slice split once by numbers divisible by 3 (i.e., `[10, 40]`, `[20, 60, 50]`):

``` rust
let v = [10, 40, 30, 20, 60, 50];

for group in v.splitn(2, |num| *num % 3 == 0) {
    println!("{group:?}");
}
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2465-2467" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.rsplitn" class="fn">rsplitn</a>\<F\>(&self, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, pred: F) -\> <a href="https://doc.rust-lang.org/nightly/core/slice/iter/struct.RSplitN.html" class="struct" title="struct core::slice::iter::RSplitN">RSplitN</a>\<'\_, T, F\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

Returns an iterator over subslices separated by elements that match `pred` limited to returning at most `n` items. This starts at the end of the slice and works backwards. The matched element is not contained in the subslices.

The last element returned, if any, will contain the remainder of the slice.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-35" class="doc-anchor">§</a>Examples

Print the slice split once, starting from the end, by numbers divisible by 3 (i.e., `[50]`, `[10, 40, 30, 20]`):

``` rust
let v = [10, 40, 30, 20, 60, 50];

for group in v.rsplitn(2, |num| *num % 3 == 0) {
    println!("{group:?}");
}
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.split_once" class="fn">split_once</a>\<F\>(&self, pred: F) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(&<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>, &<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>)\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

🔬This is a nightly-only experimental API. (`slice_split_once`)

Splits the slice on the first element that matches the specified predicate.

If any matching elements are present in the slice, returns the prefix before the match and suffix after. The matching element itself is not included. If no elements match, returns `None`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-36" class="doc-anchor">§</a>Examples

``` rust
#![feature(slice_split_once)]
let s = [1, 2, 3, 2, 4];
assert_eq!(s.split_once(|&x| x == 2), Some((
    &[1][..],
    &[3, 2, 4][..]
)));
assert_eq!(s.split_once(|&x| x == 0), None);
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.rsplit_once" class="fn">rsplit_once</a>\<F\>(&self, pred: F) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(&<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>, &<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>)\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

🔬This is a nightly-only experimental API. (`slice_split_once`)

Splits the slice on the last element that matches the specified predicate.

If any matching elements are present in the slice, returns the prefix before the match and suffix after. The matching element itself is not included. If no elements match, returns `None`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-37" class="doc-anchor">§</a>Examples

``` rust
#![feature(slice_split_once)]
let s = [1, 2, 3, 2, 4];
assert_eq!(s.rsplit_once(|&x| x == 2), Some((
    &[1, 2, 3][..],
    &[4][..]
)));
assert_eq!(s.rsplit_once(|&x| x == 0), None);
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2583-2585" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.contains" class="fn">contains</a>(&self, x: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>,

Returns `true` if the slice contains an element with the given value.

This operation is *O*(*n*).

Note that if you have a sorted slice, [`binary_search`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.binary_search "method slice::binary_search") may be faster.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-38" class="doc-anchor">§</a>Examples

``` rust
let v = [10, 40, 30];
assert!(v.contains(&30));
assert!(!v.contains(&50));
```

If you do not have a `&T`, but some other value that you can compare with one (for example, `String` implements `PartialEq<str>`), you can use `iter().any`:

``` rust
let v = [String::from("hello"), String::from("world")]; // slice of `String`
assert!(v.iter().any(|e| e == "hello")); // search with `&str`
assert!(!v.iter().any(|e| e == "hi"));
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2613-2615" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.starts_with" class="fn">starts_with</a>(&self, needle: &<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>,

Returns `true` if `needle` is a prefix of the slice or equal to the slice.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-39" class="doc-anchor">§</a>Examples

``` rust
let v = [10, 40, 30];
assert!(v.starts_with(&[10]));
assert!(v.starts_with(&[10, 40]));
assert!(v.starts_with(&v));
assert!(!v.starts_with(&[50]));
assert!(!v.starts_with(&[10, 50]));
```

Always returns `true` if `needle` is an empty slice:

``` rust
let v = &[10, 40, 30];
assert!(v.starts_with(&[]));
let v: &[u8] = &[];
assert!(v.starts_with(&[]));
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2644-2646" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.ends_with" class="fn">ends_with</a>(&self, needle: &<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>,

Returns `true` if `needle` is a suffix of the slice or equal to the slice.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-40" class="doc-anchor">§</a>Examples

``` rust
let v = [10, 40, 30];
assert!(v.ends_with(&[30]));
assert!(v.ends_with(&[40, 30]));
assert!(v.ends_with(&v));
assert!(!v.ends_with(&[50]));
assert!(!v.ends_with(&[50, 30]));
```

Always returns `true` if `needle` is an empty slice:

``` rust
let v = &[10, 40, 30];
assert!(v.ends_with(&[]));
let v: &[u8] = &[];
assert!(v.ends_with(&[]));
```

1.51.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2676-2678" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.strip_prefix" class="fn">strip_prefix</a>\<P\>(&self, prefix: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;P</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>\>

where P: <a href="https://doc.rust-lang.org/nightly/core/slice/trait.SlicePattern.html" class="trait" title="trait core::slice::SlicePattern">SlicePattern</a>\<Item = T\> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, T: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>,

Returns a subslice with the prefix removed.

If the slice starts with `prefix`, returns the subslice after the prefix, wrapped in `Some`. If `prefix` is empty, simply returns the original slice. If `prefix` is equal to the original slice, returns an empty slice.

If the slice does not start with `prefix`, returns `None`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-41" class="doc-anchor">§</a>Examples

``` rust
let v = &[10, 40, 30];
assert_eq!(v.strip_prefix(&[10]), Some(&[40, 30][..]));
assert_eq!(v.strip_prefix(&[10, 40]), Some(&[30][..]));
assert_eq!(v.strip_prefix(&[10, 40, 30]), Some(&[][..]));
assert_eq!(v.strip_prefix(&[50]), None);
assert_eq!(v.strip_prefix(&[10, 50]), None);

let prefix : &str = "he";
assert_eq!(b"hello".strip_prefix(prefix.as_bytes()),
           Some(b"llo".as_ref()));
```

1.51.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2712-2714" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.strip_suffix" class="fn">strip_suffix</a>\<P\>(&self, suffix: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;P</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>\>

where P: <a href="https://doc.rust-lang.org/nightly/core/slice/trait.SlicePattern.html" class="trait" title="trait core::slice::SlicePattern">SlicePattern</a>\<Item = T\> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, T: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>,

Returns a subslice with the suffix removed.

If the slice ends with `suffix`, returns the subslice before the suffix, wrapped in `Some`. If `suffix` is empty, simply returns the original slice. If `suffix` is equal to the original slice, returns an empty slice.

If the slice does not end with `suffix`, returns `None`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-42" class="doc-anchor">§</a>Examples

``` rust
let v = &[10, 40, 30];
assert_eq!(v.strip_suffix(&[30]), Some(&[10, 40][..]));
assert_eq!(v.strip_suffix(&[40, 30]), Some(&[10][..]));
assert_eq!(v.strip_suffix(&[10, 40, 30]), Some(&[][..]));
assert_eq!(v.strip_suffix(&[50]), None);
assert_eq!(v.strip_suffix(&[50, 30]), None);
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.trim_prefix" class="fn">trim_prefix</a>\<P\>(&self, prefix: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;P</a>) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>

where P: <a href="https://doc.rust-lang.org/nightly/core/slice/trait.SlicePattern.html" class="trait" title="trait core::slice::SlicePattern">SlicePattern</a>\<Item = T\> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, T: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>,

🔬This is a nightly-only experimental API. (`trim_prefix_suffix`)

Returns a subslice with the optional prefix removed.

If the slice starts with `prefix`, returns the subslice after the prefix. If `prefix` is empty or the slice does not start with `prefix`, simply returns the original slice. If `prefix` is equal to the original slice, returns an empty slice.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-43" class="doc-anchor">§</a>Examples

``` rust
#![feature(trim_prefix_suffix)]

let v = &[10, 40, 30];

// Prefix present - removes it
assert_eq!(v.trim_prefix(&[10]), &[40, 30][..]);
assert_eq!(v.trim_prefix(&[10, 40]), &[30][..]);
assert_eq!(v.trim_prefix(&[10, 40, 30]), &[][..]);

// Prefix absent - returns original slice
assert_eq!(v.trim_prefix(&[50]), &[10, 40, 30][..]);
assert_eq!(v.trim_prefix(&[10, 50]), &[10, 40, 30][..]);

let prefix : &str = "he";
assert_eq!(b"hello".trim_prefix(prefix.as_bytes()), b"llo".as_ref());
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.trim_suffix" class="fn">trim_suffix</a>\<P\>(&self, suffix: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;P</a>) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>

where P: <a href="https://doc.rust-lang.org/nightly/core/slice/trait.SlicePattern.html" class="trait" title="trait core::slice::SlicePattern">SlicePattern</a>\<Item = T\> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, T: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>,

🔬This is a nightly-only experimental API. (`trim_prefix_suffix`)

Returns a subslice with the optional suffix removed.

If the slice ends with `suffix`, returns the subslice before the suffix. If `suffix` is empty or the slice does not end with `suffix`, simply returns the original slice. If `suffix` is equal to the original slice, returns an empty slice.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-44" class="doc-anchor">§</a>Examples

``` rust
#![feature(trim_prefix_suffix)]

let v = &[10, 40, 30];

// Suffix present - removes it
assert_eq!(v.trim_suffix(&[30]), &[10, 40][..]);
assert_eq!(v.trim_suffix(&[40, 30]), &[10][..]);
assert_eq!(v.trim_suffix(&[10, 40, 30]), &[][..]);

// Suffix absent - returns original slice
assert_eq!(v.trim_suffix(&[50]), &[10, 40, 30][..]);
assert_eq!(v.trim_suffix(&[50, 30]), &[10, 40, 30][..]);
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2881-2883" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.binary_search" class="fn">binary_search</a>(&self, x: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a>,

Binary searches this slice for a given element. If the slice is not sorted, the returned result is unspecified and meaningless.

If the value is found then [`Result::Ok`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Ok "variant core::result::Result::Ok") is returned, containing the index of the matching element. If there are multiple matches, then any one of the matches could be returned. The index is chosen deterministically, but is subject to change in future versions of Rust. If the value is not found then [`Result::Err`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Err "variant core::result::Result::Err") is returned, containing the index where a matching element could be inserted while maintaining sorted order.

See also [`binary_search_by`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.binary_search_by "method slice::binary_search_by"), [`binary_search_by_key`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.binary_search_by_key "method slice::binary_search_by_key"), and [`partition_point`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.partition_point "method slice::partition_point").

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-45" class="doc-anchor">§</a>Examples

Looks up a series of four elements. The first is found, with a uniquely determined position; the second and third are not found; the fourth could match any position in `[1, 4]`.

``` rust
let s = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];

assert_eq!(s.binary_search(&13),  Ok(9));
assert_eq!(s.binary_search(&4),   Err(7));
assert_eq!(s.binary_search(&100), Err(13));
let r = s.binary_search(&1);
assert!(match r { Ok(1..=4) => true, _ => false, });
```

If you want to find that whole *range* of matching items, rather than an arbitrary matching one, that can be done using [`partition_point`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.partition_point "method slice::partition_point"):

``` rust
let s = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];

let low = s.partition_point(|x| x < &1);
assert_eq!(low, 1);
let high = s.partition_point(|x| x <= &1);
assert_eq!(high, 5);
let r = s.binary_search(&1);
assert!((low..high).contains(&r.unwrap()));

assert!(s[..low].iter().all(|&x| x < 1));
assert!(s[low..high].iter().all(|&x| x == 1));
assert!(s[high..].iter().all(|&x| x > 1));

// For something not found, the "range" of equal items is empty
assert_eq!(s.partition_point(|x| x < &11), 9);
assert_eq!(s.partition_point(|x| x <= &11), 9);
assert_eq!(s.binary_search(&11), Err(9));
```

If you want to insert an item to a sorted vector, while maintaining sort order, consider using [`partition_point`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.partition_point "method slice::partition_point"):

``` rust
let mut s = vec![0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
let num = 42;
let idx = s.partition_point(|&x| x <= num);
// If `num` is unique, `s.partition_point(|&x| x < num)` (with `<`) is equivalent to
// `s.binary_search(&num).unwrap_or_else(|x| x)`, but using `<=` will allow `insert`
// to shift less elements.
s.insert(idx, num);
assert_eq!(s, [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 42, 55]);
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2932-2934" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.binary_search_by" class="fn">binary_search_by</a>\<'a, F\>(&'a self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>,

Binary searches this slice with a comparator function.

The comparator function should return an order code that indicates whether its argument is `Less`, `Equal` or `Greater` the desired target. If the slice is not sorted or if the comparator function does not implement an order consistent with the sort order of the underlying slice, the returned result is unspecified and meaningless.

If the value is found then [`Result::Ok`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Ok "variant core::result::Result::Ok") is returned, containing the index of the matching element. If there are multiple matches, then any one of the matches could be returned. The index is chosen deterministically, but is subject to change in future versions of Rust. If the value is not found then [`Result::Err`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Err "variant core::result::Result::Err") is returned, containing the index where a matching element could be inserted while maintaining sorted order.

See also [`binary_search`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.binary_search "method slice::binary_search"), [`binary_search_by_key`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.binary_search_by_key "method slice::binary_search_by_key"), and [`partition_point`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.partition_point "method slice::partition_point").

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-46" class="doc-anchor">§</a>Examples

Looks up a series of four elements. The first is found, with a uniquely determined position; the second and third are not found; the fourth could match any position in `[1, 4]`.

``` rust
let s = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];

let seek = 13;
assert_eq!(s.binary_search_by(|probe| probe.cmp(&seek)), Ok(9));
let seek = 4;
assert_eq!(s.binary_search_by(|probe| probe.cmp(&seek)), Err(7));
let seek = 100;
assert_eq!(s.binary_search_by(|probe| probe.cmp(&seek)), Err(13));
let seek = 1;
let r = s.binary_search_by(|probe| probe.cmp(&seek));
assert!(match r { Ok(1..=4) => true, _ => false, });
```

1.10.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#3033-3036" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.binary_search_by_key" class="fn">binary_search_by_key</a>\<'a, B, F\>( &'a self, b: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;B</a>, f: F, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>) -\> B, B: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a>,

Binary searches this slice with a key extraction function.

Assumes that the slice is sorted by the key, for instance with [`sort_by_key`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.sort_by_key "method slice::sort_by_key") using the same key extraction function. If the slice is not sorted by the key, the returned result is unspecified and meaningless.

If the value is found then [`Result::Ok`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Ok "variant core::result::Result::Ok") is returned, containing the index of the matching element. If there are multiple matches, then any one of the matches could be returned. The index is chosen deterministically, but is subject to change in future versions of Rust. If the value is not found then [`Result::Err`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Err "variant core::result::Result::Err") is returned, containing the index where a matching element could be inserted while maintaining sorted order.

See also [`binary_search`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.binary_search "method slice::binary_search"), [`binary_search_by`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.binary_search_by "method slice::binary_search_by"), and [`partition_point`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.partition_point "method slice::partition_point").

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-47" class="doc-anchor">§</a>Examples

Looks up a series of four elements in a slice of pairs sorted by their second elements. The first is found, with a uniquely determined position; the second and third are not found; the fourth could match any position in `[1, 4]`.

``` rust
let s = [(0, 0), (2, 1), (4, 1), (5, 1), (3, 1),
         (1, 2), (2, 3), (4, 5), (5, 8), (3, 13),
         (1, 21), (2, 34), (4, 55)];

assert_eq!(s.binary_search_by_key(&13, |&(a, b)| b),  Ok(9));
assert_eq!(s.binary_search_by_key(&4, |&(a, b)| b),   Err(7));
assert_eq!(s.binary_search_by_key(&100, |&(a, b)| b), Err(13));
let r = s.binary_search_by_key(&1, |&(a, b)| b);
assert!(match r { Ok(1..=4) => true, _ => false, });
```

1.30.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#4056" class="src">Source</a>

#### pub unsafe fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.align_to" class="fn">align_to</a>\<U\>(&self) -\> (&<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>, &<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[U]</a>, &<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>)

Transmutes the slice to a slice of another type, ensuring alignment of the types is maintained.

This method splits the slice into three distinct slices: prefix, correctly aligned middle slice of a new type, and the suffix slice. The middle part will be as big as possible under the given alignment constraint and element size.

This method has no purpose when either input element `T` or output element `U` are zero-sized and will return the original slice without splitting anything.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#safety-7" class="doc-anchor">§</a>Safety

This method is essentially a `transmute` with respect to the elements in the returned middle slice, so all the usual caveats pertaining to `transmute::<T, U>` also apply here.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-48" class="doc-anchor">§</a>Examples

Basic usage:

``` rust
unsafe {
    let bytes: [u8; 7] = [1, 2, 3, 4, 5, 6, 7];
    let (prefix, shorts, suffix) = bytes.align_to::<u16>();
    // less_efficient_algorithm_for_bytes(prefix);
    // more_efficient_algorithm_for_aligned_shorts(shorts);
    // less_efficient_algorithm_for_bytes(suffix);
}
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.as_simd" class="fn">as_simd</a>\<const LANES: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>(&self) -\> (&<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>, &\[<a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<T, LANES\>\], &<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>)

where <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<T, LANES\>: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">[T; LANES]</a>\>, T: <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/trait.SimdElement.html" class="trait" title="trait core::core_simd::vector::SimdElement">SimdElement</a>, <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/struct.LaneCount.html" class="struct" title="struct core::core_simd::lane_count::LaneCount">LaneCount</a>\<LANES\>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/trait.SupportedLaneCount.html" class="trait" title="trait core::core_simd::lane_count::SupportedLaneCount">SupportedLaneCount</a>,

🔬This is a nightly-only experimental API. (`portable_simd`)

Splits a slice into a prefix, a middle of aligned SIMD types, and a suffix.

This is a safe wrapper around [`slice::align_to`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.align_to "method slice::align_to"), so inherits the same guarantees as that method.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#panics-13" class="doc-anchor">§</a>Panics

This will panic if the size of the SIMD type is different from `LANES` times that of the scalar.

At the time of writing, the trait restrictions on `Simd<T, LANES>` keeps that from ever happening, as only power-of-two numbers of lanes are supported. It’s possible that, in the future, those restrictions might be lifted in a way that would make it possible to see panics from this method for something like `LANES == 3`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-49" class="doc-anchor">§</a>Examples

``` rust
#![feature(portable_simd)]
use core::simd::prelude::*;

let short = &[1, 2, 3];
let (prefix, middle, suffix) = short.as_simd::<4>();
assert_eq!(middle, []); // Not enough elements for anything in the middle

// They might be split in any possible way between prefix and suffix
let it = prefix.iter().chain(suffix).copied();
assert_eq!(it.collect::<Vec<_>>(), vec![1, 2, 3]);

fn basic_simd_sum(x: &[f32]) -> f32 {
    use std::ops::Add;
    let (prefix, middle, suffix) = x.as_simd();
    let sums = f32x4::from_array([
        prefix.iter().copied().sum(),
        0.0,
        0.0,
        suffix.iter().copied().sum(),
    ]);
    let sums = middle.iter().copied().fold(sums, f32x4::add);
    sums.reduce_sum()
}

let numbers: Vec<f32> = (1..101).map(|x| x as _).collect();
assert_eq!(basic_simd_sum(&numbers[1..99]), 4949.0);
```

1.82.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#4287-4289" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.is_sorted" class="fn">is_sorted</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a>,

Checks if the elements of this slice are sorted.

That is, for each element `a` and its following element `b`, `a <= b` must hold. If the slice yields exactly zero or one element, `true` is returned.

Note that if `Self::Item` is only `PartialOrd`, but not `Ord`, the above definition implies that this function returns `false` if any two consecutive items are not comparable.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-50" class="doc-anchor">§</a>Examples

``` rust
let empty: [i32; 0] = [];

assert!([1, 2, 2, 9].is_sorted());
assert!(![1, 3, 2, 4].is_sorted());
assert!([0].is_sorted());
assert!(empty.is_sorted());
assert!(![0.0, 1.0, f32::NAN].is_sorted());
```

1.82.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#4330-4332" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.is_sorted_by" class="fn">is_sorted_by</a>\<'a, F\>(&'a self, compare: F) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

Checks if the elements of this slice are sorted using the given comparator function.

Instead of using `PartialOrd::partial_cmp`, this function uses the given `compare` function to determine whether two elements are to be considered in sorted order.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-51" class="doc-anchor">§</a>Examples

``` rust
assert!([1, 2, 2, 9].is_sorted_by(|a, b| a <= b));
assert!(![1, 2, 2, 9].is_sorted_by(|a, b| a < b));

assert!([0].is_sorted_by(|a, b| true));
assert!([0].is_sorted_by(|a, b| false));

let empty: [i32; 0] = [];
assert!(empty.is_sorted_by(|a, b| false));
assert!(empty.is_sorted_by(|a, b| true));
```

1.82.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#4354-4357" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.is_sorted_by_key" class="fn">is_sorted_by_key</a>\<'a, F, K\>(&'a self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>) -\> K, K: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a>,

Checks if the elements of this slice are sorted using the given key extraction function.

Instead of comparing the slice’s elements directly, this function compares the keys of the elements, as determined by `f`. Apart from that, it’s equivalent to [`is_sorted`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.is_sorted "method slice::is_sorted"); see its documentation for more information.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-52" class="doc-anchor">§</a>Examples

``` rust
assert!(["c", "bb", "aaa"].is_sorted_by_key(|s| s.len()));
assert!(![-2i32, -1, 0, 3].is_sorted_by_key(|n| n.abs()));
```

1.52.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#4413-4415" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.partition_point" class="fn">partition_point</a>\<P\>(&self, pred: P) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

where P: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

Returns the index of the partition point according to the given predicate (the index of the first element of the second partition).

The slice is assumed to be partitioned according to the given predicate. This means that all elements for which the predicate returns true are at the start of the slice and all elements for which the predicate returns false are at the end. For example, `[7, 15, 3, 5, 4, 12, 6]` is partitioned under the predicate `x % 2 != 0` (all odd numbers are at the start, all even at the end).

If this slice is not partitioned, the returned result is unspecified and meaningless, as this method performs a kind of binary search.

See also [`binary_search`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.binary_search "method slice::binary_search"), [`binary_search_by`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.binary_search_by "method slice::binary_search_by"), and [`binary_search_by_key`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.binary_search_by_key "method slice::binary_search_by_key").

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-53" class="doc-anchor">§</a>Examples

``` rust
let v = [1, 2, 3, 3, 5, 6, 7];
let i = v.partition_point(|&x| x < 5);

assert_eq!(i, 4);
assert!(v[..i].iter().all(|&x| x < 5));
assert!(v[i..].iter().all(|&x| !(x < 5)));
```

If all elements of the slice match the predicate, including if the slice is empty, then the length of the slice will be returned:

``` rust
let a = [2, 4, 8];
assert_eq!(a.partition_point(|x| x < &100), a.len());
let a: [i32; 0] = [];
assert_eq!(a.partition_point(|x| x < &100), 0);
```

If you want to insert an item to a sorted vector, while maintaining sort order:

``` rust
let mut s = vec![0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
let num = 42;
let idx = s.partition_point(|&x| x <= num);
s.insert(idx, num);
assert_eq!(s, [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 42, 55]);
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.element_offset" class="fn">element_offset</a>(&self, element: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

🔬This is a nightly-only experimental API. (`substr_range`)

Returns the index that an element reference points to.

Returns `None` if `element` does not point to the start of an element within the slice.

This method is useful for extending slice iterators like [`slice::split`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.split "method slice::split").

Note that this uses pointer arithmetic and **does not compare elements**. To find the index of an element via comparison, use [`.iter().position()`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.position "method core::iter::traits::iterator::Iterator::position") instead.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#panics-14" class="doc-anchor">§</a>Panics

Panics if `T` is zero-sized.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-54" class="doc-anchor">§</a>Examples

Basic usage:

``` rust
#![feature(substr_range)]

let nums: &[u32] = &[1, 7, 1, 1];
let num = &nums[2];

assert_eq!(num, &1);
assert_eq!(nums.element_offset(num), Some(2));
```

Returning `None` with an unaligned element:

``` rust
#![feature(substr_range)]

let arr: &[[u32; 2]] = &[[0, 1], [2, 3]];
let flat_arr: &[u32] = arr.as_flattened();

let ok_elm: &[u32; 2] = flat_arr[0..2].try_into().unwrap();
let weird_elm: &[u32; 2] = flat_arr[1..3].try_into().unwrap();

assert_eq!(ok_elm, &[0, 1]);
assert_eq!(weird_elm, &[1, 2]);

assert_eq!(arr.element_offset(ok_elm), Some(0)); // Points to element 0
assert_eq!(arr.element_offset(weird_elm), None); // Points between element 0 and 1
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.subslice_range" class="fn">subslice_range</a>(&self, subslice: &<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\>

🔬This is a nightly-only experimental API. (`substr_range`)

Returns the range of indices that a subslice points to.

Returns `None` if `subslice` does not point within the slice or if it is not aligned with the elements in the slice.

This method **does not compare elements**. Instead, this method finds the location in the slice that `subslice` was obtained from. To find the index of a subslice via comparison, instead use [`.windows()`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.windows "method slice::windows")[`.position()`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.position "method core::iter::traits::iterator::Iterator::position").

This method is useful for extending slice iterators like [`slice::split`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.split "method slice::split").

Note that this may return a false positive (either `Some(0..0)` or `Some(self.len()..self.len())`) if `subslice` has a length of zero and points to the beginning or end of another, separate, slice.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#panics-15" class="doc-anchor">§</a>Panics

Panics if `T` is zero-sized.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-55" class="doc-anchor">§</a>Examples

Basic usage:

``` rust
#![feature(substr_range)]

let nums = &[0, 5, 10, 0, 0, 5];

let mut iter = nums
    .split(|t| *t == 0)
    .map(|n| nums.subslice_range(n).unwrap());

assert_eq!(iter.next(), Some(0..0));
assert_eq!(iter.next(), Some(1..3));
assert_eq!(iter.next(), Some(4..4));
assert_eq!(iter.next(), Some(5..6));
```

1.79.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/lossy.rs.html#44" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.utf8_chunks" class="fn">utf8_chunks</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/str/lossy/struct.Utf8Chunks.html" class="struct" title="struct core::str::lossy::Utf8Chunks">Utf8Chunks</a>\<'\_\>

Creates an iterator over the contiguous valid UTF-8 ranges of this slice, and the non-UTF-8 fragments in between.

See the [`Utf8Chunk`](https://doc.rust-lang.org/nightly/core/str/lossy/struct.Utf8Chunk.html "struct core::str::lossy::Utf8Chunk") type for documentation of the items yielded by this iterator.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-56" class="doc-anchor">§</a>Examples

This function formats arbitrary but mostly-UTF-8 bytes into Rust source code in the form of a C-string literal (`c"..."`).

``` rust
use std::fmt::Write as _;

pub fn cstr_literal(bytes: &[u8]) -> String {
    let mut repr = String::new();
    repr.push_str("c\"");
    for chunk in bytes.utf8_chunks() {
        for ch in chunk.valid().chars() {
            // Escapes \0, \t, \r, \n, \\, \', \", and uses \u{...} for non-printable characters.
            write!(repr, "{}", ch.escape_debug()).unwrap();
        }
        for byte in chunk.invalid() {
            write!(repr, "\\x{:02X}", byte).unwrap();
        }
    }
    repr.push('"');
    repr
}

fn main() {
    let lit = cstr_literal(b"\xferris the \xf0\x9f\xa6\x80\x07");
    let expected = stringify!(c"\xFErris the 🦀\u{7}");
    assert_eq!(lit, expected);
}
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/slice.rs.html#370-372" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.to_vec" class="fn">to_vec</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<T\>

where T: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>,

Copies `self` into a new `Vec`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-57" class="doc-anchor">§</a>Examples

``` rust
let s = [10, 40, 30];
let x = s.to_vec();
// Here, `s` and `x` can be modified independently.
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.to_vec_in" class="fn">to_vec_in</a>\<A\>(&self, alloc: A) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<T, A\>

where A: <a href="https://doc.rust-lang.org/nightly/core/alloc/trait.Allocator.html" class="trait" title="trait core::alloc::Allocator">Allocator</a>, T: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>,

🔬This is a nightly-only experimental API. (`allocator_api`)

Copies `self` into a new `Vec` with an allocator.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-58" class="doc-anchor">§</a>Examples

``` rust
#![feature(allocator_api)]

use std::alloc::System;

let s = [10, 40, 30];
let x = s.to_vec_in(System);
// Here, `s` and `x` can be modified independently.
```

1.40.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/slice.rs.html#505-507" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.repeat" class="fn">repeat</a>(&self, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<T\>

where T: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a>,

Creates a vector by copying a slice `n` times.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#panics-16" class="doc-anchor">§</a>Panics

This function will panic if the capacity would overflow.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-59" class="doc-anchor">§</a>Examples

``` rust
assert_eq!([1, 2].repeat(3), vec![1, 2, 1, 2, 1, 2]);
```

A panic upon overflow:

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#" class="tooltip" title="This example panics">ⓘ</a>

``` rust
// this will panic at runtime
b"0123456789abcdef".repeat(usize::MAX);
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/slice.rs.html#573-575" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.concat" class="fn">concat</a>\<Item\>(&self) -\> \<<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a> as <a href="https://doc.rust-lang.org/nightly/alloc/slice/trait.Concat.html" class="trait" title="trait alloc::slice::Concat">Concat</a>\<Item\>\>::<a href="https://doc.rust-lang.org/nightly/alloc/slice/trait.Concat.html#associatedtype.Output" class="associatedtype" title="type alloc::slice::Concat::Output">Output</a> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#" class="tooltip" data-notable-ty="&lt;[T] as Concat&lt;Item&gt;&gt;::Output">ⓘ</a>

where <a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>: <a href="https://doc.rust-lang.org/nightly/alloc/slice/trait.Concat.html" class="trait" title="trait alloc::slice::Concat">Concat</a>\<Item\>, Item: ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Flattens a slice of `T` into a single value `Self::Output`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-60" class="doc-anchor">§</a>Examples

``` rust
assert_eq!(["hello", "world"].concat(), "helloworld");
assert_eq!([[1, 2], [3, 4]].concat(), [1, 2, 3, 4]);
```

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/slice.rs.html#592-594" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.join" class="fn">join</a>\<Separator\>( &self, sep: Separator, ) -\> \<<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a> as <a href="https://doc.rust-lang.org/nightly/alloc/slice/trait.Join.html" class="trait" title="trait alloc::slice::Join">Join</a>\<Separator\>\>::<a href="https://doc.rust-lang.org/nightly/alloc/slice/trait.Join.html#associatedtype.Output" class="associatedtype" title="type alloc::slice::Join::Output">Output</a> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#" class="tooltip" data-notable-ty="&lt;[T] as Join&lt;Separator&gt;&gt;::Output">ⓘ</a>

where <a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>: <a href="https://doc.rust-lang.org/nightly/alloc/slice/trait.Join.html" class="trait" title="trait alloc::slice::Join">Join</a>\<Separator\>,

Flattens a slice of `T` into a single value `Self::Output`, placing a given separator between each.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-61" class="doc-anchor">§</a>Examples

``` rust
assert_eq!(["hello", "world"].join(" "), "hello world");
assert_eq!([[1, 2], [3, 4]].join(&0), [1, 2, 0, 3, 4]);
assert_eq!([[1, 2], [3, 4]].join(&[0, 0][..]), [1, 2, 0, 0, 3, 4]);
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/slice.rs.html#612-614" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.connect" class="fn">connect</a>\<Separator\>( &self, sep: Separator, ) -\> \<<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a> as <a href="https://doc.rust-lang.org/nightly/alloc/slice/trait.Join.html" class="trait" title="trait alloc::slice::Join">Join</a>\<Separator\>\>::<a href="https://doc.rust-lang.org/nightly/alloc/slice/trait.Join.html#associatedtype.Output" class="associatedtype" title="type alloc::slice::Join::Output">Output</a> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#" class="tooltip" data-notable-ty="&lt;[T] as Join&lt;Separator&gt;&gt;::Output">ⓘ</a>

where <a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>: <a href="https://doc.rust-lang.org/nightly/alloc/slice/trait.Join.html" class="trait" title="trait alloc::slice::Join">Join</a>\<Separator\>,

👎Deprecated since 1.3.0: renamed to join

Flattens a slice of `T` into a single value `Self::Output`, placing a given separator between each.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#examples-62" class="doc-anchor">§</a>Examples

``` rust
assert_eq!(["hello", "world"].connect(" "), "hello world");
assert_eq!([[1, 2], [3, 4]].connect(&0), [1, 2, 0, 3, 4]);
```

1.23.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/slice.rs.html#636" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.to_ascii_uppercase" class="fn">to_ascii_uppercase</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#" class="tooltip" data-notable-ty="Vec&lt;u8&gt;">ⓘ</a>

Returns a vector containing a copy of this slice where each byte is mapped to its ASCII upper case equivalent.

ASCII letters ‘a’ to ‘z’ are mapped to ‘A’ to ‘Z’, but non-ASCII letters are unchanged.

To uppercase the value in-place, use [`make_ascii_uppercase`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.make_ascii_uppercase "method slice::make_ascii_uppercase").

1.23.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/slice.rs.html#657" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.to_ascii_lowercase" class="fn">to_ascii_lowercase</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#" class="tooltip" data-notable-ty="Vec&lt;u8&gt;">ⓘ</a>

Returns a vector containing a copy of this slice where each byte is mapped to its ASCII lower case equivalent.

ASCII letters ‘A’ to ‘Z’ are mapped to ‘a’ to ‘z’, but non-ASCII letters are unchanged.

To lowercase the value in-place, use [`make_ascii_lowercase`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.make_ascii_lowercase "method slice::make_ascii_lowercase").

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#impl-Clone-for-Buffer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#impl-Debug-for-Buffer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#impl-Default-for-Buffer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#impl-Deref-for-Buffer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html" class="trait" title="trait core::ops::deref::Deref">Deref</a> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#associatedtype.Target" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target" class="associatedtype">Target</a> = \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]

The resulting type after dereferencing.

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.deref" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#tymethod.deref" class="fn">deref</a>(&self) -\> &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\] <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#" class="tooltip" data-notable-ty="&amp;[u8]">ⓘ</a>

Dereferences the value.

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#impl-From%3C%26%5Bu8%5D%3E-for-Buffer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

Note that here we deliberately do not implement `impl<T: AsRef<[u8]>> From<T> for Buffer` As it would accept `Buffer::from(vec![...])` that would cause an unexpected copy. Instead, we ask user to be explicit when copying is occurring, e.g., `Buffer::from(vec![...].to_byte_slice())`. For zero-copy conversion, user should use `Buffer::from_vec(vec![...])`.

Since we removed impl for `AsRef<u8>`, we added the following three specific implementations to reduce API breakage. See <https://github.com/apache/arrow-rs/issues/6033> for more discussion on this.

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(p: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#impl-From%3C%26%5Bu8;+N%5D%3E-for-Buffer" class="anchor">§</a>

### impl\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>; <a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">N</a>\]\> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.from-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(p: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>; <a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">N</a>\]) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#impl-From%3C%5Bu8;+N%5D%3E-for-Buffer" class="anchor">§</a>

### impl\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>; <a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">N</a>\]\> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(p: \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>; <a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">N</a>\]) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#impl-From%3CBooleanBufferBuilder%3E-for-Buffer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.BooleanBufferBuilder.html" class="struct" title="struct arrow::array::BooleanBufferBuilder">BooleanBufferBuilder</a>\> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.from-10" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(builder: <a href="https://docs.rs/arrow/latest/arrow/array/struct.BooleanBufferBuilder.html" class="struct" title="struct arrow::array::BooleanBufferBuilder">BooleanBufferBuilder</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#impl-From%3CBuffer%3E-for-ScalarBuffer%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>\> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.ScalarBuffer.html" class="struct" title="struct arrow::buffer::ScalarBuffer">ScalarBuffer</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html" class="trait" title="trait arrow::datatypes::ArrowNativeType">ArrowNativeType</a>,

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.from-9" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(buffer: <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.ScalarBuffer.html" class="struct" title="struct arrow::buffer::ScalarBuffer">ScalarBuffer</a>\<T\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#impl-From%3CBufferBuilder%3CT%3E%3E-for-Buffer" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.BufferBuilder.html" class="struct" title="struct arrow::array::BufferBuilder">BufferBuilder</a>\<T\>\> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html" class="trait" title="trait arrow::datatypes::ArrowNativeType">ArrowNativeType</a>,

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.from-8" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/arrow/latest/arrow/array/struct.BufferBuilder.html" class="struct" title="struct arrow::array::BufferBuilder">BufferBuilder</a>\<T\>) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#impl-From%3CBytes%3E-for-Buffer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<Bytes\> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

Convert from internal `Bytes` (not [`bytes::Bytes`](https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html "struct bytes::bytes::Bytes")) to `Buffer`

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.from-5" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(bytes: Bytes) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#impl-From%3CBytes%3E-for-Buffer-1" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html" class="struct" title="struct bytes::bytes::Bytes">Bytes</a>\> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

Convert from [`bytes::Bytes`](https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html "struct bytes::bytes::Bytes"), not internal `Bytes` to `Buffer`

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.from-6" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(bytes: <a href="https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html" class="struct" title="struct bytes::bytes::Bytes">Bytes</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#impl-From%3CMutableBuffer%3E-for-Buffer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html" class="struct" title="struct arrow::buffer::MutableBuffer">MutableBuffer</a>\> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.from-7" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(buffer: <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html" class="struct" title="struct arrow::buffer::MutableBuffer">MutableBuffer</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#impl-From%3CScalarBuffer%3CT%3E%3E-for-Buffer" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.ScalarBuffer.html" class="struct" title="struct arrow::buffer::ScalarBuffer">ScalarBuffer</a>\<T\>\> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html" class="trait" title="trait arrow::datatypes::ArrowNativeType">ArrowNativeType</a>,

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.from-4" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.ScalarBuffer.html" class="struct" title="struct arrow::buffer::ScalarBuffer">ScalarBuffer</a>\<T\>) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#impl-From%3CVec%3CT%3E%3E-for-Buffer" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<T\>\> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html" class="trait" title="trait arrow::datatypes::ArrowNativeType">ArrowNativeType</a>,

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.from-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<T\>) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#impl-FromIterator%3CT%3E-for-Buffer" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<T\> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html" class="trait" title="trait arrow::datatypes::ArrowNativeType">ArrowNativeType</a>,

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.from_iter-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = T\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#impl-FromIterator%3Cbool%3E-for-Buffer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

Create a `Buffer` instance by storing the boolean values into the buffer

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.from_iter" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#impl-PartialEq-for-Buffer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#impl-Eq-for-Buffer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#impl-Send-for-Buffer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

where Bytes: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#impl-Sync-for-Buffer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

where Bytes: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>,

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html#blanket-implementations" class="anchor">§</a>
