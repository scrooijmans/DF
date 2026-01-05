# Struct UnionArray Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/array/union_array.rs.html#123" class="src">Source</a>

``` rust
pub struct UnionArray { /* private fields */ }
```

Expand description

An array of [values of varying types](https://arrow.apache.org/docs/format/Columnar.html#union-layout)

Each slot in a [UnionArray](https://docs.rs/arrow/latest/arrow/array/struct.UnionArray.html "struct arrow::array::UnionArray") can have a value chosen from a number of types. Each of the possible types are named like the fields of a [`StructArray`](https://docs.rs/arrow/latest/arrow/array/struct.StructArray.html "struct arrow::array::StructArray"). A `UnionArray` can have two possible memory layouts, “dense” or “sparse”. For more information on please see the [specification](https://arrow.apache.org/docs/format/Columnar.html#union-layout).

[UnionBuilder](https://docs.rs/arrow/latest/arrow/array/struct.UnionBuilder.html "struct arrow::array::UnionBuilder") can be used to create [UnionArray](https://docs.rs/arrow/latest/arrow/array/struct.UnionArray.html "struct arrow::array::UnionArray")’s of primitive types. `UnionArray`’s of nested types are also supported but not via `UnionBuilder`, see the tests for examples.

## <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.UnionArray.html#examples" class="doc-anchor">§</a>Examples

### <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.UnionArray.html#create-a-dense-unionarray-1-32-34" class="doc-anchor">§</a>Create a dense UnionArray `[1, 3.2, 34]`

``` rust
use arrow_buffer::ScalarBuffer;
use arrow_schema::*;
use std::sync::Arc;
use arrow_array::{Array, Int32Array, Float64Array, UnionArray};

let int_array = Int32Array::from(vec![1, 34]);
let float_array = Float64Array::from(vec![3.2]);
let type_ids = [0, 1, 0].into_iter().collect::<ScalarBuffer<i8>>();
let offsets = [0, 0, 1].into_iter().collect::<ScalarBuffer<i32>>();

let union_fields = [
    (0, Arc::new(Field::new("A", DataType::Int32, false))),
    (1, Arc::new(Field::new("B", DataType::Float64, false))),
].into_iter().collect::<UnionFields>();

let children = vec![
    Arc::new(int_array) as Arc<dyn Array>,
    Arc::new(float_array),
];

let array = UnionArray::try_new(
    union_fields,
    type_ids,
    Some(offsets),
    children,
).unwrap();

let value = array.value(0).as_any().downcast_ref::<Int32Array>().unwrap().value(0);
assert_eq!(1, value);

let value = array.value(1).as_any().downcast_ref::<Float64Array>().unwrap().value(0);
assert!(3.2 - value < f64::EPSILON);

let value = array.value(2).as_any().downcast_ref::<Int32Array>().unwrap().value(0);
assert_eq!(34, value);
```

### <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.UnionArray.html#create-a-sparse-unionarray-1-32-34" class="doc-anchor">§</a>Create a sparse UnionArray `[1, 3.2, 34]`

``` rust
use arrow_buffer::ScalarBuffer;
use arrow_schema::*;
use std::sync::Arc;
use arrow_array::{Array, Int32Array, Float64Array, UnionArray};

let int_array = Int32Array::from(vec![Some(1), None, Some(34)]);
let float_array = Float64Array::from(vec![None, Some(3.2), None]);
let type_ids = [0_i8, 1, 0].into_iter().collect::<ScalarBuffer<i8>>();

let union_fields = [
    (0, Arc::new(Field::new("A", DataType::Int32, false))),
    (1, Arc::new(Field::new("B", DataType::Float64, false))),
].into_iter().collect::<UnionFields>();

let children = vec![
    Arc::new(int_array) as Arc<dyn Array>,
    Arc::new(float_array),
];

let array = UnionArray::try_new(
    union_fields,
    type_ids,
    None,
    children,
).unwrap();

let value = array.value(0).as_any().downcast_ref::<Int32Array>().unwrap().value(0);
assert_eq!(1, value);

let value = array.value(1).as_any().downcast_ref::<Float64Array>().unwrap().value(0);
assert!(3.2 - value < f64::EPSILON);

let value = array.value(2).as_any().downcast_ref::<Int32Array>().unwrap().value(0);
assert_eq!(34, value);
```

## Implementations<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.UnionArray.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.UnionArray.html#impl-UnionArray" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/struct.UnionArray.html" class="struct" title="struct arrow::array::UnionArray">UnionArray</a>

#### pub unsafe fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.UnionArray.html#method.new_unchecked" class="fn">new_unchecked</a>( fields: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.UnionFields.html" class="struct" title="struct arrow::datatypes::UnionFields">UnionFields</a>, type_ids: <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.ScalarBuffer.html" class="struct" title="struct arrow::buffer::ScalarBuffer">ScalarBuffer</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>, offsets: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.ScalarBuffer.html" class="struct" title="struct arrow::buffer::ScalarBuffer">ScalarBuffer</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>\>, children: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.UnionArray.html" class="struct" title="struct arrow::array::UnionArray">UnionArray</a>

Creates a new `UnionArray`.

Accepts type ids, child arrays and optionally offsets (for dense unions) to create a new `UnionArray`. This method makes no attempt to validate the data provided by the caller and assumes that each of the components are correct and consistent with each other. See `try_new` for an alternative that validates the data provided.

##### <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.UnionArray.html#safety" class="doc-anchor">§</a>Safety

The `type_ids` values should be positive and must match one of the type ids of the fields provided in `fields`. These values are used to index into the `children` arrays.

The `offsets` is provided in the case of a dense union, sparse unions should use `None`. If provided the `offsets` values should be positive and must be less than the length of the corresponding array.

In both cases above we use signed integer types to maintain compatibility with other Arrow implementations.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.UnionArray.html#method.try_new" class="fn">try_new</a>( fields: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.UnionFields.html" class="struct" title="struct arrow::datatypes::UnionFields">UnionFields</a>, type_ids: <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.ScalarBuffer.html" class="struct" title="struct arrow::buffer::ScalarBuffer">ScalarBuffer</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>, offsets: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.ScalarBuffer.html" class="struct" title="struct arrow::buffer::ScalarBuffer">ScalarBuffer</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>\>, children: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.UnionArray.html" class="struct" title="struct arrow::array::UnionArray">UnionArray</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Attempts to create a new `UnionArray`, validating the inputs provided.

The order of child arrays child array order must match the fields order

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.UnionArray.html#method.child" class="fn">child</a>(&self, type_id: <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

Accesses the child array for `type_id`.

##### <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.UnionArray.html#panics" class="doc-anchor">§</a>Panics

Panics if the `type_id` provided is not present in the array’s DataType in the `Union`.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.UnionArray.html#method.type_id" class="fn">type_id</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>

Returns the `type_id` for the array slot at `index`.

##### <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.UnionArray.html#panics-1" class="doc-anchor">§</a>Panics

Panics if `index` is greater than or equal to the number of child arrays

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.UnionArray.html#method.type_ids" class="fn">type_ids</a>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.ScalarBuffer.html" class="struct" title="struct arrow::buffer::ScalarBuffer">ScalarBuffer</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>

Returns the `type_ids` buffer for this array

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.UnionArray.html#method.offsets" class="fn">offsets</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.ScalarBuffer.html" class="struct" title="struct arrow::buffer::ScalarBuffer">ScalarBuffer</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>\>

Returns the `offsets` buffer if this is a dense array

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.UnionArray.html#method.value_offset" class="fn">value_offset</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the offset into the underlying values array for the array slot at `index`.

##### <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.UnionArray.html#panics-2" class="doc-anchor">§</a>Panics

Panics if `index` is greater than or equal the length of the array.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.UnionArray.html#method.value" class="fn">value</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

Returns the array’s value at index `i`.

Note: This method does not check for nulls and the value is arbitrary (but still well-defined) if [`is_null`](https://docs.rs/arrow/latest/arrow/array/struct.UnionArray.html#method.is_null "method arrow::array::UnionArray::is_null") returns true for the index.

##### <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.UnionArray.html#panics-3" class="doc-anchor">§</a>Panics

Panics if index `i` is out of bounds

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.UnionArray.html#method.type_names" class="fn">type_names</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Returns the names of the types in the union.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.UnionArray.html#method.slice" class="fn">slice</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.UnionArray.html" class="struct" title="struct arrow::array::UnionArray">UnionArray</a>

Returns a zero-copy slice of this array with the indicated offset and length.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.UnionArray.html#method.into_parts" class="fn">into_parts</a>( self, ) -\> (<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.UnionFields.html" class="struct" title="struct arrow::datatypes::UnionFields">UnionFields</a>, <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.ScalarBuffer.html" class="struct" title="struct arrow::buffer::ScalarBuffer">ScalarBuffer</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.ScalarBuffer.html" class="struct" title="struct arrow::buffer::ScalarBuffer">ScalarBuffer</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>\>, <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>\>)

Deconstruct this array into its constituent parts

##### <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.UnionArray.html#example" class="doc-anchor">§</a>Example

``` rust
let mut builder = UnionBuilder::new_dense();
builder.append::<Int32Type>("a", 1).unwrap();
let union_array = builder.build()?;

// Deconstruct into parts
let (union_fields, type_ids, offsets, children) = union_array.into_parts();

// Reconstruct from parts
let union_array = UnionArray::try_new(
    union_fields,
    type_ids,
    offsets,
    children,
);
```

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.UnionArray.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.UnionArray.html#impl-Array-for-UnionArray" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.UnionArray.html" class="struct" title="struct arrow::array::UnionArray">UnionArray</a>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.UnionArray.html#method.as_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the array as [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") so that it can be downcasted to a specific implementation. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.as_any)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.UnionArray.html#method.to_data" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.to_data" class="fn">to_data</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

Returns the underlying data of this array

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.UnionArray.html#method.into_data" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.into_data" class="fn">into_data</a>(self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

Returns the underlying data of this array [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.into_data)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.UnionArray.html#method.data_type" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.data_type" class="fn">data_type</a>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a>

Returns a reference to the [`DataType`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html "enum arrow::datatypes::DataType") of this array. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.data_type)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.UnionArray.html#method.slice-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.slice" class="fn">slice</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

Returns a zero-copy slice of this array with the indicated offset and length. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.slice)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.UnionArray.html#method.len" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the length (i.e., number of elements) of this array. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.len)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.UnionArray.html#method.is_empty" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether this array is empty. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.is_empty)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.UnionArray.html#method.shrink_to_fit" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.shrink_to_fit" class="fn">shrink_to_fit</a>(&mut self)

Shrinks the capacity of any exclusively owned buffer as much as possible [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.shrink_to_fit)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.UnionArray.html#method.offset" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.offset" class="fn">offset</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the offset into the underlying data used by this array(-slice). Note that the underlying data can be shared by many arrays. This defaults to `0`. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.offset)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.UnionArray.html#method.nulls" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.nulls" class="fn">nulls</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>\>

Returns the null buffer of this array if any. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.nulls)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.UnionArray.html#method.logical_nulls" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.logical_nulls" class="fn">logical_nulls</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>\>

Returns a potentially computed [`NullBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html "struct arrow::buffer::NullBuffer") that represents the logical null values of this array, if any. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.logical_nulls)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.UnionArray.html#method.is_nullable" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_nullable" class="fn">is_nullable</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns `false` if the array is guaranteed to not contain any logical nulls [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_nullable)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.UnionArray.html#method.get_buffer_memory_size" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.get_buffer_memory_size" class="fn">get_buffer_memory_size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of bytes of memory pointed to by this array. The buffers store bytes in the Arrow memory format, and include the data as well as the validity map. Note that this does not always correspond to the exact memory usage of an array, since multiple arrays can share the same buffers or slices thereof.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.UnionArray.html#method.get_array_memory_size" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.get_array_memory_size" class="fn">get_array_memory_size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of bytes of memory occupied physically by this array. This value will always be greater than returned by `get_buffer_memory_size()` and includes the overhead of the data structures that contain the pointers to the various buffers.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.UnionArray.html#method.is_null" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_null" class="fn">is_null</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether the element at `index` is null according to [`Array::nulls`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.nulls "method arrow::array::Array::nulls") [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_null)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.UnionArray.html#method.is_valid" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_valid" class="fn">is_valid</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether the element at `index` is *not* null, the opposite of [`Self::is_null`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_null "method arrow_array::array::Array::is_null::is_null"). [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_valid)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.UnionArray.html#method.null_count" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.null_count" class="fn">null_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of physical null values in this array. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.null_count)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.UnionArray.html#method.logical_null_count" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.logical_null_count" class="fn">logical_null_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of logical null values in this array. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.logical_null_count)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.UnionArray.html#impl-Clone-for-UnionArray" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.UnionArray.html" class="struct" title="struct arrow::array::UnionArray">UnionArray</a>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.UnionArray.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.UnionArray.html" class="struct" title="struct arrow::array::UnionArray">UnionArray</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/array/array/struct.UnionArray.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.UnionArray.html#impl-Debug-for-UnionArray" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.UnionArray.html" class="struct" title="struct arrow::array::UnionArray">UnionArray</a>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.UnionArray.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.UnionArray.html#impl-From%3CArrayData%3E-for-UnionArray" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.UnionArray.html" class="struct" title="struct arrow::array::UnionArray">UnionArray</a>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.UnionArray.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(data: <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.UnionArray.html" class="struct" title="struct arrow::array::UnionArray">UnionArray</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.UnionArray.html#impl-From%3CUnionArray%3E-for-ArrayData" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.UnionArray.html" class="struct" title="struct arrow::array::UnionArray">UnionArray</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.UnionArray.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(array: <a href="https://docs.rs/arrow/latest/arrow/array/struct.UnionArray.html" class="struct" title="struct arrow::array::UnionArray">UnionArray</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

Converts to this type from the input type.

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.UnionArray.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.UnionArray.html#blanket-implementations" class="anchor">§</a>
