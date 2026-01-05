# Struct NullBufferBuilder Copy item path

<a href="https://docs.rs/arrow-buffer/56.2.0/x86_64-unknown-linux-gnu/src/arrow_buffer/builder/null.rs.html#50" class="src">Source</a>

``` rust
pub struct NullBufferBuilder { /* private fields */ }
```

Expand description

Builder for creating [`NullBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html "struct arrow::buffer::NullBuffer")

## <a href="https://docs.rs/arrow/latest/arrow/array/struct.NullBufferBuilder.html#performance" class="doc-anchor">§</a>Performance

This builder only materializes the buffer when we append `false`. If you only append `true`s to the builder, what you get will be `None` when calling [`finish`](https://docs.rs/arrow/latest/arrow/array/struct.NullBufferBuilder.html#method.finish).

This optimization is **very** important for the performance as it avoids allocating memory for the null buffer when there are no nulls.

See [`Self::allocated_size`](https://docs.rs/arrow/latest/arrow/array/struct.NullBufferBuilder.html#method.allocated_size "method arrow::array::NullBufferBuilder::allocated_size") to get the current memory allocated by the builder.

## <a href="https://docs.rs/arrow/latest/arrow/array/struct.NullBufferBuilder.html#example" class="doc-anchor">§</a>Example

``` rust
let mut builder = NullBufferBuilder::new(8);
builder.append_n_non_nulls(8);
// If no non null values are appended, the null buffer is not created
let buffer = builder.finish();
assert!(buffer.is_none());
// however, if a null value is appended, the null buffer is created
let mut builder = NullBufferBuilder::new(8);
builder.append_n_non_nulls(7);
builder.append_null();
let buffer = builder.finish().unwrap();
assert_eq!(buffer.len(), 8);
assert_eq!(buffer.iter().collect::<Vec<_>>(), vec![true, true, true, true, true, true, true, false]);
```

## Implementations<a href="https://docs.rs/arrow/latest/arrow/array/struct.NullBufferBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.NullBufferBuilder.html#impl-NullBufferBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/struct.NullBufferBuilder.html" class="struct" title="struct arrow::array::NullBufferBuilder">NullBufferBuilder</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.NullBufferBuilder.html#method.new" class="fn">new</a>(capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.NullBufferBuilder.html" class="struct" title="struct arrow::array::NullBufferBuilder">NullBufferBuilder</a>

Creates a new empty builder.

Note that this method does not allocate any memory, regardless of the `capacity` parameter. If an allocation is required, `capacity` is the size in bits (not bytes) that will be allocated at minimum.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.NullBufferBuilder.html#method.new_with_len" class="fn">new_with_len</a>(len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.NullBufferBuilder.html" class="struct" title="struct arrow::array::NullBufferBuilder">NullBufferBuilder</a>

Creates a new builder with given length.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.NullBufferBuilder.html#method.new_from_buffer" class="fn">new_from_buffer</a>(buffer: <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html" class="struct" title="struct arrow::buffer::MutableBuffer">MutableBuffer</a>, len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.NullBufferBuilder.html" class="struct" title="struct arrow::array::NullBufferBuilder">NullBufferBuilder</a>

Creates a new builder from a `MutableBuffer`.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.NullBufferBuilder.html#method.append_n_non_nulls" class="fn">append_n_non_nulls</a>(&mut self, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Appends `n` `true`s into the builder to indicate that these `n` items are not nulls.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.NullBufferBuilder.html#method.append_non_null" class="fn">append_non_null</a>(&mut self)

Appends a `true` into the builder to indicate that this item is not null.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.NullBufferBuilder.html#method.append_n_nulls" class="fn">append_n_nulls</a>(&mut self, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Appends `n` `false`s into the builder to indicate that these `n` items are nulls.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.NullBufferBuilder.html#method.append_null" class="fn">append_null</a>(&mut self)

Appends a `false` into the builder to indicate that this item is null.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.NullBufferBuilder.html#method.append" class="fn">append</a>(&mut self, not_null: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>)

Appends a boolean value into the builder.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.NullBufferBuilder.html#method.is_valid" class="fn">is_valid</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Gets a bit in the buffer at `index`

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.NullBufferBuilder.html#method.truncate" class="fn">truncate</a>(&mut self, len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Truncates the builder to the given length

If `len` is greater than the buffer’s current length, this has no effect

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.NullBufferBuilder.html#method.append_slice" class="fn">append_slice</a>(&mut self, slice: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\])

Appends a boolean slice into the builder to indicate the validations of these items.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.NullBufferBuilder.html#method.append_buffer" class="fn">append_buffer</a>(&mut self, buffer: &<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>)

Append [`NullBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html "struct arrow::buffer::NullBuffer") to this [`NullBufferBuilder`](https://docs.rs/arrow/latest/arrow/array/struct.NullBufferBuilder.html "struct arrow::array::NullBufferBuilder")

This is useful when you want to concatenate two null buffers.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.NullBufferBuilder.html#method.finish" class="fn">finish</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>\>

Builds the null buffer and resets the builder. Returns `None` if the builder only contains `true`s.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.NullBufferBuilder.html#method.finish_cloned" class="fn">finish_cloned</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>\>

Builds the [NullBuffer](https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html "struct arrow::buffer::NullBuffer") without resetting the builder.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.NullBufferBuilder.html#method.as_slice" class="fn">as_slice</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>

Returns the inner bitmap builder as slice

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.NullBufferBuilder.html#method.as_slice_mut" class="fn">as_slice_mut</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&mut \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>

Return a mutable reference to the inner bitmap slice.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.NullBufferBuilder.html#method.allocated_size" class="fn">allocated_size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Return the allocated size of this builder, in bytes, useful for memory accounting.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.NullBufferBuilder.html#impl-NullBufferBuilder-1" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/struct.NullBufferBuilder.html" class="struct" title="struct arrow::array::NullBufferBuilder">NullBufferBuilder</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.NullBufferBuilder.html#method.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Return the number of bits in the buffer.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.NullBufferBuilder.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Check if the builder is empty.

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/struct.NullBufferBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.NullBufferBuilder.html#impl-Debug-for-NullBufferBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.NullBufferBuilder.html" class="struct" title="struct arrow::array::NullBufferBuilder">NullBufferBuilder</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.NullBufferBuilder.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/struct.NullBufferBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/array/struct.NullBufferBuilder.html#blanket-implementations" class="anchor">§</a>
