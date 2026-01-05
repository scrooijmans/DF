# Struct MutableBuffer Copy item path

<a href="https://docs.rs/arrow-buffer/56.2.0/x86_64-unknown-linux-gnu/src/arrow_buffer/buffer/mutable.rs.html#59" class="src">Source</a>

``` rust
pub struct MutableBuffer { /* private fields */ }
```

Expand description

A [`MutableBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html "struct arrow::buffer::MutableBuffer") is Arrow’s interface to build a [`Buffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html "struct arrow::buffer::Buffer") out of items or slices of items.

[`Buffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html "struct arrow::buffer::Buffer")s created from [`MutableBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html "struct arrow::buffer::MutableBuffer") (via `into`) are guaranteed to have its pointer aligned along cache lines and in multiple of 64 bytes.

Use [MutableBuffer::push](https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.push "method arrow::buffer::MutableBuffer::push") to insert an item, [MutableBuffer::extend_from_slice](https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.extend_from_slice "method arrow::buffer::MutableBuffer::extend_from_slice") to insert many items, and `into` to convert it to [`Buffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html "struct arrow::buffer::Buffer").

For a safe, strongly typed API consider using [`Vec`](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html "struct alloc::vec::Vec") and [`ScalarBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.ScalarBuffer.html "struct arrow::buffer::ScalarBuffer")

