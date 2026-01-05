# Struct RunEndBuffer Copy item path

<a href="https://docs.rs/arrow-buffer/56.2.0/x86_64-unknown-linux-gnu/src/arrow_buffer/buffer/run.rs.html#65" class="src">Source</a>

``` rust
pub struct RunEndBuffer<E>where
    E: ArrowNativeType,{ /* private fields */ }
```

Expand description

A slice-able buffer of monotonically increasing, positive integers used to store run-ends

## <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.RunEndBuffer.html#logical-vs-physical" class="doc-anchor">§</a>Logical vs Physical

A [`RunEndBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.RunEndBuffer.html "struct arrow::buffer::RunEndBuffer") is used to encode runs of the same value, the index of each run is called the physical index. The logical index is then the corresponding index in the logical run-encoded array, i.e. a single run of length `3`, would have the logical indices `0..3`.

Each value in [`RunEndBuffer::values`](https://docs.rs/arrow/latest/arrow/buffer/struct.RunEndBuffer.html#method.values "method arrow::buffer::RunEndBuffer::values") is the cumulative length of all runs in the logical array, up to that physical index.

Consider a [`RunEndBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.RunEndBuffer.html "struct arrow::buffer::RunEndBuffer") containing `[3, 4, 6]`. The maximum physical index is `2`, as there are `3` values, and the maximum logical index is `5`, as the maximum run end is `6`. The physical indices are therefore `[0, 0, 0, 1, 2, 2]`

``` text
    ┌─────────┐        ┌─────────┐           ┌─────────┐
    │    3    │        │    0    │ ─┬──────▶ │    0    │
    ├─────────┤        ├─────────┤  │        ├─────────┤
    │    4    │        │    1    │ ─┤ ┌────▶ │    1    │
    ├─────────┤        ├─────────┤  │ │      ├─────────┤
    │    6    │        │    2    │ ─┘ │ ┌──▶ │    2    │
    └─────────┘        ├─────────┤    │ │    └─────────┘
     run ends          │    3    │ ───┘ │  physical indices
                       ├─────────┤      │
                       │    4    │ ─────┤
                       ├─────────┤      │
                       │    5    │ ─────┘
                       └─────────┘
                     logical indices
```

## <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.RunEndBuffer.html#slicing" class="doc-anchor">§</a>Slicing

In order to provide zero-copy slicing, this container stores a separate offset and length

For example, a [`RunEndBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.RunEndBuffer.html "struct arrow::buffer::RunEndBuffer") containing values `[3, 6, 8]` with offset and length `4` would describe the physical indices `1, 1, 2, 2`

For example, a [`RunEndBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.RunEndBuffer.html "struct arrow::buffer::RunEndBuffer") containing values `[6, 8, 9]` with offset `2` and length `5` would describe the physical indices `0, 0, 0, 0, 1`

## Implementations<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.RunEndBuffer.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.RunEndBuffer.html#impl-RunEndBuffer%3CE%3E" class="anchor">§</a>

### impl\<E\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.RunEndBuffer.html" class="struct" title="struct arrow::buffer::RunEndBuffer">RunEndBuffer</a>\<E\>

where E: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html" class="trait" title="trait arrow::datatypes::ArrowNativeType">ArrowNativeType</a>,

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.RunEndBuffer.html#method.new" class="fn">new</a>( run_ends: <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.ScalarBuffer.html" class="struct" title="struct arrow::buffer::ScalarBuffer">ScalarBuffer</a>\<E\>, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.RunEndBuffer.html" class="struct" title="struct arrow::buffer::RunEndBuffer">RunEndBuffer</a>\<E\>

Create a new [`RunEndBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.RunEndBuffer.html "struct arrow::buffer::RunEndBuffer") from a [`ScalarBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.ScalarBuffer.html "struct arrow::buffer::ScalarBuffer"), an `offset` and `len`

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.RunEndBuffer.html#panics" class="doc-anchor">§</a>Panics

- `buffer` does not contain strictly increasing values greater than zero
- the last value of `buffer` is less than `offset + len`

