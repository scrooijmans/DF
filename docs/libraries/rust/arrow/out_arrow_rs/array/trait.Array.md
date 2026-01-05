# Trait Array Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/array/mod.rs.html#82" class="src">Source</a>

``` rust
pub trait Array:
    Debug
    + Send
    + Sync {
Show 18 methods    // Required methods
    fn as_any(&self) -> &(dyn Any + 'static);
    fn to_data(&self) -> ArrayData;
    fn into_data(self) -> ArrayData;
    fn data_type(&self) -> &DataType;
    fn slice(&self, offset: usize, length: usize) -> Arc<dyn Array>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn offset(&self) -> usize;
    fn nulls(&self) -> Option<&NullBuffer>;
    fn get_buffer_memory_size(&self) -> usize;
    fn get_array_memory_size(&self) -> usize;

    // Provided methods
    fn shrink_to_fit(&mut self) { ... }
    fn logical_nulls(&self) -> Option<NullBuffer> { ... }
    fn is_null(&self, index: usize) -> bool { ... }
    fn is_valid(&self, index: usize) -> bool { ... }
    fn null_count(&self) -> usize { ... }
    fn logical_null_count(&self) -> usize { ... }
    fn is_nullable(&self) -> bool { ... }
}
```

Expand description

An array in the [arrow columnar format](https://arrow.apache.org/docs/format/Columnar.html)

## Required Methods<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the array as [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") so that it can be downcasted to a specific implementation.

##### <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#example" class="doc-anchor">§</a>Example:

``` rust

let id = Int32Array::from(vec![1, 2, 3, 4, 5]);
let batch = RecordBatch::try_new(
    Arc::new(Schema::new(vec![Field::new("id", DataType::Int32, false)])),
    vec![Arc::new(id)]
).unwrap();

let int32array = batch
    .column(0)
    .as_any()
    .downcast_ref::<Int32Array>()
    .expect("Failed to downcast");
```

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.to_data" class="fn">to_data</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

Returns the underlying data of this array

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.into_data" class="fn">into_data</a>(self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

Returns the underlying data of this array

Unlike [`Array::to_data`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.to_data "method arrow::array::Array::to_data") this consumes self, allowing it avoid unnecessary clones

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.data_type" class="fn">data_type</a>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a>

Returns a reference to the [`DataType`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html "enum arrow::datatypes::DataType") of this array.

##### <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#example-1" class="doc-anchor">§</a>Example:

``` rust
use arrow_schema::DataType;
use arrow_array::{Array, Int32Array};

let array = Int32Array::from(vec![1, 2, 3, 4, 5]);

assert_eq!(*array.data_type(), DataType::Int32);
```

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.slice" class="fn">slice</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

Returns a zero-copy slice of this array with the indicated offset and length.

##### <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#example-2" class="doc-anchor">§</a>Example:

``` rust
use arrow_array::{Array, Int32Array};

let array = Int32Array::from(vec![1, 2, 3, 4, 5]);
// Make slice over the values [2, 3, 4]
let array_slice = array.slice(1, 3);

assert_eq!(&array_slice, &Int32Array::from(vec![2, 3, 4]));
```

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the length (i.e., number of elements) of this array.

##### <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#example-3" class="doc-anchor">§</a>Example:

``` rust
use arrow_array::{Array, Int32Array};

let array = Int32Array::from(vec![1, 2, 3, 4, 5]);

assert_eq!(array.len(), 5);
```

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether this array is empty.

##### <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#example-4" class="doc-anchor">§</a>Example:

``` rust
use arrow_array::{Array, Int32Array};

let array = Int32Array::from(vec![1, 2, 3, 4, 5]);

assert_eq!(array.is_empty(), false);
```

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.offset" class="fn">offset</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the offset into the underlying data used by this array(-slice). Note that the underlying data can be shared by many arrays. This defaults to `0`.

##### <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#example-5" class="doc-anchor">§</a>Example:

``` rust
use arrow_array::{Array, BooleanArray};

let array = BooleanArray::from(vec![false, false, true, true]);
let array_slice = array.slice(1, 3);

assert_eq!(array.offset(), 0);
assert_eq!(array_slice.offset(), 1);
```

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.nulls" class="fn">nulls</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>\>

Returns the null buffer of this array if any.

The null buffer contains the “physical” nulls of an array, that is how the nulls are represented in the underlying arrow format.

The physical representation is efficient, but is sometimes non intuitive for certain array types such as those with nullable child arrays like [`DictionaryArray::values`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html#method.values "method arrow::array::DictionaryArray::values"), [`RunArray::values`](https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html#method.values "method arrow::array::RunArray::values") or [`UnionArray`](https://docs.rs/arrow/latest/arrow/array/struct.UnionArray.html "struct arrow::array::UnionArray"), or without a null buffer, such as [`NullArray`](https://docs.rs/arrow/latest/arrow/array/struct.NullArray.html "struct arrow::array::NullArray").

To determine if each element of such an array is “logically” null, use the slower [`Array::logical_nulls`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.logical_nulls "method arrow::array::Array::logical_nulls") to obtain a computed mask.

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.get_buffer_memory_size" class="fn">get_buffer_memory_size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of bytes of memory pointed to by this array. The buffers store bytes in the Arrow memory format, and include the data as well as the validity map. Note that this does not always correspond to the exact memory usage of an array, since multiple arrays can share the same buffers or slices thereof.

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.get_array_memory_size" class="fn">get_array_memory_size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of bytes of memory occupied physically by this array. This value will always be greater than returned by `get_buffer_memory_size()` and includes the overhead of the data structures that contain the pointers to the various buffers.

## Provided Methods<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.shrink_to_fit" class="fn">shrink_to_fit</a>(&mut self)

Shrinks the capacity of any exclusively owned buffer as much as possible

Shared or externally allocated buffers will be ignored, and any buffer offsets will be preserved.

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.logical_nulls" class="fn">logical_nulls</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>\>

Returns a potentially computed [`NullBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html "struct arrow::buffer::NullBuffer") that represents the logical null values of this array, if any.

Logical nulls represent the values that are null in the array, regardless of the underlying physical arrow representation.

For most array types, this is equivalent to the “physical” nulls returned by [`Array::nulls`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.nulls "method arrow::array::Array::nulls"). It is different for the following cases, because which elements are null is not encoded in a single null buffer:

- [`DictionaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html "struct arrow::array::DictionaryArray") where [`DictionaryArray::values`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html#method.values "method arrow::array::DictionaryArray::values") contains nulls
- [`RunArray`](https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html "struct arrow::array::RunArray") where [`RunArray::values`](https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html#method.values "method arrow::array::RunArray::values") contains nulls
- [`NullArray`](https://docs.rs/arrow/latest/arrow/array/struct.NullArray.html "struct arrow::array::NullArray") where all indices are nulls
- [`UnionArray`](https://docs.rs/arrow/latest/arrow/array/struct.UnionArray.html "struct arrow::array::UnionArray") where the selected values contains nulls

In these cases a logical [`NullBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html "struct arrow::buffer::NullBuffer") will be computed, encoding the logical nullability of these arrays, beyond what is encoded in [`Array::nulls`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.nulls "method arrow::array::Array::nulls")

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_null" class="fn">is_null</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether the element at `index` is null according to [`Array::nulls`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.nulls "method arrow::array::Array::nulls")

Note: For performance reasons, this method returns nullability solely as determined by the null buffer. This difference can lead to surprising results, for example, [`NullArray::is_null`](https://docs.rs/arrow/latest/arrow/array/struct.NullArray.html#method.is_null "method arrow::array::NullArray::is_null") always returns `false` as the array lacks a null buffer. Similarly [`DictionaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html "struct arrow::array::DictionaryArray"), [`RunArray`](https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html "struct arrow::array::RunArray") and [`UnionArray`](https://docs.rs/arrow/latest/arrow/array/struct.UnionArray.html "struct arrow::array::UnionArray") may encode nullability in their children. See [`Self::logical_nulls`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.logical_nulls "method arrow_array::array::Array::logical_nulls::logical_nulls") for more information.

##### <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#example-6" class="doc-anchor">§</a>Example:

``` rust
use arrow_array::{Array, Int32Array, NullArray};

let array = Int32Array::from(vec![Some(1), None]);
assert_eq!(array.is_null(0), false);
assert_eq!(array.is_null(1), true);

// NullArrays do not have a null buffer, and therefore always
// return false for is_null.
let array = NullArray::new(1);
assert_eq!(array.is_null(0), false);
```

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_valid" class="fn">is_valid</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether the element at `index` is *not* null, the opposite of [`Self::is_null`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_null "method arrow_array::array::Array::is_null::is_null").

##### <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#example-7" class="doc-anchor">§</a>Example:

``` rust
use arrow_array::{Array, Int32Array};

let array = Int32Array::from(vec![Some(1), None]);

assert_eq!(array.is_valid(0), true);
assert_eq!(array.is_valid(1), false);
```

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.null_count" class="fn">null_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of physical null values in this array.

Note: this method returns the physical null count, i.e. that encoded in [`Array::nulls`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.nulls "method arrow::array::Array::nulls"), see [`Array::logical_nulls`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.logical_nulls "method arrow::array::Array::logical_nulls") for logical nullability

##### <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#example-8" class="doc-anchor">§</a>Example:

``` rust
use arrow_array::{Array, Int32Array};

// Construct an array with values [1, NULL, NULL]
let array = Int32Array::from(vec![Some(1), None, None]);

assert_eq!(array.null_count(), 2);
```

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.logical_null_count" class="fn">logical_null_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of logical null values in this array.

Note: this method returns the logical null count, i.e. that encoded in [`Array::logical_nulls`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.logical_nulls "method arrow::array::Array::logical_nulls"). In general this is equivalent to [`Array::null_count`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.null_count "method arrow::array::Array::null_count") but may differ in the presence of logical nullability, see [`Array::nulls`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.nulls "method arrow::array::Array::nulls") and [`Array::logical_nulls`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.logical_nulls "method arrow::array::Array::logical_nulls").

##### <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#example-9" class="doc-anchor">§</a>Example:

``` rust
use arrow_array::{Array, Int32Array};

// Construct an array with values [1, NULL, NULL]
let array = Int32Array::from(vec![Some(1), None, None]);

assert_eq!(array.logical_null_count(), 2);
```

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_nullable" class="fn">is_nullable</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns `false` if the array is guaranteed to not contain any logical nulls

This is generally equivalent to `Array::logical_null_count() != 0` unless determining the logical nulls is expensive, in which case this method can return true even for an array without nulls.

This is also generally equivalent to `Array::null_count() != 0` but may differ in the presence of logical nullability, see [`Array::logical_null_count`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.logical_null_count "method arrow::array::Array::logical_null_count") and [`Array::null_count`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.null_count "method arrow::array::Array::null_count").

Implementations will return `true` unless they can cheaply prove no logical nulls are present. For example a [`DictionaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html "struct arrow::array::DictionaryArray") with nullable values will still return true, even if the nulls present in [`DictionaryArray::values`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html#method.values "method arrow::array::DictionaryArray::values") are not referenced by any key, and therefore would not appear in [`Array::logical_nulls`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.logical_nulls "method arrow::array::Array::logical_nulls").

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#impl-AsArray-for-dyn+Array" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html" class="trait" title="trait arrow::array::AsArray">AsArray</a> for dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a> + '\_

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.as_boolean_opt" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#tymethod.as_boolean_opt" class="fn">as_boolean_opt</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.BooleanArray.html" class="struct" title="struct arrow::array::BooleanArray">BooleanArray</a>\>

Downcast this to a [`BooleanArray`](https://docs.rs/arrow/latest/arrow/array/struct.BooleanArray.html "struct arrow::array::BooleanArray") returning `None` if not possible

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.as_primitive_opt" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#tymethod.as_primitive_opt" class="fn">as_primitive_opt</a>\<T\>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>\>

where T: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>,

Downcast this to a [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") returning `None` if not possible

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.as_bytes_opt" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#tymethod.as_bytes_opt" class="fn">as_bytes_opt</a>\<T\>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteArray.html" class="struct" title="struct arrow::array::GenericByteArray">GenericByteArray</a>\<T\>\>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait arrow::datatypes::ByteArrayType">ByteArrayType</a>,

Downcast this to a [`GenericByteArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteArray.html "struct arrow::array::GenericByteArray") returning `None` if not possible

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.as_byte_view_opt" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#tymethod.as_byte_view_opt" class="fn">as_byte_view_opt</a>\<T\>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<T\>\>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html" class="trait" title="trait arrow::datatypes::ByteViewType">ByteViewType</a>,

Downcast this to a [`GenericByteViewArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewArray.html "struct arrow::array::GenericByteViewArray") returning `None` if not possible

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.as_struct_opt" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#tymethod.as_struct_opt" class="fn">as_struct_opt</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.StructArray.html" class="struct" title="struct arrow::array::StructArray">StructArray</a>\>

Downcast this to a [`StructArray`](https://docs.rs/arrow/latest/arrow/array/struct.StructArray.html "struct arrow::array::StructArray") returning `None` if not possible

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.as_union_opt" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#tymethod.as_union_opt" class="fn">as_union_opt</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.UnionArray.html" class="struct" title="struct arrow::array::UnionArray">UnionArray</a>\>

Downcast this to a [`UnionArray`](https://docs.rs/arrow/latest/arrow/array/struct.UnionArray.html "struct arrow::array::UnionArray") returning `None` if not possible

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.as_list_opt" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#tymethod.as_list_opt" class="fn">as_list_opt</a>\<O\>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html" class="struct" title="struct arrow::array::GenericListArray">GenericListArray</a>\<O\>\>

where O: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

Downcast this to a [`GenericListArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html "struct arrow::array::GenericListArray") returning `None` if not possible

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.as_list_view_opt" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#tymethod.as_list_view_opt" class="fn">as_list_view_opt</a>\<O\>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListViewArray.html" class="struct" title="struct arrow::array::GenericListViewArray">GenericListViewArray</a>\<O\>\>

where O: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

Downcast this to a [`GenericListViewArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListViewArray.html "struct arrow::array::GenericListViewArray") returning `None` if not possible

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.as_fixed_size_binary_opt" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#tymethod.as_fixed_size_binary_opt" class="fn">as_fixed_size_binary_opt</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html" class="struct" title="struct arrow::array::FixedSizeBinaryArray">FixedSizeBinaryArray</a>\>

Downcast this to a [`FixedSizeBinaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html "struct arrow::array::FixedSizeBinaryArray") returning `None` if not possible

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.as_fixed_size_list_opt" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#tymethod.as_fixed_size_list_opt" class="fn">as_fixed_size_list_opt</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListArray.html" class="struct" title="struct arrow::array::FixedSizeListArray">FixedSizeListArray</a>\>

Downcast this to a [`FixedSizeListArray`](https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListArray.html "struct arrow::array::FixedSizeListArray") returning `None` if not possible

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.as_map_opt" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#tymethod.as_map_opt" class="fn">as_map_opt</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.MapArray.html" class="struct" title="struct arrow::array::MapArray">MapArray</a>\>

Downcast this to a [`MapArray`](https://docs.rs/arrow/latest/arrow/array/struct.MapArray.html "struct arrow::array::MapArray") returning `None` if not possible

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.as_dictionary_opt" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#tymethod.as_dictionary_opt" class="fn">as_dictionary_opt</a>\<K\>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html" class="struct" title="struct arrow::array::DictionaryArray">DictionaryArray</a>\<K\>\>

where K: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowDictionaryKeyType.html" class="trait" title="trait arrow::datatypes::ArrowDictionaryKeyType">ArrowDictionaryKeyType</a>,

Downcast this to a [`DictionaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html "struct arrow::array::DictionaryArray") returning `None` if not possible

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.as_run_opt" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#tymethod.as_run_opt" class="fn">as_run_opt</a>\<K\>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html" class="struct" title="struct arrow::array::RunArray">RunArray</a>\<K\>\>

where K: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.RunEndIndexType.html" class="trait" title="trait arrow::datatypes::RunEndIndexType">RunEndIndexType</a>,

Downcast this to a [`RunArray`](https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html "struct arrow::array::RunArray") returning `None` if not possible

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.as_any_dictionary_opt" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#tymethod.as_any_dictionary_opt" class="fn">as_any_dictionary_opt</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AnyDictionaryArray.html" class="trait" title="trait arrow::array::AnyDictionaryArray">AnyDictionaryArray</a>\>

Downcasts this to a [`AnyDictionaryArray`](https://docs.rs/arrow/latest/arrow/array/trait.AnyDictionaryArray.html "trait arrow::array::AnyDictionaryArray") returning `None` if not possible

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.as_boolean" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_boolean" class="fn">as_boolean</a>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/array/struct.BooleanArray.html" class="struct" title="struct arrow::array::BooleanArray">BooleanArray</a>

Downcast this to a [`BooleanArray`](https://docs.rs/arrow/latest/arrow/array/struct.BooleanArray.html "struct arrow::array::BooleanArray") panicking if not possible

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.as_primitive" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_primitive" class="fn">as_primitive</a>\<T\>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>,

Downcast this to a [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") panicking if not possible

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.as_bytes" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_bytes" class="fn">as_bytes</a>\<T\>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteArray.html" class="struct" title="struct arrow::array::GenericByteArray">GenericByteArray</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait arrow::datatypes::ByteArrayType">ByteArrayType</a>,

Downcast this to a [`GenericByteArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteArray.html "struct arrow::array::GenericByteArray") panicking if not possible

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.as_string_opt" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_string_opt" class="fn">as_string_opt</a>\<O\>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteArray.html" class="struct" title="struct arrow::array::GenericByteArray">GenericByteArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.GenericStringType.html" class="struct" title="struct arrow::datatypes::GenericStringType">GenericStringType</a>\<O\>\>\>

where O: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

Downcast this to a [`GenericStringArray`](https://docs.rs/arrow/latest/arrow/array/type.GenericStringArray.html "type arrow::array::GenericStringArray") returning `None` if not possible

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.as_string" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_string" class="fn">as_string</a>\<O\>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteArray.html" class="struct" title="struct arrow::array::GenericByteArray">GenericByteArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.GenericStringType.html" class="struct" title="struct arrow::datatypes::GenericStringType">GenericStringType</a>\<O\>\>

where O: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

Downcast this to a [`GenericStringArray`](https://docs.rs/arrow/latest/arrow/array/type.GenericStringArray.html "type arrow::array::GenericStringArray") panicking if not possible

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.as_binary_opt" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_binary_opt" class="fn">as_binary_opt</a>\<O\>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteArray.html" class="struct" title="struct arrow::array::GenericByteArray">GenericByteArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.GenericBinaryType.html" class="struct" title="struct arrow::datatypes::GenericBinaryType">GenericBinaryType</a>\<O\>\>\>

where O: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

Downcast this to a [`GenericBinaryArray`](https://docs.rs/arrow/latest/arrow/array/type.GenericBinaryArray.html "type arrow::array::GenericBinaryArray") returning `None` if not possible

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.as_binary" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_binary" class="fn">as_binary</a>\<O\>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteArray.html" class="struct" title="struct arrow::array::GenericByteArray">GenericByteArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.GenericBinaryType.html" class="struct" title="struct arrow::datatypes::GenericBinaryType">GenericBinaryType</a>\<O\>\>

where O: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

Downcast this to a [`GenericBinaryArray`](https://docs.rs/arrow/latest/arrow/array/type.GenericBinaryArray.html "type arrow::array::GenericBinaryArray") panicking if not possible

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.as_string_view_opt" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_string_view_opt" class="fn">as_string_view_opt</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.StringViewType.html" class="struct" title="struct arrow::datatypes::StringViewType">StringViewType</a>\>\>

Downcast this to a [`StringViewArray`](https://docs.rs/arrow/latest/arrow/array/type.StringViewArray.html "type arrow::array::StringViewArray") returning `None` if not possible

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.as_string_view" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_string_view" class="fn">as_string_view</a>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.StringViewType.html" class="struct" title="struct arrow::datatypes::StringViewType">StringViewType</a>\>

Downcast this to a [`StringViewArray`](https://docs.rs/arrow/latest/arrow/array/type.StringViewArray.html "type arrow::array::StringViewArray") panicking if not possible

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.as_binary_view_opt" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_binary_view_opt" class="fn">as_binary_view_opt</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.BinaryViewType.html" class="struct" title="struct arrow::datatypes::BinaryViewType">BinaryViewType</a>\>\>

Downcast this to a [`BinaryViewArray`](https://docs.rs/arrow/latest/arrow/array/type.BinaryViewArray.html "type arrow::array::BinaryViewArray") returning `None` if not possible

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.as_binary_view" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_binary_view" class="fn">as_binary_view</a>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.BinaryViewType.html" class="struct" title="struct arrow::datatypes::BinaryViewType">BinaryViewType</a>\>

Downcast this to a [`BinaryViewArray`](https://docs.rs/arrow/latest/arrow/array/type.BinaryViewArray.html "type arrow::array::BinaryViewArray") panicking if not possible

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.as_byte_view" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_byte_view" class="fn">as_byte_view</a>\<T\>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html" class="trait" title="trait arrow::datatypes::ByteViewType">ByteViewType</a>,

Downcast this to a [`GenericByteViewArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewArray.html "struct arrow::array::GenericByteViewArray") panicking if not possible

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.as_struct" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_struct" class="fn">as_struct</a>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/array/struct.StructArray.html" class="struct" title="struct arrow::array::StructArray">StructArray</a>

Downcast this to a [`StructArray`](https://docs.rs/arrow/latest/arrow/array/struct.StructArray.html "struct arrow::array::StructArray") panicking if not possible

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.as_union" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_union" class="fn">as_union</a>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/array/struct.UnionArray.html" class="struct" title="struct arrow::array::UnionArray">UnionArray</a>

Downcast this to a [`UnionArray`](https://docs.rs/arrow/latest/arrow/array/struct.UnionArray.html "struct arrow::array::UnionArray") panicking if not possible

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.as_list" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_list" class="fn">as_list</a>\<O\>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html" class="struct" title="struct arrow::array::GenericListArray">GenericListArray</a>\<O\>

where O: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

Downcast this to a [`GenericListArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html "struct arrow::array::GenericListArray") panicking if not possible

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.as_list_view" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_list_view" class="fn">as_list_view</a>\<O\>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListViewArray.html" class="struct" title="struct arrow::array::GenericListViewArray">GenericListViewArray</a>\<O\>

where O: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

Downcast this to a [`GenericListViewArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListViewArray.html "struct arrow::array::GenericListViewArray") panicking if not possible

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.as_fixed_size_binary" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_fixed_size_binary" class="fn">as_fixed_size_binary</a>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html" class="struct" title="struct arrow::array::FixedSizeBinaryArray">FixedSizeBinaryArray</a>

Downcast this to a [`FixedSizeBinaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html "struct arrow::array::FixedSizeBinaryArray") panicking if not possible

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.as_fixed_size_list" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_fixed_size_list" class="fn">as_fixed_size_list</a>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListArray.html" class="struct" title="struct arrow::array::FixedSizeListArray">FixedSizeListArray</a>

Downcast this to a [`FixedSizeListArray`](https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListArray.html "struct arrow::array::FixedSizeListArray") panicking if not possible

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.as_map" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_map" class="fn">as_map</a>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/array/struct.MapArray.html" class="struct" title="struct arrow::array::MapArray">MapArray</a>

Downcast this to a [`MapArray`](https://docs.rs/arrow/latest/arrow/array/struct.MapArray.html "struct arrow::array::MapArray") panicking if not possible

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.as_dictionary" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_dictionary" class="fn">as_dictionary</a>\<K\>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html" class="struct" title="struct arrow::array::DictionaryArray">DictionaryArray</a>\<K\>

where K: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowDictionaryKeyType.html" class="trait" title="trait arrow::datatypes::ArrowDictionaryKeyType">ArrowDictionaryKeyType</a>,

Downcast this to a [`DictionaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html "struct arrow::array::DictionaryArray") panicking if not possible

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.as_run" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_run" class="fn">as_run</a>\<K\>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html" class="struct" title="struct arrow::array::RunArray">RunArray</a>\<K\>

where K: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.RunEndIndexType.html" class="trait" title="trait arrow::datatypes::RunEndIndexType">RunEndIndexType</a>,

Downcast this to a [`RunArray`](https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html "struct arrow::array::RunArray") panicking if not possible

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.as_any_dictionary" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_any_dictionary" class="fn">as_any_dictionary</a>(&self) -\> &dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AnyDictionaryArray.html" class="trait" title="trait arrow::array::AnyDictionaryArray">AnyDictionaryArray</a>

Downcasts this to a [`AnyDictionaryArray`](https://docs.rs/arrow/latest/arrow/array/trait.AnyDictionaryArray.html "trait arrow::array::AnyDictionaryArray") panicking if not possible

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#impl-Datum-for-%26dyn+Array" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/trait.Datum.html" class="trait" title="trait arrow::array::Datum">Datum</a> for &dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.get-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Datum.html#tymethod.get" class="fn">get</a>(&self) -\> (&dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>)

Returns the value for this [`Datum`](https://docs.rs/arrow/latest/arrow/array/trait.Datum.html "trait arrow::array::Datum") and a boolean indicating if the value is scalar

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#impl-Datum-for-dyn+Array" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/trait.Datum.html" class="trait" title="trait arrow::array::Datum">Datum</a> for dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.get" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Datum.html#tymethod.get" class="fn">get</a>(&self) -\> (&dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>)

Returns the value for this [`Datum`](https://docs.rs/arrow/latest/arrow/array/trait.Datum.html "trait arrow::array::Datum") and a boolean indicating if the value is scalar

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#impl-PartialEq%3CT%3E-for-dyn+Array" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>\<T\> for dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a> + '\_

where T: <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.eq-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.ne-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#impl-PartialEq-for-dyn+Array" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a> + '\_

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

## Implementations on Foreign Types<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#impl-Array-for-Arc%3Cdyn+Array%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a> for <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

Ergonomics: Allow use of an ArrayRef as an `&dyn Array`

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.shrink_to_fit-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.shrink_to_fit" class="fn">shrink_to_fit</a>(&mut self)

For shared buffers, this is a no-op.

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.as_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.to_data" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.to_data" class="fn">to_data</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.into_data" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.into_data" class="fn">into_data</a>(self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.data_type" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.data_type" class="fn">data_type</a>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.slice" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.slice" class="fn">slice</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.len" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_empty" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.offset" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.offset" class="fn">offset</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.nulls" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.nulls" class="fn">nulls</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.logical_nulls-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.logical_nulls" class="fn">logical_nulls</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_null-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_null" class="fn">is_null</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_valid-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_valid" class="fn">is_valid</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.null_count-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.null_count" class="fn">null_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.logical_null_count-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.logical_null_count" class="fn">logical_null_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_nullable-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_nullable" class="fn">is_nullable</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.get_buffer_memory_size" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.get_buffer_memory_size" class="fn">get_buffer_memory_size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.get_array_memory_size" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.get_array_memory_size" class="fn">get_array_memory_size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#impl-Array-for-%26T" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>

where T: <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.as_any-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.to_data-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.to_data" class="fn">to_data</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.into_data-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.into_data" class="fn">into_data</a>(self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.data_type-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.data_type" class="fn">data_type</a>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.slice-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.slice" class="fn">slice</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.len-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_empty-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.offset-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.offset" class="fn">offset</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.nulls-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.nulls" class="fn">nulls</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.logical_nulls-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.logical_nulls" class="fn">logical_nulls</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_null-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_null" class="fn">is_null</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_valid-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_valid" class="fn">is_valid</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.null_count-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.null_count" class="fn">null_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.logical_null_count-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.logical_null_count" class="fn">logical_null_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_nullable-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_nullable" class="fn">is_nullable</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.get_buffer_memory_size-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.get_buffer_memory_size" class="fn">get_buffer_memory_size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.get_array_memory_size-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.get_array_memory_size" class="fn">get_array_memory_size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

## Implementors<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#impl-Array-for-BooleanArray" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.BooleanArray.html" class="struct" title="struct arrow::array::BooleanArray">BooleanArray</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#impl-Array-for-FixedSizeBinaryArray" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html" class="struct" title="struct arrow::array::FixedSizeBinaryArray">FixedSizeBinaryArray</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#impl-Array-for-FixedSizeListArray" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListArray.html" class="struct" title="struct arrow::array::FixedSizeListArray">FixedSizeListArray</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#impl-Array-for-MapArray" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.MapArray.html" class="struct" title="struct arrow::array::MapArray">MapArray</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#impl-Array-for-NullArray" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.NullArray.html" class="struct" title="struct arrow::array::NullArray">NullArray</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#impl-Array-for-StructArray" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.StructArray.html" class="struct" title="struct arrow::array::StructArray">StructArray</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#impl-Array-for-UnionArray" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.UnionArray.html" class="struct" title="struct arrow::array::UnionArray">UnionArray</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#impl-Array-for-TypedDictionaryArray%3C&#39;_,+K,+V%3E" class="anchor">§</a>

### impl\<K, V\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.TypedDictionaryArray.html" class="struct" title="struct arrow::array::TypedDictionaryArray">TypedDictionaryArray</a>\<'\_, K, V\>

where K: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowDictionaryKeyType.html" class="trait" title="trait arrow::datatypes::ArrowDictionaryKeyType">ArrowDictionaryKeyType</a>, V: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#impl-Array-for-GenericListArray%3COffsetSize%3E" class="anchor">§</a>

### impl\<OffsetSize\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html" class="struct" title="struct arrow::array::GenericListArray">GenericListArray</a>\<OffsetSize\>

where OffsetSize: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#impl-Array-for-GenericListViewArray%3COffsetSize%3E" class="anchor">§</a>

### impl\<OffsetSize\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListViewArray.html" class="struct" title="struct arrow::array::GenericListViewArray">GenericListViewArray</a>\<OffsetSize\>

where OffsetSize: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#impl-Array-for-TypedRunArray%3C&#39;_,+R,+V%3E" class="anchor">§</a>

### impl\<R, V\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.TypedRunArray.html" class="struct" title="struct arrow::array::TypedRunArray">TypedRunArray</a>\<'\_, R, V\>

where R: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.RunEndIndexType.html" class="trait" title="trait arrow::datatypes::RunEndIndexType">RunEndIndexType</a>, V: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#impl-Array-for-DictionaryArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html" class="struct" title="struct arrow::array::DictionaryArray">DictionaryArray</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowDictionaryKeyType.html" class="trait" title="trait arrow::datatypes::ArrowDictionaryKeyType">ArrowDictionaryKeyType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#impl-Array-for-GenericByteArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteArray.html" class="struct" title="struct arrow::array::GenericByteArray">GenericByteArray</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait arrow::datatypes::ByteArrayType">ByteArrayType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#impl-Array-for-GenericByteViewArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html" class="trait" title="trait arrow::datatypes::ByteViewType">ByteViewType</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#impl-Array-for-PrimitiveArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#impl-Array-for-RunArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html" class="struct" title="struct arrow::array::RunArray">RunArray</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.RunEndIndexType.html" class="trait" title="trait arrow::datatypes::RunEndIndexType">RunEndIndexType</a>,