Note: this may be deprecated in a future release ([\#1176](https://github.com/apache/arrow-rs/issues/1176))

## <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#example" class="doc-anchor">§</a>Example

``` rust
let mut buffer = MutableBuffer::new(0);
buffer.push(256u32);
buffer.extend_from_slice(&[1u32]);
let buffer: Buffer = buffer.into();
assert_eq!(buffer.as_slice(), &[0u8, 1, 0, 0, 1, 0, 0, 0])
```

## Implementations<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#impl-MutableBuffer" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html" class="struct" title="struct arrow::buffer::MutableBuffer">MutableBuffer</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.new" class="fn">new</a>(capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html" class="struct" title="struct arrow::buffer::MutableBuffer">MutableBuffer</a>

Allocate a new [MutableBuffer](https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html "struct arrow::buffer::MutableBuffer") with initial capacity to be at least `capacity`.

See [`MutableBuffer::with_capacity`](https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.with_capacity "associated function arrow::buffer::MutableBuffer::with_capacity").

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.with_capacity" class="fn">with_capacity</a>(capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html" class="struct" title="struct arrow::buffer::MutableBuffer">MutableBuffer</a>

Allocate a new [MutableBuffer](https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html "struct arrow::buffer::MutableBuffer") with initial capacity to be at least `capacity`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#panics" class="doc-anchor">§</a>Panics

If `capacity`, when rounded up to the nearest multiple of [`ALIGNMENT`](https://docs.rs/arrow/latest/arrow/alloc/constant.ALIGNMENT.html "constant arrow::alloc::ALIGNMENT"), is greater then `isize::MAX`, then this function will panic.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.from_len_zeroed" class="fn">from_len_zeroed</a>(len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html" class="struct" title="struct arrow::buffer::MutableBuffer">MutableBuffer</a>

Allocates a new [MutableBuffer](https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html "struct arrow::buffer::MutableBuffer") with `len` and capacity to be at least `len` where all bytes are guaranteed to be `0u8`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#example-1" class="doc-anchor">§</a>Example

``` rust
let mut buffer = MutableBuffer::from_len_zeroed(127);
assert_eq!(buffer.len(), 127);
assert!(buffer.capacity() >= 127);
let data = buffer.as_slice_mut();
assert_eq!(data[126], 0u8);
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.new_null" class="fn">new_null</a>(len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html" class="struct" title="struct arrow::buffer::MutableBuffer">MutableBuffer</a>

creates a new [MutableBuffer](https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html "struct arrow::buffer::MutableBuffer") with capacity and length capable of holding `len` bits. This is useful to create a buffer for packed bitmaps.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.with_bitset" class="fn">with_bitset</a>(self, end: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, val: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html" class="struct" title="struct arrow::buffer::MutableBuffer">MutableBuffer</a>

Set the bits in the range of `[0, end)` to 0 (if `val` is false), or 1 (if `val` is true). Also extend the length of this buffer to be `end`.

This is useful when one wants to clear (or set) the bits and then manipulate the buffer directly (e.g., modifying the buffer by holding a mutable reference from `data_mut()`).

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.set_null_bits" class="fn">set_null_bits</a>(&mut self, start: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, count: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Ensure that `count` bytes from `start` contain zero bits

This is used to initialize the bits in a buffer, however, it has no impact on the `len` of the buffer and so can be used to initialize the memory region from `len` to `capacity`.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.reserve" class="fn">reserve</a>(&mut self, additional: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Ensures that this buffer has at least `self.len + additional` bytes. This re-allocates iff `self.len + additional > capacity`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#example-2" class="doc-anchor">§</a>Example

``` rust
let mut buffer = MutableBuffer::new(0);
buffer.reserve(253); // allocates for the first time
(0..253u8).for_each(|i| buffer.push(i)); // no reallocation
let buffer: Buffer = buffer.into();
assert_eq!(buffer.len(), 253);
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.truncate" class="fn">truncate</a>(&mut self, len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Truncates this buffer to `len` bytes

If `len` is greater than the buffer’s current length, this has no effect

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.resize" class="fn">resize</a>(&mut self, new_len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, value: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>)

Resizes the buffer, either truncating its contents (with no change in capacity), or growing it (potentially reallocating it) and writing `value` in the newly available bytes.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#example-3" class="doc-anchor">§</a>Example

``` rust
let mut buffer = MutableBuffer::new(0);
buffer.resize(253, 2); // allocates for the first time
assert_eq!(buffer.as_slice()[252], 2u8);
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.shrink_to_fit" class="fn">shrink_to_fit</a>(&mut self)

Shrinks the capacity of the buffer as much as possible. The new capacity will aligned to the nearest 64 bit alignment.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#example-4" class="doc-anchor">§</a>Example

``` rust
// 2 cache lines
let mut buffer = MutableBuffer::new(128);
assert_eq!(buffer.capacity(), 128);
buffer.push(1);
buffer.push(2);

buffer.shrink_to_fit();
assert!(buffer.capacity() >= 64 && buffer.capacity() < 128);
```

#### pub const fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether this buffer is empty or not.

#### pub const fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the length (the number of bytes written) in this buffer. The invariant `buffer.len() <= buffer.capacity()` is always upheld.

#### pub const fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.capacity" class="fn">capacity</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total capacity in this buffer, in bytes.

The invariant `buffer.len() <= buffer.capacity()` is always upheld.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.clear" class="fn">clear</a>(&mut self)

Clear all existing data from this buffer.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.as_slice" class="fn">as_slice</a>(&self) -\> &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\] <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#" class="tooltip" data-notable-ty="&amp;[u8]">ⓘ</a>

Returns the data stored in this buffer as a slice.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.as_slice_mut" class="fn">as_slice_mut</a>(&mut self) -\> &mut \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\] <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#" class="tooltip" data-notable-ty="&amp;mut [u8]">ⓘ</a>

Returns the data stored in this buffer as a mutable slice.

#### pub const fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.as_ptr" class="fn">as_ptr</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.pointer.html" class="primitive">*const</a> <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

Returns a raw pointer to this buffer’s internal memory This pointer is guaranteed to be aligned along cache-lines.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.as_mut_ptr" class="fn">as_mut_ptr</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.pointer.html" class="primitive">*mut</a> <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

Returns a mutable raw pointer to this buffer’s internal memory This pointer is guaranteed to be aligned along cache-lines.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.typed_data_mut" class="fn">typed_data_mut</a>\<T\>(&mut self) -\> &mut <a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html" class="trait" title="trait arrow::datatypes::ArrowNativeType">ArrowNativeType</a>,

View this buffer as a mutable slice of a specific type.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#panics-1" class="doc-anchor">§</a>Panics

This function panics if the underlying buffer is not aligned correctly for type `T`.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.typed_data" class="fn">typed_data</a>\<T\>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html" class="trait" title="trait arrow::datatypes::ArrowNativeType">ArrowNativeType</a>,

View buffer as a immutable slice of a specific type.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#panics-2" class="doc-anchor">§</a>Panics

This function panics if the underlying buffer is not aligned correctly for type `T`.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.extend_from_slice" class="fn">extend_from_slice</a>\<T\>(&mut self, items: &<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>)

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html" class="trait" title="trait arrow::datatypes::ArrowNativeType">ArrowNativeType</a>,

Extends this buffer from a slice of items that can be represented in bytes, increasing its capacity if needed.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#example-5" class="doc-anchor">§</a>Example

``` rust
let mut buffer = MutableBuffer::new(0);
buffer.extend_from_slice(&[2u32, 0]);
assert_eq!(buffer.len(), 8) // u32 has 4 bytes
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.push" class="fn">push</a>\<T\>(&mut self, item: T)

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ToByteSlice.html" class="trait" title="trait arrow::datatypes::ToByteSlice">ToByteSlice</a>,

Extends the buffer with a new item, increasing its capacity if needed.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#example-6" class="doc-anchor">§</a>Example

``` rust
let mut buffer = MutableBuffer::new(0);
buffer.push(256u32);
assert_eq!(buffer.len(), 4) // u32 has 4 bytes
```

#### pub unsafe fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.push_unchecked" class="fn">push_unchecked</a>\<T\>(&mut self, item: T)

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ToByteSlice.html" class="trait" title="trait arrow::datatypes::ToByteSlice">ToByteSlice</a>,

Extends the buffer with a new item, without checking for sufficient capacity

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#safety" class="doc-anchor">§</a>Safety

Caller must ensure that the capacity()-len()\>=`size_of<T>`()

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.extend_zeros" class="fn">extend_zeros</a>(&mut self, additional: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Extends the buffer by `additional` bytes equal to `0u8`, incrementing its capacity if needed.

#### pub unsafe fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.set_len" class="fn">set_len</a>(&mut self, len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#safety-1" class="doc-anchor">§</a>Safety

The caller must ensure that the buffer was properly initialized up to `len`.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.collect_bool" class="fn">collect_bool</a>\<F\>(len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, f: F) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html" class="struct" title="struct arrow::buffer::MutableBuffer">MutableBuffer</a>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

Invokes `f` with values `0..len` collecting the boolean results into a new `MutableBuffer`

This is similar to `from_trusted_len_iter_bool`, however, can be significantly faster as it eliminates the conditional `Iterator::next`

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#impl-MutableBuffer-1" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html" class="struct" title="struct arrow::buffer::MutableBuffer">MutableBuffer</a>

#### pub unsafe fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.from_trusted_len_iter" class="fn">from_trusted_len_iter</a>\<T, I\>(iterator: I) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html" class="struct" title="struct arrow::buffer::MutableBuffer">MutableBuffer</a>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html" class="trait" title="trait arrow::datatypes::ArrowNativeType">ArrowNativeType</a>, I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = T\>,

Creates a [`MutableBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html "struct arrow::buffer::MutableBuffer") from an [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") with a trusted (upper) length. Prefer this to `collect` whenever possible, as it is faster ~60% faster.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#example-7" class="doc-anchor">§</a>Example

``` rust
let v = vec![1u32];
let iter = v.iter().map(|x| x * 2);
let buffer = unsafe { MutableBuffer::from_trusted_len_iter(iter) };
assert_eq!(buffer.len(), 4) // u32 has 4 bytes
```

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#safety-2" class="doc-anchor">§</a>Safety

This method assumes that the iterator’s size is correct and is undefined behavior to use it on an iterator that reports an incorrect length.

#### pub unsafe fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.from_trusted_len_iter_bool" class="fn">from_trusted_len_iter_bool</a>\<I\>(iterator: I) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html" class="struct" title="struct arrow::buffer::MutableBuffer">MutableBuffer</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>,

Creates a [`MutableBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html "struct arrow::buffer::MutableBuffer") from a boolean [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") with a trusted (upper) length.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#use-arrow_bufferbuffermutablebuffer" class="doc-anchor">§</a>use arrow_buffer::buffer::MutableBuffer;

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#example-8" class="doc-anchor">§</a>Example

``` rust
let v = vec![false, true, false];
let iter = v.iter().map(|x| *x || true);
let buffer = unsafe { MutableBuffer::from_trusted_len_iter_bool(iter) };
assert_eq!(buffer.len(), 1) // 3 booleans have 1 byte
```

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#safety-3" class="doc-anchor">§</a>Safety

This method assumes that the iterator’s size is correct and is undefined behavior to use it on an iterator that reports an incorrect length.

#### pub unsafe fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.try_from_trusted_len_iter" class="fn">try_from_trusted_len_iter</a>\<E, T, I\>( iterator: I, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html" class="struct" title="struct arrow::buffer::MutableBuffer">MutableBuffer</a>, E\>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html" class="trait" title="trait arrow::datatypes::ArrowNativeType">ArrowNativeType</a>, I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<T, E\>\>,

Creates a [`MutableBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html "struct arrow::buffer::MutableBuffer") from an [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") with a trusted (upper) length or errors if any of the items of the iterator is an error. Prefer this to `collect` whenever possible, as it is faster ~60% faster.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#safety-4" class="doc-anchor">§</a>Safety

This method assumes that the iterator’s size is correct and is undefined behavior to use it on an iterator that reports an incorrect length.

## Methods from <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html" class="trait" title="trait core::ops::deref::Deref">Deref</a>\<Target = \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\><a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#deref-methods-%5Bu8%5D" class="anchor">§</a>

1.23.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/ascii.rs.html#16" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.is_ascii" class="fn">is_ascii</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Checks if all bytes in this slice are within the ASCII range.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.as_ascii" class="fn">as_ascii</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&\[<a href="https://doc.rust-lang.org/nightly/core/ascii/ascii_char/enum.AsciiChar.html" class="enum" title="enum core::ascii::ascii_char::AsciiChar">AsciiChar</a>\]\>

🔬This is a nightly-only experimental API. (`ascii_char`)

If this slice [`is_ascii`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.is_ascii "method slice::is_ascii"), returns it as a slice of [ASCII characters](https://doc.rust-lang.org/nightly/core/ascii/ascii_char/enum.AsciiChar.html "enum core::ascii::ascii_char::AsciiChar"), otherwise returns `None`.

#### pub unsafe fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.as_ascii_unchecked" class="fn">as_ascii_unchecked</a>(&self) -\> &\[<a href="https://doc.rust-lang.org/nightly/core/ascii/ascii_char/enum.AsciiChar.html" class="enum" title="enum core::ascii::ascii_char::AsciiChar">AsciiChar</a>\]

🔬This is a nightly-only experimental API. (`ascii_char`)

Converts this slice of bytes into a slice of ASCII characters, without checking whether they’re valid.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#safety-5" class="doc-anchor">§</a>Safety

Every byte in the slice must be in `0..=127`, or else this is UB.

1.23.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/ascii.rs.html#58" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.eq_ignore_ascii_case" class="fn">eq_ignore_ascii_case</a>(&self, other: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Checks that two slices are an ASCII case-insensitive match.

Same as `to_ascii_lowercase(a) == to_ascii_lowercase(b)`, but without allocating and copying temporaries.

1.23.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/ascii.rs.html#93" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.make_ascii_uppercase" class="fn">make_ascii_uppercase</a>(&mut self)

Converts this slice to its ASCII upper case equivalent in-place.

ASCII letters ‘a’ to ‘z’ are mapped to ‘A’ to ‘Z’, but non-ASCII letters are unchanged.

To return a new uppercased value without modifying the existing one, use [`to_ascii_uppercase`](https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.to_ascii_uppercase).

1.23.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/ascii.rs.html#115" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.make_ascii_lowercase" class="fn">make_ascii_lowercase</a>(&mut self)

Converts this slice to its ASCII lower case equivalent in-place.

ASCII letters ‘A’ to ‘Z’ are mapped to ‘a’ to ‘z’, but non-ASCII letters are unchanged.

To return a new lowercased value without modifying the existing one, use [`to_ascii_lowercase`](https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.to_ascii_lowercase).

1.60.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/ascii.rs.html#138" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.escape_ascii" class="fn">escape_ascii</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/slice/ascii/struct.EscapeAscii.html" class="struct" title="struct core::slice::ascii::EscapeAscii">EscapeAscii</a>\<'\_\>

Returns an iterator that produces an escaped version of this slice, treating it as an ASCII string.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples" class="doc-anchor">§</a>Examples

``` rust
let s = b"0\t\r\n'\"\\\x9d";
let escaped = s.escape_ascii().to_string();
assert_eq!(escaped, "0\\t\\r\\n\\'\\\"\\\\\\x9d");
```

1.80.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/ascii.rs.html#157" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.trim_ascii_start" class="fn">trim_ascii_start</a>(&self) -\> &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\] <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#" class="tooltip" data-notable-ty="&amp;[u8]">ⓘ</a>

Returns a byte slice with leading ASCII whitespace bytes removed.

‘Whitespace’ refers to the definition used by [`u8::is_ascii_whitespace`](https://doc.rust-lang.org/nightly/std/primitive.u8.html#method.is_ascii_whitespace "method u8::is_ascii_whitespace").

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-1" class="doc-anchor">§</a>Examples

``` rust
assert_eq!(b" \t hello world\n".trim_ascii_start(), b"hello world\n");
assert_eq!(b"  ".trim_ascii_start(), b"");
assert_eq!(b"".trim_ascii_start(), b"");
```

1.80.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/ascii.rs.html#186" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.trim_ascii_end" class="fn">trim_ascii_end</a>(&self) -\> &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\] <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#" class="tooltip" data-notable-ty="&amp;[u8]">ⓘ</a>

Returns a byte slice with trailing ASCII whitespace bytes removed.

‘Whitespace’ refers to the definition used by [`u8::is_ascii_whitespace`](https://doc.rust-lang.org/nightly/std/primitive.u8.html#method.is_ascii_whitespace "method u8::is_ascii_whitespace").

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-2" class="doc-anchor">§</a>Examples

``` rust
assert_eq!(b"\r hello world\n ".trim_ascii_end(), b"\r hello world");
assert_eq!(b"  ".trim_ascii_end(), b"");
assert_eq!(b"".trim_ascii_end(), b"");
```

1.80.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/ascii.rs.html#216" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.trim_ascii" class="fn">trim_ascii</a>(&self) -\> &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\] <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#" class="tooltip" data-notable-ty="&amp;[u8]">ⓘ</a>

Returns a byte slice with leading and trailing ASCII whitespace bytes removed.

‘Whitespace’ refers to the definition used by [`u8::is_ascii_whitespace`](https://doc.rust-lang.org/nightly/std/primitive.u8.html#method.is_ascii_whitespace "method u8::is_ascii_whitespace").

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-3" class="doc-anchor">§</a>Examples

``` rust
assert_eq!(b"\r hello world\n ".trim_ascii(), b"hello world");
assert_eq!(b"  ".trim_ascii(), b"");
assert_eq!(b"".trim_ascii(), b"");
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#114" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.len-1" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the number of elements in the slice.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-4" class="doc-anchor">§</a>Examples

``` rust
let a = [1, 2, 3];
assert_eq!(a.len(), 3);
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#134" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.is_empty-1" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns `true` if the slice has a length of 0.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-5" class="doc-anchor">§</a>Examples

``` rust
let a = [1, 2, 3];
assert!(!a.is_empty());

let b: &[i32] = &[];
assert!(b.is_empty());
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#153" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.first" class="fn">first</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>\>

Returns the first element of the slice, or `None` if it is empty.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-6" class="doc-anchor">§</a>Examples

``` rust
let v = [10, 40, 30];
assert_eq!(Some(&10), v.first());

let w: &[i32] = &[];
assert_eq!(None, w.first());
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#176" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.first_mut" class="fn">first_mut</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut T</a>\>

Returns a mutable reference to the first element of the slice, or `None` if it is empty.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-7" class="doc-anchor">§</a>Examples

``` rust
let x = &mut [0, 1, 2];

if let Some(first) = x.first_mut() {
    *first = 5;
}
assert_eq!(x, &[5, 1, 2]);

let y: &mut [i32] = &mut [];
assert_eq!(None, y.first_mut());
```

1.5.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#196" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.split_first" class="fn">split_first</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>, &<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>)\>

Returns the first and all the rest of the elements of the slice, or `None` if it is empty.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-8" class="doc-anchor">§</a>Examples

``` rust
let x = &[0, 1, 2];

if let Some((first, elements)) = x.split_first() {
    assert_eq!(first, &0);
    assert_eq!(elements, &[1, 2]);
}
```

1.5.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#218" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.split_first_mut" class="fn">split_first_mut</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut T</a>, &mut <a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>)\>

Returns the first and all the rest of the elements of the slice, or `None` if it is empty.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-9" class="doc-anchor">§</a>Examples

``` rust
let x = &mut [0, 1, 2];

if let Some((first, elements)) = x.split_first_mut() {
    *first = 3;
    elements[0] = 4;
    elements[1] = 5;
}
assert_eq!(x, &[3, 4, 5]);
```

1.5.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#238" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.split_last" class="fn">split_last</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>, &<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>)\>

Returns the last and all the rest of the elements of the slice, or `None` if it is empty.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-10" class="doc-anchor">§</a>Examples

``` rust
let x = &[0, 1, 2];

if let Some((last, elements)) = x.split_last() {
    assert_eq!(last, &2);
    assert_eq!(elements, &[0, 1]);
}
```

1.5.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#260" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.split_last_mut" class="fn">split_last_mut</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut T</a>, &mut <a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>)\>

Returns the last and all the rest of the elements of the slice, or `None` if it is empty.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-11" class="doc-anchor">§</a>Examples

``` rust
let x = &mut [0, 1, 2];

if let Some((last, elements)) = x.split_last_mut() {
    *last = 3;
    elements[0] = 4;
    elements[1] = 5;
}
assert_eq!(x, &[4, 5, 3]);
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#279" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.last" class="fn">last</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>\>

Returns the last element of the slice, or `None` if it is empty.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-12" class="doc-anchor">§</a>Examples

``` rust
let v = [10, 40, 30];
assert_eq!(Some(&30), v.last());

let w: &[i32] = &[];
assert_eq!(None, w.last());
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#302" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.last_mut" class="fn">last_mut</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut T</a>\>

Returns a mutable reference to the last item in the slice, or `None` if it is empty.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-13" class="doc-anchor">§</a>Examples

``` rust
let x = &mut [0, 1, 2];

if let Some(last) = x.last_mut() {
    *last = 10;
}
assert_eq!(x, &[0, 1, 10]);

let y: &mut [i32] = &mut [];
assert_eq!(None, y.last_mut());
```

1.77.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#325" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.first_chunk" class="fn">first_chunk</a>\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">[T; N]</a>\>

Returns an array reference to the first `N` items in the slice.

If the slice is not at least `N` in length, this will return `None`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-14" class="doc-anchor">§</a>Examples

``` rust
let u = [10, 40, 30];
assert_eq!(Some(&[10, 40]), u.first_chunk::<2>());

let v: &[i32] = &[10];
assert_eq!(None, v.first_chunk::<2>());

let w: &[i32] = &[];
assert_eq!(Some(&[]), w.first_chunk::<0>());
```

1.77.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#355" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.first_chunk_mut" class="fn">first_chunk_mut</a>\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&mut <a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">[T; N]</a>\>

Returns a mutable array reference to the first `N` items in the slice.

If the slice is not at least `N` in length, this will return `None`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-15" class="doc-anchor">§</a>Examples

``` rust
let x = &mut [0, 1, 2];

if let Some(first) = x.first_chunk_mut::<2>() {
    first[0] = 5;
    first[1] = 4;
}
assert_eq!(x, &[5, 4, 2]);

assert_eq!(None, x.first_chunk_mut::<4>());
```

1.77.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#385" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.split_first_chunk" class="fn">split_first_chunk</a>\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(&<a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">[T; N]</a>, &<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>)\>

Returns an array reference to the first `N` items in the slice and the remaining slice.

If the slice is not at least `N` in length, this will return `None`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-16" class="doc-anchor">§</a>Examples

``` rust
let x = &[0, 1, 2];

if let Some((first, elements)) = x.split_first_chunk::<2>() {
    assert_eq!(first, &[0, 1]);
    assert_eq!(elements, &[2]);
}

assert_eq!(None, x.split_first_chunk::<4>());
```

1.77.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#415-417" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.split_first_chunk_mut" class="fn">split_first_chunk_mut</a>\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>( &mut self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(&mut <a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">[T; N]</a>, &mut <a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>)\>

Returns a mutable array reference to the first `N` items in the slice and the remaining slice.

If the slice is not at least `N` in length, this will return `None`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-17" class="doc-anchor">§</a>Examples

``` rust
let x = &mut [0, 1, 2];

if let Some((first, elements)) = x.split_first_chunk_mut::<2>() {
    first[0] = 3;
    first[1] = 4;
    elements[0] = 5;
}
assert_eq!(x, &[3, 4, 5]);

assert_eq!(None, x.split_first_chunk_mut::<4>());
```

1.77.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#445" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.split_last_chunk" class="fn">split_last_chunk</a>\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(&<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>, &<a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">[T; N]</a>)\>

Returns an array reference to the last `N` items in the slice and the remaining slice.

If the slice is not at least `N` in length, this will return `None`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-18" class="doc-anchor">§</a>Examples

``` rust
let x = &[0, 1, 2];

if let Some((elements, last)) = x.split_last_chunk::<2>() {
    assert_eq!(elements, &[0]);
    assert_eq!(last, &[1, 2]);
}

assert_eq!(None, x.split_last_chunk::<4>());
```

1.77.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#476-478" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.split_last_chunk_mut" class="fn">split_last_chunk_mut</a>\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>( &mut self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(&mut <a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>, &mut <a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">[T; N]</a>)\>

Returns a mutable array reference to the last `N` items in the slice and the remaining slice.

If the slice is not at least `N` in length, this will return `None`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-19" class="doc-anchor">§</a>Examples

``` rust
let x = &mut [0, 1, 2];

if let Some((elements, last)) = x.split_last_chunk_mut::<2>() {
    last[0] = 3;
    last[1] = 4;
    elements[0] = 5;
}
assert_eq!(x, &[5, 3, 4]);

assert_eq!(None, x.split_last_chunk_mut::<4>());
```

1.77.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#507" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.last_chunk" class="fn">last_chunk</a>\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">[T; N]</a>\>

Returns an array reference to the last `N` items in the slice.

If the slice is not at least `N` in length, this will return `None`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-20" class="doc-anchor">§</a>Examples

``` rust
let u = [10, 40, 30];
assert_eq!(Some(&[40, 30]), u.last_chunk::<2>());

let v: &[i32] = &[10];
assert_eq!(None, v.last_chunk::<2>());

let w: &[i32] = &[];
assert_eq!(Some(&[]), w.last_chunk::<0>());
```

1.77.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#537" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.last_chunk_mut" class="fn">last_chunk_mut</a>\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&mut <a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">[T; N]</a>\>

Returns a mutable array reference to the last `N` items in the slice.

If the slice is not at least `N` in length, this will return `None`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-21" class="doc-anchor">§</a>Examples

``` rust
let x = &mut [0, 1, 2];

if let Some(last) = x.last_chunk_mut::<2>() {
    last[0] = 10;
    last[1] = 20;
}
assert_eq!(x, &[0, 10, 20]);

assert_eq!(None, x.last_chunk_mut::<4>());
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#570-572" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.get" class="fn">get</a>\<I\>(&self, index: I) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&\<I as <a href="https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html" class="trait" title="trait core::slice::index::SliceIndex">SliceIndex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>\>\>::<a href="https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html#associatedtype.Output" class="associatedtype" title="type core::slice::index::SliceIndex::Output">Output</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html" class="trait" title="trait core::slice::index::SliceIndex">SliceIndex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>\>,

Returns a reference to an element or subslice depending on the type of index.

- If given a position, returns a reference to the element at that position or `None` if out of bounds.
- If given a range, returns the subslice corresponding to that range, or `None` if out of bounds.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-22" class="doc-anchor">§</a>Examples

``` rust
let v = [10, 40, 30];
assert_eq!(Some(&40), v.get(1));
assert_eq!(Some(&[10, 40][..]), v.get(0..2));
assert_eq!(None, v.get(3));
assert_eq!(None, v.get(0..4));
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#597-599" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.get_mut" class="fn">get_mut</a>\<I\>( &mut self, index: I, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&mut \<I as <a href="https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html" class="trait" title="trait core::slice::index::SliceIndex">SliceIndex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>\>\>::<a href="https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html#associatedtype.Output" class="associatedtype" title="type core::slice::index::SliceIndex::Output">Output</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html" class="trait" title="trait core::slice::index::SliceIndex">SliceIndex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>\>,

Returns a mutable reference to an element or subslice depending on the type of index (see [`get`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.get "method slice::get")) or `None` if the index is out of bounds.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-23" class="doc-anchor">§</a>Examples

``` rust
let x = &mut [0, 1, 2];

if let Some(elem) = x.get_mut(1) {
    *elem = 42;
}
assert_eq!(x, &[0, 42, 2]);
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#637-639" class="src">Source</a>

#### pub unsafe fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.get_unchecked" class="fn">get_unchecked</a>\<I\>( &self, index: I, ) -\> &\<I as <a href="https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html" class="trait" title="trait core::slice::index::SliceIndex">SliceIndex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>\>\>::<a href="https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html#associatedtype.Output" class="associatedtype" title="type core::slice::index::SliceIndex::Output">Output</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html" class="trait" title="trait core::slice::index::SliceIndex">SliceIndex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>\>,

Returns a reference to an element or subslice, without doing bounds checking.

For a safe alternative see [`get`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.get "method slice::get").

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#safety-6" class="doc-anchor">§</a>Safety

Calling this method with an out-of-bounds index is *[undefined behavior](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)* even if the resulting reference is not used.

You can think of this like `.get(index).unwrap_unchecked()`. It’s UB to call `.get_unchecked(len)`, even if you immediately convert to a pointer. And it’s UB to call `.get_unchecked(..len + 1)`, `.get_unchecked(..=len)`, or similar.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-24" class="doc-anchor">§</a>Examples

``` rust
let x = &[1, 2, 4];

unsafe {
    assert_eq!(x.get_unchecked(1), &2);
}
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#682-684" class="src">Source</a>

#### pub unsafe fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.get_unchecked_mut" class="fn">get_unchecked_mut</a>\<I\>( &mut self, index: I, ) -\> &mut \<I as <a href="https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html" class="trait" title="trait core::slice::index::SliceIndex">SliceIndex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>\>\>::<a href="https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html#associatedtype.Output" class="associatedtype" title="type core::slice::index::SliceIndex::Output">Output</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html" class="trait" title="trait core::slice::index::SliceIndex">SliceIndex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>\>,

Returns a mutable reference to an element or subslice, without doing bounds checking.

For a safe alternative see [`get_mut`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.get_mut "method slice::get_mut").

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#safety-7" class="doc-anchor">§</a>Safety

Calling this method with an out-of-bounds index is *[undefined behavior](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)* even if the resulting reference is not used.

You can think of this like `.get_mut(index).unwrap_unchecked()`. It’s UB to call `.get_unchecked_mut(len)`, even if you immediately convert to a pointer. And it’s UB to call `.get_unchecked_mut(..len + 1)`, `.get_unchecked_mut(..=len)`, or similar.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-25" class="doc-anchor">§</a>Examples

``` rust
let x = &mut [1, 2, 4];

unsafe {
    let elem = x.get_unchecked_mut(1);
    *elem = 13;
}
assert_eq!(x, &[1, 13, 4]);
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#724" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.as_ptr-1" class="fn">as_ptr</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.pointer.html" class="primitive">*const T</a>

Returns a raw pointer to the slice’s buffer.

The caller must ensure that the slice outlives the pointer this function returns, or else it will end up dangling.

The caller must also ensure that the memory the pointer (non-transitively) points to is never written to (except inside an `UnsafeCell`) using this pointer or any pointer derived from it. If you need to mutate the contents of the slice, use [`as_mut_ptr`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_mut_ptr "method slice::as_mut_ptr").

Modifying the container referenced by this slice may cause its buffer to be reallocated, which would also make any pointers to it invalid.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-26" class="doc-anchor">§</a>Examples

``` rust
let x = &[1, 2, 4];
let x_ptr = x.as_ptr();

unsafe {
    for i in 0..x.len() {
        assert_eq!(x.get_unchecked(i), &*x_ptr.add(i));
    }
}
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#755" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.as_mut_ptr-1" class="fn">as_mut_ptr</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.pointer.html" class="primitive">*mut T</a>

Returns an unsafe mutable pointer to the slice’s buffer.

The caller must ensure that the slice outlives the pointer this function returns, or else it will end up dangling.

Modifying the container referenced by this slice may cause its buffer to be reallocated, which would also make any pointers to it invalid.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-27" class="doc-anchor">§</a>Examples

``` rust
let x = &mut [1, 2, 4];
let x_ptr = x.as_mut_ptr();

unsafe {
    for i in 0..x.len() {
        *x_ptr.add(i) += 2;
    }
}
assert_eq!(x, &[3, 4, 6]);
```

1.48.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#791" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.as_ptr_range" class="fn">as_ptr_range</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.pointer.html" class="primitive">*const T</a>\>

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

1.48.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#834" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.as_mut_ptr_range" class="fn">as_mut_ptr_range</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.pointer.html" class="primitive">*mut T</a>\>

Returns the two unsafe mutable pointers spanning the slice.

The returned range is half-open, which means that the end pointer points *one past* the last element of the slice. This way, an empty slice is represented by two equal pointers, and the difference between the two pointers represents the size of the slice.

See [`as_mut_ptr`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_mut_ptr "method slice::as_mut_ptr") for warnings on using these pointers. The end pointer requires extra caution, as it does not point to a valid element in the slice.

This function is useful for interacting with foreign interfaces which use two pointers to refer to a range of elements in memory, as is common in C++.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.as_array" class="fn">as_array</a>\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">[T; N]</a>\>

🔬This is a nightly-only experimental API. (`slice_as_array`)

Gets a reference to the underlying array.

If `N` is not exactly equal to the length of `self`, then this method returns `None`.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.as_mut_array" class="fn">as_mut_array</a>\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&mut <a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">[T; N]</a>\>

🔬This is a nightly-only experimental API. (`slice_as_array`)

Gets a mutable reference to the slice’s underlying array.

If `N` is not exactly equal to the length of `self`, then this method returns `None`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#901" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.swap" class="fn">swap</a>(&mut self, a: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, b: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Swaps two elements in the slice.

If `a` equals to `b`, it’s guaranteed that elements won’t change value.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#arguments" class="doc-anchor">§</a>Arguments

- a - The index of the first element
- b - The index of the second element

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#panics-3" class="doc-anchor">§</a>Panics

Panics if `a` or `b` are out of bounds.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-28" class="doc-anchor">§</a>Examples

``` rust
let mut v = ["a", "b", "c", "d", "e"];
v.swap(2, 4);
assert!(v == ["a", "b", "e", "d", "c"]);
```

#### pub unsafe fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.swap_unchecked" class="fn">swap_unchecked</a>(&mut self, a: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, b: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

🔬This is a nightly-only experimental API. (`slice_swap_unchecked`)

Swaps two elements in the slice, without doing bounds checking.

For a safe alternative see [`swap`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.swap "method slice::swap").

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#arguments-1" class="doc-anchor">§</a>Arguments

- a - The index of the first element
- b - The index of the second element

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#safety-8" class="doc-anchor">§</a>Safety

Calling this method with an out-of-bounds index is *[undefined behavior](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)*. The caller has to ensure that `a < self.len()` and `b < self.len()`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-29" class="doc-anchor">§</a>Examples

``` rust
#![feature(slice_swap_unchecked)]

let mut v = ["a", "b", "c", "d"];
// SAFETY: we know that 1 and 3 are both indices of the slice
unsafe { v.swap_unchecked(1, 3) };
assert!(v == ["a", "d", "c", "b"]);
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#974" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.reverse" class="fn">reverse</a>(&mut self)

Reverses the order of elements in the slice, in place.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-30" class="doc-anchor">§</a>Examples

``` rust
let mut v = [1, 2, 3];
v.reverse();
assert!(v == [3, 2, 1]);
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1036" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.iter" class="fn">iter</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/slice/iter/struct.Iter.html" class="struct" title="struct core::slice::iter::Iter">Iter</a>\<'\_, T\>

Returns an iterator over the slice.

The iterator yields all items from start to end.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-31" class="doc-anchor">§</a>Examples

``` rust
let x = &[1, 2, 4];
let mut iterator = x.iter();

assert_eq!(iterator.next(), Some(&1));
assert_eq!(iterator.next(), Some(&2));
assert_eq!(iterator.next(), Some(&4));
assert_eq!(iterator.next(), None);
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1056" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.iter_mut" class="fn">iter_mut</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/slice/iter/struct.IterMut.html" class="struct" title="struct core::slice::iter::IterMut">IterMut</a>\<'\_, T\>

Returns an iterator that allows modifying each value.

The iterator yields all items from start to end.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-32" class="doc-anchor">§</a>Examples

``` rust
let x = &mut [1, 2, 4];
for elem in x.iter_mut() {
    *elem += 2;
}
assert_eq!(x, &[3, 4, 6]);
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1111" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.windows" class="fn">windows</a>(&self, size: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/slice/iter/struct.Windows.html" class="struct" title="struct core::slice::iter::Windows">Windows</a>\<'\_, T\>

Returns an iterator over all contiguous windows of length `size`. The windows overlap. If the slice is shorter than `size`, the iterator returns no values.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#panics-4" class="doc-anchor">§</a>Panics

Panics if `size` is zero.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-33" class="doc-anchor">§</a>Examples

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

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.chunks" class="fn">chunks</a>(&self, chunk_size: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/slice/iter/struct.Chunks.html" class="struct" title="struct core::slice::iter::Chunks">Chunks</a>\<'\_, T\>

Returns an iterator over `chunk_size` elements of the slice at a time, starting at the beginning of the slice.

The chunks are slices and do not overlap. If `chunk_size` does not divide the length of the slice, then the last chunk will not have length `chunk_size`.

See [`chunks_exact`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.chunks_exact "method slice::chunks_exact") for a variant of this iterator that returns chunks of always exactly `chunk_size` elements, and [`rchunks`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.rchunks "method slice::rchunks") for the same iterator but starting at the end of the slice.

If your `chunk_size` is a constant, consider using [`as_chunks`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_chunks "method slice::as_chunks") instead, which will give references to arrays of exactly that length, rather than slices.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#panics-5" class="doc-anchor">§</a>Panics

Panics if `chunk_size` is zero.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-34" class="doc-anchor">§</a>Examples

``` rust
let slice = ['l', 'o', 'r', 'e', 'm'];
let mut iter = slice.chunks(2);
assert_eq!(iter.next().unwrap(), &['l', 'o']);
assert_eq!(iter.next().unwrap(), &['r', 'e']);
assert_eq!(iter.next().unwrap(), &['m']);
assert!(iter.next().is_none());
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1195" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.chunks_mut" class="fn">chunks_mut</a>(&mut self, chunk_size: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/slice/iter/struct.ChunksMut.html" class="struct" title="struct core::slice::iter::ChunksMut">ChunksMut</a>\<'\_, T\>

Returns an iterator over `chunk_size` elements of the slice at a time, starting at the beginning of the slice.

The chunks are mutable slices, and do not overlap. If `chunk_size` does not divide the length of the slice, then the last chunk will not have length `chunk_size`.

See [`chunks_exact_mut`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.chunks_exact_mut "method slice::chunks_exact_mut") for a variant of this iterator that returns chunks of always exactly `chunk_size` elements, and [`rchunks_mut`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.rchunks_mut "method slice::rchunks_mut") for the same iterator but starting at the end of the slice.

If your `chunk_size` is a constant, consider using [`as_chunks_mut`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_chunks_mut "method slice::as_chunks_mut") instead, which will give references to arrays of exactly that length, rather than slices.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#panics-6" class="doc-anchor">§</a>Panics

Panics if `chunk_size` is zero.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-35" class="doc-anchor">§</a>Examples

``` rust
let v = &mut [0, 0, 0, 0, 0];
let mut count = 1;

for chunk in v.chunks_mut(2) {
    for elem in chunk.iter_mut() {
        *elem += count;
    }
    count += 1;
}
assert_eq!(v, &[1, 1, 2, 2, 3]);
```

1.31.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1238" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.chunks_exact" class="fn">chunks_exact</a>(&self, chunk_size: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/slice/iter/struct.ChunksExact.html" class="struct" title="struct core::slice::iter::ChunksExact">ChunksExact</a>\<'\_, T\>

Returns an iterator over `chunk_size` elements of the slice at a time, starting at the beginning of the slice.

The chunks are slices and do not overlap. If `chunk_size` does not divide the length of the slice, then the last up to `chunk_size-1` elements will be omitted and can be retrieved from the `remainder` function of the iterator.

Due to each chunk having exactly `chunk_size` elements, the compiler can often optimize the resulting code better than in the case of [`chunks`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.chunks "method slice::chunks").

See [`chunks`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.chunks "method slice::chunks") for a variant of this iterator that also returns the remainder as a smaller chunk, and [`rchunks_exact`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.rchunks_exact "method slice::rchunks_exact") for the same iterator but starting at the end of the slice.

If your `chunk_size` is a constant, consider using [`as_chunks`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_chunks "method slice::as_chunks") instead, which will give references to arrays of exactly that length, rather than slices.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#panics-7" class="doc-anchor">§</a>Panics

Panics if `chunk_size` is zero.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-36" class="doc-anchor">§</a>Examples

``` rust
let slice = ['l', 'o', 'r', 'e', 'm'];
let mut iter = slice.chunks_exact(2);
assert_eq!(iter.next().unwrap(), &['l', 'o']);
assert_eq!(iter.next().unwrap(), &['r', 'e']);
assert!(iter.next().is_none());
assert_eq!(iter.remainder(), &['m']);
```

1.31.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1286" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.chunks_exact_mut" class="fn">chunks_exact_mut</a>(&mut self, chunk_size: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/slice/iter/struct.ChunksExactMut.html" class="struct" title="struct core::slice::iter::ChunksExactMut">ChunksExactMut</a>\<'\_, T\>

Returns an iterator over `chunk_size` elements of the slice at a time, starting at the beginning of the slice.

The chunks are mutable slices, and do not overlap. If `chunk_size` does not divide the length of the slice, then the last up to `chunk_size-1` elements will be omitted and can be retrieved from the `into_remainder` function of the iterator.

Due to each chunk having exactly `chunk_size` elements, the compiler can often optimize the resulting code better than in the case of [`chunks_mut`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.chunks_mut "method slice::chunks_mut").

See [`chunks_mut`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.chunks_mut "method slice::chunks_mut") for a variant of this iterator that also returns the remainder as a smaller chunk, and [`rchunks_exact_mut`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.rchunks_exact_mut "method slice::rchunks_exact_mut") for the same iterator but starting at the end of the slice.

If your `chunk_size` is a constant, consider using [`as_chunks_mut`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_chunks_mut "method slice::as_chunks_mut") instead, which will give references to arrays of exactly that length, rather than slices.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#panics-8" class="doc-anchor">§</a>Panics

Panics if `chunk_size` is zero.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-37" class="doc-anchor">§</a>Examples

``` rust
let v = &mut [0, 0, 0, 0, 0];
let mut count = 1;

for chunk in v.chunks_exact_mut(2) {
    for elem in chunk.iter_mut() {
        *elem += count;
    }
    count += 1;
}
assert_eq!(v, &[1, 1, 2, 2, 0]);
```

1.88.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1334" class="src">Source</a>

#### pub unsafe fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.as_chunks_unchecked" class="fn">as_chunks_unchecked</a>\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>(&self) -\> &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">[T; N]</a>\]

Splits the slice into a slice of `N`-element arrays, assuming that there’s no remainder.

This is the inverse operation to [`as_flattened`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_flattened "method slice::as_flattened").

As this is `unsafe`, consider whether you could use [`as_chunks`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_chunks "method slice::as_chunks") or [`as_rchunks`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_rchunks "method slice::as_rchunks") instead, perhaps via something like `if let (chunks, []) = slice.as_chunks()` or `let (chunks, []) = slice.as_chunks() else { unreachable!() };`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#safety-9" class="doc-anchor">§</a>Safety

This may only be called when

- The slice splits exactly into `N`-element chunks (aka `self.len() % N == 0`).
- `N != 0`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-38" class="doc-anchor">§</a>Examples

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

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.as_chunks" class="fn">as_chunks</a>\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>(&self) -\> (&\[<a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">[T; N]</a>\], &<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>)

Splits the slice into a slice of `N`-element arrays, starting at the beginning of the slice, and a remainder slice with length strictly less than `N`.

The remainder is meaningful in the division sense. Given `let (chunks, remainder) = slice.as_chunks()`, then:

- `chunks.len()` equals `slice.len() / N`,
- `remainder.len()` equals `slice.len() % N`, and
- `slice.len()` equals `chunks.len() * N + remainder.len()`.

You can flatten the chunks back into a slice-of-`T` with [`as_flattened`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_flattened "method slice::as_flattened").

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#panics-9" class="doc-anchor">§</a>Panics

Panics if `N` is zero.

Note that this check is against a const generic parameter, not a runtime value, and thus a particular monomorphization will either always panic or it will never panic.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-39" class="doc-anchor">§</a>Examples

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

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.as_rchunks" class="fn">as_rchunks</a>\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>(&self) -\> (&<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>, &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">[T; N]</a>\])

Splits the slice into a slice of `N`-element arrays, starting at the end of the slice, and a remainder slice with length strictly less than `N`.

The remainder is meaningful in the division sense. Given `let (remainder, chunks) = slice.as_rchunks()`, then:

- `remainder.len()` equals `slice.len() % N`,
- `chunks.len()` equals `slice.len() / N`, and
- `slice.len()` equals `chunks.len() * N + remainder.len()`.

You can flatten the chunks back into a slice-of-`T` with [`as_flattened`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_flattened "method slice::as_flattened").

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#panics-10" class="doc-anchor">§</a>Panics

Panics if `N` is zero.

Note that this check is against a const generic parameter, not a runtime value, and thus a particular monomorphization will either always panic or it will never panic.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-40" class="doc-anchor">§</a>Examples

``` rust
let slice = ['l', 'o', 'r', 'e', 'm'];
let (remainder, chunks) = slice.as_rchunks();
assert_eq!(remainder, &['l']);
assert_eq!(chunks, &[['o', 'r'], ['e', 'm']]);
```

1.88.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1494" class="src">Source</a>

#### pub unsafe fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.as_chunks_unchecked_mut" class="fn">as_chunks_unchecked_mut</a>\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>( &mut self, ) -\> &mut \[<a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">[T; N]</a>\]

Splits the slice into a slice of `N`-element arrays, assuming that there’s no remainder.

This is the inverse operation to [`as_flattened_mut`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_flattened_mut "method slice::as_flattened_mut").

As this is `unsafe`, consider whether you could use [`as_chunks_mut`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_chunks_mut "method slice::as_chunks_mut") or [`as_rchunks_mut`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_rchunks_mut "method slice::as_rchunks_mut") instead, perhaps via something like `if let (chunks, []) = slice.as_chunks_mut()` or `let (chunks, []) = slice.as_chunks_mut() else { unreachable!() };`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#safety-10" class="doc-anchor">§</a>Safety

This may only be called when

- The slice splits exactly into `N`-element chunks (aka `self.len() % N == 0`).
- `N != 0`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-41" class="doc-anchor">§</a>Examples

``` rust
let slice: &mut [char] = &mut ['l', 'o', 'r', 'e', 'm', '!'];
let chunks: &mut [[char; 1]] =
    // SAFETY: 1-element chunks never have remainder
    unsafe { slice.as_chunks_unchecked_mut() };
chunks[0] = ['L'];
assert_eq!(chunks, &[['L'], ['o'], ['r'], ['e'], ['m'], ['!']]);
let chunks: &mut [[char; 3]] =
    // SAFETY: The slice length (6) is a multiple of 3
    unsafe { slice.as_chunks_unchecked_mut() };
chunks[1] = ['a', 'x', '?'];
assert_eq!(slice, &['L', 'o', 'r', 'a', 'x', '?']);

// These would be unsound:
// let chunks: &[[_; 5]] = slice.as_chunks_unchecked_mut() // The slice length is not a multiple of 5
// let chunks: &[[_; 0]] = slice.as_chunks_unchecked_mut() // Zero-length chunks are never allowed
```

1.88.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1548" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.as_chunks_mut" class="fn">as_chunks_mut</a>\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>(&mut self) -\> (&mut \[<a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">[T; N]</a>\], &mut <a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>)

Splits the slice into a slice of `N`-element arrays, starting at the beginning of the slice, and a remainder slice with length strictly less than `N`.

The remainder is meaningful in the division sense. Given `let (chunks, remainder) = slice.as_chunks_mut()`, then:

- `chunks.len()` equals `slice.len() / N`,
- `remainder.len()` equals `slice.len() % N`, and
- `slice.len()` equals `chunks.len() * N + remainder.len()`.

You can flatten the chunks back into a slice-of-`T` with [`as_flattened_mut`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_flattened_mut "method slice::as_flattened_mut").

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#panics-11" class="doc-anchor">§</a>Panics

Panics if `N` is zero.

Note that this check is against a const generic parameter, not a runtime value, and thus a particular monomorphization will either always panic or it will never panic.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-42" class="doc-anchor">§</a>Examples

``` rust
let v = &mut [0, 0, 0, 0, 0];
let mut count = 1;

let (chunks, remainder) = v.as_chunks_mut();
remainder[0] = 9;
for chunk in chunks {
    *chunk = [count; 2];
    count += 1;
}
assert_eq!(v, &[1, 1, 2, 2, 9]);
```

1.88.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1601" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.as_rchunks_mut" class="fn">as_rchunks_mut</a>\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>(&mut self) -\> (&mut <a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>, &mut \[<a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">[T; N]</a>\])

Splits the slice into a slice of `N`-element arrays, starting at the end of the slice, and a remainder slice with length strictly less than `N`.

The remainder is meaningful in the division sense. Given `let (remainder, chunks) = slice.as_rchunks_mut()`, then:

- `remainder.len()` equals `slice.len() % N`,
- `chunks.len()` equals `slice.len() / N`, and
- `slice.len()` equals `chunks.len() * N + remainder.len()`.

You can flatten the chunks back into a slice-of-`T` with [`as_flattened_mut`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_flattened_mut "method slice::as_flattened_mut").

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#panics-12" class="doc-anchor">§</a>Panics

Panics if `N` is zero.

Note that this check is against a const generic parameter, not a runtime value, and thus a particular monomorphization will either always panic or it will never panic.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-43" class="doc-anchor">§</a>Examples

``` rust
let v = &mut [0, 0, 0, 0, 0];
let mut count = 1;

let (remainder, chunks) = v.as_rchunks_mut();
remainder[0] = 9;
for chunk in chunks {
    *chunk = [count; 2];
    count += 1;
}
assert_eq!(v, &[9, 1, 1, 2, 2]);
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.array_windows" class="fn">array_windows</a>\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/slice/iter/struct.ArrayWindows.html" class="struct" title="struct core::slice::iter::ArrayWindows">ArrayWindows</a>\<'\_, T, N\>

🔬This is a nightly-only experimental API. (`array_windows`)

Returns an iterator over overlapping windows of `N` elements of a slice, starting at the beginning of the slice.

This is the const generic equivalent of [`windows`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.windows "method slice::windows").

If `N` is greater than the size of the slice, it will return no windows.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#panics-13" class="doc-anchor">§</a>Panics

Panics if `N` is zero. This check will most probably get changed to a compile time error before this method gets stabilized.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-44" class="doc-anchor">§</a>Examples

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

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.rchunks" class="fn">rchunks</a>(&self, chunk_size: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/slice/iter/struct.RChunks.html" class="struct" title="struct core::slice::iter::RChunks">RChunks</a>\<'\_, T\>

Returns an iterator over `chunk_size` elements of the slice at a time, starting at the end of the slice.

The chunks are slices and do not overlap. If `chunk_size` does not divide the length of the slice, then the last chunk will not have length `chunk_size`.

See [`rchunks_exact`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.rchunks_exact "method slice::rchunks_exact") for a variant of this iterator that returns chunks of always exactly `chunk_size` elements, and [`chunks`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.chunks "method slice::chunks") for the same iterator but starting at the beginning of the slice.

If your `chunk_size` is a constant, consider using [`as_rchunks`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_rchunks "method slice::as_rchunks") instead, which will give references to arrays of exactly that length, rather than slices.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#panics-14" class="doc-anchor">§</a>Panics

Panics if `chunk_size` is zero.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-45" class="doc-anchor">§</a>Examples

``` rust
let slice = ['l', 'o', 'r', 'e', 'm'];
let mut iter = slice.rchunks(2);
assert_eq!(iter.next().unwrap(), &['e', 'm']);
assert_eq!(iter.next().unwrap(), &['o', 'r']);
assert_eq!(iter.next().unwrap(), &['l']);
assert!(iter.next().is_none());
```

1.31.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1724" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.rchunks_mut" class="fn">rchunks_mut</a>(&mut self, chunk_size: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/slice/iter/struct.RChunksMut.html" class="struct" title="struct core::slice::iter::RChunksMut">RChunksMut</a>\<'\_, T\>

Returns an iterator over `chunk_size` elements of the slice at a time, starting at the end of the slice.

The chunks are mutable slices, and do not overlap. If `chunk_size` does not divide the length of the slice, then the last chunk will not have length `chunk_size`.

See [`rchunks_exact_mut`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.rchunks_exact_mut "method slice::rchunks_exact_mut") for a variant of this iterator that returns chunks of always exactly `chunk_size` elements, and [`chunks_mut`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.chunks_mut "method slice::chunks_mut") for the same iterator but starting at the beginning of the slice.

If your `chunk_size` is a constant, consider using [`as_rchunks_mut`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_rchunks_mut "method slice::as_rchunks_mut") instead, which will give references to arrays of exactly that length, rather than slices.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#panics-15" class="doc-anchor">§</a>Panics

Panics if `chunk_size` is zero.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-46" class="doc-anchor">§</a>Examples

``` rust
let v = &mut [0, 0, 0, 0, 0];
let mut count = 1;

for chunk in v.rchunks_mut(2) {
    for elem in chunk.iter_mut() {
        *elem += count;
    }
    count += 1;
}
assert_eq!(v, &[3, 2, 2, 1, 1]);
```

1.31.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1769" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.rchunks_exact" class="fn">rchunks_exact</a>(&self, chunk_size: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/slice/iter/struct.RChunksExact.html" class="struct" title="struct core::slice::iter::RChunksExact">RChunksExact</a>\<'\_, T\>

Returns an iterator over `chunk_size` elements of the slice at a time, starting at the end of the slice.

The chunks are slices and do not overlap. If `chunk_size` does not divide the length of the slice, then the last up to `chunk_size-1` elements will be omitted and can be retrieved from the `remainder` function of the iterator.

Due to each chunk having exactly `chunk_size` elements, the compiler can often optimize the resulting code better than in the case of [`rchunks`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.rchunks "method slice::rchunks").

See [`rchunks`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.rchunks "method slice::rchunks") for a variant of this iterator that also returns the remainder as a smaller chunk, and [`chunks_exact`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.chunks_exact "method slice::chunks_exact") for the same iterator but starting at the beginning of the slice.

If your `chunk_size` is a constant, consider using [`as_rchunks`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_rchunks "method slice::as_rchunks") instead, which will give references to arrays of exactly that length, rather than slices.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#panics-16" class="doc-anchor">§</a>Panics

Panics if `chunk_size` is zero.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-47" class="doc-anchor">§</a>Examples

``` rust
let slice = ['l', 'o', 'r', 'e', 'm'];
let mut iter = slice.rchunks_exact(2);
assert_eq!(iter.next().unwrap(), &['e', 'm']);
assert_eq!(iter.next().unwrap(), &['o', 'r']);
assert!(iter.next().is_none());
assert_eq!(iter.remainder(), &['l']);
```

1.31.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1818" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.rchunks_exact_mut" class="fn">rchunks_exact_mut</a>(&mut self, chunk_size: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/slice/iter/struct.RChunksExactMut.html" class="struct" title="struct core::slice::iter::RChunksExactMut">RChunksExactMut</a>\<'\_, T\>

Returns an iterator over `chunk_size` elements of the slice at a time, starting at the end of the slice.

The chunks are mutable slices, and do not overlap. If `chunk_size` does not divide the length of the slice, then the last up to `chunk_size-1` elements will be omitted and can be retrieved from the `into_remainder` function of the iterator.

Due to each chunk having exactly `chunk_size` elements, the compiler can often optimize the resulting code better than in the case of [`chunks_mut`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.chunks_mut "method slice::chunks_mut").

See [`rchunks_mut`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.rchunks_mut "method slice::rchunks_mut") for a variant of this iterator that also returns the remainder as a smaller chunk, and [`chunks_exact_mut`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.chunks_exact_mut "method slice::chunks_exact_mut") for the same iterator but starting at the beginning of the slice.

If your `chunk_size` is a constant, consider using [`as_rchunks_mut`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_rchunks_mut "method slice::as_rchunks_mut") instead, which will give references to arrays of exactly that length, rather than slices.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#panics-17" class="doc-anchor">§</a>Panics

Panics if `chunk_size` is zero.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-48" class="doc-anchor">§</a>Examples

``` rust
let v = &mut [0, 0, 0, 0, 0];
let mut count = 1;

for chunk in v.rchunks_exact_mut(2) {
    for elem in chunk.iter_mut() {
        *elem += count;
    }
    count += 1;
}
assert_eq!(v, &[0, 2, 2, 1, 1]);
```

1.77.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1858-1860" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.chunk_by" class="fn">chunk_by</a>\<F\>(&self, pred: F) -\> <a href="https://doc.rust-lang.org/nightly/core/slice/iter/struct.ChunkBy.html" class="struct" title="struct core::slice::iter::ChunkBy">ChunkBy</a>\<'\_, T, F\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

Returns an iterator over the slice producing non-overlapping runs of elements using the predicate to separate them.

The predicate is called for every pair of consecutive elements, meaning that it is called on `slice[0]` and `slice[1]`, followed by `slice[1]` and `slice[2]`, and so on.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-49" class="doc-anchor">§</a>Examples

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

1.77.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1900-1902" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.chunk_by_mut" class="fn">chunk_by_mut</a>\<F\>(&mut self, pred: F) -\> <a href="https://doc.rust-lang.org/nightly/core/slice/iter/struct.ChunkByMut.html" class="struct" title="struct core::slice::iter::ChunkByMut">ChunkByMut</a>\<'\_, T, F\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

Returns an iterator over the slice producing non-overlapping mutable runs of elements using the predicate to separate them.

The predicate is called for every pair of consecutive elements, meaning that it is called on `slice[0]` and `slice[1]`, followed by `slice[1]` and `slice[2]`, and so on.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-50" class="doc-anchor">§</a>Examples

``` rust
let slice = &mut [1, 1, 1, 3, 3, 2, 2, 2];

let mut iter = slice.chunk_by_mut(|a, b| a == b);

assert_eq!(iter.next(), Some(&mut [1, 1, 1][..]));
assert_eq!(iter.next(), Some(&mut [3, 3][..]));
assert_eq!(iter.next(), Some(&mut [2, 2, 2][..]));
assert_eq!(iter.next(), None);
```

This method can be used to extract the sorted subslices:

``` rust
let slice = &mut [1, 1, 2, 3, 2, 3, 2, 3, 4];

let mut iter = slice.chunk_by_mut(|a, b| a <= b);

assert_eq!(iter.next(), Some(&mut [1, 1, 2, 3][..]));
assert_eq!(iter.next(), Some(&mut [2, 3][..]));
assert_eq!(iter.next(), Some(&mut [2, 3, 4][..]));
assert_eq!(iter.next(), None);
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1946" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.split_at" class="fn">split_at</a>(&self, mid: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> (&<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>, &<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>)

Divides one slice into two at an index.

The first will contain all indices from `[0, mid)` (excluding the index `mid` itself) and the second will contain all indices from `[mid, len)` (excluding the index `len` itself).

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#panics-18" class="doc-anchor">§</a>Panics

Panics if `mid > len`. For a non-panicking alternative see [`split_at_checked`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.split_at_checked "method slice::split_at_checked").

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-51" class="doc-anchor">§</a>Examples

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

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1980" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.split_at_mut" class="fn">split_at_mut</a>(&mut self, mid: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> (&mut <a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>, &mut <a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>)

Divides one mutable slice into two at an index.

The first will contain all indices from `[0, mid)` (excluding the index `mid` itself) and the second will contain all indices from `[mid, len)` (excluding the index `len` itself).

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#panics-19" class="doc-anchor">§</a>Panics

Panics if `mid > len`. For a non-panicking alternative see [`split_at_mut_checked`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.split_at_mut_checked "method slice::split_at_mut_checked").

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-52" class="doc-anchor">§</a>Examples

``` rust
let mut v = [1, 0, 3, 0, 5, 6];
let (left, right) = v.split_at_mut(2);
assert_eq!(left, [1, 0]);
assert_eq!(right, [3, 0, 5, 6]);
left[1] = 2;
right[1] = 4;
assert_eq!(v, [1, 2, 3, 4, 5, 6]);
```

1.79.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2032" class="src">Source</a>

#### pub unsafe fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.split_at_unchecked" class="fn">split_at_unchecked</a>(&self, mid: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> (&<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>, &<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>)

Divides one slice into two at an index, without doing bounds checking.

The first will contain all indices from `[0, mid)` (excluding the index `mid` itself) and the second will contain all indices from `[mid, len)` (excluding the index `len` itself).

For a safe alternative see [`split_at`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.split_at "method slice::split_at").

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#safety-11" class="doc-anchor">§</a>Safety

Calling this method with an out-of-bounds index is *[undefined behavior](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)* even if the resulting reference is not used. The caller has to ensure that `0 <= mid <= self.len()`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-53" class="doc-anchor">§</a>Examples

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

1.79.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2086" class="src">Source</a>

#### pub unsafe fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.split_at_mut_unchecked" class="fn">split_at_mut_unchecked</a>( &mut self, mid: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> (&mut <a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>, &mut <a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>)

Divides one mutable slice into two at an index, without doing bounds checking.

The first will contain all indices from `[0, mid)` (excluding the index `mid` itself) and the second will contain all indices from `[mid, len)` (excluding the index `len` itself).

For a safe alternative see [`split_at_mut`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.split_at_mut "method slice::split_at_mut").

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#safety-12" class="doc-anchor">§</a>Safety

Calling this method with an out-of-bounds index is *[undefined behavior](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)* even if the resulting reference is not used. The caller has to ensure that `0 <= mid <= self.len()`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-54" class="doc-anchor">§</a>Examples

``` rust
let mut v = [1, 0, 3, 0, 5, 6];
// scoped to restrict the lifetime of the borrows
unsafe {
    let (left, right) = v.split_at_mut_unchecked(2);
    assert_eq!(left, [1, 0]);
    assert_eq!(right, [3, 0, 5, 6]);
    left[1] = 2;
    right[1] = 4;
}
assert_eq!(v, [1, 2, 3, 4, 5, 6]);
```

1.80.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2147" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.split_at_checked" class="fn">split_at_checked</a>(&self, mid: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(&<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>, &<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>)\>

Divides one slice into two at an index, returning `None` if the slice is too short.

If `mid ≤ len` returns a pair of slices where the first will contain all indices from `[0, mid)` (excluding the index `mid` itself) and the second will contain all indices from `[mid, len)` (excluding the index `len` itself).

Otherwise, if `mid > len`, returns `None`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-55" class="doc-anchor">§</a>Examples

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

1.80.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2186" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.split_at_mut_checked" class="fn">split_at_mut_checked</a>( &mut self, mid: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(&mut <a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>, &mut <a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>)\>

Divides one mutable slice into two at an index, returning `None` if the slice is too short.

If `mid ≤ len` returns a pair of slices where the first will contain all indices from `[0, mid)` (excluding the index `mid` itself) and the second will contain all indices from `[mid, len)` (excluding the index `len` itself).

Otherwise, if `mid > len`, returns `None`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-56" class="doc-anchor">§</a>Examples

``` rust
let mut v = [1, 0, 3, 0, 5, 6];

if let Some((left, right)) = v.split_at_mut_checked(2) {
    assert_eq!(left, [1, 0]);
    assert_eq!(right, [3, 0, 5, 6]);
    left[1] = 2;
    right[1] = 4;
}
assert_eq!(v, [1, 2, 3, 4, 5, 6]);

assert_eq!(None, v.split_at_mut_checked(7));
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2238-2240" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.split" class="fn">split</a>\<F\>(&self, pred: F) -\> <a href="https://doc.rust-lang.org/nightly/core/slice/iter/struct.Split.html" class="struct" title="struct core::slice::iter::Split">Split</a>\<'\_, T, F\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

Returns an iterator over subslices separated by elements that match `pred`. The matched element is not contained in the subslices.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-57" class="doc-anchor">§</a>Examples

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

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2260-2262" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.split_mut" class="fn">split_mut</a>\<F\>(&mut self, pred: F) -\> <a href="https://doc.rust-lang.org/nightly/core/slice/iter/struct.SplitMut.html" class="struct" title="struct core::slice::iter::SplitMut">SplitMut</a>\<'\_, T, F\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

Returns an iterator over mutable subslices separated by elements that match `pred`. The matched element is not contained in the subslices.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-58" class="doc-anchor">§</a>Examples

``` rust
let mut v = [10, 40, 30, 20, 60, 50];

for group in v.split_mut(|num| *num % 3 == 0) {
    group[0] = 1;
}
assert_eq!(v, [1, 40, 30, 1, 60, 1]);
```

1.51.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2296-2298" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.split_inclusive" class="fn">split_inclusive</a>\<F\>(&self, pred: F) -\> <a href="https://doc.rust-lang.org/nightly/core/slice/iter/struct.SplitInclusive.html" class="struct" title="struct core::slice::iter::SplitInclusive">SplitInclusive</a>\<'\_, T, F\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

Returns an iterator over subslices separated by elements that match `pred`. The matched element is contained in the end of the previous subslice as a terminator.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-59" class="doc-anchor">§</a>Examples

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

1.51.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2320-2322" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.split_inclusive_mut" class="fn">split_inclusive_mut</a>\<F\>(&mut self, pred: F) -\> <a href="https://doc.rust-lang.org/nightly/core/slice/iter/struct.SplitInclusiveMut.html" class="struct" title="struct core::slice::iter::SplitInclusiveMut">SplitInclusiveMut</a>\<'\_, T, F\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

Returns an iterator over mutable subslices separated by elements that match `pred`. The matched element is contained in the previous subslice as a terminator.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-60" class="doc-anchor">§</a>Examples

``` rust
let mut v = [10, 40, 30, 20, 60, 50];

for group in v.split_inclusive_mut(|num| *num % 3 == 0) {
    let terminator_idx = group.len()-1;
    group[terminator_idx] = 1;
}
assert_eq!(v, [10, 40, 1, 20, 1, 1]);
```

1.27.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2356-2358" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.rsplit" class="fn">rsplit</a>\<F\>(&self, pred: F) -\> <a href="https://doc.rust-lang.org/nightly/core/slice/iter/struct.RSplit.html" class="struct" title="struct core::slice::iter::RSplit">RSplit</a>\<'\_, T, F\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

Returns an iterator over subslices separated by elements that match `pred`, starting at the end of the slice and working backwards. The matched element is not contained in the subslices.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-61" class="doc-anchor">§</a>Examples

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

1.27.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2382-2384" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.rsplit_mut" class="fn">rsplit_mut</a>\<F\>(&mut self, pred: F) -\> <a href="https://doc.rust-lang.org/nightly/core/slice/iter/struct.RSplitMut.html" class="struct" title="struct core::slice::iter::RSplitMut">RSplitMut</a>\<'\_, T, F\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

Returns an iterator over mutable subslices separated by elements that match `pred`, starting at the end of the slice and working backwards. The matched element is not contained in the subslices.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-62" class="doc-anchor">§</a>Examples

``` rust
let mut v = [100, 400, 300, 200, 600, 500];

let mut count = 0;
for group in v.rsplit_mut(|num| *num % 3 == 0) {
    count += 1;
    group[0] = count;
}
assert_eq!(v, [3, 400, 300, 2, 600, 1]);
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2410-2412" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.splitn" class="fn">splitn</a>\<F\>(&self, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, pred: F) -\> <a href="https://doc.rust-lang.org/nightly/core/slice/iter/struct.SplitN.html" class="struct" title="struct core::slice::iter::SplitN">SplitN</a>\<'\_, T, F\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

Returns an iterator over subslices separated by elements that match `pred`, limited to returning at most `n` items. The matched element is not contained in the subslices.

The last element returned, if any, will contain the remainder of the slice.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-63" class="doc-anchor">§</a>Examples

Print the slice split once by numbers divisible by 3 (i.e., `[10, 40]`, `[20, 60, 50]`):

``` rust
let v = [10, 40, 30, 20, 60, 50];

for group in v.splitn(2, |num| *num % 3 == 0) {
    println!("{group:?}");
}
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2436-2438" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.splitn_mut" class="fn">splitn_mut</a>\<F\>(&mut self, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, pred: F) -\> <a href="https://doc.rust-lang.org/nightly/core/slice/iter/struct.SplitNMut.html" class="struct" title="struct core::slice::iter::SplitNMut">SplitNMut</a>\<'\_, T, F\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

Returns an iterator over mutable subslices separated by elements that match `pred`, limited to returning at most `n` items. The matched element is not contained in the subslices.

The last element returned, if any, will contain the remainder of the slice.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-64" class="doc-anchor">§</a>Examples

``` rust
let mut v = [10, 40, 30, 20, 60, 50];

for group in v.splitn_mut(2, |num| *num % 3 == 0) {
    group[0] = 1;
}
assert_eq!(v, [1, 40, 30, 1, 60, 50]);
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2465-2467" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.rsplitn" class="fn">rsplitn</a>\<F\>(&self, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, pred: F) -\> <a href="https://doc.rust-lang.org/nightly/core/slice/iter/struct.RSplitN.html" class="struct" title="struct core::slice::iter::RSplitN">RSplitN</a>\<'\_, T, F\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

Returns an iterator over subslices separated by elements that match `pred` limited to returning at most `n` items. This starts at the end of the slice and works backwards. The matched element is not contained in the subslices.

The last element returned, if any, will contain the remainder of the slice.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-65" class="doc-anchor">§</a>Examples

Print the slice split once, starting from the end, by numbers divisible by 3 (i.e., `[50]`, `[10, 40, 30, 20]`):

``` rust
let v = [10, 40, 30, 20, 60, 50];

for group in v.rsplitn(2, |num| *num % 3 == 0) {
    println!("{group:?}");
}
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2492-2494" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.rsplitn_mut" class="fn">rsplitn_mut</a>\<F\>(&mut self, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, pred: F) -\> <a href="https://doc.rust-lang.org/nightly/core/slice/iter/struct.RSplitNMut.html" class="struct" title="struct core::slice::iter::RSplitNMut">RSplitNMut</a>\<'\_, T, F\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

Returns an iterator over subslices separated by elements that match `pred` limited to returning at most `n` items. This starts at the end of the slice and works backwards. The matched element is not contained in the subslices.

The last element returned, if any, will contain the remainder of the slice.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-66" class="doc-anchor">§</a>Examples

``` rust
let mut s = [10, 40, 30, 20, 60, 50];

for group in s.rsplitn_mut(2, |num| *num % 3 == 0) {
    group[0] = 1;
}
assert_eq!(s, [1, 40, 30, 20, 60, 1]);
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.split_once" class="fn">split_once</a>\<F\>(&self, pred: F) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(&<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>, &<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>)\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

🔬This is a nightly-only experimental API. (`slice_split_once`)

Splits the slice on the first element that matches the specified predicate.

If any matching elements are present in the slice, returns the prefix before the match and suffix after. The matching element itself is not included. If no elements match, returns `None`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-67" class="doc-anchor">§</a>Examples

``` rust
#![feature(slice_split_once)]
let s = [1, 2, 3, 2, 4];
assert_eq!(s.split_once(|&x| x == 2), Some((
    &[1][..],
    &[3, 2, 4][..]
)));
assert_eq!(s.split_once(|&x| x == 0), None);
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.rsplit_once" class="fn">rsplit_once</a>\<F\>(&self, pred: F) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(&<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>, &<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>)\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

🔬This is a nightly-only experimental API. (`slice_split_once`)

Splits the slice on the last element that matches the specified predicate.

If any matching elements are present in the slice, returns the prefix before the match and suffix after. The matching element itself is not included. If no elements match, returns `None`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-68" class="doc-anchor">§</a>Examples

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

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.contains" class="fn">contains</a>(&self, x: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>,

Returns `true` if the slice contains an element with the given value.

This operation is *O*(*n*).

Note that if you have a sorted slice, [`binary_search`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.binary_search "method slice::binary_search") may be faster.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-69" class="doc-anchor">§</a>Examples

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

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.starts_with" class="fn">starts_with</a>(&self, needle: &<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>,

Returns `true` if `needle` is a prefix of the slice or equal to the slice.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-70" class="doc-anchor">§</a>Examples

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

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.ends_with" class="fn">ends_with</a>(&self, needle: &<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>,

Returns `true` if `needle` is a suffix of the slice or equal to the slice.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-71" class="doc-anchor">§</a>Examples

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

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.strip_prefix" class="fn">strip_prefix</a>\<P\>(&self, prefix: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;P</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>\>

where P: <a href="https://doc.rust-lang.org/nightly/core/slice/trait.SlicePattern.html" class="trait" title="trait core::slice::SlicePattern">SlicePattern</a>\<Item = T\> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, T: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>,

Returns a subslice with the prefix removed.

If the slice starts with `prefix`, returns the subslice after the prefix, wrapped in `Some`. If `prefix` is empty, simply returns the original slice. If `prefix` is equal to the original slice, returns an empty slice.

If the slice does not start with `prefix`, returns `None`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-72" class="doc-anchor">§</a>Examples

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

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.strip_suffix" class="fn">strip_suffix</a>\<P\>(&self, suffix: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;P</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>\>

where P: <a href="https://doc.rust-lang.org/nightly/core/slice/trait.SlicePattern.html" class="trait" title="trait core::slice::SlicePattern">SlicePattern</a>\<Item = T\> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, T: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>,

Returns a subslice with the suffix removed.

If the slice ends with `suffix`, returns the subslice before the suffix, wrapped in `Some`. If `suffix` is empty, simply returns the original slice. If `suffix` is equal to the original slice, returns an empty slice.

If the slice does not end with `suffix`, returns `None`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-73" class="doc-anchor">§</a>Examples

``` rust
let v = &[10, 40, 30];
assert_eq!(v.strip_suffix(&[30]), Some(&[10, 40][..]));
assert_eq!(v.strip_suffix(&[40, 30]), Some(&[10][..]));
assert_eq!(v.strip_suffix(&[10, 40, 30]), Some(&[][..]));
assert_eq!(v.strip_suffix(&[50]), None);
assert_eq!(v.strip_suffix(&[50, 30]), None);
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.trim_prefix" class="fn">trim_prefix</a>\<P\>(&self, prefix: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;P</a>) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>

where P: <a href="https://doc.rust-lang.org/nightly/core/slice/trait.SlicePattern.html" class="trait" title="trait core::slice::SlicePattern">SlicePattern</a>\<Item = T\> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, T: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>,

🔬This is a nightly-only experimental API. (`trim_prefix_suffix`)

Returns a subslice with the optional prefix removed.

If the slice starts with `prefix`, returns the subslice after the prefix. If `prefix` is empty or the slice does not start with `prefix`, simply returns the original slice. If `prefix` is equal to the original slice, returns an empty slice.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-74" class="doc-anchor">§</a>Examples

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

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.trim_suffix" class="fn">trim_suffix</a>\<P\>(&self, suffix: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;P</a>) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>

where P: <a href="https://doc.rust-lang.org/nightly/core/slice/trait.SlicePattern.html" class="trait" title="trait core::slice::SlicePattern">SlicePattern</a>\<Item = T\> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, T: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>,

🔬This is a nightly-only experimental API. (`trim_prefix_suffix`)

Returns a subslice with the optional suffix removed.

If the slice ends with `suffix`, returns the subslice before the suffix. If `suffix` is empty or the slice does not end with `suffix`, simply returns the original slice. If `suffix` is equal to the original slice, returns an empty slice.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-75" class="doc-anchor">§</a>Examples

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

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.binary_search" class="fn">binary_search</a>(&self, x: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a>,

Binary searches this slice for a given element. If the slice is not sorted, the returned result is unspecified and meaningless.

If the value is found then [`Result::Ok`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Ok "variant core::result::Result::Ok") is returned, containing the index of the matching element. If there are multiple matches, then any one of the matches could be returned. The index is chosen deterministically, but is subject to change in future versions of Rust. If the value is not found then [`Result::Err`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Err "variant core::result::Result::Err") is returned, containing the index where a matching element could be inserted while maintaining sorted order.

See also [`binary_search_by`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.binary_search_by "method slice::binary_search_by"), [`binary_search_by_key`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.binary_search_by_key "method slice::binary_search_by_key"), and [`partition_point`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.partition_point "method slice::partition_point").

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-76" class="doc-anchor">§</a>Examples

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

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.binary_search_by" class="fn">binary_search_by</a>\<'a, F\>(&'a self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>,

Binary searches this slice with a comparator function.

The comparator function should return an order code that indicates whether its argument is `Less`, `Equal` or `Greater` the desired target. If the slice is not sorted or if the comparator function does not implement an order consistent with the sort order of the underlying slice, the returned result is unspecified and meaningless.

If the value is found then [`Result::Ok`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Ok "variant core::result::Result::Ok") is returned, containing the index of the matching element. If there are multiple matches, then any one of the matches could be returned. The index is chosen deterministically, but is subject to change in future versions of Rust. If the value is not found then [`Result::Err`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Err "variant core::result::Result::Err") is returned, containing the index where a matching element could be inserted while maintaining sorted order.

See also [`binary_search`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.binary_search "method slice::binary_search"), [`binary_search_by_key`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.binary_search_by_key "method slice::binary_search_by_key"), and [`partition_point`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.partition_point "method slice::partition_point").

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-77" class="doc-anchor">§</a>Examples

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

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.binary_search_by_key" class="fn">binary_search_by_key</a>\<'a, B, F\>( &'a self, b: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;B</a>, f: F, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>) -\> B, B: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a>,

Binary searches this slice with a key extraction function.

Assumes that the slice is sorted by the key, for instance with [`sort_by_key`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.sort_by_key "method slice::sort_by_key") using the same key extraction function. If the slice is not sorted by the key, the returned result is unspecified and meaningless.

If the value is found then [`Result::Ok`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Ok "variant core::result::Result::Ok") is returned, containing the index of the matching element. If there are multiple matches, then any one of the matches could be returned. The index is chosen deterministically, but is subject to change in future versions of Rust. If the value is not found then [`Result::Err`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Err "variant core::result::Result::Err") is returned, containing the index where a matching element could be inserted while maintaining sorted order.

See also [`binary_search`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.binary_search "method slice::binary_search"), [`binary_search_by`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.binary_search_by "method slice::binary_search_by"), and [`partition_point`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.partition_point "method slice::partition_point").

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-78" class="doc-anchor">§</a>Examples

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

1.20.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#3095-3097" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.sort_unstable" class="fn">sort_unstable</a>(&mut self)

where T: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a>,

Sorts the slice in ascending order **without** preserving the initial order of equal elements.

This sort is unstable (i.e., may reorder equal elements), in-place (i.e., does not allocate), and *O*(*n* \* log(*n*)) worst-case.

If the implementation of [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") for `T` does not implement a [total order](https://en.wikipedia.org/wiki/Total_order), the function may panic; even if the function exits normally, the resulting order of elements in the slice is unspecified. See also the note on panicking below.

For example `|a, b| (a - b).cmp(a)` is a comparison function that is neither transitive nor reflexive nor total, `a < b < c < a` with `a = 1, b = 2, c = 3`. For more information and examples see the [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") documentation.

All original elements will remain in the slice and any possible modifications via interior mutability are observed in the input. Same is true if the implementation of [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") for `T` panics.

Sorting types that only implement [`PartialOrd`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd") such as [`f32`](https://doc.rust-lang.org/nightly/std/primitive.f32.html "primitive f32") and [`f64`](https://doc.rust-lang.org/nightly/std/primitive.f64.html "primitive f64") require additional precautions. For example, `f32::NAN != f32::NAN`, which doesn’t fulfill the reflexivity requirement of [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"). By using an alternative comparison function with `slice::sort_unstable_by` such as [`f32::total_cmp`](https://doc.rust-lang.org/nightly/std/primitive.f32.html#method.total_cmp "method f32::total_cmp") or [`f64::total_cmp`](https://doc.rust-lang.org/nightly/std/primitive.f64.html#method.total_cmp "method f64::total_cmp") that defines a [total order](https://en.wikipedia.org/wiki/Total_order) users can sort slices containing floating-point values. Alternatively, if all values in the slice are guaranteed to be in a subset for which [`PartialOrd::partial_cmp`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp "method core::cmp::PartialOrd::partial_cmp") forms a [total order](https://en.wikipedia.org/wiki/Total_order), it’s possible to sort the slice with `sort_unstable_by(|a, b| a.partial_cmp(b).unwrap())`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#current-implementation" class="doc-anchor">§</a>Current implementation

The current implementation is based on [ipnsort](https://github.com/Voultapher/sort-research-rs/tree/main/ipnsort) by Lukas Bergdoll and Orson Peters, which combines the fast average case of quicksort with the fast worst case of heapsort, achieving linear time on fully sorted and reversed inputs. On inputs with k distinct elements, the expected time to sort the data is *O*(*n* \* log(*k*)).

It is typically faster than stable sorting, except in a few special cases, e.g., when the slice is partially sorted.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#panics-20" class="doc-anchor">§</a>Panics

May panic if the implementation of [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") for `T` does not implement a [total order](https://en.wikipedia.org/wiki/Total_order), or if the [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") implementation panics.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-79" class="doc-anchor">§</a>Examples

``` rust
let mut v = [4, -5, 1, -3, 2];

v.sort_unstable();
assert_eq!(v, [-5, -3, 1, 2, 4]);
```

1.20.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#3150-3152" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.sort_unstable_by" class="fn">sort_unstable_by</a>\<F\>(&mut self, compare: F)

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>,

Sorts the slice in ascending order with a comparison function, **without** preserving the initial order of equal elements.

This sort is unstable (i.e., may reorder equal elements), in-place (i.e., does not allocate), and *O*(*n* \* log(*n*)) worst-case.

If the comparison function `compare` does not implement a [total order](https://en.wikipedia.org/wiki/Total_order), the function may panic; even if the function exits normally, the resulting order of elements in the slice is unspecified. See also the note on panicking below.

For example `|a, b| (a - b).cmp(a)` is a comparison function that is neither transitive nor reflexive nor total, `a < b < c < a` with `a = 1, b = 2, c = 3`. For more information and examples see the [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") documentation.

All original elements will remain in the slice and any possible modifications via interior mutability are observed in the input. Same is true if `compare` panics.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#current-implementation-1" class="doc-anchor">§</a>Current implementation

The current implementation is based on [ipnsort](https://github.com/Voultapher/sort-research-rs/tree/main/ipnsort) by Lukas Bergdoll and Orson Peters, which combines the fast average case of quicksort with the fast worst case of heapsort, achieving linear time on fully sorted and reversed inputs. On inputs with k distinct elements, the expected time to sort the data is *O*(*n* \* log(*k*)).

It is typically faster than stable sorting, except in a few special cases, e.g., when the slice is partially sorted.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#panics-21" class="doc-anchor">§</a>Panics

May panic if the `compare` does not implement a [total order](https://en.wikipedia.org/wiki/Total_order), or if the `compare` itself panics.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-80" class="doc-anchor">§</a>Examples

``` rust
let mut v = [4, -5, 1, -3, 2];
v.sort_unstable_by(|a, b| a.cmp(b));
assert_eq!(v, [-5, -3, 1, 2, 4]);

// reverse sorting
v.sort_unstable_by(|a, b| b.cmp(a));
assert_eq!(v, [4, 2, 1, -3, -5]);
```

1.20.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#3202-3205" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.sort_unstable_by_key" class="fn">sort_unstable_by_key</a>\<K, F\>(&mut self, f: F)

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>) -\> K, K: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a>,

Sorts the slice in ascending order with a key extraction function, **without** preserving the initial order of equal elements.

This sort is unstable (i.e., may reorder equal elements), in-place (i.e., does not allocate), and *O*(*n* \* log(*n*)) worst-case.

If the implementation of [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") for `K` does not implement a [total order](https://en.wikipedia.org/wiki/Total_order), the function may panic; even if the function exits normally, the resulting order of elements in the slice is unspecified. See also the note on panicking below.

For example `|a, b| (a - b).cmp(a)` is a comparison function that is neither transitive nor reflexive nor total, `a < b < c < a` with `a = 1, b = 2, c = 3`. For more information and examples see the [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") documentation.

All original elements will remain in the slice and any possible modifications via interior mutability are observed in the input. Same is true if the implementation of [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") for `K` panics.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#current-implementation-2" class="doc-anchor">§</a>Current implementation

The current implementation is based on [ipnsort](https://github.com/Voultapher/sort-research-rs/tree/main/ipnsort) by Lukas Bergdoll and Orson Peters, which combines the fast average case of quicksort with the fast worst case of heapsort, achieving linear time on fully sorted and reversed inputs. On inputs with k distinct elements, the expected time to sort the data is *O*(*n* \* log(*k*)).

It is typically faster than stable sorting, except in a few special cases, e.g., when the slice is partially sorted.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#panics-22" class="doc-anchor">§</a>Panics

May panic if the implementation of [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") for `K` does not implement a [total order](https://en.wikipedia.org/wiki/Total_order), or if the [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") implementation panics.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-81" class="doc-anchor">§</a>Examples

``` rust
let mut v = [4i32, -5, 1, -3, 2];

v.sort_unstable_by_key(|k| k.abs());
assert_eq!(v, [1, 2, -3, 4, -5]);
```

1.49.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#3265-3267" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.select_nth_unstable" class="fn">select_nth_unstable</a>( &mut self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> (&mut <a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut T</a>, &mut <a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>)

where T: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a>,

Reorders the slice such that the element at `index` is at a sort-order position. All elements before `index` will be `<=` to this value, and all elements after will be `>=` to it.

This reordering is unstable (i.e. any element that compares equal to the nth element may end up at that position), in-place (i.e. does not allocate), and runs in *O*(*n*) time. This function is also known as “kth element” in other libraries.

Returns a triple that partitions the reordered slice:

- The unsorted subslice before `index`, whose elements all satisfy `x <= self[index]`.

- The element at `index`.

- The unsorted subslice after `index`, whose elements all satisfy `x >= self[index]`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#current-implementation-3" class="doc-anchor">§</a>Current implementation

The current algorithm is an introselect implementation based on [ipnsort](https://github.com/Voultapher/sort-research-rs/tree/main/ipnsort) by Lukas Bergdoll and Orson Peters, which is also the basis for [`sort_unstable`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.sort_unstable "method slice::sort_unstable"). The fallback algorithm is Median of Medians using Tukey’s Ninther for pivot selection, which guarantees linear runtime for all inputs.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#panics-23" class="doc-anchor">§</a>Panics

Panics when `index >= len()`, and so always panics on empty slices.

May panic if the implementation of [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") for `T` does not implement a [total order](https://en.wikipedia.org/wiki/Total_order).

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-82" class="doc-anchor">§</a>Examples

``` rust
let mut v = [-5i32, 4, 2, -3, 1];

// Find the items `<=` to the median, the median itself, and the items `>=` to it.
let (lesser, median, greater) = v.select_nth_unstable(2);

assert!(lesser == [-3, -5] || lesser == [-5, -3]);
assert_eq!(median, &mut 1);
assert!(greater == [4, 2] || greater == [2, 4]);

// We are only guaranteed the slice will be one of the following, based on the way we sort
// about the specified index.
assert!(v == [-3, -5, 1, 2, 4] ||
        v == [-5, -3, 1, 2, 4] ||
        v == [-3, -5, 1, 4, 2] ||
        v == [-5, -3, 1, 4, 2]);
```

1.49.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#3330-3336" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.select_nth_unstable_by" class="fn">select_nth_unstable_by</a>\<F\>( &mut self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, compare: F, ) -\> (&mut <a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut T</a>, &mut <a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>)

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>,

Reorders the slice with a comparator function such that the element at `index` is at a sort-order position. All elements before `index` will be `<=` to this value, and all elements after will be `>=` to it, according to the comparator function.

This reordering is unstable (i.e. any element that compares equal to the nth element may end up at that position), in-place (i.e. does not allocate), and runs in *O*(*n*) time. This function is also known as “kth element” in other libraries.

Returns a triple partitioning the reordered slice:

- The unsorted subslice before `index`, whose elements all satisfy `compare(x, self[index]).is_le()`.

- The element at `index`.

- The unsorted subslice after `index`, whose elements all satisfy `compare(x, self[index]).is_ge()`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#current-implementation-4" class="doc-anchor">§</a>Current implementation

The current algorithm is an introselect implementation based on [ipnsort](https://github.com/Voultapher/sort-research-rs/tree/main/ipnsort) by Lukas Bergdoll and Orson Peters, which is also the basis for [`sort_unstable`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.sort_unstable "method slice::sort_unstable"). The fallback algorithm is Median of Medians using Tukey’s Ninther for pivot selection, which guarantees linear runtime for all inputs.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#panics-24" class="doc-anchor">§</a>Panics

Panics when `index >= len()`, and so always panics on empty slices.

May panic if `compare` does not implement a [total order](https://en.wikipedia.org/wiki/Total_order).

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-83" class="doc-anchor">§</a>Examples

``` rust
let mut v = [-5i32, 4, 2, -3, 1];

// Find the items `>=` to the median, the median itself, and the items `<=` to it, by using
// a reversed comparator.
let (before, median, after) = v.select_nth_unstable_by(2, |a, b| b.cmp(a));

assert!(before == [4, 2] || before == [2, 4]);
assert_eq!(median, &mut 1);
assert!(after == [-3, -5] || after == [-5, -3]);

// We are only guaranteed the slice will be one of the following, based on the way we sort
// about the specified index.
assert!(v == [2, 4, 1, -5, -3] ||
        v == [2, 4, 1, -3, -5] ||
        v == [4, 2, 1, -5, -3] ||
        v == [4, 2, 1, -3, -5]);
```

1.49.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#3397-3404" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.select_nth_unstable_by_key" class="fn">select_nth_unstable_by_key</a>\<K, F\>( &mut self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, f: F, ) -\> (&mut <a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut T</a>, &mut <a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>)

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>) -\> K, K: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a>,

Reorders the slice with a key extraction function such that the element at `index` is at a sort-order position. All elements before `index` will have keys `<=` to the key at `index`, and all elements after will have keys `>=` to it.

This reordering is unstable (i.e. any element that compares equal to the nth element may end up at that position), in-place (i.e. does not allocate), and runs in *O*(*n*) time. This function is also known as “kth element” in other libraries.

Returns a triple partitioning the reordered slice:

- The unsorted subslice before `index`, whose elements all satisfy `f(x) <= f(self[index])`.

- The element at `index`.

- The unsorted subslice after `index`, whose elements all satisfy `f(x) >= f(self[index])`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#current-implementation-5" class="doc-anchor">§</a>Current implementation

The current algorithm is an introselect implementation based on [ipnsort](https://github.com/Voultapher/sort-research-rs/tree/main/ipnsort) by Lukas Bergdoll and Orson Peters, which is also the basis for [`sort_unstable`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.sort_unstable "method slice::sort_unstable"). The fallback algorithm is Median of Medians using Tukey’s Ninther for pivot selection, which guarantees linear runtime for all inputs.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#panics-25" class="doc-anchor">§</a>Panics

Panics when `index >= len()`, meaning it always panics on empty slices.

May panic if `K: Ord` does not implement a total order.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-84" class="doc-anchor">§</a>Examples

``` rust
let mut v = [-5i32, 4, 1, -3, 2];

// Find the items `<=` to the absolute median, the absolute median itself, and the items
// `>=` to it.
let (lesser, median, greater) = v.select_nth_unstable_by_key(2, |a| a.abs());

assert!(lesser == [1, 2] || lesser == [2, 1]);
assert_eq!(median, &mut -3);
assert!(greater == [4, -5] || greater == [-5, 4]);

// We are only guaranteed the slice will be one of the following, based on the way we sort
// about the specified index.
assert!(v == [1, 2, -3, 4, -5] ||
        v == [1, 2, -3, -5, 4] ||
        v == [2, 1, -3, 4, -5] ||
        v == [2, 1, -3, -5, 4]);
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.partition_dedup" class="fn">partition_dedup</a>(&mut self) -\> (&mut <a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>, &mut <a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>)

where T: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>,

🔬This is a nightly-only experimental API. (`slice_partition_dedup`)

Moves all consecutive repeated elements to the end of the slice according to the [`PartialEq`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") trait implementation.

Returns two slices. The first contains no consecutive repeated elements. The second contains all the duplicates in no specified order.

If the slice is sorted, the first returned slice contains no duplicates.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-85" class="doc-anchor">§</a>Examples

``` rust
#![feature(slice_partition_dedup)]

let mut slice = [1, 2, 2, 3, 3, 2, 1, 1];

let (dedup, duplicates) = slice.partition_dedup();

assert_eq!(dedup, [1, 2, 3, 2, 1]);
assert_eq!(duplicates, [2, 3, 1]);
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.partition_dedup_by" class="fn">partition_dedup_by</a>\<F\>(&mut self, same_bucket: F) -\> (&mut <a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>, &mut <a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>)

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut T</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut T</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

🔬This is a nightly-only experimental API. (`slice_partition_dedup`)

Moves all but the first of consecutive elements to the end of the slice satisfying a given equality relation.

Returns two slices. The first contains no consecutive repeated elements. The second contains all the duplicates in no specified order.

The `same_bucket` function is passed references to two elements from the slice and must determine if the elements compare equal. The elements are passed in opposite order from their order in the slice, so if `same_bucket(a, b)` returns `true`, `a` is moved at the end of the slice.

If the slice is sorted, the first returned slice contains no duplicates.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-86" class="doc-anchor">§</a>Examples

``` rust
#![feature(slice_partition_dedup)]

let mut slice = ["foo", "Foo", "BAZ", "Bar", "bar", "baz", "BAZ"];

let (dedup, duplicates) = slice.partition_dedup_by(|a, b| a.eq_ignore_ascii_case(b));

assert_eq!(dedup, ["foo", "BAZ", "Bar", "baz"]);
assert_eq!(duplicates, ["bar", "Foo", "BAZ"]);
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.partition_dedup_by_key" class="fn">partition_dedup_by_key</a>\<K, F\>(&mut self, key: F) -\> (&mut <a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>, &mut <a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>)

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut T</a>) -\> K, K: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>,

🔬This is a nightly-only experimental API. (`slice_partition_dedup`)

Moves all but the first of consecutive elements to the end of the slice that resolve to the same key.

Returns two slices. The first contains no consecutive repeated elements. The second contains all the duplicates in no specified order.

If the slice is sorted, the first returned slice contains no duplicates.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-87" class="doc-anchor">§</a>Examples

``` rust
#![feature(slice_partition_dedup)]

let mut slice = [10, 20, 21, 30, 30, 20, 11, 13];

let (dedup, duplicates) = slice.partition_dedup_by_key(|i| *i / 10);

assert_eq!(dedup, [10, 20, 30, 20, 11]);
assert_eq!(duplicates, [21, 30, 13]);
```

1.26.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#3633" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.rotate_left" class="fn">rotate_left</a>(&mut self, mid: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Rotates the slice in-place such that the first `mid` elements of the slice move to the end while the last `self.len() - mid` elements move to the front.

After calling `rotate_left`, the element previously at index `mid` will become the first element in the slice.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#panics-26" class="doc-anchor">§</a>Panics

This function will panic if `mid` is greater than the length of the slice. Note that `mid == self.len()` does *not* panic and is a no-op rotation.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#complexity" class="doc-anchor">§</a>Complexity

Takes linear (in `self.len()`) time.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-88" class="doc-anchor">§</a>Examples

``` rust
let mut a = ['a', 'b', 'c', 'd', 'e', 'f'];
a.rotate_left(2);
assert_eq!(a, ['c', 'd', 'e', 'f', 'a', 'b']);
```

Rotating a subslice:

``` rust
let mut a = ['a', 'b', 'c', 'd', 'e', 'f'];
a[1..5].rotate_left(1);
assert_eq!(a, ['a', 'c', 'd', 'e', 'b', 'f']);
```

1.26.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#3679" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.rotate_right" class="fn">rotate_right</a>(&mut self, k: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Rotates the slice in-place such that the first `self.len() - k` elements of the slice move to the end while the last `k` elements move to the front.

After calling `rotate_right`, the element previously at index `self.len() - k` will become the first element in the slice.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#panics-27" class="doc-anchor">§</a>Panics

This function will panic if `k` is greater than the length of the slice. Note that `k == self.len()` does *not* panic and is a no-op rotation.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#complexity-1" class="doc-anchor">§</a>Complexity

Takes linear (in `self.len()`) time.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-89" class="doc-anchor">§</a>Examples

``` rust
let mut a = ['a', 'b', 'c', 'd', 'e', 'f'];
a.rotate_right(2);
assert_eq!(a, ['e', 'f', 'a', 'b', 'c', 'd']);
```

Rotating a subslice:

``` rust
let mut a = ['a', 'b', 'c', 'd', 'e', 'f'];
a[1..5].rotate_right(1);
assert_eq!(a, ['a', 'e', 'b', 'c', 'd', 'f']);
```

1.50.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#3702-3704" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.fill" class="fn">fill</a>(&mut self, value: T)

where T: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>,

Fills `self` with elements by cloning `value`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-90" class="doc-anchor">§</a>Examples

``` rust
let mut buf = vec![0; 10];
buf.fill(1);
assert_eq!(buf, vec![1; 10]);
```

1.51.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#3726-3728" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.fill_with" class="fn">fill_with</a>\<F\>(&mut self, f: F)

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>() -\> T,

Fills `self` with elements returned by calling a closure repeatedly.

This method uses a closure to create new values. If you’d rather [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") a given value, use [`fill`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.fill "method slice::fill"). If you want to use the [`Default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") trait to generate values, you can pass [`Default::default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default "associated function core::default::Default::default") as the argument.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-91" class="doc-anchor">§</a>Examples

``` rust
let mut buf = vec![1; 10];
buf.fill_with(Default::default);
assert_eq!(buf, vec![0; 10]);
```

1.7.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#3789-3791" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.clone_from_slice" class="fn">clone_from_slice</a>(&mut self, src: &<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>)

where T: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>,

Copies the elements from `src` into `self`.

The length of `src` must be the same as `self`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#panics-28" class="doc-anchor">§</a>Panics

This function will panic if the two slices have different lengths.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-92" class="doc-anchor">§</a>Examples

Cloning two elements from a slice into another:

``` rust
let src = [1, 2, 3, 4];
let mut dst = [0, 0];

// Because the slices have to be the same length,
// we slice the source slice from four elements
// to two. It will panic if we don't do this.
dst.clone_from_slice(&src[2..]);

assert_eq!(src, [1, 2, 3, 4]);
assert_eq!(dst, [3, 4]);
```

Rust enforces that there can only be one mutable reference with no immutable references to a particular piece of data in a particular scope. Because of this, attempting to use `clone_from_slice` on a single slice will result in a compile failure:

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#" class="tooltip" title="This example deliberately fails to compile">ⓘ</a>

``` rust
let mut slice = [1, 2, 3, 4, 5];

slice[..2].clone_from_slice(&slice[3..]); // compile fail!
```

To work around this, we can use [`split_at_mut`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.split_at_mut "method slice::split_at_mut") to create two distinct sub-slices from a slice:

``` rust
let mut slice = [1, 2, 3, 4, 5];

{
    let (left, right) = slice.split_at_mut(2);
    left.clone_from_slice(&right[1..]);
}

assert_eq!(slice, [4, 5, 3, 4, 5]);
```

1.9.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#3855-3857" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.copy_from_slice" class="fn">copy_from_slice</a>(&mut self, src: &<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>)

where T: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a>,

Copies all elements from `src` into `self`, using a memcpy.

The length of `src` must be the same as `self`.

If `T` does not implement `Copy`, use [`clone_from_slice`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.clone_from_slice "method slice::clone_from_slice").

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#panics-29" class="doc-anchor">§</a>Panics

This function will panic if the two slices have different lengths.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-93" class="doc-anchor">§</a>Examples

Copying two elements from a slice into another:

``` rust
let src = [1, 2, 3, 4];
let mut dst = [0, 0];

// Because the slices have to be the same length,
// we slice the source slice from four elements
// to two. It will panic if we don't do this.
dst.copy_from_slice(&src[2..]);

assert_eq!(src, [1, 2, 3, 4]);
assert_eq!(dst, [3, 4]);
```

Rust enforces that there can only be one mutable reference with no immutable references to a particular piece of data in a particular scope. Because of this, attempting to use `copy_from_slice` on a single slice will result in a compile failure:

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#" class="tooltip" title="This example deliberately fails to compile">ⓘ</a>

``` rust
let mut slice = [1, 2, 3, 4, 5];

slice[..2].copy_from_slice(&slice[3..]); // compile fail!
```

To work around this, we can use [`split_at_mut`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.split_at_mut "method slice::split_at_mut") to create two distinct sub-slices from a slice:

``` rust
let mut slice = [1, 2, 3, 4, 5];

{
    let (left, right) = slice.split_at_mut(2);
    left.copy_from_slice(&right[1..]);
}

assert_eq!(slice, [4, 5, 3, 4, 5]);
```

1.37.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#3911-3913" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.copy_within" class="fn">copy_within</a>\<R\>(&mut self, src: R, dest: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

where R: <a href="https://doc.rust-lang.org/nightly/core/ops/range/trait.RangeBounds.html" class="trait" title="trait core::ops::range::RangeBounds">RangeBounds</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>, T: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a>,

Copies elements from one part of the slice to another part of itself, using a memmove.

`src` is the range within `self` to copy from. `dest` is the starting index of the range within `self` to copy to, which will have the same length as `src`. The two ranges may overlap. The ends of the two ranges must be less than or equal to `self.len()`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#panics-30" class="doc-anchor">§</a>Panics

This function will panic if either range exceeds the end of the slice, or if the end of `src` is before the start.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-94" class="doc-anchor">§</a>Examples

Copying four bytes within a slice:

``` rust
let mut bytes = *b"Hello, World!";

bytes.copy_within(1..5, 8);

assert_eq!(&bytes, b"Hello, Wello!");
```

1.27.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#3979" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.swap_with_slice" class="fn">swap_with_slice</a>(&mut self, other: &mut <a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>)

Swaps all elements in `self` with those in `other`.

The length of `other` must be the same as `self`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#panics-31" class="doc-anchor">§</a>Panics

This function will panic if the two slices have different lengths.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#example-9" class="doc-anchor">§</a>Example

Swapping two elements across slices:

``` rust
let mut slice1 = [0, 0];
let mut slice2 = [1, 2, 3, 4];

slice1.swap_with_slice(&mut slice2[2..]);

assert_eq!(slice1, [3, 4]);
assert_eq!(slice2, [1, 2, 0, 0]);
```

Rust enforces that there can only be one mutable reference to a particular piece of data in a particular scope. Because of this, attempting to use `swap_with_slice` on a single slice will result in a compile failure:

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#" class="tooltip" title="This example deliberately fails to compile">ⓘ</a>

``` rust
let mut slice = [1, 2, 3, 4, 5];
slice[..2].swap_with_slice(&mut slice[3..]); // compile fail!
```

To work around this, we can use [`split_at_mut`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.split_at_mut "method slice::split_at_mut") to create two distinct mutable sub-slices from a slice:

``` rust
let mut slice = [1, 2, 3, 4, 5];

{
    let (left, right) = slice.split_at_mut(2);
    left.swap_with_slice(&mut right[1..]);
}

assert_eq!(slice, [4, 5, 3, 1, 2]);
```

1.30.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#4056" class="src">Source</a>

#### pub unsafe fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.align_to" class="fn">align_to</a>\<U\>(&self) -\> (&<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>, &<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[U]</a>, &<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>)

Transmutes the slice to a slice of another type, ensuring alignment of the types is maintained.

This method splits the slice into three distinct slices: prefix, correctly aligned middle slice of a new type, and the suffix slice. The middle part will be as big as possible under the given alignment constraint and element size.

This method has no purpose when either input element `T` or output element `U` are zero-sized and will return the original slice without splitting anything.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#safety-13" class="doc-anchor">§</a>Safety

This method is essentially a `transmute` with respect to the elements in the returned middle slice, so all the usual caveats pertaining to `transmute::<T, U>` also apply here.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-95" class="doc-anchor">§</a>Examples

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

1.30.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#4121" class="src">Source</a>

#### pub unsafe fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.align_to_mut" class="fn">align_to_mut</a>\<U\>(&mut self) -\> (&mut <a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>, &mut <a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[U]</a>, &mut <a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>)

Transmutes the mutable slice to a mutable slice of another type, ensuring alignment of the types is maintained.

This method splits the slice into three distinct slices: prefix, correctly aligned middle slice of a new type, and the suffix slice. The middle part will be as big as possible under the given alignment constraint and element size.

This method has no purpose when either input element `T` or output element `U` are zero-sized and will return the original slice without splitting anything.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#safety-14" class="doc-anchor">§</a>Safety

This method is essentially a `transmute` with respect to the elements in the returned middle slice, so all the usual caveats pertaining to `transmute::<T, U>` also apply here.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-96" class="doc-anchor">§</a>Examples

Basic usage:

``` rust
unsafe {
    let mut bytes: [u8; 7] = [1, 2, 3, 4, 5, 6, 7];
    let (prefix, shorts, suffix) = bytes.align_to_mut::<u16>();
    // less_efficient_algorithm_for_bytes(prefix);
    // more_efficient_algorithm_for_aligned_shorts(shorts);
    // less_efficient_algorithm_for_bytes(suffix);
}
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.as_simd" class="fn">as_simd</a>\<const LANES: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>(&self) -\> (&<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>, &\[<a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<T, LANES\>\], &<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>)

where <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<T, LANES\>: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">[T; LANES]</a>\>, T: <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/trait.SimdElement.html" class="trait" title="trait core::core_simd::vector::SimdElement">SimdElement</a>, <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/struct.LaneCount.html" class="struct" title="struct core::core_simd::lane_count::LaneCount">LaneCount</a>\<LANES\>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/trait.SupportedLaneCount.html" class="trait" title="trait core::core_simd::lane_count::SupportedLaneCount">SupportedLaneCount</a>,

🔬This is a nightly-only experimental API. (`portable_simd`)

Splits a slice into a prefix, a middle of aligned SIMD types, and a suffix.

This is a safe wrapper around [`slice::align_to`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.align_to "method slice::align_to"), so inherits the same guarantees as that method.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#panics-32" class="doc-anchor">§</a>Panics

This will panic if the size of the SIMD type is different from `LANES` times that of the scalar.

At the time of writing, the trait restrictions on `Simd<T, LANES>` keeps that from ever happening, as only power-of-two numbers of lanes are supported. It’s possible that, in the future, those restrictions might be lifted in a way that would make it possible to see panics from this method for something like `LANES == 3`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-97" class="doc-anchor">§</a>Examples

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

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.as_simd_mut" class="fn">as_simd_mut</a>\<const LANES: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>( &mut self, ) -\> (&mut <a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>, &mut \[<a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<T, LANES\>\], &mut <a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>)

where <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<T, LANES\>: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html" class="trait" title="trait core::convert::AsMut">AsMut</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">[T; LANES]</a>\>, T: <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/trait.SimdElement.html" class="trait" title="trait core::core_simd::vector::SimdElement">SimdElement</a>, <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/struct.LaneCount.html" class="struct" title="struct core::core_simd::lane_count::LaneCount">LaneCount</a>\<LANES\>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/trait.SupportedLaneCount.html" class="trait" title="trait core::core_simd::lane_count::SupportedLaneCount">SupportedLaneCount</a>,

🔬This is a nightly-only experimental API. (`portable_simd`)

Splits a mutable slice into a mutable prefix, a middle of aligned SIMD types, and a mutable suffix.

This is a safe wrapper around [`slice::align_to_mut`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.align_to_mut "method slice::align_to_mut"), so inherits the same guarantees as that method.

This is the mutable version of [`slice::as_simd`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_simd "method slice::as_simd"); see that for examples.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#panics-33" class="doc-anchor">§</a>Panics

This will panic if the size of the SIMD type is different from `LANES` times that of the scalar.

At the time of writing, the trait restrictions on `Simd<T, LANES>` keeps that from ever happening, as only power-of-two numbers of lanes are supported. It’s possible that, in the future, those restrictions might be lifted in a way that would make it possible to see panics from this method for something like `LANES == 3`.

1.82.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#4287-4289" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.is_sorted" class="fn">is_sorted</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a>,

Checks if the elements of this slice are sorted.

That is, for each element `a` and its following element `b`, `a <= b` must hold. If the slice yields exactly zero or one element, `true` is returned.

Note that if `Self::Item` is only `PartialOrd`, but not `Ord`, the above definition implies that this function returns `false` if any two consecutive items are not comparable.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-98" class="doc-anchor">§</a>Examples

``` rust
let empty: [i32; 0] = [];

assert!([1, 2, 2, 9].is_sorted());
assert!(![1, 3, 2, 4].is_sorted());
assert!([0].is_sorted());
assert!(empty.is_sorted());
assert!(![0.0, 1.0, f32::NAN].is_sorted());
```

1.82.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#4330-4332" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.is_sorted_by" class="fn">is_sorted_by</a>\<'a, F\>(&'a self, compare: F) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

Checks if the elements of this slice are sorted using the given comparator function.

Instead of using `PartialOrd::partial_cmp`, this function uses the given `compare` function to determine whether two elements are to be considered in sorted order.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-99" class="doc-anchor">§</a>Examples

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

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.is_sorted_by_key" class="fn">is_sorted_by_key</a>\<'a, F, K\>(&'a self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>) -\> K, K: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a>,

Checks if the elements of this slice are sorted using the given key extraction function.

Instead of comparing the slice’s elements directly, this function compares the keys of the elements, as determined by `f`. Apart from that, it’s equivalent to [`is_sorted`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.is_sorted "method slice::is_sorted"); see its documentation for more information.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-100" class="doc-anchor">§</a>Examples

``` rust
assert!(["c", "bb", "aaa"].is_sorted_by_key(|s| s.len()));
assert!(![-2i32, -1, 0, 3].is_sorted_by_key(|n| n.abs()));
```

1.52.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#4413-4415" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.partition_point" class="fn">partition_point</a>\<P\>(&self, pred: P) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

where P: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

Returns the index of the partition point according to the given predicate (the index of the first element of the second partition).

The slice is assumed to be partitioned according to the given predicate. This means that all elements for which the predicate returns true are at the start of the slice and all elements for which the predicate returns false are at the end. For example, `[7, 15, 3, 5, 4, 12, 6]` is partitioned under the predicate `x % 2 != 0` (all odd numbers are at the start, all even at the end).

If this slice is not partitioned, the returned result is unspecified and meaningless, as this method performs a kind of binary search.

See also [`binary_search`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.binary_search "method slice::binary_search"), [`binary_search_by`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.binary_search_by "method slice::binary_search_by"), and [`binary_search_by_key`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.binary_search_by_key "method slice::binary_search_by_key").

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-101" class="doc-anchor">§</a>Examples

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

1.87.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#4465-4468" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.split_off" class="fn">split_off</a>\<'a, R\>(self: &mut &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>, range: R) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>\>

where R: <a href="https://doc.rust-lang.org/nightly/core/ops/range/trait.OneSidedRange.html" class="trait" title="trait core::ops::range::OneSidedRange">OneSidedRange</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>,

Removes the subslice corresponding to the given range and returns a reference to it.

Returns `None` and does not modify the slice if the given range is out of bounds.

Note that this method only accepts one-sided ranges such as `2..` or `..6`, but not `2..6`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-102" class="doc-anchor">§</a>Examples

Splitting off the first three elements of a slice:

``` rust
let mut slice: &[_] = &['a', 'b', 'c', 'd'];
let mut first_three = slice.split_off(..3).unwrap();

assert_eq!(slice, &['d']);
assert_eq!(first_three, &['a', 'b', 'c']);
```

Splitting off a slice starting with the third element:

``` rust
let mut slice: &[_] = &['a', 'b', 'c', 'd'];
let mut tail = slice.split_off(2..).unwrap();

assert_eq!(slice, &['a', 'b']);
assert_eq!(tail, &['c', 'd']);
```

Getting `None` when `range` is out of bounds:

``` rust
let mut slice: &[_] = &['a', 'b', 'c', 'd'];

assert_eq!(None, slice.split_off(5..));
assert_eq!(None, slice.split_off(..5));
assert_eq!(None, slice.split_off(..=4));
let expected: &[char] = &['a', 'b', 'c', 'd'];
assert_eq!(Some(expected), slice.split_off(..4));
```

1.87.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#4531-4534" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.split_off_mut" class="fn">split_off_mut</a>\<'a, R\>( self: &mut &'a mut <a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>, range: R, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a mut <a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>\>

where R: <a href="https://doc.rust-lang.org/nightly/core/ops/range/trait.OneSidedRange.html" class="trait" title="trait core::ops::range::OneSidedRange">OneSidedRange</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>,

Removes the subslice corresponding to the given range and returns a mutable reference to it.

Returns `None` and does not modify the slice if the given range is out of bounds.

Note that this method only accepts one-sided ranges such as `2..` or `..6`, but not `2..6`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-103" class="doc-anchor">§</a>Examples

Splitting off the first three elements of a slice:

``` rust
let mut slice: &mut [_] = &mut ['a', 'b', 'c', 'd'];
let mut first_three = slice.split_off_mut(..3).unwrap();

assert_eq!(slice, &mut ['d']);
assert_eq!(first_three, &mut ['a', 'b', 'c']);
```

Splitting off a slice starting with the third element:

``` rust
let mut slice: &mut [_] = &mut ['a', 'b', 'c', 'd'];
let mut tail = slice.split_off_mut(2..).unwrap();

assert_eq!(slice, &mut ['a', 'b']);
assert_eq!(tail, &mut ['c', 'd']);
```

Getting `None` when `range` is out of bounds:

``` rust
let mut slice: &mut [_] = &mut ['a', 'b', 'c', 'd'];

assert_eq!(None, slice.split_off_mut(5..));
assert_eq!(None, slice.split_off_mut(..5));
assert_eq!(None, slice.split_off_mut(..=4));
let expected: &mut [_] = &mut ['a', 'b', 'c', 'd'];
assert_eq!(Some(expected), slice.split_off_mut(..4));
```

1.87.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#4569" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.split_off_first" class="fn">split_off_first</a>\<'a\>(self: &mut &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>\>

Removes the first element of the slice and returns a reference to it.

Returns `None` if the slice is empty.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-104" class="doc-anchor">§</a>Examples

``` rust
let mut slice: &[_] = &['a', 'b', 'c'];
let first = slice.split_off_first().unwrap();

assert_eq!(slice, &['b', 'c']);
assert_eq!(first, &'a');
```

1.87.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#4594" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.split_off_first_mut" class="fn">split_off_first_mut</a>\<'a\>(self: &mut &'a mut <a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a mut T</a>\>

Removes the first element of the slice and returns a mutable reference to it.

Returns `None` if the slice is empty.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-105" class="doc-anchor">§</a>Examples

``` rust
let mut slice: &mut [_] = &mut ['a', 'b', 'c'];
let first = slice.split_off_first_mut().unwrap();
*first = 'd';

assert_eq!(slice, &['b', 'c']);
assert_eq!(first, &'d');
```

1.87.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#4619" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.split_off_last" class="fn">split_off_last</a>\<'a\>(self: &mut &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>\>

Removes the last element of the slice and returns a reference to it.

Returns `None` if the slice is empty.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-106" class="doc-anchor">§</a>Examples

``` rust
let mut slice: &[_] = &['a', 'b', 'c'];
let last = slice.split_off_last().unwrap();

assert_eq!(slice, &['a', 'b']);
assert_eq!(last, &'c');
```

1.87.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#4644" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.split_off_last_mut" class="fn">split_off_last_mut</a>\<'a\>(self: &mut &'a mut <a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a mut T</a>\>

Removes the last element of the slice and returns a mutable reference to it.

Returns `None` if the slice is empty.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-107" class="doc-anchor">§</a>Examples

``` rust
let mut slice: &mut [_] = &mut ['a', 'b', 'c'];
let last = slice.split_off_last_mut().unwrap();
*last = 'd';

assert_eq!(slice, &['a', 'b']);
assert_eq!(last, &'d');
```

1.86.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#4701-4706" class="src">Source</a>

#### pub unsafe fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.get_disjoint_unchecked_mut" class="fn">get_disjoint_unchecked_mut</a>\<I, const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>( &mut self, indices: <a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">[I; N]</a>, ) -\> \[&mut \<I as <a href="https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html" class="trait" title="trait core::slice::index::SliceIndex">SliceIndex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>\>\>::<a href="https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html#associatedtype.Output" class="associatedtype" title="type core::slice::index::SliceIndex::Output">Output</a>; <a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">N</a>\]

where I: <a href="https://doc.rust-lang.org/nightly/core/slice/trait.GetDisjointMutIndex.html" class="trait" title="trait core::slice::GetDisjointMutIndex">GetDisjointMutIndex</a> + <a href="https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html" class="trait" title="trait core::slice::index::SliceIndex">SliceIndex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>\>,

Returns mutable references to many indices at once, without doing any checks.

An index can be either a `usize`, a [`Range`](https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html "struct core::ops::range::Range") or a [`RangeInclusive`](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeInclusive.html "struct core::ops::range::RangeInclusive"). Note that this method takes an array, so all indices must be of the same type. If passed an array of `usize`s this method gives back an array of mutable references to single elements, while if passed an array of ranges it gives back an array of mutable references to slices.

For a safe alternative see [`get_disjoint_mut`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.get_disjoint_mut "method slice::get_disjoint_mut").

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#safety-15" class="doc-anchor">§</a>Safety

Calling this method with overlapping or out-of-bounds indices is *[undefined behavior](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)* even if the resulting references are not used.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-108" class="doc-anchor">§</a>Examples

``` rust
let x = &mut [1, 2, 4];

unsafe {
    let [a, b] = x.get_disjoint_unchecked_mut([0, 2]);
    *a *= 10;
    *b *= 100;
}
assert_eq!(x, &[10, 2, 400]);

unsafe {
    let [a, b] = x.get_disjoint_unchecked_mut([0..1, 1..3]);
    a[0] = 8;
    b[0] = 88;
    b[1] = 888;
}
assert_eq!(x, &[8, 88, 888]);

unsafe {
    let [a, b] = x.get_disjoint_unchecked_mut([1..=2, 0..=0]);
    a[0] = 11;
    a[1] = 111;
    b[0] = 1;
}
assert_eq!(x, &[1, 11, 111]);
```

1.86.0 · <a href="https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#4768-4773" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.get_disjoint_mut" class="fn">get_disjoint_mut</a>\<I, const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>( &mut self, indices: <a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">[I; N]</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\[&mut \<I as <a href="https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html" class="trait" title="trait core::slice::index::SliceIndex">SliceIndex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>\>\>::<a href="https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html#associatedtype.Output" class="associatedtype" title="type core::slice::index::SliceIndex::Output">Output</a>; <a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">N</a>\], <a href="https://doc.rust-lang.org/nightly/core/slice/enum.GetDisjointMutError.html" class="enum" title="enum core::slice::GetDisjointMutError">GetDisjointMutError</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/slice/trait.GetDisjointMutIndex.html" class="trait" title="trait core::slice::GetDisjointMutIndex">GetDisjointMutIndex</a> + <a href="https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html" class="trait" title="trait core::slice::index::SliceIndex">SliceIndex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>\>,

Returns mutable references to many indices at once.

An index can be either a `usize`, a [`Range`](https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html "struct core::ops::range::Range") or a [`RangeInclusive`](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeInclusive.html "struct core::ops::range::RangeInclusive"). Note that this method takes an array, so all indices must be of the same type. If passed an array of `usize`s this method gives back an array of mutable references to single elements, while if passed an array of ranges it gives back an array of mutable references to slices.

Returns an error if any index is out-of-bounds, or if there are overlapping indices. An empty range is not considered to overlap if it is located at the beginning or at the end of another range, but is considered to overlap if it is located in the middle.

This method does a O(n^2) check to check that there are no overlapping indices, so be careful when passing many indices.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-109" class="doc-anchor">§</a>Examples

``` rust
let v = &mut [1, 2, 3];
if let Ok([a, b]) = v.get_disjoint_mut([0, 2]) {
    *a = 413;
    *b = 612;
}
assert_eq!(v, &[413, 2, 612]);

if let Ok([a, b]) = v.get_disjoint_mut([0..1, 1..3]) {
    a[0] = 8;
    b[0] = 88;
    b[1] = 888;
}
assert_eq!(v, &[8, 88, 888]);

if let Ok([a, b]) = v.get_disjoint_mut([1..=2, 0..=0]) {
    a[0] = 11;
    a[1] = 111;
    b[0] = 1;
}
assert_eq!(v, &[1, 11, 111]);
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.element_offset" class="fn">element_offset</a>(&self, element: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

🔬This is a nightly-only experimental API. (`substr_range`)

Returns the index that an element reference points to.

Returns `None` if `element` does not point to the start of an element within the slice.

This method is useful for extending slice iterators like [`slice::split`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.split "method slice::split").

Note that this uses pointer arithmetic and **does not compare elements**. To find the index of an element via comparison, use [`.iter().position()`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.position "method core::iter::traits::iterator::Iterator::position") instead.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#panics-34" class="doc-anchor">§</a>Panics

Panics if `T` is zero-sized.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-110" class="doc-anchor">§</a>Examples

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

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.subslice_range" class="fn">subslice_range</a>(&self, subslice: &<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\>

🔬This is a nightly-only experimental API. (`substr_range`)

Returns the range of indices that a subslice points to.

Returns `None` if `subslice` does not point within the slice or if it is not aligned with the elements in the slice.

This method **does not compare elements**. Instead, this method finds the location in the slice that `subslice` was obtained from. To find the index of a subslice via comparison, instead use [`.windows()`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.windows "method slice::windows")[`.position()`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.position "method core::iter::traits::iterator::Iterator::position").

This method is useful for extending slice iterators like [`slice::split`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.split "method slice::split").

Note that this may return a false positive (either `Some(0..0)` or `Some(self.len()..self.len())`) if `subslice` has a length of zero and points to the beginning or end of another, separate, slice.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#panics-35" class="doc-anchor">§</a>Panics

Panics if `T` is zero-sized.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-111" class="doc-anchor">§</a>Examples

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

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.utf8_chunks" class="fn">utf8_chunks</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/str/lossy/struct.Utf8Chunks.html" class="struct" title="struct core::str::lossy::Utf8Chunks">Utf8Chunks</a>\<'\_\>

Creates an iterator over the contiguous valid UTF-8 ranges of this slice, and the non-UTF-8 fragments in between.

See the [`Utf8Chunk`](https://doc.rust-lang.org/nightly/core/str/lossy/struct.Utf8Chunk.html "struct core::str::lossy::Utf8Chunk") type for documentation of the items yielded by this iterator.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-112" class="doc-anchor">§</a>Examples

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

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/slice.rs.html#129-131" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.sort" class="fn">sort</a>(&mut self)

where T: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a>,

Sorts the slice in ascending order, preserving initial order of equal elements.

This sort is stable (i.e., does not reorder equal elements) and *O*(*n* \* log(*n*)) worst-case.

If the implementation of [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") for `T` does not implement a [total order](https://en.wikipedia.org/wiki/Total_order), the function may panic; even if the function exits normally, the resulting order of elements in the slice is unspecified. See also the note on panicking below.

When applicable, unstable sorting is preferred because it is generally faster than stable sorting and it doesn’t allocate auxiliary memory. See [`sort_unstable`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.sort_unstable "method slice::sort_unstable"). The exception are partially sorted slices, which may be better served with `slice::sort`.

Sorting types that only implement [`PartialOrd`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd") such as [`f32`](https://doc.rust-lang.org/nightly/std/primitive.f32.html "primitive f32") and [`f64`](https://doc.rust-lang.org/nightly/std/primitive.f64.html "primitive f64") require additional precautions. For example, `f32::NAN != f32::NAN`, which doesn’t fulfill the reflexivity requirement of [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"). By using an alternative comparison function with `slice::sort_by` such as [`f32::total_cmp`](https://doc.rust-lang.org/nightly/std/primitive.f32.html#method.total_cmp "method f32::total_cmp") or [`f64::total_cmp`](https://doc.rust-lang.org/nightly/std/primitive.f64.html#method.total_cmp "method f64::total_cmp") that defines a [total order](https://en.wikipedia.org/wiki/Total_order) users can sort slices containing floating-point values. Alternatively, if all values in the slice are guaranteed to be in a subset for which [`PartialOrd::partial_cmp`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp "method core::cmp::PartialOrd::partial_cmp") forms a [total order](https://en.wikipedia.org/wiki/Total_order), it’s possible to sort the slice with `sort_by(|a, b| a.partial_cmp(b).unwrap())`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#current-implementation-6" class="doc-anchor">§</a>Current implementation

The current implementation is based on [driftsort](https://github.com/Voultapher/driftsort) by Orson Peters and Lukas Bergdoll, which combines the fast average case of quicksort with the fast worst case and partial run detection of mergesort, achieving linear time on fully sorted and reversed inputs. On inputs with k distinct elements, the expected time to sort the data is *O*(*n* \* log(*k*)).

The auxiliary memory allocation behavior depends on the input length. Short slices are handled without allocation, medium sized slices allocate `self.len()` and beyond that it clamps at `self.len() / 2`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#panics-36" class="doc-anchor">§</a>Panics

May panic if the implementation of [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") for `T` does not implement a [total order](https://en.wikipedia.org/wiki/Total_order), or if the [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") implementation itself panics.

All safe functions on slices preserve the invariant that even if the function panics, all original elements will remain in the slice and any possible modifications via interior mutability are observed in the input. This ensures that recovery code (for instance inside of a `Drop` or following a `catch_unwind`) will still have access to all the original elements. For instance, if the slice belongs to a `Vec`, the `Vec::drop` method will be able to dispose of all contained elements.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-113" class="doc-anchor">§</a>Examples

``` rust
let mut v = [4, -5, 1, -3, 2];

v.sort();
assert_eq!(v, [-5, -3, 1, 2, 4]);
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/slice.rs.html#190-192" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.sort_by" class="fn">sort_by</a>\<F\>(&mut self, compare: F)

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>,

Sorts the slice in ascending order with a comparison function, preserving initial order of equal elements.

This sort is stable (i.e., does not reorder equal elements) and *O*(*n* \* log(*n*)) worst-case.

If the comparison function `compare` does not implement a [total order](https://en.wikipedia.org/wiki/Total_order), the function may panic; even if the function exits normally, the resulting order of elements in the slice is unspecified. See also the note on panicking below.

For example `|a, b| (a - b).cmp(a)` is a comparison function that is neither transitive nor reflexive nor total, `a < b < c < a` with `a = 1, b = 2, c = 3`. For more information and examples see the [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") documentation.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#current-implementation-7" class="doc-anchor">§</a>Current implementation

The current implementation is based on [driftsort](https://github.com/Voultapher/driftsort) by Orson Peters and Lukas Bergdoll, which combines the fast average case of quicksort with the fast worst case and partial run detection of mergesort, achieving linear time on fully sorted and reversed inputs. On inputs with k distinct elements, the expected time to sort the data is *O*(*n* \* log(*k*)).

The auxiliary memory allocation behavior depends on the input length. Short slices are handled without allocation, medium sized slices allocate `self.len()` and beyond that it clamps at `self.len() / 2`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#panics-37" class="doc-anchor">§</a>Panics

May panic if `compare` does not implement a [total order](https://en.wikipedia.org/wiki/Total_order), or if `compare` itself panics.

All safe functions on slices preserve the invariant that even if the function panics, all original elements will remain in the slice and any possible modifications via interior mutability are observed in the input. This ensures that recovery code (for instance inside of a `Drop` or following a `catch_unwind`) will still have access to all the original elements. For instance, if the slice belongs to a `Vec`, the `Vec::drop` method will be able to dispose of all contained elements.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-114" class="doc-anchor">§</a>Examples

``` rust
let mut v = [4, -5, 1, -3, 2];
v.sort_by(|a, b| a.cmp(b));
assert_eq!(v, [-5, -3, 1, 2, 4]);

// reverse sorting
v.sort_by(|a, b| b.cmp(a));
assert_eq!(v, [4, 2, 1, -3, -5]);
```

1.7.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/slice.rs.html#245-248" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.sort_by_key" class="fn">sort_by_key</a>\<K, F\>(&mut self, f: F)

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>) -\> K, K: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a>,

Sorts the slice in ascending order with a key extraction function, preserving initial order of equal elements.

This sort is stable (i.e., does not reorder equal elements) and *O*(*m* \* *n* \* log(*n*)) worst-case, where the key function is *O*(*m*).

If the implementation of [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") for `K` does not implement a [total order](https://en.wikipedia.org/wiki/Total_order), the function may panic; even if the function exits normally, the resulting order of elements in the slice is unspecified. See also the note on panicking below.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#current-implementation-8" class="doc-anchor">§</a>Current implementation

The current implementation is based on [driftsort](https://github.com/Voultapher/driftsort) by Orson Peters and Lukas Bergdoll, which combines the fast average case of quicksort with the fast worst case and partial run detection of mergesort, achieving linear time on fully sorted and reversed inputs. On inputs with k distinct elements, the expected time to sort the data is *O*(*n* \* log(*k*)).

The auxiliary memory allocation behavior depends on the input length. Short slices are handled without allocation, medium sized slices allocate `self.len()` and beyond that it clamps at `self.len() / 2`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#panics-38" class="doc-anchor">§</a>Panics

May panic if the implementation of [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") for `K` does not implement a [total order](https://en.wikipedia.org/wiki/Total_order), or if the [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") implementation or the key-function `f` panics.

All safe functions on slices preserve the invariant that even if the function panics, all original elements will remain in the slice and any possible modifications via interior mutability are observed in the input. This ensures that recovery code (for instance inside of a `Drop` or following a `catch_unwind`) will still have access to all the original elements. For instance, if the slice belongs to a `Vec`, the `Vec::drop` method will be able to dispose of all contained elements.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-115" class="doc-anchor">§</a>Examples

``` rust
let mut v = [4i32, -5, 1, -3, 2];

v.sort_by_key(|k| k.abs());
assert_eq!(v, [1, 2, -3, 4, -5]);
```

1.34.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/slice.rs.html#310-313" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.sort_by_cached_key" class="fn">sort_by_cached_key</a>\<K, F\>(&mut self, f: F)

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>) -\> K, K: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a>,

Sorts the slice in ascending order with a key extraction function, preserving initial order of equal elements.

This sort is stable (i.e., does not reorder equal elements) and *O*(*m* \* *n* + *n* \* log(*n*)) worst-case, where the key function is *O*(*m*).

During sorting, the key function is called at most once per element, by using temporary storage to remember the results of key evaluation. The order of calls to the key function is unspecified and may change in future versions of the standard library.

If the implementation of [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") for `K` does not implement a [total order](https://en.wikipedia.org/wiki/Total_order), the function may panic; even if the function exits normally, the resulting order of elements in the slice is unspecified. See also the note on panicking below.

For simple key functions (e.g., functions that are property accesses or basic operations), [`sort_by_key`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.sort_by_key "method slice::sort_by_key") is likely to be faster.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#current-implementation-9" class="doc-anchor">§</a>Current implementation

The current implementation is based on [instruction-parallel-network sort](https://github.com/Voultapher/sort-research-rs/tree/main/ipnsort) by Lukas Bergdoll, which combines the fast average case of randomized quicksort with the fast worst case of heapsort, while achieving linear time on fully sorted and reversed inputs. And *O*(*k* \* log(*n*)) where *k* is the number of distinct elements in the input. It leverages superscalar out-of-order execution capabilities commonly found in CPUs, to efficiently perform the operation.

In the worst case, the algorithm allocates temporary storage in a `Vec<(K, usize)>` the length of the slice.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#panics-39" class="doc-anchor">§</a>Panics

May panic if the implementation of [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") for `K` does not implement a [total order](https://en.wikipedia.org/wiki/Total_order), or if the [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") implementation panics.

All safe functions on slices preserve the invariant that even if the function panics, all original elements will remain in the slice and any possible modifications via interior mutability are observed in the input. This ensures that recovery code (for instance inside of a `Drop` or following a `catch_unwind`) will still have access to all the original elements. For instance, if the slice belongs to a `Vec`, the `Vec::drop` method will be able to dispose of all contained elements.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-116" class="doc-anchor">§</a>Examples

``` rust
let mut v = [4i32, -5, 1, -3, 2, 10];

// Strings are sorted by lexicographical order.
v.sort_by_cached_key(|k| k.to_string());
assert_eq!(v, [-3, -5, 1, 10, 2, 4]);
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/slice.rs.html#370-372" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.to_vec" class="fn">to_vec</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<T\>

where T: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>,

Copies `self` into a new `Vec`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-117" class="doc-anchor">§</a>Examples

``` rust
let s = [10, 40, 30];
let x = s.to_vec();
// Here, `s` and `x` can be modified independently.
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.to_vec_in" class="fn">to_vec_in</a>\<A\>(&self, alloc: A) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<T, A\>

where A: <a href="https://doc.rust-lang.org/nightly/core/alloc/trait.Allocator.html" class="trait" title="trait core::alloc::Allocator">Allocator</a>, T: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>,

🔬This is a nightly-only experimental API. (`allocator_api`)

Copies `self` into a new `Vec` with an allocator.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-118" class="doc-anchor">§</a>Examples

``` rust
#![feature(allocator_api)]

use std::alloc::System;

let s = [10, 40, 30];
let x = s.to_vec_in(System);
// Here, `s` and `x` can be modified independently.
```

1.40.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/slice.rs.html#505-507" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.repeat" class="fn">repeat</a>(&self, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<T\>

where T: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a>,

Creates a vector by copying a slice `n` times.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#panics-40" class="doc-anchor">§</a>Panics

This function will panic if the capacity would overflow.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-119" class="doc-anchor">§</a>Examples

``` rust
assert_eq!([1, 2].repeat(3), vec![1, 2, 1, 2, 1, 2]);
```

A panic upon overflow:

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#" class="tooltip" title="This example panics">ⓘ</a>

``` rust
// this will panic at runtime
b"0123456789abcdef".repeat(usize::MAX);
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/slice.rs.html#573-575" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.concat" class="fn">concat</a>\<Item\>(&self) -\> \<<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a> as <a href="https://doc.rust-lang.org/nightly/alloc/slice/trait.Concat.html" class="trait" title="trait alloc::slice::Concat">Concat</a>\<Item\>\>::<a href="https://doc.rust-lang.org/nightly/alloc/slice/trait.Concat.html#associatedtype.Output" class="associatedtype" title="type alloc::slice::Concat::Output">Output</a> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#" class="tooltip" data-notable-ty="&lt;[T] as Concat&lt;Item&gt;&gt;::Output">ⓘ</a>

where <a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>: <a href="https://doc.rust-lang.org/nightly/alloc/slice/trait.Concat.html" class="trait" title="trait alloc::slice::Concat">Concat</a>\<Item\>, Item: ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Flattens a slice of `T` into a single value `Self::Output`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-120" class="doc-anchor">§</a>Examples

``` rust
assert_eq!(["hello", "world"].concat(), "helloworld");
assert_eq!([[1, 2], [3, 4]].concat(), [1, 2, 3, 4]);
```

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/slice.rs.html#592-594" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.join" class="fn">join</a>\<Separator\>( &self, sep: Separator, ) -\> \<<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a> as <a href="https://doc.rust-lang.org/nightly/alloc/slice/trait.Join.html" class="trait" title="trait alloc::slice::Join">Join</a>\<Separator\>\>::<a href="https://doc.rust-lang.org/nightly/alloc/slice/trait.Join.html#associatedtype.Output" class="associatedtype" title="type alloc::slice::Join::Output">Output</a> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#" class="tooltip" data-notable-ty="&lt;[T] as Join&lt;Separator&gt;&gt;::Output">ⓘ</a>

where <a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>: <a href="https://doc.rust-lang.org/nightly/alloc/slice/trait.Join.html" class="trait" title="trait alloc::slice::Join">Join</a>\<Separator\>,

Flattens a slice of `T` into a single value `Self::Output`, placing a given separator between each.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-121" class="doc-anchor">§</a>Examples

``` rust
assert_eq!(["hello", "world"].join(" "), "hello world");
assert_eq!([[1, 2], [3, 4]].join(&0), [1, 2, 0, 3, 4]);
assert_eq!([[1, 2], [3, 4]].join(&[0, 0][..]), [1, 2, 0, 0, 3, 4]);
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/slice.rs.html#612-614" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.connect" class="fn">connect</a>\<Separator\>( &self, sep: Separator, ) -\> \<<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a> as <a href="https://doc.rust-lang.org/nightly/alloc/slice/trait.Join.html" class="trait" title="trait alloc::slice::Join">Join</a>\<Separator\>\>::<a href="https://doc.rust-lang.org/nightly/alloc/slice/trait.Join.html#associatedtype.Output" class="associatedtype" title="type alloc::slice::Join::Output">Output</a> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#" class="tooltip" data-notable-ty="&lt;[T] as Join&lt;Separator&gt;&gt;::Output">ⓘ</a>

where <a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>: <a href="https://doc.rust-lang.org/nightly/alloc/slice/trait.Join.html" class="trait" title="trait alloc::slice::Join">Join</a>\<Separator\>,

👎Deprecated since 1.3.0: renamed to join

Flattens a slice of `T` into a single value `Self::Output`, placing a given separator between each.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#examples-122" class="doc-anchor">§</a>Examples

``` rust
assert_eq!(["hello", "world"].connect(" "), "hello world");
assert_eq!([[1, 2], [3, 4]].connect(&0), [1, 2, 0, 3, 4]);
```

1.23.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/slice.rs.html#636" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.to_ascii_uppercase" class="fn">to_ascii_uppercase</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#" class="tooltip" data-notable-ty="Vec&lt;u8&gt;">ⓘ</a>

Returns a vector containing a copy of this slice where each byte is mapped to its ASCII upper case equivalent.

ASCII letters ‘a’ to ‘z’ are mapped to ‘A’ to ‘Z’, but non-ASCII letters are unchanged.

To uppercase the value in-place, use [`make_ascii_uppercase`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.make_ascii_uppercase "method slice::make_ascii_uppercase").

1.23.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/slice.rs.html#657" class="src">Source</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.to_ascii_lowercase" class="fn">to_ascii_lowercase</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#" class="tooltip" data-notable-ty="Vec&lt;u8&gt;">ⓘ</a>

Returns a vector containing a copy of this slice where each byte is mapped to its ASCII lower case equivalent.

ASCII letters ‘A’ to ‘Z’ are mapped to ‘a’ to ‘z’, but non-ASCII letters are unchanged.

To lowercase the value in-place, use [`make_ascii_lowercase`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.make_ascii_lowercase "method slice::make_ascii_lowercase").

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#impl-Debug-for-MutableBuffer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html" class="struct" title="struct arrow::buffer::MutableBuffer">MutableBuffer</a>

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#impl-Default-for-MutableBuffer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html" class="struct" title="struct arrow::buffer::MutableBuffer">MutableBuffer</a>

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html" class="struct" title="struct arrow::buffer::MutableBuffer">MutableBuffer</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#impl-Deref-for-MutableBuffer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html" class="trait" title="trait core::ops::deref::Deref">Deref</a> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html" class="struct" title="struct arrow::buffer::MutableBuffer">MutableBuffer</a>

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#associatedtype.Target" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target" class="associatedtype">Target</a> = \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]

The resulting type after dereferencing.

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.deref" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#tymethod.deref" class="fn">deref</a>(&self) -\> &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\] <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#" class="tooltip" data-notable-ty="&amp;[u8]">ⓘ</a>

Dereferences the value.

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#impl-DerefMut-for-MutableBuffer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html" class="trait" title="trait core::ops::deref::DerefMut">DerefMut</a> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html" class="struct" title="struct arrow::buffer::MutableBuffer">MutableBuffer</a>

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.deref_mut" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html#tymethod.deref_mut" class="fn">deref_mut</a>(&mut self) -\> &mut \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\] <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#" class="tooltip" data-notable-ty="&amp;mut [u8]">ⓘ</a>

Mutably dereferences the value.

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#impl-Drop-for-MutableBuffer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html" class="trait" title="trait core::ops::drop::Drop">Drop</a> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html" class="struct" title="struct arrow::buffer::MutableBuffer">MutableBuffer</a>

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.drop" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop" class="fn">drop</a>(&mut self)

Executes the destructor for this type. [Read more](https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop)

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#impl-Extend%3CA%3E-for-MutableBuffer" class="anchor">§</a>

### impl\<A\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html" class="trait" title="trait core::iter::traits::collect::Extend">Extend</a>\<A\> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html" class="struct" title="struct arrow::buffer::MutableBuffer">MutableBuffer</a>

where A: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html" class="trait" title="trait arrow::datatypes::ArrowNativeType">ArrowNativeType</a>,

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.extend" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend" class="fn">extend</a>\<T\>(&mut self, iter: T)

where T: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = A\>,

Extends a collection with the contents of an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend)

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.extend_one" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_one" class="fn">extend_one</a>(&mut self, item: A)

🔬This is a nightly-only experimental API. (`extend_one`)

Extends a collection with exactly one element.

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.extend_reserve" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve" class="fn">extend_reserve</a>(&mut self, additional: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

🔬This is a nightly-only experimental API. (`extend_one`)

Reserves capacity in a collection for the given number of additional elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve)

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#impl-From%3CMutableBuffer%3E-for-Buffer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html" class="struct" title="struct arrow::buffer::MutableBuffer">MutableBuffer</a>\> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(buffer: <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html" class="struct" title="struct arrow::buffer::MutableBuffer">MutableBuffer</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#impl-From%3CMutableBuffer%3E-for-ScalarBuffer%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html" class="struct" title="struct arrow::buffer::MutableBuffer">MutableBuffer</a>\> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.ScalarBuffer.html" class="struct" title="struct arrow::buffer::ScalarBuffer">ScalarBuffer</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html" class="trait" title="trait arrow::datatypes::ArrowNativeType">ArrowNativeType</a>,

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.from-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html" class="struct" title="struct arrow::buffer::MutableBuffer">MutableBuffer</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.ScalarBuffer.html" class="struct" title="struct arrow::buffer::ScalarBuffer">ScalarBuffer</a>\<T\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#impl-From%3CVec%3CT%3E%3E-for-MutableBuffer" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<T\>\> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html" class="struct" title="struct arrow::buffer::MutableBuffer">MutableBuffer</a>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html" class="trait" title="trait arrow::datatypes::ArrowNativeType">ArrowNativeType</a>,

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<T\>) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html" class="struct" title="struct arrow::buffer::MutableBuffer">MutableBuffer</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#impl-FromIterator%3CT%3E-for-MutableBuffer" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<T\> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html" class="struct" title="struct arrow::buffer::MutableBuffer">MutableBuffer</a>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html" class="trait" title="trait arrow::datatypes::ArrowNativeType">ArrowNativeType</a>,

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.from_iter-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html" class="struct" title="struct arrow::buffer::MutableBuffer">MutableBuffer</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = T\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#impl-FromIterator%3Cbool%3E-for-MutableBuffer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html" class="struct" title="struct arrow::buffer::MutableBuffer">MutableBuffer</a>

Creating a `MutableBuffer` instance by setting bits according to the boolean values

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.from_iter" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html" class="struct" title="struct arrow::buffer::MutableBuffer">MutableBuffer</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#impl-PartialEq-for-MutableBuffer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html" class="struct" title="struct arrow::buffer::MutableBuffer">MutableBuffer</a>

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html" class="struct" title="struct arrow::buffer::MutableBuffer">MutableBuffer</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#impl-Send-for-MutableBuffer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html" class="struct" title="struct arrow::buffer::MutableBuffer">MutableBuffer</a>

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#impl-Sync-for-MutableBuffer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html" class="struct" title="struct arrow::buffer::MutableBuffer">MutableBuffer</a>

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html#blanket-implementations" class="anchor">§</a>
