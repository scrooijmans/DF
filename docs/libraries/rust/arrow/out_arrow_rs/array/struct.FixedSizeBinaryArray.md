# Struct FixedSizeBinaryArray Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/array/fixed_size_binary_array.rs.html#53" class="src">Source</a>

``` rust
pub struct FixedSizeBinaryArray { /* private fields */ }
```

Expand description

An array of [fixed size binary arrays](https://arrow.apache.org/docs/format/Columnar.html#fixed-size-primitive-layout)

## <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#examples" class="doc-anchor">§</a>Examples

Create an array from an iterable argument of byte slices.

``` rust
   use arrow_array::{Array, FixedSizeBinaryArray};
   let input_arg = vec![ vec![1, 2], vec![3, 4], vec![5, 6] ];
   let arr = FixedSizeBinaryArray::try_from_iter(input_arg.into_iter()).unwrap();

   assert_eq!(3, arr.len());
```

Create an array from an iterable argument of sparse byte slices. Sparsity means that the input argument can contain `None` items.

``` rust
   use arrow_array::{Array, FixedSizeBinaryArray};
   let input_arg = vec![ None, Some(vec![7, 8]), Some(vec![9, 10]), None, Some(vec![13, 14]) ];
   let arr = FixedSizeBinaryArray::try_from_sparse_iter_with_size(input_arg.into_iter(), 2).unwrap();
   assert_eq!(5, arr.len())
```

## Implementations<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#impl-FixedSizeBinaryArray" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html" class="struct" title="struct arrow::array::FixedSizeBinaryArray">FixedSizeBinaryArray</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#method.new" class="fn">new</a>( size: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, values: <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>, nulls: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html" class="struct" title="struct arrow::array::FixedSizeBinaryArray">FixedSizeBinaryArray</a>

Create a new [`FixedSizeBinaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html "struct arrow::array::FixedSizeBinaryArray") with `size` element size, panicking on failure

##### <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#panics" class="doc-anchor">§</a>Panics

Panics if [`Self::try_new`](https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#method.try_new "associated function arrow::array::FixedSizeBinaryArray::try_new") returns an error

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#method.new_scalar" class="fn">new_scalar</a>(value: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.Scalar.html" class="struct" title="struct arrow::array::Scalar">Scalar</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html" class="struct" title="struct arrow::array::FixedSizeBinaryArray">FixedSizeBinaryArray</a>\>

Create a new [`Scalar`](https://docs.rs/arrow/latest/arrow/array/struct.Scalar.html "struct arrow::array::Scalar") from `value`

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#method.try_new" class="fn">try_new</a>( size: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, values: <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>, nulls: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html" class="struct" title="struct arrow::array::FixedSizeBinaryArray">FixedSizeBinaryArray</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Create a new [`FixedSizeBinaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html "struct arrow::array::FixedSizeBinaryArray") from the provided parts, returning an error on failure

##### <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#errors" class="doc-anchor">§</a>Errors

- `size < 0`
- `values.len() / size != nulls.len()`

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#method.new_null" class="fn">new_null</a>(size: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html" class="struct" title="struct arrow::array::FixedSizeBinaryArray">FixedSizeBinaryArray</a>

Create a new [`FixedSizeBinaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html "struct arrow::array::FixedSizeBinaryArray") of length `len` where all values are null

##### <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#panics-1" class="doc-anchor">§</a>Panics

Panics if

- `size < 0`
- `size * len` would overflow `usize`

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#method.into_parts" class="fn">into_parts</a>(self) -\> (<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>\>)

Deconstruct this array into its constituent parts

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#method.value" class="fn">value</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\] <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#" class="tooltip" data-notable-ty="&amp;[u8]">ⓘ</a>

Returns the element at index `i` as a byte slice.

Note: This method does not check for nulls and the value is arbitrary (but still well-defined) if [`is_null`](https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#method.is_null "method arrow::array::FixedSizeBinaryArray::is_null") returns true for the index.

##### <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#panics-2" class="doc-anchor">§</a>Panics

Panics if index `i` is out of bounds.

#### pub unsafe fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#method.value_unchecked" class="fn">value_unchecked</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\] <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#" class="tooltip" data-notable-ty="&amp;[u8]">ⓘ</a>

Returns the element at index `i` as a byte slice.

Note: This method does not check for nulls and the value is arbitrary if [`is_null`](https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#method.is_null "method arrow::array::FixedSizeBinaryArray::is_null") returns true for the index.

##### <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#safety" class="doc-anchor">§</a>Safety

Caller is responsible for ensuring that the index is within the bounds of the array

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#method.value_offset" class="fn">value_offset</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

Returns the offset for the element at index `i`.

Note this doesn’t do any bound checking, for performance reason.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#method.value_length" class="fn">value_length</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

Returns the length for an element.

All elements have the same length as the array is a fixed size.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#method.values" class="fn">values</a>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

Returns the values of this array.

Unlike [`Self::value_data`](https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#method.value_data "method arrow::array::FixedSizeBinaryArray::value_data") this returns the [`Buffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html "struct arrow::buffer::Buffer") allowing for zero-copy cloning.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#method.value_data" class="fn">value_data</a>(&self) -\> &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\] <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#" class="tooltip" data-notable-ty="&amp;[u8]">ⓘ</a>

Returns the raw value data.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#method.slice" class="fn">slice</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html" class="struct" title="struct arrow::array::FixedSizeBinaryArray">FixedSizeBinaryArray</a>

Returns a zero-copy slice of this array with the indicated offset and length.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#method.try_from_sparse_iter" class="fn">try_from_sparse_iter</a>\<T, U\>( iter: T, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html" class="struct" title="struct arrow::array::FixedSizeBinaryArray">FixedSizeBinaryArray</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<U\>\>, U: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>,

👎Deprecated since 28.0.0: This function will fail if the iterator produces only None values; prefer `try_from_sparse_iter_with_size`

Create an array from an iterable argument of sparse byte slices. Sparsity means that items returned by the iterator are optional, i.e input argument can contain `None` items.

##### <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#examples-1" class="doc-anchor">§</a>Examples

``` rust
use arrow_array::FixedSizeBinaryArray;
let input_arg = vec![
    None,
    Some(vec![7, 8]),
    Some(vec![9, 10]),
    None,
    Some(vec![13, 14]),
    None,
];
let array = FixedSizeBinaryArray::try_from_sparse_iter(input_arg.into_iter()).unwrap();
```

##### <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#errors-1" class="doc-anchor">§</a>Errors

Returns error if argument has length zero, or sizes of nested slices don’t match.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#method.try_from_sparse_iter_with_size" class="fn">try_from_sparse_iter_with_size</a>\<T, U\>( iter: T, size: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html" class="struct" title="struct arrow::array::FixedSizeBinaryArray">FixedSizeBinaryArray</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<U\>\>, U: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>,

Create an array from an iterable argument of sparse byte slices. Sparsity means that items returned by the iterator are optional, i.e input argument can contain `None` items. In cases where the iterator returns only `None` values, this also takes a size parameter to ensure that the a valid FixedSizeBinaryArray is still created.

##### <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#examples-2" class="doc-anchor">§</a>Examples

``` rust
use arrow_array::FixedSizeBinaryArray;
let input_arg = vec![
    None,
    Some(vec![7, 8]),
    Some(vec![9, 10]),
    None,
    Some(vec![13, 14]),
    None,
];
let array = FixedSizeBinaryArray::try_from_sparse_iter_with_size(input_arg.into_iter(), 2).unwrap();
```

##### <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#errors-2" class="doc-anchor">§</a>Errors

Returns error if argument has length zero, or sizes of nested slices don’t match.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#method.try_from_iter" class="fn">try_from_iter</a>\<T, U\>(iter: T) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html" class="struct" title="struct arrow::array::FixedSizeBinaryArray">FixedSizeBinaryArray</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = U\>, U: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>,

Create an array from an iterable argument of byte slices.

##### <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#examples-3" class="doc-anchor">§</a>Examples

``` rust
use arrow_array::FixedSizeBinaryArray;
let input_arg = vec![
    vec![1, 2],
    vec![3, 4],
    vec![5, 6],
];
let array = FixedSizeBinaryArray::try_from_iter(input_arg.into_iter()).unwrap();
```

##### <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#errors-3" class="doc-anchor">§</a>Errors

Returns error if argument has length zero, or sizes of nested slices don’t match.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#method.iter" class="fn">iter</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayIter.html" class="struct" title="struct arrow::array::ArrayIter">ArrayIter</a>\<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html" class="struct" title="struct arrow::array::FixedSizeBinaryArray">FixedSizeBinaryArray</a>\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#" class="tooltip" data-notable-ty="ArrayIter&lt;&amp;FixedSizeBinaryArray&gt;">ⓘ</a>

constructs a new iterator

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#impl-Array-for-FixedSizeBinaryArray" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html" class="struct" title="struct arrow::array::FixedSizeBinaryArray">FixedSizeBinaryArray</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#method.as_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the array as [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") so that it can be downcasted to a specific implementation. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.as_any)

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#method.to_data" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.to_data" class="fn">to_data</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

Returns the underlying data of this array

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#method.into_data" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.into_data" class="fn">into_data</a>(self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

Returns the underlying data of this array [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.into_data)

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#method.data_type" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.data_type" class="fn">data_type</a>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a>

Returns a reference to the [`DataType`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html "enum arrow::datatypes::DataType") of this array. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.data_type)

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#method.slice-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.slice" class="fn">slice</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

