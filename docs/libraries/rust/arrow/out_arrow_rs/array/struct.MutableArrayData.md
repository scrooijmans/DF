# Struct MutableArrayData Copy item path

<a href="https://docs.rs/arrow-data/56.2.0/x86_64-unknown-linux-gnu/src/arrow_data/transform/mod.rs.html#134" class="src">Source</a>

``` rust
pub struct MutableArrayData<'a> { /* private fields */ }
```

Expand description

Efficiently create an [ArrayData](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html "struct arrow::array::ArrayData") from one or more existing [ArrayData](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html "struct arrow::array::ArrayData")s by copying chunks.

The main use case of this struct is to perform unary operations to arrays of arbitrary types, such as `filter` and `take`.

## <a href="https://docs.rs/arrow/latest/arrow/array/struct.MutableArrayData.html#example" class="doc-anchor">§</a>Example

``` rust
use arrow_buffer::Buffer;
use arrow_data::ArrayData;
use arrow_data::transform::MutableArrayData;
use arrow_schema::DataType;
fn i32_array(values: &[i32]) -> ArrayData {
  ArrayData::try_new(DataType::Int32, 5, None, 0, vec![Buffer::from_slice_ref(values)], vec![]).unwrap()
}
let arr1  = i32_array(&[1, 2, 3, 4, 5]);
let arr2  = i32_array(&[6, 7, 8, 9, 10]);
// Create a mutable array for copying values from arr1 and arr2, with a capacity for 6 elements
let capacity = 3 * std::mem::size_of::<i32>();
let mut mutable = MutableArrayData::new(vec![&arr1, &arr2], false, 10);
// Copy the first 3 elements from arr1
mutable.extend(0, 0, 3);
// Copy the last 3 elements from arr2
mutable.extend(1, 2, 4);
// Complete the MutableArrayData into a new ArrayData
let frozen = mutable.freeze();
assert_eq!(frozen, i32_array(&[1, 2, 3, 8, 9, 10]));
```

## Implementations<a href="https://docs.rs/arrow/latest/arrow/array/struct.MutableArrayData.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.MutableArrayData.html#impl-MutableArrayData%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.MutableArrayData.html" class="struct" title="struct arrow::array::MutableArrayData">MutableArrayData</a>\<'a\>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.MutableArrayData.html#method.new" class="fn">new</a>( arrays: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<&'a <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>\>, use_nulls: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.MutableArrayData.html" class="struct" title="struct arrow::array::MutableArrayData">MutableArrayData</a>\<'a\>

Returns a new [MutableArrayData](https://docs.rs/arrow/latest/arrow/array/struct.MutableArrayData.html "struct arrow::array::MutableArrayData") with capacity to `capacity` slots and specialized to create an [ArrayData](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html "struct arrow::array::ArrayData") from multiple `arrays`.

##### <a href="https://docs.rs/arrow/latest/arrow/array/struct.MutableArrayData.html#arguments" class="doc-anchor">§</a>Arguments

- `arrays` - the source arrays to copy from
- `use_nulls` - a flag used to optimize insertions
  - `false` if the only source of nulls are the arrays themselves
  - `true` if the user plans to call [MutableArrayData::extend_nulls](https://docs.rs/arrow/latest/arrow/array/struct.MutableArrayData.html#method.extend_nulls "method arrow::array::MutableArrayData::extend_nulls").
- capacity - the preallocated capacity of the output array, in bytes

Thus, if `use_nulls` is `false`, calling [MutableArrayData::extend_nulls](https://docs.rs/arrow/latest/arrow/array/struct.MutableArrayData.html#method.extend_nulls "method arrow::array::MutableArrayData::extend_nulls") should not be used.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.MutableArrayData.html#method.with_capacities" class="fn">with_capacities</a>( arrays: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<&'a <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>\>, use_nulls: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, capacities: <a href="https://docs.rs/arrow/latest/arrow/array/enum.Capacities.html" class="enum" title="enum arrow::array::Capacities">Capacities</a>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.MutableArrayData.html" class="struct" title="struct arrow::array::MutableArrayData">MutableArrayData</a>\<'a\>

Similar to [MutableArrayData::new](https://docs.rs/arrow/latest/arrow/array/struct.MutableArrayData.html#method.new "associated function arrow::array::MutableArrayData::new"), but lets users define the preallocated capacities of the array with more granularity.

See [MutableArrayData::new](https://docs.rs/arrow/latest/arrow/array/struct.MutableArrayData.html#method.new "associated function arrow::array::MutableArrayData::new") for more information on the arguments.

##### <a href="https://docs.rs/arrow/latest/arrow/array/struct.MutableArrayData.html#panics" class="doc-anchor">§</a>Panics

This function panics if the given `capacities` don’t match the data type of `arrays`. Or when a [Capacities](https://docs.rs/arrow/latest/arrow/array/enum.Capacities.html "enum arrow::array::Capacities") variant is not yet supported.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.MutableArrayData.html#method.extend" class="fn">extend</a>(&mut self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, start: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, end: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Extends the in progress array with a region of the input arrays

##### <a href="https://docs.rs/arrow/latest/arrow/array/struct.MutableArrayData.html#arguments-1" class="doc-anchor">§</a>Arguments

- `index` - the index of array that you what to copy values from
- `start` - the start index of the chunk (inclusive)
- `end` - the end index of the chunk (exclusive)

##### <a href="https://docs.rs/arrow/latest/arrow/array/struct.MutableArrayData.html#panic" class="doc-anchor">§</a>Panic

This function panics if there is an invalid index, i.e. `index` \>= the number of source arrays or `end` \> the length of the `index`th array

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.MutableArrayData.html#method.extend_nulls" class="fn">extend_nulls</a>(&mut self, len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Extends the in progress array with null elements, ignoring the input arrays.

##### <a href="https://docs.rs/arrow/latest/arrow/array/struct.MutableArrayData.html#panics-1" class="doc-anchor">§</a>Panics

Panics if [`MutableArrayData`](https://docs.rs/arrow/latest/arrow/array/struct.MutableArrayData.html "struct arrow::array::MutableArrayData") not created with `use_nulls` or nullable source arrays

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.MutableArrayData.html#method.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the current length

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.MutableArrayData.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if len is 0

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.MutableArrayData.html#method.null_count" class="fn">null_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the current null count

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.MutableArrayData.html#method.freeze" class="fn">freeze</a>(self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

Creates a [ArrayData](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html "struct arrow::array::ArrayData") from the in progress array, consuming `self`.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.MutableArrayData.html#method.into_builder" class="fn">into_builder</a>(self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html" class="struct" title="struct arrow::array::ArrayDataBuilder">ArrayDataBuilder</a>

Consume self and returns the in progress array as [`ArrayDataBuilder`](https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html "struct arrow::array::ArrayDataBuilder").

This is useful for extending the default behavior of MutableArrayData.

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/struct.MutableArrayData.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.MutableArrayData.html#impl-Debug-for-MutableArrayData%3C&#39;_%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.MutableArrayData.html" class="struct" title="struct arrow::array::MutableArrayData">MutableArrayData</a>\<'\_\>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.MutableArrayData.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/struct.MutableArrayData.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/array/struct.MutableArrayData.html#blanket-implementations" class="anchor">§</a>
