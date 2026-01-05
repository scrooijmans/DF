# Struct BooleanBuffer Copy item path

<a href="https://docs.rs/arrow-buffer/56.2.0/x86_64-unknown-linux-gnu/src/arrow_buffer/buffer/boolean.rs.html#37" class="src">Source</a>

``` rust
pub struct BooleanBuffer { /* private fields */ }
```

Expand description

A slice-able [`Buffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html "struct arrow::buffer::Buffer") containing bit-packed booleans

`BooleanBuffer`s can be creating using [`BooleanBufferBuilder`](https://docs.rs/arrow/latest/arrow/array/struct.BooleanBufferBuilder.html "struct arrow::array::BooleanBufferBuilder")

## <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#see-also" class="doc-anchor">§</a>See Also

- [`NullBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html "struct arrow::buffer::NullBuffer") for representing null values in Arrow arrays

## Implementations<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#impl-BooleanBuffer" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#method.new" class="fn">new</a>(buffer: <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a>

Create a new [`BooleanBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html "struct arrow::buffer::BooleanBuffer") from a [`Buffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html "struct arrow::buffer::Buffer"), an `offset` and `length` in bits

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#panics" class="doc-anchor">§</a>Panics

This method will panic if `buffer` is not large enough

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#method.new_set" class="fn">new_set</a>(length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a>

Create a new [`BooleanBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html "struct arrow::buffer::BooleanBuffer") of `length` where all values are `true`

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#method.new_unset" class="fn">new_unset</a>(length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a>

Create a new [`BooleanBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html "struct arrow::buffer::BooleanBuffer") of `length` where all values are `false`

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#method.collect_bool" class="fn">collect_bool</a>\<F\>(len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, f: F) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

Invokes `f` with indexes `0..len` collecting the boolean results into a new `BooleanBuffer`

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#method.count_set_bits" class="fn">count_set_bits</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the number of set bits in this buffer

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#method.bit_chunks" class="fn">bit_chunks</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/util/bit_chunk_iterator/struct.BitChunks.html" class="struct" title="struct arrow::util::bit_chunk_iterator::BitChunks">BitChunks</a>\<'\_\>

Returns a `BitChunks` instance which can be used to iterate over this buffer’s bits in `u64` chunks

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#method.offset" class="fn">offset</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the offset of this [`BooleanBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html "struct arrow::buffer::BooleanBuffer") in bits

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#method.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the length of this [`BooleanBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html "struct arrow::buffer::BooleanBuffer") in bits

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if this [`BooleanBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html "struct arrow::buffer::BooleanBuffer") is empty

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#method.shrink_to_fit" class="fn">shrink_to_fit</a>(&mut self)

Free up unused memory.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#method.value" class="fn">value</a>(&self, idx: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns the boolean value at index `i`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#panics-1" class="doc-anchor">§</a>Panics

Panics if `i >= self.len()`

#### pub unsafe fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#method.value_unchecked" class="fn">value_unchecked</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns the boolean value at index `i`.

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#safety" class="doc-anchor">§</a>Safety

This doesn’t check bounds, the caller must ensure that index \< self.len()

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#method.values" class="fn">values</a>(&self) -\> &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\] <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#" class="tooltip" data-notable-ty="&amp;[u8]">ⓘ</a>

Returns the packed values of this [`BooleanBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html "struct arrow::buffer::BooleanBuffer") not including any offset

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#method.slice" class="fn">slice</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a>

Slices this [`BooleanBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html "struct arrow::buffer::BooleanBuffer") by the provided `offset` and `length`

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#method.sliced" class="fn">sliced</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

Returns a [`Buffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html "struct arrow::buffer::Buffer") containing the sliced contents of this [`BooleanBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html "struct arrow::buffer::BooleanBuffer")

Equivalent to `self.buffer.bit_slice(self.offset, self.len)`

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#method.ptr_eq" class="fn">ptr_eq</a>(&self, other: &<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if this [`BooleanBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html "struct arrow::buffer::BooleanBuffer") is equal to `other`, using pointer comparisons to determine buffer equality. This is cheaper than `PartialEq::eq` but may return false when the arrays are logically equal

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#method.inner" class="fn">inner</a>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

Returns the inner [`Buffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html "struct arrow::buffer::Buffer")

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#method.into_inner" class="fn">into_inner</a>(self) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

Returns the inner [`Buffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html "struct arrow::buffer::Buffer"), consuming self

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#method.iter" class="fn">iter</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/util/bit_iterator/struct.BitIterator.html" class="struct" title="struct arrow::util::bit_iterator::BitIterator">BitIterator</a>\<'\_\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#" class="tooltip" data-notable-ty="BitIterator&lt;&#39;_&gt;">ⓘ</a>

Returns an iterator over the bits in this [`BooleanBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html "struct arrow::buffer::BooleanBuffer")

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#method.set_indices" class="fn">set_indices</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/util/bit_iterator/struct.BitIndexIterator.html" class="struct" title="struct arrow::util::bit_iterator::BitIndexIterator">BitIndexIterator</a>\<'\_\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#" class="tooltip" data-notable-ty="BitIndexIterator&lt;&#39;_&gt;">ⓘ</a>

Returns an iterator over the set bit positions in this [`BooleanBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html "struct arrow::buffer::BooleanBuffer")

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#method.set_indices_u32" class="fn">set_indices_u32</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/util/bit_iterator/struct.BitIndexU32Iterator.html" class="struct" title="struct arrow::util::bit_iterator::BitIndexU32Iterator">BitIndexU32Iterator</a>\<'\_\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#" class="tooltip" data-notable-ty="BitIndexU32Iterator&lt;&#39;_&gt;">ⓘ</a>

