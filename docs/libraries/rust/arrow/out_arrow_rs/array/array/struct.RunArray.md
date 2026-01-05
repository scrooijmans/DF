# Struct RunArray Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/array/run_array.rs.html#63" class="src">Source</a>

``` rust
pub struct RunArray<R>where
    R: RunEndIndexType,{ /* private fields */ }
```

Expand description

An array of [run-end encoded values](https://arrow.apache.org/docs/format/Columnar.html#run-end-encoded-layout)

This encoding is variation on [run-length encoding (RLE)](https://en.wikipedia.org/wiki/Run-length_encoding) and is good for representing data containing same values repeated consecutively.

[`RunArray`](https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html "struct arrow::array::RunArray") contains `run_ends` array and `values` array of same length. The `run_ends` array stores the indexes at which the run ends. The `values` array stores the value of each run. Below example illustrates how a logical array is represented in [`RunArray`](https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html "struct arrow::array::RunArray")

``` text
┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─┐
  ┌─────────────────┐  ┌─────────┐       ┌─────────────────┐
│ │        A        │  │    2    │ │     │        A        │
  ├─────────────────┤  ├─────────┤       ├─────────────────┤
│ │        D        │  │    3    │ │     │        A        │    run length of 'A' = runs_ends[0] - 0 = 2
  ├─────────────────┤  ├─────────┤       ├─────────────────┤
│ │        B        │  │    6    │ │     │        D        │    run length of 'D' = run_ends[1] - run_ends[0] = 1
  └─────────────────┘  └─────────┘       ├─────────────────┤
│        values          run_ends  │     │        B        │
                                         ├─────────────────┤
└ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─┘     │        B        │
                                         ├─────────────────┤
               RunArray                  │        B        │    run length of 'B' = run_ends[2] - run_ends[1] = 3
              length = 3                 └─────────────────┘

                                            Logical array
                                               Contents
```

## Implementations<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.RunArray.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.RunArray.html#impl-RunArray%3CR%3E" class="anchor">§</a>

### impl\<R\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html" class="struct" title="struct arrow::array::RunArray">RunArray</a>\<R\>

where R: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.RunEndIndexType.html" class="trait" title="trait arrow::datatypes::RunEndIndexType">RunEndIndexType</a>,

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.RunArray.html#method.logical_len" class="fn">logical_len</a>(run_ends: &<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<R\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Calculates the logical length of the array encoded by the given run_ends array.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.RunArray.html#method.try_new" class="fn">try_new</a>( run_ends: &<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<R\>, values: &dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html" class="struct" title="struct arrow::array::RunArray">RunArray</a>\<R\>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Attempts to create RunArray using given run_ends (index where a run ends) and the values (value of the run). Returns an error if the given data is not compatible with RunEndEncoded specification.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.RunArray.html#method.run_ends" class="fn">run_ends</a>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.RunEndBuffer.html" class="struct" title="struct arrow::buffer::RunEndBuffer">RunEndBuffer</a>\<\<R as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>

Returns a reference to [`RunEndBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.RunEndBuffer.html "struct arrow::buffer::RunEndBuffer")

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.RunArray.html#method.values" class="fn">values</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

Returns a reference to values array

Note: any slicing of this [`RunArray`](https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html "struct arrow::array::RunArray") array is not applied to the returned array and must be handled separately

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.RunArray.html#method.get_start_physical_index" class="fn">get_start_physical_index</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the physical index at which the array slice starts.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.RunArray.html#method.get_end_physical_index" class="fn">get_end_physical_index</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the physical index at which the array slice ends.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.RunArray.html#method.downcast" class="fn">downcast</a>\<V\>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.TypedRunArray.html" class="struct" title="struct arrow::array::TypedRunArray">TypedRunArray</a>\<'\_, R, V\>\>

where V: 'static,

Downcast this [`RunArray`](https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html "struct arrow::array::RunArray") to a [`TypedRunArray`](https://docs.rs/arrow/latest/arrow/array/struct.TypedRunArray.html "struct arrow::array::TypedRunArray")

``` rust
use arrow_array::{Array, ArrayAccessor, RunArray, StringArray, types::Int32Type};

let orig = [Some("a"), Some("b"), None];
let run_array = RunArray::<Int32Type>::from_iter(orig);
let typed = run_array.downcast::<StringArray>().unwrap();
assert_eq!(typed.value(0), "a");
assert_eq!(typed.value(1), "b");
assert!(typed.values().is_null(2));
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.RunArray.html#method.get_physical_index" class="fn">get_physical_index</a>(&self, logical_index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns index to the physical array for the given index to the logical array. This function adjusts the input logical index based on `ArrayData::offset` Performs a binary search on the run_ends array for the input index.

The result is arbitrary if `logical_index >= self.len()`

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.RunArray.html#method.get_physical_indices" class="fn">get_physical_indices</a>\<I\>( &self, logical_indices: &<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[I]</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

where I: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html" class="trait" title="trait arrow::datatypes::ArrowNativeType">ArrowNativeType</a>,

Returns the physical indices of the input logical indices. Returns error if any of the logical index cannot be converted to physical index. The logical indices are sorted and iterated along with run_ends array to find matching physical index. The approach used here was chosen over finding physical index for each logical index using binary search using the function `get_physical_index`. Running benchmarks on both approaches showed that the approach used here scaled well for larger inputs. See <https://github.com/apache/arrow-rs/pull/3622#issuecomment-1407753727> for more details.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.RunArray.html#method.slice" class="fn">slice</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html" class="struct" title="struct arrow::array::RunArray">RunArray</a>\<R\>

Returns a zero-copy slice of this array with the indicated offset and length.

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.RunArray.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.RunArray.html#impl-Array-for-RunArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html" class="struct" title="struct arrow::array::RunArray">RunArray</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.RunEndIndexType.html" class="trait" title="trait arrow::datatypes::RunEndIndexType">RunEndIndexType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.RunArray.html#method.as_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the array as [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") so that it can be downcasted to a specific implementation. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.as_any)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.RunArray.html#method.to_data" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.to_data" class="fn">to_data</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

Returns the underlying data of this array

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.RunArray.html#method.into_data" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.into_data" class="fn">into_data</a>(self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

Returns the underlying data of this array [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.into_data)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.RunArray.html#method.data_type" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.data_type" class="fn">data_type</a>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a>

Returns a reference to the [`DataType`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html "enum arrow::datatypes::DataType") of this array. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.data_type)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.RunArray.html#method.slice-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.slice" class="fn">slice</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

Returns a zero-copy slice of this array with the indicated offset and length. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.slice)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.RunArray.html#method.len" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the length (i.e., number of elements) of this array. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.len)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.RunArray.html#method.is_empty" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether this array is empty. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.is_empty)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.RunArray.html#method.shrink_to_fit" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.shrink_to_fit" class="fn">shrink_to_fit</a>(&mut self)

Shrinks the capacity of any exclusively owned buffer as much as possible [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.shrink_to_fit)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.RunArray.html#method.offset" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.offset" class="fn">offset</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the offset into the underlying data used by this array(-slice). Note that the underlying data can be shared by many arrays. This defaults to `0`. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.offset)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.RunArray.html#method.nulls" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.nulls" class="fn">nulls</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>\>

Returns the null buffer of this array if any. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.nulls)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.RunArray.html#method.logical_nulls" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.logical_nulls" class="fn">logical_nulls</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>\>

