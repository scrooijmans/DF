# Struct ArrayData Copy item path

<a href="https://docs.rs/arrow-data/56.2.0/x86_64-unknown-linux-gnu/src/arrow_data/data.rs.html#205" class="src">Source</a>

``` rust
pub struct ArrayData { /* private fields */ }
```

Expand description

A generic representation of Arrow array data which encapsulates common attributes and operations for Arrow array.

Specific operations for different arrays types (e.g., primitive, list, struct) are implemented in `Array`.

## <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#memory-layout" class="doc-anchor">§</a>Memory Layout

`ArrayData` has references to one or more underlying data buffers and optional child ArrayData, depending on type as illustrated below. Bitmaps are not shown for simplicity but they are stored similarly to the buffers.

``` text
                       offset
                      points to
┌───────────────────┐ start of  ┌───────┐       Different
│                   │   data    │       │     ArrayData may
│ArrayData {        │           │....   │     also refers to
│  data_type: ...   │   ─ ─ ─ ─▶│1234   │  ┌ ─  the same
│  offset: ... ─ ─ ─│─ ┘        │4372   │      underlying
│  len: ...    ─ ─ ─│─ ┐        │4888   │  │     buffer with different offset/len
│  buffers: [       │           │5882   │◀─
│    ...            │  │        │4323   │
│  ]                │   ─ ─ ─ ─▶│4859   │
│  child_data: [    │           │....   │
│    ...            │           │       │
│  ]                │           └───────┘
│}                  │
│                   │            Shared Buffer uses
│               │   │            bytes::Bytes to hold
└───────────────────┘            actual data values
          ┌ ─ ─ ┘

          ▼
┌───────────────────┐
│ArrayData {        │
│  ...              │
│}                  │
│                   │
└───────────────────┘

Child ArrayData may also have its own buffers and children
```

## Implementations<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#impl-ArrayData" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

#### pub unsafe fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.new_unchecked" class="fn">new_unchecked</a>( data_type: <a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a>, len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, null_count: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>, null_bit_buffer: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>\>, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, buffers: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>\>, child_data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

Create a new ArrayData instance;

If `null_count` is not specified, the number of nulls in null_bit_buffer is calculated.

If the number of nulls is 0 then the null_bit_buffer is set to `None`.

##### <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#safety" class="doc-anchor">§</a>Safety

The input values *must* form a valid Arrow array for `data_type`, or undefined behavior can result.

Note: This is a low level API and most users of the arrow crate should create arrays using the methods in the `array` module.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.try_new" class="fn">try_new</a>( data_type: <a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a>, len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, null_bit_buffer: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>\>, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, buffers: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>\>, child_data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Create a new ArrayData, validating that the provided buffers form a valid Arrow array of the specified data type.

If the number of nulls in `null_bit_buffer` is 0 then the null_bit_buffer is set to `None`.

Internally this calls through to [`Self::validate_data`](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.validate_data "method arrow::array::ArrayData::validate_data")

