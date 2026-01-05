# Struct GenericListViewArray Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/array/list_view_array.rs.html#104" class="src">Source</a>

``` rust
pub struct GenericListViewArray<OffsetSize>where
    OffsetSize: OffsetSizeTrait,{ /* private fields */ }
```

Expand description

An array of [variable length lists](https://arrow.apache.org/docs/format/Columnar.html#variable-size-list-layout), specifically in the [list-view layout](https://arrow.apache.org/docs/format/Columnar.html#listview-layout).

Differs from [`GenericListArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html "struct arrow::array::GenericListArray") (which represents the [list layout](https://arrow.apache.org/docs/format/Columnar.html#list-layout)) in that the sizes of the child arrays are explicitly encoded in a separate buffer, instead of being derived from the difference between subsequent offsets in the offset buffer.

This allows the offsets (and subsequently child data) to be out of order. It also allows take / filter operations to be implemented without copying the underlying data.

## <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#representation" class="doc-anchor">§</a>Representation

Given the same example array from [`GenericListArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html "struct arrow::array::GenericListArray"), it would be represented as such via a list-view layout array:

``` text
                                        ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
                                                                        ┌ ─ ─ ─ ─ ─ ─ ┐    │
 ┌─────────────┐  ┌───────┐             │     ┌───┐   ┌───┐   ┌───┐       ┌───┐ ┌───┐
 │   [A,B,C]   │  │ (0,3) │                   │ 1 │   │ 0 │   │ 3 │     │ │ 1 │ │ A │ │ 0  │
 ├─────────────┤  ├───────┤             │     ├───┤   ├───┤   ├───┤       ├───┤ ├───┤
 │      []     │  │ (3,0) │                   │ 1 │   │ 3 │   │ 0 │     │ │ 1 │ │ B │ │ 1  │
 ├─────────────┤  ├───────┤             │     ├───┤   ├───┤   ├───┤       ├───┤ ├───┤
 │    NULL     │  │ (?,?) │                   │ 0 │   │ ? │   │ ? │     │ │ 1 │ │ C │ │ 2  │
 ├─────────────┤  ├───────┤             │     ├───┤   ├───┤   ├───┤       ├───┤ ├───┤
 │     [D]     │  │ (4,1) │                   │ 1 │   │ 4 │   │ 1 │     │ │ ? │ │ ? │ │ 3  │
 ├─────────────┤  ├───────┤             │     ├───┤   ├───┤   ├───┤       ├───┤ ├───┤
 │  [NULL, F]  │  │ (5,2) │                   │ 1 │   │ 5 │   │ 2 │     │ │ 1 │ │ D │ │ 4  │
 └─────────────┘  └───────┘             │     └───┘   └───┘   └───┘       ├───┤ ├───┤
                                                                        │ │ 0 │ │ ? │ │ 5  │
    Logical       Logical               │  Validity  Offsets  Sizes       ├───┤ ├───┤
     Values       Offset                   (nulls)                      │ │ 1 │ │ F │ │ 6  │
                  & Size                │                                 └───┘ └───┘
                                                                        │    Values   │    │
                (offsets[i],            │   ListViewArray                   (Array)
                 sizes[i])                                              └ ─ ─ ─ ─ ─ ─ ┘    │
                                        └ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
```

Another way of representing the same array but taking advantage of the offsets being out of order:

``` text
                                        ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
                                                                        ┌ ─ ─ ─ ─ ─ ─ ┐    │
 ┌─────────────┐  ┌───────┐             │     ┌───┐   ┌───┐   ┌───┐       ┌───┐ ┌───┐
 │   [A,B,C]   │  │ (2,3) │                   │ 1 │   │ 2 │   │ 3 │     │ │ 0 │ │ ? │ │ 0  │
 ├─────────────┤  ├───────┤             │     ├───┤   ├───┤   ├───┤       ├───┤ ├───┤
 │      []     │  │ (0,0) │                   │ 1 │   │ 0 │   │ 0 │     │ │ 1 │ │ F │ │ 1  │
 ├─────────────┤  ├───────┤             │     ├───┤   ├───┤   ├───┤       ├───┤ ├───┤
 │    NULL     │  │ (?,?) │                   │ 0 │   │ ? │   │ ? │     │ │ 1 │ │ A │ │ 2  │
 ├─────────────┤  ├───────┤             │     ├───┤   ├───┤   ├───┤       ├───┤ ├───┤
 │     [D]     │  │ (5,1) │                   │ 1 │   │ 5 │   │ 1 │     │ │ 1 │ │ B │ │ 3  │
 ├─────────────┤  ├───────┤             │     ├───┤   ├───┤   ├───┤       ├───┤ ├───┤
 │  [NULL, F]  │  │ (0,2) │                   │ 1 │   │ 0 │   │ 2 │     │ │ 1 │ │ C │ │ 4  │
 └─────────────┘  └───────┘             │     └───┘   └───┘   └───┘       ├───┤ ├───┤
                                                                        │ │ 1 │ │ D │ │ 5  │
    Logical       Logical               │  Validity  Offsets  Sizes       └───┘ └───┘
     Values       Offset                   (nulls)                      │    Values   │    │
                  & Size                │                                   (Array)  
                                                                        └ ─ ─ ─ ─ ─ ─ ┘    │
                (offsets[i],            │   ListViewArray                          
                 sizes[i])                                                                 │
                                        └ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
```

## Implementations<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#impl-GenericListViewArray%3COffsetSize%3E" class="anchor">§</a>

### impl\<OffsetSize\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListViewArray.html" class="struct" title="struct arrow::array::GenericListViewArray">GenericListViewArray</a>\<OffsetSize\>

where OffsetSize: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

#### pub const <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#associatedconstant.DATA_TYPE_CONSTRUCTOR" class="constant">DATA_TYPE_CONSTRUCTOR</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.fn.html" class="primitive">fn</a>(<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>\>) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a>

The data type constructor of listview array. The input is the schema of the child array and the output is the [`DataType`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html "enum arrow::datatypes::DataType"), ListView or LargeListView.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#method.try_new" class="fn">try_new</a>( field: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>\>, offsets: <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.ScalarBuffer.html" class="struct" title="struct arrow::buffer::ScalarBuffer">ScalarBuffer</a>\<OffsetSize\>, sizes: <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.ScalarBuffer.html" class="struct" title="struct arrow::buffer::ScalarBuffer">ScalarBuffer</a>\<OffsetSize\>, values: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>, nulls: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListViewArray.html" class="struct" title="struct arrow::array::GenericListViewArray">GenericListViewArray</a>\<OffsetSize\>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Create a new [`GenericListViewArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListViewArray.html "struct arrow::array::GenericListViewArray") from the provided parts

##### <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#errors" class="doc-anchor">§</a>Errors

Errors if

- `offsets.len() != sizes.len()`
- `offsets.len() != nulls.len()`
- `offsets[i] > values.len()`
- `!field.is_nullable() && values.is_nullable()`
- `field.data_type() != values.data_type()`
- `0 <= offsets[i] <= length of the child array`
- `0 <= offsets[i] + size[i] <= length of the child array`

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#method.new" class="fn">new</a>( field: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>\>, offsets: <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.ScalarBuffer.html" class="struct" title="struct arrow::buffer::ScalarBuffer">ScalarBuffer</a>\<OffsetSize\>, sizes: <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.ScalarBuffer.html" class="struct" title="struct arrow::buffer::ScalarBuffer">ScalarBuffer</a>\<OffsetSize\>, values: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>, nulls: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListViewArray.html" class="struct" title="struct arrow::array::GenericListViewArray">GenericListViewArray</a>\<OffsetSize\>

Create a new [`GenericListViewArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListViewArray.html "struct arrow::array::GenericListViewArray") from the provided parts

##### <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#panics" class="doc-anchor">§</a>Panics

Panics if [`Self::try_new`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListViewArray.html#method.try_new "associated function arrow::array::GenericListViewArray::try_new") returns an error

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#method.new_null" class="fn">new_null</a>( field: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>\>, len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListViewArray.html" class="struct" title="struct arrow::array::GenericListViewArray">GenericListViewArray</a>\<OffsetSize\>

Create a new [`GenericListViewArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListViewArray.html "struct arrow::array::GenericListViewArray") of length `len` where all values are null

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#method.into_parts" class="fn">into_parts</a>( self, ) -\> (<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>\>, <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.ScalarBuffer.html" class="struct" title="struct arrow::buffer::ScalarBuffer">ScalarBuffer</a>\<OffsetSize\>, <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.ScalarBuffer.html" class="struct" title="struct arrow::buffer::ScalarBuffer">ScalarBuffer</a>\<OffsetSize\>, <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>\>)

Deconstruct this array into its constituent parts

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#method.offsets" class="fn">offsets</a>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.ScalarBuffer.html" class="struct" title="struct arrow::buffer::ScalarBuffer">ScalarBuffer</a>\<OffsetSize\>

Returns a reference to the offsets of this list

Unlike [`Self::value_offsets`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListViewArray.html#method.value_offsets "method arrow::array::GenericListViewArray::value_offsets") this returns the [`ScalarBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.ScalarBuffer.html "struct arrow::buffer::ScalarBuffer") allowing for zero-copy cloning

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#method.values" class="fn">values</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

Returns a reference to the values of this list

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#method.sizes" class="fn">sizes</a>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.ScalarBuffer.html" class="struct" title="struct arrow::buffer::ScalarBuffer">ScalarBuffer</a>\<OffsetSize\>

Returns a reference to the sizes of this list

Unlike [`Self::value_sizes`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListViewArray.html#method.value_sizes "method arrow::array::GenericListViewArray::value_sizes") this returns the [`ScalarBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.ScalarBuffer.html "struct arrow::buffer::ScalarBuffer") allowing for zero-copy cloning

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#method.value_type" class="fn">value_type</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a>

Returns a clone of the value type of this list.

#### pub unsafe fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#method.value_unchecked" class="fn">value_unchecked</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

Returns ith value of this list view array.

Note: This method does not check for nulls and the value is arbitrary if [`is_null`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListViewArray.html#method.is_null "method arrow::array::GenericListViewArray::is_null") returns true for the index.

##### <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#safety" class="doc-anchor">§</a>Safety

Caller must ensure that the index is within the array bounds

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#method.value" class="fn">value</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

Returns ith value of this list view array.

Note: This method does not check for nulls and the value is arbitrary (but still well-defined) if [`is_null`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListViewArray.html#method.is_null "method arrow::array::GenericListViewArray::is_null") returns true for the index.

##### <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#panics-1" class="doc-anchor">§</a>Panics

Panics if the index is out of bounds

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#method.value_offsets" class="fn">value_offsets</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[OffsetSize]</a>

Returns the offset values in the offsets buffer

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#method.value_sizes" class="fn">value_sizes</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[OffsetSize]</a>

Returns the sizes values in the offsets buffer

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#method.value_size" class="fn">value_size</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> OffsetSize

Returns the size for value at index `i`.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#method.value_offset" class="fn">value_offset</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> OffsetSize

Returns the offset for value at index `i`.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#method.iter" class="fn">iter</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayIter.html" class="struct" title="struct arrow::array::ArrayIter">ArrayIter</a>\<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListViewArray.html" class="struct" title="struct arrow::array::GenericListViewArray">GenericListViewArray</a>\<OffsetSize\>\> <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#" class="tooltip" data-notable-ty="ArrayIter&lt;&amp;GenericListViewArray&lt;OffsetSize&gt;&gt;">ⓘ</a>

Constructs a new iterator

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#method.slice" class="fn">slice</a>( &self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListViewArray.html" class="struct" title="struct arrow::array::GenericListViewArray">GenericListViewArray</a>\<OffsetSize\>

Returns a zero-copy slice of this array with the indicated offset and length.

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#impl-Array-for-GenericListViewArray%3COffsetSize%3E" class="anchor">§</a>

### impl\<OffsetSize\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListViewArray.html" class="struct" title="struct arrow::array::GenericListViewArray">GenericListViewArray</a>\<OffsetSize\>

where OffsetSize: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#method.as_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the array as [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") so that it can be downcasted to a specific implementation. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.as_any)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#method.to_data" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.to_data" class="fn">to_data</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

Returns the underlying data of this array

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#method.into_data" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.into_data" class="fn">into_data</a>(self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

Returns the underlying data of this array [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.into_data)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#method.data_type" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.data_type" class="fn">data_type</a>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a>

Returns a reference to the [`DataType`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html "enum arrow::datatypes::DataType") of this array. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.data_type)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#method.slice-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.slice" class="fn">slice</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

Returns a zero-copy slice of this array with the indicated offset and length. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.slice)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#method.len" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the length (i.e., number of elements) of this array. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.len)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#method.is_empty" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether this array is empty. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.is_empty)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#method.shrink_to_fit" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.shrink_to_fit" class="fn">shrink_to_fit</a>(&mut self)

Shrinks the capacity of any exclusively owned buffer as much as possible [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.shrink_to_fit)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#method.offset" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.offset" class="fn">offset</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the offset into the underlying data used by this array(-slice). Note that the underlying data can be shared by many arrays. This defaults to `0`. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.offset)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#method.nulls" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.nulls" class="fn">nulls</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>\>

Returns the null buffer of this array if any. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.nulls)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#method.logical_null_count" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.logical_null_count" class="fn">logical_null_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of logical null values in this array. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.logical_null_count)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#method.get_buffer_memory_size" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.get_buffer_memory_size" class="fn">get_buffer_memory_size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of bytes of memory pointed to by this array. The buffers store bytes in the Arrow memory format, and include the data as well as the validity map. Note that this does not always correspond to the exact memory usage of an array, since multiple arrays can share the same buffers or slices thereof.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#method.get_array_memory_size" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.get_array_memory_size" class="fn">get_array_memory_size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of bytes of memory occupied physically by this array. This value will always be greater than returned by `get_buffer_memory_size()` and includes the overhead of the data structures that contain the pointers to the various buffers.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#method.logical_nulls" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.logical_nulls" class="fn">logical_nulls</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>\>

Returns a potentially computed [`NullBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html "struct arrow::buffer::NullBuffer") that represents the logical null values of this array, if any. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.logical_nulls)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#method.is_null" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_null" class="fn">is_null</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether the element at `index` is null according to [`Array::nulls`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.nulls "method arrow::array::Array::nulls") [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_null)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#method.is_valid" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_valid" class="fn">is_valid</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether the element at `index` is *not* null, the opposite of [`Self::is_null`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_null "method arrow_array::array::Array::is_null::is_null"). [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_valid)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#method.null_count" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.null_count" class="fn">null_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of physical null values in this array. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.null_count)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#method.is_nullable" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_nullable" class="fn">is_nullable</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns `false` if the array is guaranteed to not contain any logical nulls [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_nullable)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#impl-ArrayAccessor-for-%26GenericListViewArray%3COffsetSize%3E" class="anchor">§</a>

### impl\<OffsetSize\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayAccessor.html" class="trait" title="trait arrow::array::ArrayAccessor">ArrayAccessor</a> for &<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListViewArray.html" class="struct" title="struct arrow::array::GenericListViewArray">GenericListViewArray</a>\<OffsetSize\>

where OffsetSize: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#associatedtype.Item" class="anchor">§</a>

#### type <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayAccessor.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

The Arrow type of the element being accessed.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#method.value-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayAccessor.html#tymethod.value" class="fn">value</a>( &self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> \<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListViewArray.html" class="struct" title="struct arrow::array::GenericListViewArray">GenericListViewArray</a>\<OffsetSize\> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayAccessor.html" class="trait" title="trait arrow::array::ArrayAccessor">ArrayAccessor</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayAccessor.html#associatedtype.Item" class="associatedtype" title="type arrow::array::ArrayAccessor::Item">Item</a>

Returns the element at index `i` [Read more](https://docs.rs/arrow/latest/arrow/array/trait.ArrayAccessor.html#tymethod.value)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#method.value_unchecked-1" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayAccessor.html#tymethod.value_unchecked" class="fn">value_unchecked</a>( &self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> \<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListViewArray.html" class="struct" title="struct arrow::array::GenericListViewArray">GenericListViewArray</a>\<OffsetSize\> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayAccessor.html" class="trait" title="trait arrow::array::ArrayAccessor">ArrayAccessor</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayAccessor.html#associatedtype.Item" class="associatedtype" title="type arrow::array::ArrayAccessor::Item">Item</a>

Returns the element at index `i` [Read more](https://docs.rs/arrow/latest/arrow/array/trait.ArrayAccessor.html#tymethod.value_unchecked)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#impl-Clone-for-GenericListViewArray%3COffsetSize%3E" class="anchor">§</a>

### impl\<OffsetSize\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListViewArray.html" class="struct" title="struct arrow::array::GenericListViewArray">GenericListViewArray</a>\<OffsetSize\>

where OffsetSize: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> + <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListViewArray.html" class="struct" title="struct arrow::array::GenericListViewArray">GenericListViewArray</a>\<OffsetSize\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#impl-Debug-for-GenericListViewArray%3COffsetSize%3E" class="anchor">§</a>

### impl\<OffsetSize\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListViewArray.html" class="struct" title="struct arrow::array::GenericListViewArray">GenericListViewArray</a>\<OffsetSize\>

where OffsetSize: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#impl-From%3CArrayData%3E-for-GenericListViewArray%3COffsetSize%3E" class="anchor">§</a>

### impl\<OffsetSize\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListViewArray.html" class="struct" title="struct arrow::array::GenericListViewArray">GenericListViewArray</a>\<OffsetSize\>

where OffsetSize: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(data: <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListViewArray.html" class="struct" title="struct arrow::array::GenericListViewArray">GenericListViewArray</a>\<OffsetSize\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#impl-From%3CFixedSizeListArray%3E-for-GenericListViewArray%3COffsetSize%3E" class="anchor">§</a>

### impl\<OffsetSize\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListArray.html" class="struct" title="struct arrow::array::FixedSizeListArray">FixedSizeListArray</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListViewArray.html" class="struct" title="struct arrow::array::GenericListViewArray">GenericListViewArray</a>\<OffsetSize\>

where OffsetSize: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#method.from-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListArray.html" class="struct" title="struct arrow::array::FixedSizeListArray">FixedSizeListArray</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListViewArray.html" class="struct" title="struct arrow::array::GenericListViewArray">GenericListViewArray</a>\<OffsetSize\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#impl-From%3CGenericListViewArray%3COffsetSize%3E%3E-for-ArrayData" class="anchor">§</a>

### impl\<OffsetSize\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListViewArray.html" class="struct" title="struct arrow::array::GenericListViewArray">GenericListViewArray</a>\<OffsetSize\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

where OffsetSize: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(array: <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListViewArray.html" class="struct" title="struct arrow::array::GenericListViewArray">GenericListViewArray</a>\<OffsetSize\>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#impl-PartialEq-for-GenericListViewArray%3COffsetSize%3E" class="anchor">§</a>

### impl\<OffsetSize\> <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListViewArray.html" class="struct" title="struct arrow::array::GenericListViewArray">GenericListViewArray</a>\<OffsetSize\>

where OffsetSize: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListViewArray.html" class="struct" title="struct arrow::array::GenericListViewArray">GenericListViewArray</a>\<OffsetSize\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html#blanket-implementations" class="anchor">§</a>