Returns a potentially computed [`NullBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html "struct arrow::buffer::NullBuffer") that represents the logical null values of this array, if any. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.logical_nulls)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.RunArray.html#method.is_nullable" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_nullable" class="fn">is_nullable</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns `false` if the array is guaranteed to not contain any logical nulls [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_nullable)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.RunArray.html#method.get_buffer_memory_size" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.get_buffer_memory_size" class="fn">get_buffer_memory_size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of bytes of memory pointed to by this array. The buffers store bytes in the Arrow memory format, and include the data as well as the validity map. Note that this does not always correspond to the exact memory usage of an array, since multiple arrays can share the same buffers or slices thereof.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.RunArray.html#method.get_array_memory_size" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.get_array_memory_size" class="fn">get_array_memory_size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of bytes of memory occupied physically by this array. This value will always be greater than returned by `get_buffer_memory_size()` and includes the overhead of the data structures that contain the pointers to the various buffers.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.RunArray.html#method.is_null" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_null" class="fn">is_null</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether the element at `index` is null according to [`Array::nulls`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.nulls "method arrow::array::Array::nulls") [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_null)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.RunArray.html#method.is_valid" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_valid" class="fn">is_valid</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether the element at `index` is *not* null, the opposite of [`Self::is_null`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_null "method arrow_array::array::Array::is_null::is_null"). [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_valid)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.RunArray.html#method.null_count" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.null_count" class="fn">null_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of physical null values in this array. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.null_count)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.RunArray.html#method.logical_null_count" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.logical_null_count" class="fn">logical_null_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of logical null values in this array. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.logical_null_count)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.RunArray.html#impl-Clone-for-RunArray%3CR%3E" class="anchor">§</a>

### impl\<R\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html" class="struct" title="struct arrow::array::RunArray">RunArray</a>\<R\>

where R: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.RunEndIndexType.html" class="trait" title="trait arrow::datatypes::RunEndIndexType">RunEndIndexType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.RunArray.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html" class="struct" title="struct arrow::array::RunArray">RunArray</a>\<R\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/array/array/struct.RunArray.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.RunArray.html#impl-Debug-for-RunArray%3CR%3E" class="anchor">§</a>

### impl\<R\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html" class="struct" title="struct arrow::array::RunArray">RunArray</a>\<R\>

where R: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.RunEndIndexType.html" class="trait" title="trait arrow::datatypes::RunEndIndexType">RunEndIndexType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.RunArray.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.RunArray.html#impl-From%3CArrayData%3E-for-RunArray%3CR%3E" class="anchor">§</a>

### impl\<R\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html" class="struct" title="struct arrow::array::RunArray">RunArray</a>\<R\>

where R: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.RunEndIndexType.html" class="trait" title="trait arrow::datatypes::RunEndIndexType">RunEndIndexType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.RunArray.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(data: <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html" class="struct" title="struct arrow::array::RunArray">RunArray</a>\<R\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.RunArray.html#impl-From%3CRunArray%3CR%3E%3E-for-ArrayData" class="anchor">§</a>

### impl\<R\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html" class="struct" title="struct arrow::array::RunArray">RunArray</a>\<R\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

where R: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.RunEndIndexType.html" class="trait" title="trait arrow::datatypes::RunEndIndexType">RunEndIndexType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.RunArray.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(array: <a href="https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html" class="struct" title="struct arrow::array::RunArray">RunArray</a>\<R\>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.RunArray.html#impl-FromIterator%3C%26str%3E-for-RunArray%3CT%3E" class="anchor">§</a>

### impl\<'a, T\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html" class="struct" title="struct arrow::array::RunArray">RunArray</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.RunEndIndexType.html" class="trait" title="trait arrow::datatypes::RunEndIndexType">RunEndIndexType</a>,

Constructs a `RunArray` from an iterator of strings.

#### <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.RunArray.html#example-1" class="doc-anchor">§</a>Example:

``` rust
use arrow_array::{RunArray, PrimitiveArray, StringArray, types::Int16Type};

let test = vec!["a", "a", "b", "c"];
let array: RunArray<Int16Type> = test.into_iter().collect();
assert_eq!(
    "RunArray {run_ends: [2, 3, 4], values: StringArray\n[\n  \"a\",\n  \"b\",\n  \"c\",\n]}\n",
    format!("{:?}", array)
);
```

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.RunArray.html#method.from_iter-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html" class="struct" title="struct arrow::array::RunArray">RunArray</a>\<T\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.RunArray.html#impl-FromIterator%3COption%3C%26str%3E%3E-for-RunArray%3CT%3E" class="anchor">§</a>

### impl\<'a, T\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html" class="struct" title="struct arrow::array::RunArray">RunArray</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.RunEndIndexType.html" class="trait" title="trait arrow::datatypes::RunEndIndexType">RunEndIndexType</a>,

Constructs a `RunArray` from an iterator of optional strings.

#### <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.RunArray.html#example" class="doc-anchor">§</a>Example:

``` rust
use arrow_array::{RunArray, PrimitiveArray, StringArray, types::Int16Type};

let test = vec!["a", "a", "b", "c", "c"];
let array: RunArray<Int16Type> = test
    .iter()
    .map(|&x| if x == "b" { None } else { Some(x) })
    .collect();
assert_eq!(
    "RunArray {run_ends: [2, 3, 5], values: StringArray\n[\n  \"a\",\n  null,\n  \"c\",\n]}\n",
    format!("{:?}", array)
);
```

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.RunArray.html#method.from_iter" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html" class="struct" title="struct arrow::array::RunArray">RunArray</a>\<T\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.RunArray.html#impl-PartialEq-for-RunArray%3CR%3E" class="anchor">§</a>

### impl\<R\> <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html" class="struct" title="struct arrow::array::RunArray">RunArray</a>\<R\>

where R: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.RunEndIndexType.html" class="trait" title="trait arrow::datatypes::RunEndIndexType">RunEndIndexType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.RunArray.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html" class="struct" title="struct arrow::array::RunArray">RunArray</a>\<R\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/array/array/struct.RunArray.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.RunArray.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.RunArray.html#blanket-implementations" class="anchor">§</a>