Returns a zero-copy slice of this array with the indicated offset and length. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.slice)

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#method.len" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the length (i.e., number of elements) of this array. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.len)

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#method.is_empty" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether this array is empty. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.is_empty)

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#method.shrink_to_fit" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.shrink_to_fit" class="fn">shrink_to_fit</a>(&mut self)

Shrinks the capacity of any exclusively owned buffer as much as possible [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.shrink_to_fit)

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#method.offset" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.offset" class="fn">offset</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the offset into the underlying data used by this array(-slice). Note that the underlying data can be shared by many arrays. This defaults to `0`. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.offset)

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#method.nulls" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.nulls" class="fn">nulls</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>\>

Returns the null buffer of this array if any. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.nulls)

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#method.logical_null_count" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.logical_null_count" class="fn">logical_null_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of logical null values in this array. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.logical_null_count)

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#method.get_buffer_memory_size" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.get_buffer_memory_size" class="fn">get_buffer_memory_size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of bytes of memory pointed to by this array. The buffers store bytes in the Arrow memory format, and include the data as well as the validity map. Note that this does not always correspond to the exact memory usage of an array, since multiple arrays can share the same buffers or slices thereof.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#method.get_array_memory_size" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.get_array_memory_size" class="fn">get_array_memory_size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of bytes of memory occupied physically by this array. This value will always be greater than returned by `get_buffer_memory_size()` and includes the overhead of the data structures that contain the pointers to the various buffers.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#method.logical_nulls" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.logical_nulls" class="fn">logical_nulls</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>\>

