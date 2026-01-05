# Struct NullBuffer Copy item path

<a href="https://docs.rs/arrow-buffer/56.2.0/x86_64-unknown-linux-gnu/src/arrow_buffer/buffer/null.rs.html#33" class="src">Source</a>

``` rust
pub struct NullBuffer { /* private fields */ }
```

Expand description

A [`BooleanBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html "struct arrow::buffer::BooleanBuffer") used to encode validity for Arrow arrays

In the [Arrow specification](https://arrow.apache.org/docs/format/Columnar.html#validity-bitmaps), array validity is encoded in a packed bitmask with a `true` value indicating the corresponding slot is not null, and `false` indicating that it is null.

`NullBuffer`s can be creating using [`NullBufferBuilder`](https://docs.rs/arrow/latest/arrow/array/struct.NullBufferBuilder.html "struct arrow::array::NullBufferBuilder")

## Implementations<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#impl-NullBuffer" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#method.new" class="fn">new</a>(buffer: <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>

Create a new [`NullBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html "struct arrow::buffer::NullBuffer") computing the null count

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#method.new_null" class="fn">new_null</a>(len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>

Create a new [`NullBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html "struct arrow::buffer::NullBuffer") of length `len` where all values are null

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#method.new_valid" class="fn">new_valid</a>(len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>

Create a new [`NullBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html "struct arrow::buffer::NullBuffer") of length `len` where all values are valid

Note: it is more efficient to not set the null buffer if it is known to be all valid (aka all values are not null)

#### pub unsafe fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#method.new_unchecked" class="fn">new_unchecked</a>( buffer: <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a>, null_count: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>

Create a new [`NullBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html "struct arrow::buffer::NullBuffer") with the provided `buffer` and `null_count`

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#safety" class="doc-anchor">§</a>Safety

`buffer` must contain `null_count` `0` bits

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#method.union" class="fn">union</a>( lhs: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>\>, rhs: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>\>

Computes the union of the nulls in two optional [`NullBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html "struct arrow::buffer::NullBuffer")

This is commonly used by binary operations where the result is NULL if either of the input values is NULL. Handling the null mask separately in this way can yield significant performance improvements over an iterator approach

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#method.contains" class="fn">contains</a>(&self, other: &<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if all nulls in `other` also exist in self

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#method.expand" class="fn">expand</a>(&self, count: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>

Returns a new [`NullBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html "struct arrow::buffer::NullBuffer") where each bit in the current null buffer is repeated `count` times. This is useful for masking the nulls of the child of a FixedSizeListArray based on its parent

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#method.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the length of this [`NullBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html "struct arrow::buffer::NullBuffer") in bits

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#method.offset" class="fn">offset</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the offset of this [`NullBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html "struct arrow::buffer::NullBuffer") in bits

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if this [`NullBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html "struct arrow::buffer::NullBuffer") is empty

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#method.shrink_to_fit" class="fn">shrink_to_fit</a>(&mut self)

Free up unused memory.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#method.null_count" class="fn">null_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the null count for this [`NullBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html "struct arrow::buffer::NullBuffer")

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#method.is_valid" class="fn">is_valid</a>(&self, idx: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns `true` if the value at `idx` is not null

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#method.is_null" class="fn">is_null</a>(&self, idx: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns `true` if the value at `idx` is null

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#method.validity" class="fn">validity</a>(&self) -\> &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\] <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#" class="tooltip" data-notable-ty="&amp;[u8]">ⓘ</a>

Returns the packed validity of this [`NullBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html "struct arrow::buffer::NullBuffer") not including any offset

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#method.slice" class="fn">slice</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>

Slices this [`NullBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html "struct arrow::buffer::NullBuffer") by the provided `offset` and `length`

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#method.iter" class="fn">iter</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/util/bit_iterator/struct.BitIterator.html" class="struct" title="struct arrow::util::bit_iterator::BitIterator">BitIterator</a>\<'\_\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#" class="tooltip" data-notable-ty="BitIterator&lt;&#39;_&gt;">ⓘ</a>

