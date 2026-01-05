# Struct NullArray Copy item path

<a href="https://docs.rs/arrow-array/56.0.0/x86_64-unknown-linux-gnu/src/arrow_array/array/null_array.rs.html#46" class="src">Source</a>

``` rust
pub struct NullArray { /* private fields */ }
```

Expand description

An array of [null values](https://arrow.apache.org/docs/format/Columnar.html#null-layout)

A `NullArray` is a simplified array where all values are null.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html#example-create-an-array" class="doc-anchor">§</a>Example: Create an array

``` rust
use arrow_array::{Array, NullArray};

let array = NullArray::new(10);

assert!(array.is_nullable());
assert_eq!(array.len(), 10);
assert_eq!(array.null_count(), 0);
assert_eq!(array.logical_null_count(), 10);
assert_eq!(array.logical_nulls().unwrap().null_count(), 10);
```

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html#impl-NullArray" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html" class="struct" title="struct datafusion::common::arrow::array::NullArray">NullArray</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html#method.new" class="fn">new</a>(length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html" class="struct" title="struct datafusion::common::arrow::array::NullArray">NullArray</a>

Create a new [`NullArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html "struct datafusion::common::arrow::array::NullArray") of the specified length

*Note*: Use [`crate::array::new_null_array`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/fn.new_null_array.html "fn datafusion::common::arrow::array::new_null_array") if you need an array of some other [`DataType`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html "enum datafusion::common::arrow::datatypes::DataType").

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html#method.slice" class="fn">slice</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html" class="struct" title="struct datafusion::common::arrow::array::NullArray">NullArray</a>

Returns a zero-copy slice of this array with the indicated offset and length.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html#method.builder" class="fn">builder</a>(\_capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullBuilder.html" class="struct" title="struct datafusion::common::arrow::array::NullBuilder">NullBuilder</a>

Returns a new null array builder

Note that the `capacity` parameter to this function is *deprecated*. It now does nothing, and will be removed in a future version.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html#impl-Array-for-NullArray" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html" class="struct" title="struct datafusion::common::arrow::array::NullArray">NullArray</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html#method.as_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the array as [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") so that it can be downcasted to a specific implementation. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.as_any)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html#method.to_data" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.to_data" class="fn">to_data</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.ArrayData.html" class="struct" title="struct datafusion::common::arrow::array::ArrayData">ArrayData</a>

Returns the underlying data of this array

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html#method.into_data" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.into_data" class="fn">into_data</a>(self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.ArrayData.html" class="struct" title="struct datafusion::common::arrow::array::ArrayData">ArrayData</a>

Returns the underlying data of this array [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.into_data)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html#method.data_type" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.data_type" class="fn">data_type</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

Returns a reference to the [`DataType`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html "enum datafusion::common::arrow::datatypes::DataType") of this array. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.data_type)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html#method.slice-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.slice" class="fn">slice</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>

Returns a zero-copy slice of this array with the indicated offset and length. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.slice)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html#method.len" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the length (i.e., number of elements) of this array. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.len)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html#method.is_empty" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether this array is empty. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.is_empty)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html#method.offset" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.offset" class="fn">offset</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the offset into the underlying data used by this array(-slice). Note that the underlying data can be shared by many arrays. This defaults to `0`. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.offset)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html#method.nulls" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.nulls" class="fn">nulls</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct datafusion::common::arrow::buffer::NullBuffer">NullBuffer</a>\>

Returns the null buffer of this array if any. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.nulls)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html#method.logical_nulls" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.logical_nulls" class="fn">logical_nulls</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct datafusion::common::arrow::buffer::NullBuffer">NullBuffer</a>\>

Returns a potentially computed [`NullBuffer`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.NullBuffer.html "struct datafusion::common::arrow::buffer::NullBuffer") that represents the logical null values of this array, if any. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.logical_nulls)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html#method.is_nullable" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.is_nullable" class="fn">is_nullable</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns `false` if the array is guaranteed to not contain any logical nulls [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.is_nullable)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html#method.logical_null_count" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.logical_null_count" class="fn">logical_null_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of logical null values in this array. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.logical_null_count)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html#method.get_buffer_memory_size" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.get_buffer_memory_size" class="fn">get_buffer_memory_size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of bytes of memory pointed to by this array. The buffers store bytes in the Arrow memory format, and include the data as well as the validity map. Note that this does not always correspond to the exact memory usage of an array, since multiple arrays can share the same buffers or slices thereof.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html#method.get_array_memory_size" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.get_array_memory_size" class="fn">get_array_memory_size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of bytes of memory occupied physically by this array. This value will always be greater than returned by `get_buffer_memory_size()` and includes the overhead of the data structures that contain the pointers to the various buffers.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html#method.shrink_to_fit" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.shrink_to_fit" class="fn">shrink_to_fit</a>(&mut self)

Shrinks the capacity of any exclusively owned buffer as much as possible [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.shrink_to_fit)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html#method.is_null" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.is_null" class="fn">is_null</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether the element at `index` is null according to [`Array::nulls`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.nulls "method datafusion::common::arrow::array::Array::nulls") [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.is_null)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html#method.is_valid" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.is_valid" class="fn">is_valid</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether the element at `index` is *not* null, the opposite of [`Self::is_null`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.is_null "method arrow_array::array::Array::is_null::is_null"). [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.is_valid)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html#method.null_count" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.null_count" class="fn">null_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of physical null values in this array. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.null_count)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html#impl-Clone-for-NullArray" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html" class="struct" title="struct datafusion::common::arrow::array::NullArray">NullArray</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html" class="struct" title="struct datafusion::common::arrow::array::NullArray">NullArray</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html#impl-Debug-for-NullArray" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html" class="struct" title="struct datafusion::common::arrow::array::NullArray">NullArray</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html#impl-From%3CArrayData%3E-for-NullArray" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.ArrayData.html" class="struct" title="struct datafusion::common::arrow::array::ArrayData">ArrayData</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html" class="struct" title="struct datafusion::common::arrow::array::NullArray">NullArray</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(data: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.ArrayData.html" class="struct" title="struct datafusion::common::arrow::array::ArrayData">ArrayData</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html" class="struct" title="struct datafusion::common::arrow::array::NullArray">NullArray</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html#impl-From%3CNullArray%3E-for-ArrayData" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html" class="struct" title="struct datafusion::common::arrow::array::NullArray">NullArray</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.ArrayData.html" class="struct" title="struct datafusion::common::arrow::array::ArrayData">ArrayData</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(array: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html" class="struct" title="struct datafusion::common::arrow::array::NullArray">NullArray</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.ArrayData.html" class="struct" title="struct datafusion::common::arrow::array::ArrayData">ArrayData</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html#impl-PartialEq-for-NullArray" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html" class="struct" title="struct datafusion::common::arrow::array::NullArray">NullArray</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html" class="struct" title="struct datafusion::common::arrow::array::NullArray">NullArray</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html#blanket-implementations" class="anchor">§</a>