Returns a potentially computed [`NullBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html "struct arrow::buffer::NullBuffer") that represents the logical null values of this array, if any. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.logical_nulls)

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#method.is_null" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_null" class="fn">is_null</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether the element at `index` is null according to [`Array::nulls`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.nulls "method arrow::array::Array::nulls") [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_null)

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#method.is_valid" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_valid" class="fn">is_valid</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether the element at `index` is *not* null, the opposite of [`Self::is_null`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_null "method arrow_array::array::Array::is_null::is_null"). [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_valid)

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#method.null_count" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.null_count" class="fn">null_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of physical null values in this array. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.null_count)

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#method.is_nullable" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_nullable" class="fn">is_nullable</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns `false` if the array is guaranteed to not contain any logical nulls [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_nullable)

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#impl-ArrayAccessor-for-%26FixedSizeBinaryArray" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayAccessor.html" class="trait" title="trait arrow::array::ArrayAccessor">ArrayAccessor</a> for &'a <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html" class="struct" title="struct arrow::array::FixedSizeBinaryArray">FixedSizeBinaryArray</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#associatedtype.Item" class="anchor">§</a>

#### type <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayAccessor.html#associatedtype.Item" class="associatedtype">Item</a> = &'a \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]

The Arrow type of the element being accessed.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#method.value-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayAccessor.html#tymethod.value" class="fn">value</a>( &self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> \<&'a <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html" class="struct" title="struct arrow::array::FixedSizeBinaryArray">FixedSizeBinaryArray</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayAccessor.html" class="trait" title="trait arrow::array::ArrayAccessor">ArrayAccessor</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayAccessor.html#associatedtype.Item" class="associatedtype" title="type arrow::array::ArrayAccessor::Item">Item</a>

Returns the element at index `i` [Read more](https://docs.rs/arrow/latest/arrow/array/trait.ArrayAccessor.html#tymethod.value)

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#method.value_unchecked-1" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayAccessor.html#tymethod.value_unchecked" class="fn">value_unchecked</a>( &self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> \<&'a <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html" class="struct" title="struct arrow::array::FixedSizeBinaryArray">FixedSizeBinaryArray</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayAccessor.html" class="trait" title="trait arrow::array::ArrayAccessor">ArrayAccessor</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayAccessor.html#associatedtype.Item" class="associatedtype" title="type arrow::array::ArrayAccessor::Item">Item</a>

Returns the element at index `i` [Read more](https://docs.rs/arrow/latest/arrow/array/trait.ArrayAccessor.html#tymethod.value_unchecked)

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#impl-Clone-for-FixedSizeBinaryArray" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html" class="struct" title="struct arrow::array::FixedSizeBinaryArray">FixedSizeBinaryArray</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html" class="struct" title="struct arrow::array::FixedSizeBinaryArray">FixedSizeBinaryArray</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#impl-Debug-for-FixedSizeBinaryArray" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html" class="struct" title="struct arrow::array::FixedSizeBinaryArray">FixedSizeBinaryArray</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#impl-From%3CArrayData%3E-for-FixedSizeBinaryArray" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html" class="struct" title="struct arrow::array::FixedSizeBinaryArray">FixedSizeBinaryArray</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(data: <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html" class="struct" title="struct arrow::array::FixedSizeBinaryArray">FixedSizeBinaryArray</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#impl-From%3CFixedSizeBinaryArray%3E-for-ArrayData" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html" class="struct" title="struct arrow::array::FixedSizeBinaryArray">FixedSizeBinaryArray</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(array: <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html" class="struct" title="struct arrow::array::FixedSizeBinaryArray">FixedSizeBinaryArray</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#impl-From%3CFixedSizeListArray%3E-for-FixedSizeBinaryArray" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListArray.html" class="struct" title="struct arrow::array::FixedSizeListArray">FixedSizeListArray</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html" class="struct" title="struct arrow::array::FixedSizeBinaryArray">FixedSizeBinaryArray</a>

Creates a `FixedSizeBinaryArray` from `FixedSizeList<u8>` array

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#method.from-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListArray.html" class="struct" title="struct arrow::array::FixedSizeListArray">FixedSizeListArray</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html" class="struct" title="struct arrow::array::FixedSizeBinaryArray">FixedSizeBinaryArray</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#impl-From%3CVec%3C%26%5Bu8%5D%3E%3E-for-FixedSizeBinaryArray" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<&\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html" class="struct" title="struct arrow::array::FixedSizeBinaryArray">FixedSizeBinaryArray</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#method.from-4" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<&\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html" class="struct" title="struct arrow::array::FixedSizeBinaryArray">FixedSizeBinaryArray</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#impl-From%3CVec%3C%26%5Bu8;+N%5D%3E%3E-for-FixedSizeBinaryArray" class="anchor">§</a>

### impl\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<&\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>; <a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">N</a>\]\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html" class="struct" title="struct arrow::array::FixedSizeBinaryArray">FixedSizeBinaryArray</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#method.from-5" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<&\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>; <a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">N</a>\]\>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html" class="struct" title="struct arrow::array::FixedSizeBinaryArray">FixedSizeBinaryArray</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#impl-From%3CVec%3COption%3C%26%5Bu8%5D%3E%3E%3E-for-FixedSizeBinaryArray" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html" class="struct" title="struct arrow::array::FixedSizeBinaryArray">FixedSizeBinaryArray</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#method.from-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>\>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html" class="struct" title="struct arrow::array::FixedSizeBinaryArray">FixedSizeBinaryArray</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#impl-IntoIterator-for-%26FixedSizeBinaryArray" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a> for &'a <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html" class="struct" title="struct arrow::array::FixedSizeBinaryArray">FixedSizeBinaryArray</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#associatedtype.Item-1" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>

The type of the elements being iterated over.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#associatedtype.IntoIter" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype">IntoIter</a> = <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayIter.html" class="struct" title="struct arrow::array::ArrayIter">ArrayIter</a>\<&'a <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html" class="struct" title="struct arrow::array::FixedSizeBinaryArray">FixedSizeBinaryArray</a>\>

Which kind of iterator are we turning this into?

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#method.into_iter" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter" class="fn">into_iter</a>(self) -\> \<&'a <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html" class="struct" title="struct arrow::array::FixedSizeBinaryArray">FixedSizeBinaryArray</a> as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::IntoIter">IntoIter</a>

Creates an iterator from a value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#impl-PartialEq-for-FixedSizeBinaryArray" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html" class="struct" title="struct arrow::array::FixedSizeBinaryArray">FixedSizeBinaryArray</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html" class="struct" title="struct arrow::array::FixedSizeBinaryArray">FixedSizeBinaryArray</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html#blanket-implementations" class="anchor">§</a>
