# Struct MapArray Copy item path

<a href="https://docs.rs/arrow-array/56.0.0/x86_64-unknown-linux-gnu/src/arrow_array/array/map_array.rs.html#36" class="src">Source</a>

``` rust
pub struct MapArray { /* private fields */ }
```

Expand description

An array of key-value maps

Keys should always be non-null, but values can be null.

[`MapArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html "struct datafusion::common::arrow::array::MapArray") is physically a [`ListArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.ListArray.html "type datafusion::common::arrow::array::ListArray") of key values pairs stored as an `entries` [`StructArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html "struct datafusion::common::arrow::array::StructArray") with 2 child fields.

See [`MapBuilder`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapBuilder.html "struct datafusion::common::arrow::array::MapBuilder") for how to construct a [`MapArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html "struct datafusion::common::arrow::array::MapArray")

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#impl-MapArray" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html" class="struct" title="struct datafusion::common::arrow::array::MapArray">MapArray</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#method.try_new" class="fn">try_new</a>( field: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>, offsets: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.OffsetBuffer.html" class="struct" title="struct datafusion::common::arrow::buffer::OffsetBuffer">OffsetBuffer</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>, entries: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html" class="struct" title="struct datafusion::common::arrow::array::StructArray">StructArray</a>, nulls: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct datafusion::common::arrow::buffer::NullBuffer">NullBuffer</a>\>, ordered: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html" class="struct" title="struct datafusion::common::arrow::array::MapArray">MapArray</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

Create a new [`MapArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html "struct datafusion::common::arrow::array::MapArray") from the provided parts

See [`MapBuilder`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapBuilder.html "struct datafusion::common::arrow::array::MapBuilder") for a higher-level interface to construct a [`MapArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html "struct datafusion::common::arrow::array::MapArray")

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#errors" class="doc-anchor">§</a>Errors

Errors if

- `offsets.len() - 1 != nulls.len()`
- `offsets.last() > entries.len()`
- `field.is_nullable()`
- `entries.null_count() != 0`
- `entries.columns().len() != 2`
- `field.data_type() != entries.data_type()`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#method.new" class="fn">new</a>( field: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>, offsets: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.OffsetBuffer.html" class="struct" title="struct datafusion::common::arrow::buffer::OffsetBuffer">OffsetBuffer</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>, entries: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html" class="struct" title="struct datafusion::common::arrow::array::StructArray">StructArray</a>, nulls: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct datafusion::common::arrow::buffer::NullBuffer">NullBuffer</a>\>, ordered: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html" class="struct" title="struct datafusion::common::arrow::array::MapArray">MapArray</a>

Create a new [`MapArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html "struct datafusion::common::arrow::array::MapArray") from the provided parts

See [`MapBuilder`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapBuilder.html "struct datafusion::common::arrow::array::MapBuilder") for a higher-level interface to construct a [`MapArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html "struct datafusion::common::arrow::array::MapArray")

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#panics" class="doc-anchor">§</a>Panics