Note: This is a low level API and most users of the arrow crate should create arrays using the builders found in [arrow_array](https://docs.rs/arrow-array)

#### pub const fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.builder" class="fn">builder</a>(data_type: <a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html" class="struct" title="struct arrow::array::ArrayDataBuilder">ArrayDataBuilder</a>

Returns a builder to construct a [`ArrayData`](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html "struct arrow::array::ArrayData") instance of the same [`DataType`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html "enum arrow::datatypes::DataType")

#### pub const fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.data_type" class="fn">data_type</a>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a>

Returns a reference to the [`DataType`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html "enum arrow::datatypes::DataType") of this [`ArrayData`](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html "struct arrow::array::ArrayData")

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.buffers" class="fn">buffers</a>(&self) -\> &\[<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>\]

Returns the [`Buffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html "struct arrow::buffer::Buffer") storing data for this [`ArrayData`](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html "struct arrow::array::ArrayData")

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.child_data" class="fn">child_data</a>(&self) -\> &\[<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>\]

Returns a slice of children [`ArrayData`](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html "struct arrow::array::ArrayData"). This will be non empty for type such as lists and structs.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.is_null" class="fn">is_null</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether the element at index `i` is null

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.nulls" class="fn">nulls</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>\>

Returns a reference to the null buffer of this [`ArrayData`](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html "struct arrow::array::ArrayData") if any

Note: [`ArrayData::offset`](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.offset "method arrow::array::ArrayData::offset") does NOT apply to the returned [`NullBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html "struct arrow::buffer::NullBuffer")

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.is_valid" class="fn">is_valid</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether the element at index `i` is not null

#### pub const fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the length (i.e., number of elements) of this [`ArrayData`](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html "struct arrow::array::ArrayData").

#### pub const fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether this [`ArrayData`](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html "struct arrow::array::ArrayData") is empty

#### pub const fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.offset" class="fn">offset</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the offset of this [`ArrayData`](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html "struct arrow::array::ArrayData")

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.null_count" class="fn">null_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of nulls in this array

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.get_buffer_memory_size" class="fn">get_buffer_memory_size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of bytes of memory occupied by the buffers owned by this [`ArrayData`](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html "struct arrow::array::ArrayData") and all of its children. (See also diagram on [`ArrayData`](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html "struct arrow::array::ArrayData")).

Note that this [`ArrayData`](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html "struct arrow::array::ArrayData") may only refer to a subset of the data in the underlying [`Buffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html "struct arrow::buffer::Buffer")s (due to `offset` and `length`), but the size returned includes the entire size of the buffers.

If multiple [`ArrayData`](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html "struct arrow::array::ArrayData")s refer to the same underlying [`Buffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html "struct arrow::buffer::Buffer")s they will both report the same size.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.get_slice_memory_size" class="fn">get_slice_memory_size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Returns the total number of the bytes of memory occupied by the buffers by this slice of [`ArrayData`](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html "struct arrow::array::ArrayData") (See also diagram on [`ArrayData`](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html "struct arrow::array::ArrayData")).

This is approximately the number of bytes if a new [`ArrayData`](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html "struct arrow::array::ArrayData") was formed by creating new [`Buffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html "struct arrow::buffer::Buffer")s with exactly the data needed.

For example, a [`DataType::Int64`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Int64 "variant arrow::datatypes::DataType::Int64") with `100` elements, [`Self::get_slice_memory_size`](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.get_slice_memory_size "method arrow::array::ArrayData::get_slice_memory_size") would return `100 * 8 = 800`. If the [`ArrayData`](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html "struct arrow::array::ArrayData") was then [`Self::slice`](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.slice "method arrow::array::ArrayData::slice")ed to refer to its first `20` elements, then [`Self::get_slice_memory_size`](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.get_slice_memory_size "method arrow::array::ArrayData::get_slice_memory_size") on the sliced [`ArrayData`](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html "struct arrow::array::ArrayData") would return `20 * 8 = 160`.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.get_array_memory_size" class="fn">get_array_memory_size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of bytes of memory occupied physically by this [`ArrayData`](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html "struct arrow::array::ArrayData") and all its [`Buffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html "struct arrow::buffer::Buffer")s and children. (See also diagram on [`ArrayData`](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html "struct arrow::array::ArrayData")).

Equivalent to: `size_of_val(self)` + [`Self::get_buffer_memory_size`](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.get_buffer_memory_size "method arrow::array::ArrayData::get_buffer_memory_size") + `size_of_val(child)` for all children

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.slice" class="fn">slice</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

Creates a zero-copy slice of itself. This creates a new [`ArrayData`](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html "struct arrow::array::ArrayData") pointing at the same underlying [`Buffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html "struct arrow::buffer::Buffer")s with a different offset and len

##### <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#panics" class="doc-anchor">§</a>Panics

Panics if `offset + length > self.len()`.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.buffer" class="fn">buffer</a>\<T\>(&self, buffer: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html" class="trait" title="trait arrow::datatypes::ArrowNativeType">ArrowNativeType</a>,

Returns the `buffer` as a slice of type `T` starting at self.offset

##### <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#panics-1" class="doc-anchor">§</a>Panics

This function panics if:

- the buffer is not byte-aligned with type T, or
- the datatype is `Boolean` (it corresponds to a bit-packed buffer where the offset is not applicable)

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.new_null" class="fn">new_null</a>(data_type: &<a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a>, len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

Returns a new [`ArrayData`](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html "struct arrow::array::ArrayData") valid for `data_type` containing `len` null values

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.new_empty" class="fn">new_empty</a>(data_type: &<a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

Returns a new empty [ArrayData](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html "struct arrow::array::ArrayData") valid for `data_type`.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.align_buffers" class="fn">align_buffers</a>(&mut self)

Verifies that the buffers meet the minimum alignment requirements for the data type

Buffers that are not adequately aligned will be copied to a new aligned allocation

This can be useful for when interacting with data sent over IPC or FFI, that may not meet the minimum alignment requirements

This also aligns buffers of children data

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.validate" class="fn">validate</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

“cheap” validation of an `ArrayData`. Ensures buffers are sufficiently sized to store `len` + `offset` total elements of `data_type` and performs other inexpensive consistency checks.

This check is “cheap” in the sense that it does not validate the contents of the buffers (e.g. that all offsets for UTF8 arrays are within the bounds of the values buffer).

See [ArrayData::validate_data](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.validate_data "method arrow::array::ArrayData::validate_data") to validate fully the offset content and the validity of utf8 data

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.validate_data" class="fn">validate_data</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Validate that the data contained within this [`ArrayData`](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html "struct arrow::array::ArrayData") is valid

1.  Null count is correct
2.  All offsets are valid
3.  All String data is valid UTF-8
4.  All dictionary offsets are valid

Internally this calls:

- [`Self::validate`](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.validate "method arrow::array::ArrayData::validate")
- [`Self::validate_nulls`](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.validate_nulls "method arrow::array::ArrayData::validate_nulls")
- [`Self::validate_values`](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.validate_values "method arrow::array::ArrayData::validate_values")

Note: this does not recurse into children, for a recursive variant see [`Self::validate_full`](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.validate_full "method arrow::array::ArrayData::validate_full")

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.validate_full" class="fn">validate_full</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Performs a full recursive validation of this [`ArrayData`](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html "struct arrow::array::ArrayData") and all its children

This is equivalent to calling [`Self::validate_data`](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.validate_data "method arrow::array::ArrayData::validate_data") on this [`ArrayData`](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html "struct arrow::array::ArrayData") and all its children recursively

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.validate_nulls" class="fn">validate_nulls</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Validates the values stored within this [`ArrayData`](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html "struct arrow::array::ArrayData") are valid without recursing into child [`ArrayData`](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html "struct arrow::array::ArrayData")

Does not (yet) check

1.  Union type_ids are valid see [\#85](https://github.com/apache/arrow-rs/issues/85)
2.  the the null count is correct and that any
3.  nullability requirements of its children are correct

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.validate_values" class="fn">validate_values</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Validates the values stored within this [`ArrayData`](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html "struct arrow::array::ArrayData") are valid without recursing into child [`ArrayData`](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html "struct arrow::array::ArrayData")

Does not (yet) check

1.  Union type_ids are valid see [\#85](https://github.com/apache/arrow-rs/issues/85)

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.ptr_eq" class="fn">ptr_eq</a>(&self, other: &<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if this `ArrayData` is equal to `other`, using pointer comparisons to determine buffer equality. This is cheaper than `PartialEq::eq` but may return false when the arrays are logically equal

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.into_builder" class="fn">into_builder</a>(self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html" class="struct" title="struct arrow::array::ArrayDataBuilder">ArrayDataBuilder</a>

Converts this [`ArrayData`](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html "struct arrow::array::ArrayData") into an [`ArrayDataBuilder`](https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html "struct arrow::array::ArrayDataBuilder")

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#impl-Clone-for-ArrayData" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#impl-Debug-for-ArrayData" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#impl-From%3CArrayData%3E-for-ArrayDataBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html" class="struct" title="struct arrow::array::ArrayDataBuilder">ArrayDataBuilder</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.from-28" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(d: <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html" class="struct" title="struct arrow::array::ArrayDataBuilder">ArrayDataBuilder</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#impl-From%3CArrayData%3E-for-BooleanArray" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.BooleanArray.html" class="struct" title="struct arrow::array::BooleanArray">BooleanArray</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(data: <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.BooleanArray.html" class="struct" title="struct arrow::array::BooleanArray">BooleanArray</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#impl-From%3CArrayData%3E-for-DictionaryArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html" class="struct" title="struct arrow::array::DictionaryArray">DictionaryArray</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowDictionaryKeyType.html" class="trait" title="trait arrow::datatypes::ArrowDictionaryKeyType">ArrowDictionaryKeyType</a>,

Constructs a `DictionaryArray` from an array data reference.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.from-4" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(data: <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html" class="struct" title="struct arrow::array::DictionaryArray">DictionaryArray</a>\<T\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#impl-From%3CArrayData%3E-for-FixedSizeBinaryArray" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html" class="struct" title="struct arrow::array::FixedSizeBinaryArray">FixedSizeBinaryArray</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.from-6" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(data: <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html" class="struct" title="struct arrow::array::FixedSizeBinaryArray">FixedSizeBinaryArray</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#impl-From%3CArrayData%3E-for-FixedSizeListArray" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListArray.html" class="struct" title="struct arrow::array::FixedSizeListArray">FixedSizeListArray</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.from-8" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(data: <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListArray.html" class="struct" title="struct arrow::array::FixedSizeListArray">FixedSizeListArray</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#impl-From%3CArrayData%3E-for-GenericByteArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteArray.html" class="struct" title="struct arrow::array::GenericByteArray">GenericByteArray</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait arrow::datatypes::ByteArrayType">ByteArrayType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.from-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(data: <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteArray.html" class="struct" title="struct arrow::array::GenericByteArray">GenericByteArray</a>\<T\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#impl-From%3CArrayData%3E-for-GenericByteViewArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html" class="trait" title="trait arrow::datatypes::ByteViewType">ByteViewType</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.from-24" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<T\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#impl-From%3CArrayData%3E-for-GenericListArray%3COffsetSize%3E" class="anchor">§</a>

### impl\<OffsetSize\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html" class="struct" title="struct arrow::array::GenericListArray">GenericListArray</a>\<OffsetSize\>

where OffsetSize: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.from-10" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(data: <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html" class="struct" title="struct arrow::array::GenericListArray">GenericListArray</a>\<OffsetSize\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#impl-From%3CArrayData%3E-for-GenericListViewArray%3COffsetSize%3E" class="anchor">§</a>

### impl\<OffsetSize\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListViewArray.html" class="struct" title="struct arrow::array::GenericListViewArray">GenericListViewArray</a>\<OffsetSize\>

where OffsetSize: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.from-27" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(data: <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListViewArray.html" class="struct" title="struct arrow::array::GenericListViewArray">GenericListViewArray</a>\<OffsetSize\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#impl-From%3CArrayData%3E-for-MapArray" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.MapArray.html" class="struct" title="struct arrow::array::MapArray">MapArray</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.from-12" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(data: <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.MapArray.html" class="struct" title="struct arrow::array::MapArray">MapArray</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#impl-From%3CArrayData%3E-for-NullArray" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.NullArray.html" class="struct" title="struct arrow::array::NullArray">NullArray</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.from-14" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(data: <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.NullArray.html" class="struct" title="struct arrow::array::NullArray">NullArray</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#impl-From%3CArrayData%3E-for-PrimitiveArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>,

Constructs a `PrimitiveArray` from an array data reference.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.from-17" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(data: <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#impl-From%3CArrayData%3E-for-RunArray%3CR%3E" class="anchor">§</a>

### impl\<R\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html" class="struct" title="struct arrow::array::RunArray">RunArray</a>\<R\>

where R: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.RunEndIndexType.html" class="trait" title="trait arrow::datatypes::RunEndIndexType">RunEndIndexType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.from-22" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(data: <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html" class="struct" title="struct arrow::array::RunArray">RunArray</a>\<R\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#impl-From%3CArrayData%3E-for-StructArray" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.StructArray.html" class="struct" title="struct arrow::array::StructArray">StructArray</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.from-18" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(data: <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.StructArray.html" class="struct" title="struct arrow::array::StructArray">StructArray</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#impl-From%3CArrayData%3E-for-UnionArray" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.UnionArray.html" class="struct" title="struct arrow::array::UnionArray">UnionArray</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.from-20" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(data: <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.UnionArray.html" class="struct" title="struct arrow::array::UnionArray">UnionArray</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#impl-From%3CBooleanArray%3E-for-ArrayData" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.BooleanArray.html" class="struct" title="struct arrow::array::BooleanArray">BooleanArray</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(array: <a href="https://docs.rs/arrow/latest/arrow/array/struct.BooleanArray.html" class="struct" title="struct arrow::array::BooleanArray">BooleanArray</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#impl-From%3CDictionaryArray%3CT%3E%3E-for-ArrayData" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html" class="struct" title="struct arrow::array::DictionaryArray">DictionaryArray</a>\<T\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowDictionaryKeyType.html" class="trait" title="trait arrow::datatypes::ArrowDictionaryKeyType">ArrowDictionaryKeyType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.from-5" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(array: <a href="https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html" class="struct" title="struct arrow::array::DictionaryArray">DictionaryArray</a>\<T\>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#impl-From%3CFixedSizeBinaryArray%3E-for-ArrayData" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html" class="struct" title="struct arrow::array::FixedSizeBinaryArray">FixedSizeBinaryArray</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.from-7" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(array: <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html" class="struct" title="struct arrow::array::FixedSizeBinaryArray">FixedSizeBinaryArray</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#impl-From%3CFixedSizeListArray%3E-for-ArrayData" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListArray.html" class="struct" title="struct arrow::array::FixedSizeListArray">FixedSizeListArray</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.from-9" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(array: <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListArray.html" class="struct" title="struct arrow::array::FixedSizeListArray">FixedSizeListArray</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#impl-From%3CGenericByteArray%3CT%3E%3E-for-ArrayData" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteArray.html" class="struct" title="struct arrow::array::GenericByteArray">GenericByteArray</a>\<T\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait arrow::datatypes::ByteArrayType">ByteArrayType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.from-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(array: <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteArray.html" class="struct" title="struct arrow::array::GenericByteArray">GenericByteArray</a>\<T\>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#impl-From%3CGenericByteViewArray%3CT%3E%3E-for-ArrayData" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<T\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html" class="trait" title="trait arrow::datatypes::ByteViewType">ByteViewType</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.from-25" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(array: <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<T\>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#impl-From%3CGenericListArray%3COffsetSize%3E%3E-for-ArrayData" class="anchor">§</a>

### impl\<OffsetSize\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html" class="struct" title="struct arrow::array::GenericListArray">GenericListArray</a>\<OffsetSize\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

where OffsetSize: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.from-11" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(array: <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html" class="struct" title="struct arrow::array::GenericListArray">GenericListArray</a>\<OffsetSize\>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#impl-From%3CGenericListViewArray%3COffsetSize%3E%3E-for-ArrayData" class="anchor">§</a>

### impl\<OffsetSize\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListViewArray.html" class="struct" title="struct arrow::array::GenericListViewArray">GenericListViewArray</a>\<OffsetSize\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

where OffsetSize: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.from-26" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(array: <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListViewArray.html" class="struct" title="struct arrow::array::GenericListViewArray">GenericListViewArray</a>\<OffsetSize\>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#impl-From%3CMapArray%3E-for-ArrayData" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.MapArray.html" class="struct" title="struct arrow::array::MapArray">MapArray</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.from-13" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(array: <a href="https://docs.rs/arrow/latest/arrow/array/struct.MapArray.html" class="struct" title="struct arrow::array::MapArray">MapArray</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#impl-From%3CNullArray%3E-for-ArrayData" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.NullArray.html" class="struct" title="struct arrow::array::NullArray">NullArray</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.from-15" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(array: <a href="https://docs.rs/arrow/latest/arrow/array/struct.NullArray.html" class="struct" title="struct arrow::array::NullArray">NullArray</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#impl-From%3CPrimitiveArray%3CT%3E%3E-for-ArrayData" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

where T: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.from-16" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(array: <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#impl-From%3CRunArray%3CR%3E%3E-for-ArrayData" class="anchor">§</a>

### impl\<R\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html" class="struct" title="struct arrow::array::RunArray">RunArray</a>\<R\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

where R: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.RunEndIndexType.html" class="trait" title="trait arrow::datatypes::RunEndIndexType">RunEndIndexType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.from-23" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(array: <a href="https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html" class="struct" title="struct arrow::array::RunArray">RunArray</a>\<R\>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#impl-From%3CStructArray%3E-for-ArrayData" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.StructArray.html" class="struct" title="struct arrow::array::StructArray">StructArray</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.from-19" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(array: <a href="https://docs.rs/arrow/latest/arrow/array/struct.StructArray.html" class="struct" title="struct arrow::array::StructArray">StructArray</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#impl-From%3CUnionArray%3E-for-ArrayData" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.UnionArray.html" class="struct" title="struct arrow::array::UnionArray">UnionArray</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.from-21" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(array: <a href="https://docs.rs/arrow/latest/arrow/array/struct.UnionArray.html" class="struct" title="struct arrow::array::UnionArray">UnionArray</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#impl-FromPyArrow-for-ArrayData" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow-pyarrow/56.2.0/x86_64-unknown-linux-gnu/arrow_pyarrow/trait.FromPyArrow.html" class="trait" title="trait arrow_pyarrow::FromPyArrow">FromPyArrow</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.from_pyarrow_bound" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow-pyarrow/56.2.0/x86_64-unknown-linux-gnu/arrow_pyarrow/trait.FromPyArrow.html#tymethod.from_pyarrow_bound" class="fn">from_pyarrow_bound</a>(value: &<a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/instance/struct.Bound.html" class="struct" title="struct pyo3::instance::Bound">Bound</a>\<'\_, <a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/types/any/struct.PyAny.html" class="struct" title="struct pyo3::types::any::PyAny">PyAny</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>, <a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/err/struct.PyErr.html" class="struct" title="struct pyo3::err::PyErr">PyErr</a>\>

Convert a Python object to an arrow-rs type. [Read more](https://docs.rs/arrow-pyarrow/56.2.0/x86_64-unknown-linux-gnu/arrow_pyarrow/trait.FromPyArrow.html#tymethod.from_pyarrow_bound)

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#impl-PartialEq-for-ArrayData" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#impl-ToPyArrow-for-ArrayData" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow-pyarrow/56.2.0/x86_64-unknown-linux-gnu/arrow_pyarrow/trait.ToPyArrow.html" class="trait" title="trait arrow_pyarrow::ToPyArrow">ToPyArrow</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#method.to_pyarrow" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow-pyarrow/56.2.0/x86_64-unknown-linux-gnu/arrow_pyarrow/trait.ToPyArrow.html#tymethod.to_pyarrow" class="fn">to_pyarrow</a>(&self, py: <a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/marker/struct.Python.html" class="struct" title="struct pyo3::marker::Python">Python</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/instance/struct.Py.html" class="struct" title="struct pyo3::instance::Py">Py</a>\<<a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/types/any/struct.PyAny.html" class="struct" title="struct pyo3::types::any::PyAny">PyAny</a>\>, <a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/err/struct.PyErr.html" class="struct" title="struct pyo3::err::PyErr">PyErr</a>\>

Convert the implemented type into a Python object without consuming it.

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html#blanket-implementations" class="anchor">§</a>