Returns an iterator over the bits in this [`NullBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html "struct arrow::buffer::NullBuffer")

- `true` indicates that the corresponding value is not NULL
- `false` indicates that the corresponding value is NULL

Note: [`Self::valid_indices`](https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#method.valid_indices "method arrow::buffer::NullBuffer::valid_indices") will be significantly faster for most use-cases

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#method.valid_indices" class="fn">valid_indices</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/util/bit_iterator/struct.BitIndexIterator.html" class="struct" title="struct arrow::util::bit_iterator::BitIndexIterator">BitIndexIterator</a>\<'\_\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#" class="tooltip" data-notable-ty="BitIndexIterator&lt;&#39;_&gt;">ⓘ</a>

Returns a [`BitIndexIterator`](https://docs.rs/arrow/latest/arrow/util/bit_iterator/struct.BitIndexIterator.html "struct arrow::util::bit_iterator::BitIndexIterator") over the valid indices in this [`NullBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html "struct arrow::buffer::NullBuffer")

Valid indices indicate the corresponding value is not NULL

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#method.valid_slices" class="fn">valid_slices</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/util/bit_iterator/struct.BitSliceIterator.html" class="struct" title="struct arrow::util::bit_iterator::BitSliceIterator">BitSliceIterator</a>\<'\_\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#" class="tooltip" data-notable-ty="BitSliceIterator&lt;&#39;_&gt;">ⓘ</a>

Returns a [`BitSliceIterator`](https://docs.rs/arrow/latest/arrow/util/bit_iterator/struct.BitSliceIterator.html "struct arrow::util::bit_iterator::BitSliceIterator") yielding contiguous ranges of valid indices

Valid indices indicate the corresponding value is not NULL

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#method.try_for_each_valid_idx" class="fn">try_for_each_valid_idx</a>\<E, F\>(&self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, E\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, E\>,

Calls the provided closure for each index in this null mask that is set

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#method.inner" class="fn">inner</a>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a>

Returns the inner [`BooleanBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html "struct arrow::buffer::BooleanBuffer")

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#method.into_inner" class="fn">into_inner</a>(self) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a>

Returns the inner [`BooleanBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html "struct arrow::buffer::BooleanBuffer")

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#method.buffer" class="fn">buffer</a>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

Returns the underlying [`Buffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html "struct arrow::buffer::Buffer")

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#impl-Clone-for-NullBuffer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#impl-Debug-for-NullBuffer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#impl-From%3C%26%5Bbool%5D%3E-for-NullBuffer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&\[<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\]\> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\]) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#impl-From%3C%26%5Bbool;+N%5D%3E-for-NullBuffer" class="anchor">§</a>

### impl\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&\[<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>; <a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">N</a>\]\> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#method.from-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>; <a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">N</a>\]) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#impl-From%3CBooleanBuffer%3E-for-NullBuffer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a>\> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#impl-From%3CVec%3Cbool%3E%3E-for-NullBuffer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>\> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#method.from-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#impl-FromIterator%3Cbool%3E-for-NullBuffer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#method.from_iter" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<T\>(iter: T) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#impl-IntoIterator-for-%26NullBuffer" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a> for &'a <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#associatedtype.Item" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

The type of the elements being iterated over.

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#associatedtype.IntoIter" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype">IntoIter</a> = <a href="https://docs.rs/arrow/latest/arrow/util/bit_iterator/struct.BitIterator.html" class="struct" title="struct arrow::util::bit_iterator::BitIterator">BitIterator</a>\<'a\>

Which kind of iterator are we turning this into?

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#method.into_iter" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter" class="fn">into_iter</a>(self) -\> \<&'a <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a> as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::IntoIter">IntoIter</a>

Creates an iterator from a value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#impl-PartialEq-for-NullBuffer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#impl-Eq-for-NullBuffer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#impl-StructuralPartialEq-for-NullBuffer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html#blanket-implementations" class="anchor">§</a>
