# Struct ArrayDataBuilder Copy item path

<a href="https://docs.rs/arrow-data/56.2.0/x86_64-unknown-linux-gnu/src/arrow_data/data.rs.html#1873" class="src">Source</a>

``` rust
pub struct ArrayDataBuilder { /* private fields */ }
```

Expand description

Builder for [`ArrayData`](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html "struct arrow::array::ArrayData") type

## Implementations<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html#impl-ArrayDataBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html" class="struct" title="struct arrow::array::ArrayDataBuilder">ArrayDataBuilder</a>

#### pub const fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html#method.new" class="fn">new</a>(data_type: <a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html" class="struct" title="struct arrow::array::ArrayDataBuilder">ArrayDataBuilder</a>

Creates a new array data builder

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html#method.data_type" class="fn">data_type</a>(self, data_type: <a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html" class="struct" title="struct arrow::array::ArrayDataBuilder">ArrayDataBuilder</a>

Creates a new array data builder from an existing one, changing the data type

#### pub const fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html#method.len" class="fn">len</a>(self, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html" class="struct" title="struct arrow::array::ArrayDataBuilder">ArrayDataBuilder</a>

Sets the length of the [ArrayData](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html "struct arrow::array::ArrayData")

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html#method.nulls" class="fn">nulls</a>(self, nulls: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>\>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html" class="struct" title="struct arrow::array::ArrayDataBuilder">ArrayDataBuilder</a>

Sets the null buffer of the [ArrayData](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html "struct arrow::array::ArrayData")

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html#method.null_count" class="fn">null_count</a>(self, null_count: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html" class="struct" title="struct arrow::array::ArrayDataBuilder">ArrayDataBuilder</a>

Sets the null count of the [ArrayData](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html "struct arrow::array::ArrayData")

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html#method.null_bit_buffer" class="fn">null_bit_buffer</a>(self, buf: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>\>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html" class="struct" title="struct arrow::array::ArrayDataBuilder">ArrayDataBuilder</a>

Sets the `null_bit_buffer` of the [ArrayData](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html "struct arrow::array::ArrayData")

#### pub const fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html#method.offset" class="fn">offset</a>(self, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html" class="struct" title="struct arrow::array::ArrayDataBuilder">ArrayDataBuilder</a>

Sets the offset of the [ArrayData](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html "struct arrow::array::ArrayData")

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html#method.buffers" class="fn">buffers</a>(self, v: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>\>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html" class="struct" title="struct arrow::array::ArrayDataBuilder">ArrayDataBuilder</a>

Sets the buffers of the [ArrayData](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html "struct arrow::array::ArrayData")

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html#method.add_buffer" class="fn">add_buffer</a>(self, b: <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html" class="struct" title="struct arrow::array::ArrayDataBuilder">ArrayDataBuilder</a>

Adds a single buffer to the [ArrayData](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html "struct arrow::array::ArrayData")’s buffers

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html#method.add_buffers" class="fn">add_buffers</a>\<I\>(self, bs: I) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html" class="struct" title="struct arrow::array::ArrayDataBuilder">ArrayDataBuilder</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>\>,

Adds multiple buffers to the [ArrayData](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html "struct arrow::array::ArrayData")’s buffers

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html#method.child_data" class="fn">child_data</a>(self, v: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>\>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html" class="struct" title="struct arrow::array::ArrayDataBuilder">ArrayDataBuilder</a>

Sets the child data of the [ArrayData](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html "struct arrow::array::ArrayData")

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html#method.add_child_data" class="fn">add_child_data</a>(self, r: <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html" class="struct" title="struct arrow::array::ArrayDataBuilder">ArrayDataBuilder</a>

Adds a single child data to the [ArrayData](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html "struct arrow::array::ArrayData")’s child data

#### pub unsafe fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html#method.build_unchecked" class="fn">build_unchecked</a>(self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

Creates an array data, without any validation

Note: This is shorthand for

``` rust
builder.skip_validation(true).build().unwrap()
```

##### <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html#safety" class="doc-anchor">§</a>Safety

The same caveats as [`ArrayData::new_unchecked`](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.new_unchecked "associated function arrow::array::ArrayData::new_unchecked") apply.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html#method.build" class="fn">build</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Creates an `ArrayData`, consuming `self`

##### <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html#safety-1" class="doc-anchor">§</a>Safety

By default the underlying buffers are checked to ensure they are valid Arrow data. However, if the [`Self::skip_validation`](https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html#method.skip_validation "method arrow::array::ArrayDataBuilder::skip_validation") flag has been set to true (by the `unsafe` API) this validation is skipped. If the data is not valid, undefined behavior will result.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html#method.build_aligned" class="fn">build_aligned</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

👎Deprecated since 54.1.0: Use ArrayData::align_buffers instead

Creates an array data, validating all inputs, and aligning any buffers

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html#method.align_buffers" class="fn">align_buffers</a>(self, align_buffers: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html" class="struct" title="struct arrow::array::ArrayDataBuilder">ArrayDataBuilder</a>

Ensure that all buffers are aligned, copying data if necessary

Rust requires that arrays are aligned to their corresponding primitive, see [`Layout::array`](https://doc.rust-lang.org/nightly/core/alloc/layout/struct.Layout.html#method.array "associated function core::alloc::layout::Layout::array") and [`std::mem::align_of`](https://doc.rust-lang.org/nightly/core/mem/fn.align_of.html "fn core::mem::align_of").

[`ArrayData`](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html "struct arrow::array::ArrayData") therefore requires that all buffers have at least this alignment, to allow for [slice](https://doc.rust-lang.org/nightly/alloc/slice/index.html "mod alloc::slice") based APIs. See [`BufferSpec::FixedWidth`](https://docs.rs/arrow/latest/arrow/array/enum.BufferSpec.html#variant.FixedWidth "variant arrow::array::BufferSpec::FixedWidth").

As this alignment is architecture specific, and not guaranteed by all arrow implementations, this flag is provided to automatically copy buffers to a new correctly aligned allocation when necessary, making it useful when interacting with buffers produced by other systems, e.g. IPC or FFI.

If this flag is not enabled, `[Self::build`\] return an error on encountering insufficiently aligned buffers.

#### pub unsafe fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html#method.skip_validation" class="fn">skip_validation</a>(self, skip_validation: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html" class="struct" title="struct arrow::array::ArrayDataBuilder">ArrayDataBuilder</a>

Skips validation of the data.

If this flag is enabled, `[Self::build`\] will skip validation of the data

If this flag is not enabled, `[Self::build`\] will validate that all buffers are valid and will return an error if any data is invalid. Validation can be expensive.

##### <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html#safety-2" class="doc-anchor">§</a>Safety

If validation is skipped, the buffers must form a valid Arrow array, otherwise undefined behavior will result

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html#impl-Debug-for-ArrayDataBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html" class="struct" title="struct arrow::array::ArrayDataBuilder">ArrayDataBuilder</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html#impl-From%3CArrayData%3E-for-ArrayDataBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html" class="struct" title="struct arrow::array::ArrayDataBuilder">ArrayDataBuilder</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(d: <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html" class="struct" title="struct arrow::array::ArrayDataBuilder">ArrayDataBuilder</a>

Converts to this type from the input type.

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html#blanket-implementations" class="anchor">§</a>
