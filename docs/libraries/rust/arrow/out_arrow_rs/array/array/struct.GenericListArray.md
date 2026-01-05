# Struct GenericListArray Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/array/list_array.rs.html#169" class="src">Source</a>

``` rust
pub struct GenericListArray<OffsetSize>where
    OffsetSize: OffsetSizeTrait,{ /* private fields */ }
```

Expand description

An array of [variable length lists](https://arrow.apache.org/docs/format/Columnar.html#variable-size-list-layout), similar to JSON arrays (e.g. `["A", "B", "C"]`). This struct specifically represents the [list layout](https://arrow.apache.org/docs/format/Columnar.html#list-layout). Refer to [`GenericListViewArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListViewArray.html "struct arrow::array::GenericListViewArray") for the [list-view layout](https://arrow.apache.org/docs/format/Columnar.html#listview-layout).

Lists are represented using `offsets` into a `values` child array. Offsets are stored in two adjacent entries of an [`OffsetBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.OffsetBuffer.html "struct arrow::buffer::OffsetBuffer").

Arrow defines [`ListArray`](https://docs.rs/arrow/latest/arrow/array/type.ListArray.html "type arrow::array::ListArray") with `i32` offsets and [`LargeListArray`](https://docs.rs/arrow/latest/arrow/array/type.LargeListArray.html "type arrow::array::LargeListArray") with `i64` offsets.

Use [`GenericListBuilder`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListBuilder.html "struct arrow::array::GenericListBuilder") to construct a [`GenericListArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html "struct arrow::array::GenericListArray").

## <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#representation" class="doc-anchor">§</a>Representation

A [`ListArray`](https://docs.rs/arrow/latest/arrow/array/type.ListArray.html "type arrow::array::ListArray") can represent a list of values of any other supported Arrow type. Each element of the `ListArray` itself is a list which may be empty, may contain NULL and non-null values, or may itself be NULL.

For example, the `ListArray` shown in the following diagram stores lists of strings. Note that `[]` represents an empty (length 0), but non NULL list.

``` text
┌─────────────┐
│   [A,B,C]   │
├─────────────┤
│     []      │
├─────────────┤
│    NULL     │
├─────────────┤
│     [D]     │
├─────────────┤
│  [NULL, F]  │
└─────────────┘
```

The `values` are stored in a child [`StringArray`](https://docs.rs/arrow/latest/arrow/array/type.StringArray.html "type arrow::array::StringArray") and the offsets are stored in an [`OffsetBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.OffsetBuffer.html "struct arrow::buffer::OffsetBuffer") as shown in the following diagram. The logical values and offsets are shown on the left, and the actual `ListArray` encoding on the right.

``` text
                                        ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
                                                                ┌ ─ ─ ─ ─ ─ ─ ┐    │
 ┌─────────────┐  ┌───────┐             │     ┌───┐   ┌───┐       ┌───┐ ┌───┐
 │   [A,B,C]   │  │ (0,3) │                   │ 1 │   │ 0 │     │ │ 1 │ │ A │ │ 0  │
 ├─────────────┤  ├───────┤             │     ├───┤   ├───┤       ├───┤ ├───┤
 │      []     │  │ (3,3) │                   │ 1 │   │ 3 │     │ │ 1 │ │ B │ │ 1  │
 ├─────────────┤  ├───────┤             │     ├───┤   ├───┤       ├───┤ ├───┤
 │    NULL     │  │ (3,4) │                   │ 0 │   │ 3 │     │ │ 1 │ │ C │ │ 2  │
 ├─────────────┤  ├───────┤             │     ├───┤   ├───┤       ├───┤ ├───┤
 │     [D]     │  │ (4,5) │                   │ 1 │   │ 4 │     │ │ ? │ │ ? │ │ 3  │
 ├─────────────┤  ├───────┤             │     ├───┤   ├───┤       ├───┤ ├───┤
 │  [NULL, F]  │  │ (5,7) │                   │ 1 │   │ 5 │     │ │ 1 │ │ D │ │ 4  │
 └─────────────┘  └───────┘             │     └───┘   ├───┤       ├───┤ ├───┤
                                                      │ 7 │     │ │ 0 │ │ ? │ │ 5  │
                                        │  Validity   └───┘       ├───┤ ├───┤
    Logical       Logical                  (nulls)   Offsets    │ │ 1 │ │ F │ │ 6  │
     Values       Offsets               │                         └───┘ └───┘
                                                                │    Values   │    │
                (offsets[i],            │   ListArray               (Array)
               offsets[i+1])                                    └ ─ ─ ─ ─ ─ ─ ┘    │
                                        └ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
```

## <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#slicing" class="doc-anchor">§</a>Slicing

Slicing a `ListArray` creates a new `ListArray` without copying any data, but this means the [`Self::values`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html#method.values "method arrow::array::GenericListArray::values") and [`Self::offsets`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html#method.offsets "method arrow::array::GenericListArray::offsets") may have “unused” data

For example, calling `slice(1, 3)` on the `ListArray` in the above example would result in the following. Note

1.  `Values` array is unchanged
2.  `Offsets` do not start at `0`, nor cover all values in the Values array.

``` text
                                ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
                                                        ┌ ─ ─ ─ ─ ─ ─ ┐    │  ╔═══╗
                                │                         ╔═══╗ ╔═══╗         ║   ║  Not used
                                                        │ ║ 1 ║ ║ A ║ │ 0  │  ╚═══╝
 ┌─────────────┐  ┌───────┐     │     ┌───┐   ┌───┐       ╠═══╣ ╠═══╣
 │ [] (empty)  │  │ (3,3) │           │ 1 │   │ 3 │     │ ║ 1 ║ ║ B ║ │ 1  │
 ├─────────────┤  ├───────┤     │     ├───┤   ├───┤       ╠═══╣ ╠═══╣
 │    NULL     │  │ (3,4) │           │ 0 │   │ 3 │     │ ║ 1 ║ ║ C ║ │ 2  │
 ├─────────────┤  ├───────┤     │     ├───┤   ├───┤       ╠───╣ ╠───╣
 │     [D]     │  │ (4,5) │           │ 1 │   │ 4 │     │ │ 0 │ │ ? │ │ 3  │
 └─────────────┘  └───────┘     │     └───┘   ├───┤       ├───┤ ├───┤
                                              │ 5 │     │ │ 1 │ │ D │ │ 4  │
                                │             └───┘       ├───┤ ├───┤
                                                        │ │ 0 │ │ ? │ │ 5  │
                                │  Validity               ╠═══╣ ╠═══╣
    Logical       Logical          (nulls)   Offsets    │ ║ 1 ║ ║ F ║ │ 6  │
     Values       Offsets       │                         ╚═══╝ ╚═══╝
                                                        │    Values   │    │
                (offsets[i],    │   ListArray               (Array)
               offsets[i+1])                            └ ─ ─ ─ ─ ─ ─ ┘    │
                                └ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
```

## Implementations<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#impl-GenericListArray%3COffsetSize%3E" class="anchor">§</a>

### impl\<OffsetSize\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html" class="struct" title="struct arrow::array::GenericListArray">GenericListArray</a>\<OffsetSize\>

where OffsetSize: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

#### pub const <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#associatedconstant.DATA_TYPE_CONSTRUCTOR" class="constant">DATA_TYPE_CONSTRUCTOR</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.fn.html" class="primitive">fn</a>(<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>\>) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a>

The data type constructor of list array. The input is the schema of the child array and the output is the [`DataType`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html "enum arrow::datatypes::DataType"), List or LargeList.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#method.try_new" class="fn">try_new</a>( field: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>\>, offsets: <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.OffsetBuffer.html" class="struct" title="struct arrow::buffer::OffsetBuffer">OffsetBuffer</a>\<OffsetSize\>, values: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>, nulls: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html" class="struct" title="struct arrow::array::GenericListArray">GenericListArray</a>\<OffsetSize\>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Create a new [`GenericListArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html "struct arrow::array::GenericListArray") from the provided parts

##### <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#errors" class="doc-anchor">§</a>Errors

Errors if

- `offsets.len() - 1 != nulls.len()`
- `offsets.last() > values.len()`
- `!field.is_nullable() && values.is_nullable()`
- `field.data_type() != values.data_type()`

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#method.new" class="fn">new</a>( field: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>\>, offsets: <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.OffsetBuffer.html" class="struct" title="struct arrow::buffer::OffsetBuffer">OffsetBuffer</a>\<OffsetSize\>, values: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>, nulls: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html" class="struct" title="struct arrow::array::GenericListArray">GenericListArray</a>\<OffsetSize\>

Create a new [`GenericListArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html "struct arrow::array::GenericListArray") from the provided parts

##### <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#panics" class="doc-anchor">§</a>Panics

Panics if [`Self::try_new`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html#method.try_new "associated function arrow::array::GenericListArray::try_new") returns an error

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#method.new_null" class="fn">new_null</a>(field: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>\>, len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html" class="struct" title="struct arrow::array::GenericListArray">GenericListArray</a>\<OffsetSize\>

Create a new [`GenericListArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html "struct arrow::array::GenericListArray") of length `len` where all values are null

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#method.into_parts" class="fn">into_parts</a>( self, ) -\> (<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>\>, <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.OffsetBuffer.html" class="struct" title="struct arrow::buffer::OffsetBuffer">OffsetBuffer</a>\<OffsetSize\>, <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>\>)

Deconstruct this array into its constituent parts

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#method.offsets" class="fn">offsets</a>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.OffsetBuffer.html" class="struct" title="struct arrow::buffer::OffsetBuffer">OffsetBuffer</a>\<OffsetSize\>

Returns a reference to the offsets of this list

Unlike [`Self::value_offsets`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html#method.value_offsets "method arrow::array::GenericListArray::value_offsets") this returns the [`OffsetBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.OffsetBuffer.html "struct arrow::buffer::OffsetBuffer") allowing for zero-copy cloning.

Notes: The `offsets` may not start at 0 and may not cover all values in [`Self::values`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html#method.values "method arrow::array::GenericListArray::values"). This can happen when the list array was sliced via [`Self::slice`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html#method.slice "method arrow::array::GenericListArray::slice"). See documentation for [`Self`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html "struct arrow::array::GenericListArray") for more details.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#method.values" class="fn">values</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

Returns a reference to the values of this list

Note: The list array may not refer to all values in the `values` array. For example if the list array was sliced via [`Self::slice`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html#method.slice "method arrow::array::GenericListArray::slice") values will still contain values both before and after the slice. See documentation for [`Self`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html "struct arrow::array::GenericListArray") for more details.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#method.value_type" class="fn">value_type</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a>

Returns a clone of the value type of this list.

#### pub unsafe fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#method.value_unchecked" class="fn">value_unchecked</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

Returns ith value of this list array.

Note: This method does not check for nulls and the value is arbitrary if [`is_null`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html#method.is_null "method arrow::array::GenericListArray::is_null") returns true for the index.

##### <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#safety" class="doc-anchor">§</a>Safety

Caller must ensure that the index is within the array bounds

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#method.value" class="fn">value</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

Returns ith value of this list array.

Note: This method does not check for nulls and the value is arbitrary (but still well-defined) if [`is_null`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html#method.is_null "method arrow::array::GenericListArray::is_null") returns true for the index.

##### <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#panics-1" class="doc-anchor">§</a>Panics

Panics if index `i` is out of bounds

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#method.value_offsets" class="fn">value_offsets</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[OffsetSize]</a>

Returns the offset values in the offsets buffer.

See [`Self::offsets`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html#method.offsets "method arrow::array::GenericListArray::offsets") for more details.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#method.value_length" class="fn">value_length</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> OffsetSize

Returns the length for value at index `i`.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#method.iter" class="fn">iter</a>\<'a\>(&'a self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayIter.html" class="struct" title="struct arrow::array::ArrayIter">ArrayIter</a>\<&'a <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html" class="struct" title="struct arrow::array::GenericListArray">GenericListArray</a>\<OffsetSize\>\> <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#" class="tooltip" data-notable-ty="ArrayIter&lt;&amp;&#39;a GenericListArray&lt;OffsetSize&gt;&gt;">ⓘ</a>

constructs a new iterator

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#method.slice" class="fn">slice</a>( &self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html" class="struct" title="struct arrow::array::GenericListArray">GenericListArray</a>\<OffsetSize\>

Returns a zero-copy slice of this array with the indicated offset and length.

Notes: this method does *NOT* slice the underlying values array or modify the values in the offsets buffer. See [`Self::values`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html#method.values "method arrow::array::GenericListArray::values") and [`Self::offsets`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html#method.offsets "method arrow::array::GenericListArray::offsets") for more information.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#method.from_iter_primitive" class="fn">from_iter_primitive</a>\<T, P, I\>(iter: I) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html" class="struct" title="struct arrow::array::GenericListArray">GenericListArray</a>\<OffsetSize\>

where T: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>, P: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<T as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>, I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<P\>\>,

Creates a [`GenericListArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html "struct arrow::array::GenericListArray") from an iterator of primitive values

##### <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#example" class="doc-anchor">§</a>Example

``` rust

let data = vec![
   Some(vec![Some(0), Some(1), Some(2)]),
   None,
   Some(vec![Some(3), None, Some(5)]),
   Some(vec![Some(6), Some(7)]),
];
let list_array = ListArray::from_iter_primitive::<Int32Type, _, _>(data);
println!("{:?}", list_array);
```

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#impl-Array-for-GenericListArray%3COffsetSize%3E" class="anchor">§</a>

### impl\<OffsetSize\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html" class="struct" title="struct arrow::array::GenericListArray">GenericListArray</a>\<OffsetSize\>

where OffsetSize: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#method.as_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the array as [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") so that it can be downcasted to a specific implementation. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.as_any)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#method.to_data" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.to_data" class="fn">to_data</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

Returns the underlying data of this array

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#method.into_data" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.into_data" class="fn">into_data</a>(self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

Returns the underlying data of this array [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.into_data)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#method.data_type" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.data_type" class="fn">data_type</a>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a>

Returns a reference to the [`DataType`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html "enum arrow::datatypes::DataType") of this array. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.data_type)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#method.slice-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.slice" class="fn">slice</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

Returns a zero-copy slice of this array with the indicated offset and length. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.slice)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#method.len" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the length (i.e., number of elements) of this array. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.len)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#method.is_empty" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether this array is empty. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.is_empty)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#method.shrink_to_fit" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.shrink_to_fit" class="fn">shrink_to_fit</a>(&mut self)

Shrinks the capacity of any exclusively owned buffer as much as possible [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.shrink_to_fit)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#method.offset" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.offset" class="fn">offset</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the offset into the underlying data used by this array(-slice). Note that the underlying data can be shared by many arrays. This defaults to `0`. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.offset)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#method.nulls" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.nulls" class="fn">nulls</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>\>

Returns the null buffer of this array if any. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.nulls)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#method.logical_null_count" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.logical_null_count" class="fn">logical_null_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of logical null values in this array. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.logical_null_count)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#method.get_buffer_memory_size" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.get_buffer_memory_size" class="fn">get_buffer_memory_size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of bytes of memory pointed to by this array. The buffers store bytes in the Arrow memory format, and include the data as well as the validity map. Note that this does not always correspond to the exact memory usage of an array, since multiple arrays can share the same buffers or slices thereof.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#method.get_array_memory_size" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.get_array_memory_size" class="fn">get_array_memory_size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of bytes of memory occupied physically by this array. This value will always be greater than returned by `get_buffer_memory_size()` and includes the overhead of the data structures that contain the pointers to the various buffers.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#method.logical_nulls" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.logical_nulls" class="fn">logical_nulls</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>\>

Returns a potentially computed [`NullBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html "struct arrow::buffer::NullBuffer") that represents the logical null values of this array, if any. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.logical_nulls)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#method.is_null" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_null" class="fn">is_null</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether the element at `index` is null according to [`Array::nulls`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.nulls "method arrow::array::Array::nulls") [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_null)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#method.is_valid" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_valid" class="fn">is_valid</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether the element at `index` is *not* null, the opposite of [`Self::is_null`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_null "method arrow_array::array::Array::is_null::is_null"). [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_valid)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#method.null_count" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.null_count" class="fn">null_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of physical null values in this array. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.null_count)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#method.is_nullable" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_nullable" class="fn">is_nullable</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns `false` if the array is guaranteed to not contain any logical nulls [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_nullable)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#impl-ArrayAccessor-for-%26GenericListArray%3COffsetSize%3E" class="anchor">§</a>

### impl\<OffsetSize\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayAccessor.html" class="trait" title="trait arrow::array::ArrayAccessor">ArrayAccessor</a> for &<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html" class="struct" title="struct arrow::array::GenericListArray">GenericListArray</a>\<OffsetSize\>

where OffsetSize: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#associatedtype.Item" class="anchor">§</a>

#### type <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayAccessor.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

The Arrow type of the element being accessed.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#method.value-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayAccessor.html#tymethod.value" class="fn">value</a>( &self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> \<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html" class="struct" title="struct arrow::array::GenericListArray">GenericListArray</a>\<OffsetSize\> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayAccessor.html" class="trait" title="trait arrow::array::ArrayAccessor">ArrayAccessor</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayAccessor.html#associatedtype.Item" class="associatedtype" title="type arrow::array::ArrayAccessor::Item">Item</a>

Returns the element at index `i` [Read more](https://docs.rs/arrow/latest/arrow/array/trait.ArrayAccessor.html#tymethod.value)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#method.value_unchecked-1" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayAccessor.html#tymethod.value_unchecked" class="fn">value_unchecked</a>( &self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> \<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html" class="struct" title="struct arrow::array::GenericListArray">GenericListArray</a>\<OffsetSize\> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayAccessor.html" class="trait" title="trait arrow::array::ArrayAccessor">ArrayAccessor</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayAccessor.html#associatedtype.Item" class="associatedtype" title="type arrow::array::ArrayAccessor::Item">Item</a>

Returns the element at index `i` [Read more](https://docs.rs/arrow/latest/arrow/array/trait.ArrayAccessor.html#tymethod.value_unchecked)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#impl-Clone-for-GenericListArray%3COffsetSize%3E" class="anchor">§</a>

### impl\<OffsetSize\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html" class="struct" title="struct arrow::array::GenericListArray">GenericListArray</a>\<OffsetSize\>

where OffsetSize: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html" class="struct" title="struct arrow::array::GenericListArray">GenericListArray</a>\<OffsetSize\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#impl-Debug-for-GenericListArray%3COffsetSize%3E" class="anchor">§</a>

### impl\<OffsetSize\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html" class="struct" title="struct arrow::array::GenericListArray">GenericListArray</a>\<OffsetSize\>

where OffsetSize: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#impl-From%3CArrayData%3E-for-GenericListArray%3COffsetSize%3E" class="anchor">§</a>

### impl\<OffsetSize\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html" class="struct" title="struct arrow::array::GenericListArray">GenericListArray</a>\<OffsetSize\>

where OffsetSize: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(data: <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html" class="struct" title="struct arrow::array::GenericListArray">GenericListArray</a>\<OffsetSize\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#impl-From%3CFixedSizeListArray%3E-for-GenericListArray%3COffsetSize%3E" class="anchor">§</a>

### impl\<OffsetSize\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListArray.html" class="struct" title="struct arrow::array::FixedSizeListArray">FixedSizeListArray</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html" class="struct" title="struct arrow::array::GenericListArray">GenericListArray</a>\<OffsetSize\>

where OffsetSize: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#method.from-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListArray.html" class="struct" title="struct arrow::array::FixedSizeListArray">FixedSizeListArray</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html" class="struct" title="struct arrow::array::GenericListArray">GenericListArray</a>\<OffsetSize\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#impl-From%3CGenericListArray%3COffsetSize%3E%3E-for-ArrayData" class="anchor">§</a>

### impl\<OffsetSize\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html" class="struct" title="struct arrow::array::GenericListArray">GenericListArray</a>\<OffsetSize\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

where OffsetSize: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#method.from-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(array: <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html" class="struct" title="struct arrow::array::GenericListArray">GenericListArray</a>\<OffsetSize\>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#impl-From%3CGenericListArray%3COffsetSize%3E%3E-for-GenericByteArray%3CGenericStringType%3COffsetSize%3E%3E" class="anchor">§</a>

### impl\<OffsetSize\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html" class="struct" title="struct arrow::array::GenericListArray">GenericListArray</a>\<OffsetSize\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteArray.html" class="struct" title="struct arrow::array::GenericByteArray">GenericByteArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.GenericStringType.html" class="struct" title="struct arrow::datatypes::GenericStringType">GenericStringType</a>\<OffsetSize\>\>

where OffsetSize: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#method.from-5" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( v: <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html" class="struct" title="struct arrow::array::GenericListArray">GenericListArray</a>\<OffsetSize\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteArray.html" class="struct" title="struct arrow::array::GenericByteArray">GenericByteArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.GenericStringType.html" class="struct" title="struct arrow::datatypes::GenericStringType">GenericStringType</a>\<OffsetSize\>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#impl-From%3CGenericListArray%3CT%3E%3E-for-GenericByteArray%3CGenericBinaryType%3CT%3E%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html" class="struct" title="struct arrow::array::GenericListArray">GenericListArray</a>\<T\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteArray.html" class="struct" title="struct arrow::array::GenericByteArray">GenericByteArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.GenericBinaryType.html" class="struct" title="struct arrow::datatypes::GenericBinaryType">GenericBinaryType</a>\<T\>\>

where T: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html" class="struct" title="struct arrow::array::GenericListArray">GenericListArray</a>\<T\>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteArray.html" class="struct" title="struct arrow::array::GenericByteArray">GenericByteArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.GenericBinaryType.html" class="struct" title="struct arrow::datatypes::GenericBinaryType">GenericBinaryType</a>\<T\>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#impl-From%3CMapArray%3E-for-GenericListArray%3Ci32%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.MapArray.html" class="struct" title="struct arrow::array::MapArray">MapArray</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html" class="struct" title="struct arrow::array::GenericListArray">GenericListArray</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#method.from-4" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/arrow/latest/arrow/array/struct.MapArray.html" class="struct" title="struct arrow::array::MapArray">MapArray</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html" class="struct" title="struct arrow::array::GenericListArray">GenericListArray</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#impl-PartialEq-for-GenericListArray%3COffsetSize%3E" class="anchor">§</a>

### impl\<OffsetSize\> <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html" class="struct" title="struct arrow::array::GenericListArray">GenericListArray</a>\<OffsetSize\>

where OffsetSize: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html" class="struct" title="struct arrow::array::GenericListArray">GenericListArray</a>\<OffsetSize\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html#blanket-implementations" class="anchor">§</a>