#### pub unsafe fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.RunEndBuffer.html#method.new_unchecked" class="fn">new_unchecked</a>( run_ends: <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.ScalarBuffer.html" class="struct" title="struct arrow::buffer::ScalarBuffer">ScalarBuffer</a>\<E\>, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.RunEndBuffer.html" class="struct" title="struct arrow::buffer::RunEndBuffer">RunEndBuffer</a>\<E\>

Create a new [`RunEndBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.RunEndBuffer.html "struct arrow::buffer::RunEndBuffer") from an [`ScalarBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.ScalarBuffer.html "struct arrow::buffer::ScalarBuffer"), an `offset` and `len`

##### <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.RunEndBuffer.html#safety" class="doc-anchor">§</a>Safety

- `buffer` must contain strictly increasing values greater than zero
- The last value of `buffer` must be greater than or equal to `offset + len`

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.RunEndBuffer.html#method.offset" class="fn">offset</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the logical offset into the run-ends stored by this buffer

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.RunEndBuffer.html#method.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the logical length of the run-ends stored by this buffer

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.RunEndBuffer.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if this buffer is empty

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.RunEndBuffer.html#method.shrink_to_fit" class="fn">shrink_to_fit</a>(&mut self)

Free up unused memory.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.RunEndBuffer.html#method.values" class="fn">values</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[E]</a>

Returns the values of this [`RunEndBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.RunEndBuffer.html "struct arrow::buffer::RunEndBuffer") not including any offset

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.RunEndBuffer.html#method.max_value" class="fn">max_value</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the maximum run-end encoded in the underlying buffer

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.RunEndBuffer.html#method.get_physical_index" class="fn">get_physical_index</a>(&self, logical_index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Performs a binary search to find the physical index for the given logical index

The result is arbitrary if `logical_index >= self.len()`

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.RunEndBuffer.html#method.get_start_physical_index" class="fn">get_start_physical_index</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the physical index at which the logical array starts

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.RunEndBuffer.html#method.get_end_physical_index" class="fn">get_end_physical_index</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the physical index at which the logical array ends

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.RunEndBuffer.html#method.slice" class="fn">slice</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.RunEndBuffer.html" class="struct" title="struct arrow::buffer::RunEndBuffer">RunEndBuffer</a>\<E\>

Slices this [`RunEndBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.RunEndBuffer.html "struct arrow::buffer::RunEndBuffer") by the provided `offset` and `length`

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.RunEndBuffer.html#method.inner" class="fn">inner</a>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.ScalarBuffer.html" class="struct" title="struct arrow::buffer::ScalarBuffer">ScalarBuffer</a>\<E\>

Returns the inner [`ScalarBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.ScalarBuffer.html "struct arrow::buffer::ScalarBuffer")

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.RunEndBuffer.html#method.into_inner" class="fn">into_inner</a>(self) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.ScalarBuffer.html" class="struct" title="struct arrow::buffer::ScalarBuffer">ScalarBuffer</a>\<E\>

Returns the inner [`ScalarBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.ScalarBuffer.html "struct arrow::buffer::ScalarBuffer"), consuming self

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.RunEndBuffer.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.RunEndBuffer.html#impl-Clone-for-RunEndBuffer%3CE%3E" class="anchor">§</a>

### impl\<E\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.RunEndBuffer.html" class="struct" title="struct arrow::buffer::RunEndBuffer">RunEndBuffer</a>\<E\>

where E: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> + <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html" class="trait" title="trait arrow::datatypes::ArrowNativeType">ArrowNativeType</a>,

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.RunEndBuffer.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.RunEndBuffer.html" class="struct" title="struct arrow::buffer::RunEndBuffer">RunEndBuffer</a>\<E\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/buffer/struct.RunEndBuffer.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.RunEndBuffer.html#impl-Debug-for-RunEndBuffer%3CE%3E" class="anchor">§</a>

### impl\<E\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.RunEndBuffer.html" class="struct" title="struct arrow::buffer::RunEndBuffer">RunEndBuffer</a>\<E\>

where E: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> + <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html" class="trait" title="trait arrow::datatypes::ArrowNativeType">ArrowNativeType</a>,

<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.RunEndBuffer.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.RunEndBuffer.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.RunEndBuffer.html#blanket-implementations" class="anchor">§</a>
