# Struct BooleanBufferBuilder Copy item path

<a href="https://docs.rs/arrow-buffer/56.2.0/x86_64-unknown-linux-gnu/src/arrow_buffer/builder/boolean.rs.html#29" class="src">Source</a>

``` rust
pub struct BooleanBufferBuilder { /* private fields */ }
```

Expand description

Builder for [`BooleanBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html "struct arrow::buffer::BooleanBuffer")

## <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BooleanBufferBuilder.html#see-also" class="doc-anchor">§</a>See Also

- [`NullBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html "struct arrow::buffer::NullBuffer") for building [`BooleanBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html "struct arrow::buffer::BooleanBuffer")s for representing nulls

## Implementations<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BooleanBufferBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BooleanBufferBuilder.html#impl-BooleanBufferBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/struct.BooleanBufferBuilder.html" class="struct" title="struct arrow::array::BooleanBufferBuilder">BooleanBufferBuilder</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BooleanBufferBuilder.html#method.new" class="fn">new</a>(capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.BooleanBufferBuilder.html" class="struct" title="struct arrow::array::BooleanBufferBuilder">BooleanBufferBuilder</a>

Creates a new `BooleanBufferBuilder` with sufficient space for `capacity` bits (not bytes).

The capacity is rounded up to the nearest multiple of 8 for the allocation.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BooleanBufferBuilder.html#method.new_from_buffer" class="fn">new_from_buffer</a>( buffer: <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html" class="struct" title="struct arrow::buffer::MutableBuffer">MutableBuffer</a>, len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.BooleanBufferBuilder.html" class="struct" title="struct arrow::array::BooleanBufferBuilder">BooleanBufferBuilder</a>

Creates a new `BooleanBufferBuilder` from [`MutableBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html "struct arrow::buffer::MutableBuffer") of `len`

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BooleanBufferBuilder.html#method.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the length of the buffer

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BooleanBufferBuilder.html#method.set_bit" class="fn">set_bit</a>(&mut self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, v: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>)

Sets a bit in the buffer at `index`

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BooleanBufferBuilder.html#method.get_bit" class="fn">get_bit</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Gets a bit in the buffer at `index`

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BooleanBufferBuilder.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if empty

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BooleanBufferBuilder.html#method.capacity" class="fn">capacity</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the capacity of the buffer, in bits (not bytes)

Note this

##### <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BooleanBufferBuilder.html#example" class="doc-anchor">§</a>Example

``` rust
// empty requires 0 bytes
let b = BooleanBufferBuilder::new(0);
assert_eq!(0, b.capacity());
// Creating space for 1 bit results in 64 bytes (space for 512 bits)
// (64 is the minimum allocation size for 64 bit architectures)
let mut b = BooleanBufferBuilder::new(1);
assert_eq!(512, b.capacity());
// 1000 bits requires 128 bytes (space for 1024 bits)
b.append_n(1000, true);
assert_eq!(1024, b.capacity());
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BooleanBufferBuilder.html#method.advance" class="fn">advance</a>(&mut self, additional: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Advances the buffer by `additional` bits

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BooleanBufferBuilder.html#method.truncate" class="fn">truncate</a>(&mut self, len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Truncates the builder to the given length

If `len` is greater than the buffer’s current length, this has no effect

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BooleanBufferBuilder.html#method.reserve" class="fn">reserve</a>(&mut self, additional: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Reserve space to at least `additional` new bits. Capacity will be `>= self.len() + additional`. New bytes are uninitialized and reading them is undefined behavior.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BooleanBufferBuilder.html#method.resize" class="fn">resize</a>(&mut self, len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Resizes the buffer, either truncating its contents (with no change in capacity), or growing it (potentially reallocating it) and writing `false` in the newly available bits.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BooleanBufferBuilder.html#method.append" class="fn">append</a>(&mut self, v: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>)

Appends a boolean `v` into the buffer

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BooleanBufferBuilder.html#method.append_n" class="fn">append_n</a>(&mut self, additional: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, v: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>)

Appends n `additional` bits of value `v` into the buffer

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BooleanBufferBuilder.html#method.append_slice" class="fn">append_slice</a>(&mut self, slice: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\])

Appends a slice of booleans into the buffer

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BooleanBufferBuilder.html#method.append_packed_range" class="fn">append_packed_range</a>(&mut self, range: <a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>, to_set: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\])

Append `range` bits from `to_set`

`to_set` is a slice of bits packed LSB-first into `[u8]`

##### <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BooleanBufferBuilder.html#panics" class="doc-anchor">§</a>Panics

Panics if `to_set` does not contain `ceil(range.end / 8)` bytes

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BooleanBufferBuilder.html#method.append_buffer" class="fn">append_buffer</a>(&mut self, buffer: &<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a>)

Append [`BooleanBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html "struct arrow::buffer::BooleanBuffer") to this [`BooleanBufferBuilder`](https://docs.rs/arrow/latest/arrow/array/struct.BooleanBufferBuilder.html "struct arrow::array::BooleanBufferBuilder")

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BooleanBufferBuilder.html#method.as_slice" class="fn">as_slice</a>(&self) -\> &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\] <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BooleanBufferBuilder.html#" class="tooltip" data-notable-ty="&amp;[u8]">ⓘ</a>

Returns the packed bits

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BooleanBufferBuilder.html#method.as_slice_mut" class="fn">as_slice_mut</a>(&mut self) -\> &mut \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\] <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BooleanBufferBuilder.html#" class="tooltip" data-notable-ty="&amp;mut [u8]">ⓘ</a>

Returns the packed bits

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BooleanBufferBuilder.html#method.finish" class="fn">finish</a>(&mut self) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a>

Creates a [`BooleanBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html "struct arrow::buffer::BooleanBuffer")

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BooleanBufferBuilder.html#method.finish_cloned" class="fn">finish_cloned</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a>

Builds the [BooleanBuffer](https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html "struct arrow::buffer::BooleanBuffer") without resetting the builder.

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BooleanBufferBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BooleanBufferBuilder.html#impl-Debug-for-BooleanBufferBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.BooleanBufferBuilder.html" class="struct" title="struct arrow::array::BooleanBufferBuilder">BooleanBufferBuilder</a>

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BooleanBufferBuilder.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BooleanBufferBuilder.html#impl-From%3CBooleanBufferBuilder%3E-for-BooleanBuffer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.BooleanBufferBuilder.html" class="struct" title="struct arrow::array::BooleanBufferBuilder">BooleanBufferBuilder</a>\> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a>

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BooleanBufferBuilder.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(builder: <a href="https://docs.rs/arrow/latest/arrow/array/struct.BooleanBufferBuilder.html" class="struct" title="struct arrow::array::BooleanBufferBuilder">BooleanBufferBuilder</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BooleanBufferBuilder.html#impl-From%3CBooleanBufferBuilder%3E-for-Buffer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.BooleanBufferBuilder.html" class="struct" title="struct arrow::array::BooleanBufferBuilder">BooleanBufferBuilder</a>\> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BooleanBufferBuilder.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(builder: <a href="https://docs.rs/arrow/latest/arrow/array/struct.BooleanBufferBuilder.html" class="struct" title="struct arrow::array::BooleanBufferBuilder">BooleanBufferBuilder</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

Converts to this type from the input type.

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BooleanBufferBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BooleanBufferBuilder.html#blanket-implementations" class="anchor">§</a>