Returns a `u32` iterator over set bit positions without any usize-\>u32 conversion

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#method.set_slices" class="fn">set_slices</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/util/bit_iterator/struct.BitSliceIterator.html" class="struct" title="struct arrow::util::bit_iterator::BitSliceIterator">BitSliceIterator</a>\<'\_\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#" class="tooltip" data-notable-ty="BitSliceIterator&lt;&#39;_&gt;">ⓘ</a>

Returns a [`BitSliceIterator`](https://docs.rs/arrow/latest/arrow/util/bit_iterator/struct.BitSliceIterator.html "struct arrow::util::bit_iterator::BitSliceIterator") yielding contiguous ranges of set bits

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#impl-BitAnd%3C%26BooleanBuffer%3E-for-%26BooleanBuffer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html" class="trait" title="trait core::ops::bit::BitAnd">BitAnd</a>\<&<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a>\> for &<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a>

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#associatedtype.Output-1" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a>

The resulting type after applying the `&` operator.

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#method.bitand" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand" class="fn">bitand</a>( self, rhs: &<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a>, ) -\> \<&<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html" class="trait" title="trait core::ops::bit::BitAnd">BitAnd</a>\<&<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a>\>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output" class="associatedtype" title="type core::ops::bit::BitAnd::Output">Output</a>

Performs the `&` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#impl-BitOr%3C%26BooleanBuffer%3E-for-%26BooleanBuffer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html" class="trait" title="trait core::ops::bit::BitOr">BitOr</a>\<&<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a>\> for &<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a>

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#associatedtype.Output-2" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a>

The resulting type after applying the `|` operator.

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#method.bitor" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor" class="fn">bitor</a>( self, rhs: &<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a>, ) -\> \<&<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html" class="trait" title="trait core::ops::bit::BitOr">BitOr</a>\<&<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a>\>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type core::ops::bit::BitOr::Output">Output</a>

Performs the `|` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#impl-BitXor%3C%26BooleanBuffer%3E-for-%26BooleanBuffer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html" class="trait" title="trait core::ops::bit::BitXor">BitXor</a>\<&<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a>\> for &<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a>

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#associatedtype.Output-3" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a>

The resulting type after applying the `^` operator.

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#method.bitxor" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor" class="fn">bitxor</a>( self, rhs: &<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a>, ) -\> \<&<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html" class="trait" title="trait core::ops::bit::BitXor">BitXor</a>\<&<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a>\>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output" class="associatedtype" title="type core::ops::bit::BitXor::Output">Output</a>

Performs the `^` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#impl-Clone-for-BooleanBuffer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a>

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#impl-Debug-for-BooleanBuffer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a>

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#impl-From%3C%26%5Bbool%5D%3E-for-BooleanBuffer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&\[<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\]\> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a>

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\]) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#impl-From%3CBooleanBuffer%3E-for-BooleanArray" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.BooleanArray.html" class="struct" title="struct arrow::array::BooleanArray">BooleanArray</a>

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(values: <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.BooleanArray.html" class="struct" title="struct arrow::array::BooleanArray">BooleanArray</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#impl-From%3CBooleanBuffer%3E-for-NullBuffer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a>\> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#method.from-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#impl-From%3CBooleanBufferBuilder%3E-for-BooleanBuffer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.BooleanBufferBuilder.html" class="struct" title="struct arrow::array::BooleanBufferBuilder">BooleanBufferBuilder</a>\> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a>

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#method.from-4" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(builder: <a href="https://docs.rs/arrow/latest/arrow/array/struct.BooleanBufferBuilder.html" class="struct" title="struct arrow::array::BooleanBufferBuilder">BooleanBufferBuilder</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#impl-From%3CVec%3Cbool%3E%3E-for-BooleanBuffer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>\> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a>

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#method.from-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#impl-FromIterator%3Cbool%3E-for-BooleanBuffer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a>

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#method.from_iter" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<T\>(iter: T) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#impl-IntoIterator-for-%26BooleanBuffer" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a> for &'a <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a>

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#associatedtype.Item" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

The type of the elements being iterated over.

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#associatedtype.IntoIter" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype">IntoIter</a> = <a href="https://docs.rs/arrow/latest/arrow/util/bit_iterator/struct.BitIterator.html" class="struct" title="struct arrow::util::bit_iterator::BitIterator">BitIterator</a>\<'a\>

Which kind of iterator are we turning this into?

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#method.into_iter" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter" class="fn">into_iter</a>(self) -\> \<&'a <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a> as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::IntoIter">IntoIter</a>

Creates an iterator from a value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#impl-Not-for-%26BooleanBuffer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html" class="trait" title="trait core::ops::bit::Not">Not</a> for &<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a>

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#associatedtype.Output" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a>

The resulting type after applying the `!` operator.

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#method.not" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html#tymethod.not" class="fn">not</a>(self) -\> \<&<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html" class="trait" title="trait core::ops::bit::Not">Not</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html#associatedtype.Output" class="associatedtype" title="type core::ops::bit::Not::Output">Output</a>

Performs the unary `!` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html#tymethod.not)

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#impl-PartialEq-for-BooleanBuffer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a>

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#impl-Eq-for-BooleanBuffer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a>

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html#blanket-implementations" class="anchor">§</a>