Panics if [`Self::try_new`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#method.try_new "associated function datafusion::common::arrow::array::MapArray::try_new") returns an error

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#method.into_parts" class="fn">into_parts</a>( self, ) -\> (<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.OffsetBuffer.html" class="struct" title="struct datafusion::common::arrow::buffer::OffsetBuffer">OffsetBuffer</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html" class="struct" title="struct datafusion::common::arrow::array::StructArray">StructArray</a>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct datafusion::common::arrow::buffer::NullBuffer">NullBuffer</a>\>, <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>)

Deconstruct this array into its constituent parts

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#method.offsets" class="fn">offsets</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.OffsetBuffer.html" class="struct" title="struct datafusion::common::arrow::buffer::OffsetBuffer">OffsetBuffer</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>

Returns a reference to the offsets of this map

Unlike [`Self::value_offsets`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#method.value_offsets "method datafusion::common::arrow::array::MapArray::value_offsets") this returns the [`OffsetBuffer`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.OffsetBuffer.html "struct datafusion::common::arrow::buffer::OffsetBuffer") allowing for zero-copy cloning

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#method.keys" class="fn">keys</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>

Returns a reference to the keys of this map

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#method.values" class="fn">values</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>

Returns a reference to the values of this map

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#method.entries" class="fn">entries</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html" class="struct" title="struct datafusion::common::arrow::array::StructArray">StructArray</a>

Returns a reference to the [`StructArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html "struct datafusion::common::arrow::array::StructArray") entries of this map

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#method.key_type" class="fn">key_type</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

Returns the data type of the map’s keys.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#method.value_type" class="fn">value_type</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

Returns the data type of the map’s values.

#### pub unsafe fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#method.value_unchecked" class="fn">value_unchecked</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html" class="struct" title="struct datafusion::common::arrow::array::StructArray">StructArray</a>

Returns ith value of this map array.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#safety" class="doc-anchor">§</a>Safety

Caller must ensure that the index is within the array bounds

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#method.value" class="fn">value</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html" class="struct" title="struct datafusion::common::arrow::array::StructArray">StructArray</a>

Returns ith value of this map array.

This is a [`StructArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html "struct datafusion::common::arrow::array::StructArray") containing two fields

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#method.value_offsets" class="fn">value_offsets</a>(&self) -\> &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\]

Returns the offset values in the offsets buffer

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#method.value_length" class="fn">value_length</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

Returns the length for value at index `i`.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#method.slice" class="fn">slice</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html" class="struct" title="struct datafusion::common::arrow::array::MapArray">MapArray</a>

Returns a zero-copy slice of this array with the indicated offset and length.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#method.iter" class="fn">iter</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.ArrayIter.html" class="struct" title="struct datafusion::common::arrow::array::ArrayIter">ArrayIter</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html" class="struct" title="struct datafusion::common::arrow::array::MapArray">MapArray</a>\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#" class="tooltip" data-notable-ty="ArrayIter&lt;&amp;MapArray&gt;">ⓘ</a>

constructs a new iterator

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#impl-MapArray-1" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html" class="struct" title="struct datafusion::common::arrow::array::MapArray">MapArray</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#method.new_from_strings" class="fn">new_from_strings</a>\<'a\>( keys: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>, values: &dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>, entry_offsets: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html" class="struct" title="struct datafusion::common::arrow::array::MapArray">MapArray</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

Creates map array from provided keys, values and entry_offsets.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#impl-Array-for-MapArray" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html" class="struct" title="struct datafusion::common::arrow::array::MapArray">MapArray</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#method.as_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the array as [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") so that it can be downcasted to a specific implementation. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.as_any)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#method.to_data" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.to_data" class="fn">to_data</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.ArrayData.html" class="struct" title="struct datafusion::common::arrow::array::ArrayData">ArrayData</a>

Returns the underlying data of this array

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#method.into_data" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.into_data" class="fn">into_data</a>(self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.ArrayData.html" class="struct" title="struct datafusion::common::arrow::array::ArrayData">ArrayData</a>

Returns the underlying data of this array [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.into_data)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#method.data_type" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.data_type" class="fn">data_type</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

Returns a reference to the [`DataType`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html "enum datafusion::common::arrow::datatypes::DataType") of this array. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.data_type)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#method.slice-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.slice" class="fn">slice</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>

Returns a zero-copy slice of this array with the indicated offset and length. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.slice)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#method.len" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the length (i.e., number of elements) of this array. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.len)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#method.is_empty" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether this array is empty. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.is_empty)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#method.shrink_to_fit" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.shrink_to_fit" class="fn">shrink_to_fit</a>(&mut self)

Shrinks the capacity of any exclusively owned buffer as much as possible [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.shrink_to_fit)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#method.offset" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.offset" class="fn">offset</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the offset into the underlying data used by this array(-slice). Note that the underlying data can be shared by many arrays. This defaults to `0`. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.offset)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#method.nulls" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.nulls" class="fn">nulls</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct datafusion::common::arrow::buffer::NullBuffer">NullBuffer</a>\>

Returns the null buffer of this array if any. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.nulls)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#method.logical_null_count" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.logical_null_count" class="fn">logical_null_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of logical null values in this array. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.logical_null_count)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#method.get_buffer_memory_size" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.get_buffer_memory_size" class="fn">get_buffer_memory_size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of bytes of memory pointed to by this array. The buffers store bytes in the Arrow memory format, and include the data as well as the validity map. Note that this does not always correspond to the exact memory usage of an array, since multiple arrays can share the same buffers or slices thereof.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#method.get_array_memory_size" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.get_array_memory_size" class="fn">get_array_memory_size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of bytes of memory occupied physically by this array. This value will always be greater than returned by `get_buffer_memory_size()` and includes the overhead of the data structures that contain the pointers to the various buffers.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#method.logical_nulls" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.logical_nulls" class="fn">logical_nulls</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct datafusion::common::arrow::buffer::NullBuffer">NullBuffer</a>\>

Returns a potentially computed [`NullBuffer`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.NullBuffer.html "struct datafusion::common::arrow::buffer::NullBuffer") that represents the logical null values of this array, if any. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.logical_nulls)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#method.is_null" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.is_null" class="fn">is_null</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether the element at `index` is null according to [`Array::nulls`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.nulls "method datafusion::common::arrow::array::Array::nulls") [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.is_null)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#method.is_valid" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.is_valid" class="fn">is_valid</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether the element at `index` is *not* null, the opposite of [`Self::is_null`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.is_null "method arrow_array::array::Array::is_null::is_null"). [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.is_valid)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#method.null_count" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.null_count" class="fn">null_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of physical null values in this array. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.null_count)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#method.is_nullable" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.is_nullable" class="fn">is_nullable</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns `false` if the array is guaranteed to not contain any logical nulls [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.is_nullable)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#impl-ArrayAccessor-for-%26MapArray" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html" class="trait" title="trait datafusion::common::arrow::array::ArrayAccessor">ArrayAccessor</a> for &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html" class="struct" title="struct datafusion::common::arrow::array::MapArray">MapArray</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#associatedtype.Item" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html" class="struct" title="struct datafusion::common::arrow::array::StructArray">StructArray</a>

The Arrow type of the element being accessed.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#method.value-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#tymethod.value" class="fn">value</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> \<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html" class="struct" title="struct datafusion::common::arrow::array::MapArray">MapArray</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html" class="trait" title="trait datafusion::common::arrow::array::ArrayAccessor">ArrayAccessor</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#associatedtype.Item" class="associatedtype" title="type datafusion::common::arrow::array::ArrayAccessor::Item">Item</a>

Returns the element at index `i` [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#tymethod.value)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#method.value_unchecked-1" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#tymethod.value_unchecked" class="fn">value_unchecked</a>( &self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> \<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html" class="struct" title="struct datafusion::common::arrow::array::MapArray">MapArray</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html" class="trait" title="trait datafusion::common::arrow::array::ArrayAccessor">ArrayAccessor</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#associatedtype.Item" class="associatedtype" title="type datafusion::common::arrow::array::ArrayAccessor::Item">Item</a>

Returns the element at index `i` [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#tymethod.value_unchecked)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#impl-Clone-for-MapArray" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html" class="struct" title="struct datafusion::common::arrow::array::MapArray">MapArray</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html" class="struct" title="struct datafusion::common::arrow::array::MapArray">MapArray</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#impl-Debug-for-MapArray" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html" class="struct" title="struct datafusion::common::arrow::array::MapArray">MapArray</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#impl-From%3CArrayData%3E-for-MapArray" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.ArrayData.html" class="struct" title="struct datafusion::common::arrow::array::ArrayData">ArrayData</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html" class="struct" title="struct datafusion::common::arrow::array::MapArray">MapArray</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(data: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.ArrayData.html" class="struct" title="struct datafusion::common::arrow::array::ArrayData">ArrayData</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html" class="struct" title="struct datafusion::common::arrow::array::MapArray">MapArray</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#impl-From%3CMapArray%3E-for-ArrayData" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html" class="struct" title="struct datafusion::common::arrow::array::MapArray">MapArray</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.ArrayData.html" class="struct" title="struct datafusion::common::arrow::array::ArrayData">ArrayData</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(array: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html" class="struct" title="struct datafusion::common::arrow::array::MapArray">MapArray</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.ArrayData.html" class="struct" title="struct datafusion::common::arrow::array::ArrayData">ArrayData</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#impl-From%3CMapArray%3E-for-GenericListArray%3Ci32%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html" class="struct" title="struct datafusion::common::arrow::array::MapArray">MapArray</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericListArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericListArray">GenericListArray</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#method.from-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html" class="struct" title="struct datafusion::common::arrow::array::MapArray">MapArray</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericListArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericListArray">GenericListArray</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#impl-PartialEq-for-MapArray" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html" class="struct" title="struct datafusion::common::arrow::array::MapArray">MapArray</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html" class="struct" title="struct datafusion::common::arrow::array::MapArray">MapArray</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html#blanket-implementations" class="anchor">§</a>
