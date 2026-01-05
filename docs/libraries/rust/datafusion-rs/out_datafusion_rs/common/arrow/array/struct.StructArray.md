# Struct StructArray Copy item path

<a href="https://docs.rs/arrow-array/56.0.0/x86_64-unknown-linux-gnu/src/arrow_array/array/struct_array.rs.html#77" class="src">Source</a>

``` rust
pub struct StructArray { /* private fields */ }
```

Expand description

An array of [structs](https://arrow.apache.org/docs/format/Columnar.html#struct-layout)

Each child (called *field*) is represented by a separate array.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#comparison-with-recordbatch" class="doc-anchor">§</a>Comparison with [RecordBatch](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch")

Both [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch") and [`StructArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html "struct datafusion::common::arrow::array::StructArray") represent a collection of columns / arrays with the same length.

However, there are a couple of key differences:

- [`StructArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html "struct datafusion::common::arrow::array::StructArray") can be nested within other [`Array`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html "trait datafusion::common::arrow::array::Array"), including itself
- [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch") can contain top-level metadata on its associated [`Schema`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html "struct datafusion::common::arrow::datatypes::Schema")
- [`StructArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html "struct datafusion::common::arrow::array::StructArray") can contain top-level nulls, i.e. `null`
- [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch") can only represent nulls in its child columns, i.e. `{"field": null}`

[`StructArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html "struct datafusion::common::arrow::array::StructArray") is therefore a more general data container than [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch"), and as such code that needs to handle both will typically share an implementation in terms of [`StructArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html "struct datafusion::common::arrow::array::StructArray") and convert to/from [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch") as necessary.

[`From`](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") implementations are provided to facilitate this conversion, however, converting from a [`StructArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html "struct datafusion::common::arrow::array::StructArray") containing top-level nulls to a [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch") will panic, as there is no way to preserve them.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#example-create-an-array-from-a-vector-of-fields" class="doc-anchor">§</a>Example: Create an array from a vector of fields

``` rust
use std::sync::Arc;
use arrow_array::{Array, ArrayRef, BooleanArray, Int32Array, StructArray};
use arrow_schema::{DataType, Field};

let boolean = Arc::new(BooleanArray::from(vec![false, false, true, true]));
let int = Arc::new(Int32Array::from(vec![42, 28, 19, 31]));

let struct_array = StructArray::from(vec![
    (
        Arc::new(Field::new("b", DataType::Boolean, false)),
        boolean.clone() as ArrayRef,
    ),
    (
        Arc::new(Field::new("c", DataType::Int32, false)),
        int.clone() as ArrayRef,
    ),
]);
assert_eq!(struct_array.column(0).as_ref(), boolean.as_ref());
assert_eq!(struct_array.column(1).as_ref(), int.as_ref());
assert_eq!(4, struct_array.len());
assert_eq!(0, struct_array.null_count());
assert_eq!(0, struct_array.offset());
```

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#impl-StructArray" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html" class="struct" title="struct datafusion::common::arrow::array::StructArray">StructArray</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#method.new" class="fn">new</a>( fields: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Fields.html" class="struct" title="struct datafusion::common::arrow::datatypes::Fields">Fields</a>, arrays: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\>, nulls: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct datafusion::common::arrow::buffer::NullBuffer">NullBuffer</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html" class="struct" title="struct datafusion::common::arrow::array::StructArray">StructArray</a>

Create a new [`StructArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html "struct datafusion::common::arrow::array::StructArray") from the provided parts, panicking on failure

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#panics" class="doc-anchor">§</a>Panics

Panics if [`Self::try_new`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#method.try_new "associated function datafusion::common::arrow::array::StructArray::try_new") returns an error

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#method.try_new" class="fn">try_new</a>( fields: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Fields.html" class="struct" title="struct datafusion::common::arrow::datatypes::Fields">Fields</a>, arrays: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\>, nulls: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct datafusion::common::arrow::buffer::NullBuffer">NullBuffer</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html" class="struct" title="struct datafusion::common::arrow::array::StructArray">StructArray</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

Create a new [`StructArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html "struct datafusion::common::arrow::array::StructArray") from the provided parts, returning an error on failure

The length will be inferred from the length of the child arrays. Returns an error if there are no child arrays. Consider using [`Self::try_new_with_length`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#method.try_new_with_length "associated function datafusion::common::arrow::array::StructArray::try_new_with_length") if the length is known to avoid this.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#errors" class="doc-anchor">§</a>Errors

Errors if

- `fields.len() == 0`
- Any reason that [`Self::try_new_with_length`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#method.try_new_with_length "associated function datafusion::common::arrow::array::StructArray::try_new_with_length") would error

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#method.try_new_with_length" class="fn">try_new_with_length</a>( fields: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Fields.html" class="struct" title="struct datafusion::common::arrow::datatypes::Fields">Fields</a>, arrays: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\>, nulls: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct datafusion::common::arrow::buffer::NullBuffer">NullBuffer</a>\>, len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html" class="struct" title="struct datafusion::common::arrow::array::StructArray">StructArray</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

Create a new [`StructArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html "struct datafusion::common::arrow::array::StructArray") from the provided parts, returning an error on failure

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#errors-1" class="doc-anchor">§</a>Errors

Errors if

- `fields.len() != arrays.len()`
- `fields[i].data_type() != arrays[i].data_type()`
- `arrays[i].len() != arrays[j].len()`
- `arrays[i].len() != nulls.len()`
- `!fields[i].is_nullable() && !nulls.contains(arrays[i].nulls())`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#method.new_null" class="fn">new_null</a>(fields: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Fields.html" class="struct" title="struct datafusion::common::arrow::datatypes::Fields">Fields</a>, len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html" class="struct" title="struct datafusion::common::arrow::array::StructArray">StructArray</a>

Create a new [`StructArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html "struct datafusion::common::arrow::array::StructArray") of length `len` where all values are null

#### pub unsafe fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#method.new_unchecked" class="fn">new_unchecked</a>( fields: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Fields.html" class="struct" title="struct datafusion::common::arrow::datatypes::Fields">Fields</a>, arrays: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\>, nulls: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct datafusion::common::arrow::buffer::NullBuffer">NullBuffer</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html" class="struct" title="struct datafusion::common::arrow::array::StructArray">StructArray</a>

Create a new [`StructArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html "struct datafusion::common::arrow::array::StructArray") from the provided parts without validation

The length will be inferred from the length of the child arrays. Panics if there are no child arrays. Consider using [`Self::new_unchecked_with_length`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#method.new_unchecked_with_length "associated function datafusion::common::arrow::array::StructArray::new_unchecked_with_length") if the length is known to avoid this.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#safety" class="doc-anchor">§</a>Safety

Safe if [`Self::new`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#method.new "associated function datafusion::common::arrow::array::StructArray::new") would not panic with the given arguments

#### pub unsafe fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#method.new_unchecked_with_length" class="fn">new_unchecked_with_length</a>( fields: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Fields.html" class="struct" title="struct datafusion::common::arrow::datatypes::Fields">Fields</a>, arrays: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\>, nulls: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct datafusion::common::arrow::buffer::NullBuffer">NullBuffer</a>\>, len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html" class="struct" title="struct datafusion::common::arrow::array::StructArray">StructArray</a>

Create a new [`StructArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html "struct datafusion::common::arrow::array::StructArray") from the provided parts without validation

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#safety-1" class="doc-anchor">§</a>Safety

Safe if [`Self::new`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#method.new "associated function datafusion::common::arrow::array::StructArray::new") would not panic with the given arguments

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#method.new_empty_fields" class="fn">new_empty_fields</a>(len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, nulls: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct datafusion::common::arrow::buffer::NullBuffer">NullBuffer</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html" class="struct" title="struct datafusion::common::arrow::array::StructArray">StructArray</a>

Create a new [`StructArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html "struct datafusion::common::arrow::array::StructArray") containing no fields

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#panics-1" class="doc-anchor">§</a>Panics

If `len != nulls.len()`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#method.into_parts" class="fn">into_parts</a>(self) -\> (<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Fields.html" class="struct" title="struct datafusion::common::arrow::datatypes::Fields">Fields</a>, <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct datafusion::common::arrow::buffer::NullBuffer">NullBuffer</a>\>)

Deconstruct this array into its constituent parts

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#method.column" class="fn">column</a>(&self, pos: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>

Returns the field at `pos`.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#method.num_columns" class="fn">num_columns</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Return the number of fields in this struct array

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#method.columns" class="fn">columns</a>(&self) -\> &\[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\]

Returns the fields of the struct array

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#method.column_names" class="fn">column_names</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Return field names in this struct array

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#method.fields" class="fn">fields</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Fields.html" class="struct" title="struct datafusion::common::arrow::datatypes::Fields">Fields</a>

Returns the [`Fields`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Fields.html "struct datafusion::common::arrow::datatypes::Fields") of this [`StructArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html "struct datafusion::common::arrow::array::StructArray")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#method.column_by_name" class="fn">column_by_name</a>(&self, column_name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\>

Return child array whose field name equals to column_name

Note: A schema can currently have duplicate field names, in which case the first field will always be selected. This issue will be addressed in [ARROW-11178](https://issues.apache.org/jira/browse/ARROW-11178)

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#method.slice" class="fn">slice</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html" class="struct" title="struct datafusion::common::arrow::array::StructArray">StructArray</a>

Returns a zero-copy slice of this array with the indicated offset and length.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#impl-Array-for-StructArray" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html" class="struct" title="struct datafusion::common::arrow::array::StructArray">StructArray</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#method.as_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the array as [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") so that it can be downcasted to a specific implementation. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.as_any)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#method.to_data" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.to_data" class="fn">to_data</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.ArrayData.html" class="struct" title="struct datafusion::common::arrow::array::ArrayData">ArrayData</a>

Returns the underlying data of this array

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#method.into_data" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.into_data" class="fn">into_data</a>(self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.ArrayData.html" class="struct" title="struct datafusion::common::arrow::array::ArrayData">ArrayData</a>

Returns the underlying data of this array [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.into_data)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#method.data_type" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.data_type" class="fn">data_type</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

Returns a reference to the [`DataType`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html "enum datafusion::common::arrow::datatypes::DataType") of this array. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.data_type)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#method.slice-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.slice" class="fn">slice</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>

Returns a zero-copy slice of this array with the indicated offset and length. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.slice)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#method.len" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the length (i.e., number of elements) of this array. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.len)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#method.is_empty" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether this array is empty. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.is_empty)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#method.shrink_to_fit" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.shrink_to_fit" class="fn">shrink_to_fit</a>(&mut self)

Shrinks the capacity of any exclusively owned buffer as much as possible [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.shrink_to_fit)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#method.offset" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.offset" class="fn">offset</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the offset into the underlying data used by this array(-slice). Note that the underlying data can be shared by many arrays. This defaults to `0`. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.offset)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#method.nulls" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.nulls" class="fn">nulls</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct datafusion::common::arrow::buffer::NullBuffer">NullBuffer</a>\>

Returns the null buffer of this array if any. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.nulls)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#method.logical_null_count" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.logical_null_count" class="fn">logical_null_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of logical null values in this array. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.logical_null_count)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#method.get_buffer_memory_size" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.get_buffer_memory_size" class="fn">get_buffer_memory_size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of bytes of memory pointed to by this array. The buffers store bytes in the Arrow memory format, and include the data as well as the validity map. Note that this does not always correspond to the exact memory usage of an array, since multiple arrays can share the same buffers or slices thereof.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#method.get_array_memory_size" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.get_array_memory_size" class="fn">get_array_memory_size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of bytes of memory occupied physically by this array. This value will always be greater than returned by `get_buffer_memory_size()` and includes the overhead of the data structures that contain the pointers to the various buffers.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#method.logical_nulls" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.logical_nulls" class="fn">logical_nulls</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct datafusion::common::arrow::buffer::NullBuffer">NullBuffer</a>\>

Returns a potentially computed [`NullBuffer`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.NullBuffer.html "struct datafusion::common::arrow::buffer::NullBuffer") that represents the logical null values of this array, if any. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.logical_nulls)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#method.is_null" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.is_null" class="fn">is_null</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether the element at `index` is null according to [`Array::nulls`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.nulls "method datafusion::common::arrow::array::Array::nulls") [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.is_null)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#method.is_valid" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.is_valid" class="fn">is_valid</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether the element at `index` is *not* null, the opposite of [`Self::is_null`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.is_null "method arrow_array::array::Array::is_null::is_null"). [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.is_valid)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#method.null_count" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.null_count" class="fn">null_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of physical null values in this array. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.null_count)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#method.is_nullable" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.is_nullable" class="fn">is_nullable</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns `false` if the array is guaranteed to not contain any logical nulls [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.is_nullable)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#impl-Clone-for-StructArray" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html" class="struct" title="struct datafusion::common::arrow::array::StructArray">StructArray</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html" class="struct" title="struct datafusion::common::arrow::array::StructArray">StructArray</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#impl-Debug-for-StructArray" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html" class="struct" title="struct datafusion::common::arrow::array::StructArray">StructArray</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#impl-From%3C%26StructArray%3E-for-RecordBatch" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html" class="struct" title="struct datafusion::common::arrow::array::StructArray">StructArray</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#method.from-6" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(struct_array: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html" class="struct" title="struct datafusion::common::arrow::array::StructArray">StructArray</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#impl-From%3C(Vec%3C(Arc%3CField%3E,+Arc%3Cdyn+Array%3E)%3E,+Buffer)%3E-for-StructArray" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<(<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<(<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>, <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>)\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.Buffer.html" class="struct" title="struct datafusion::common::arrow::buffer::Buffer">Buffer</a>)\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html" class="struct" title="struct datafusion::common::arrow::array::StructArray">StructArray</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#method.from-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(pair: (<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<(<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>, <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>)\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.Buffer.html" class="struct" title="struct datafusion::common::arrow::buffer::Buffer">Buffer</a>)) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html" class="struct" title="struct datafusion::common::arrow::array::StructArray">StructArray</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#impl-From%3CArrayData%3E-for-StructArray" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.ArrayData.html" class="struct" title="struct datafusion::common::arrow::array::ArrayData">ArrayData</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html" class="struct" title="struct datafusion::common::arrow::array::StructArray">StructArray</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(data: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.ArrayData.html" class="struct" title="struct datafusion::common::arrow::array::ArrayData">ArrayData</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html" class="struct" title="struct datafusion::common::arrow::array::StructArray">StructArray</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#impl-From%3CRecordBatch%3E-for-StructArray" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html" class="struct" title="struct datafusion::common::arrow::array::StructArray">StructArray</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#method.from-4" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html" class="struct" title="struct datafusion::common::arrow::array::StructArray">StructArray</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#impl-From%3CStructArray%3E-for-ArrayData" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html" class="struct" title="struct datafusion::common::arrow::array::StructArray">StructArray</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.ArrayData.html" class="struct" title="struct datafusion::common::arrow::array::ArrayData">ArrayData</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(array: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html" class="struct" title="struct datafusion::common::arrow::array::StructArray">StructArray</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.ArrayData.html" class="struct" title="struct datafusion::common::arrow::array::ArrayData">ArrayData</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#impl-From%3CStructArray%3E-for-RecordBatch" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html" class="struct" title="struct datafusion::common::arrow::array::StructArray">StructArray</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#method.from-5" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html" class="struct" title="struct datafusion::common::arrow::array::StructArray">StructArray</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#impl-From%3CVec%3C(Arc%3CField%3E,+Arc%3Cdyn+Array%3E)%3E%3E-for-StructArray" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<(<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>, <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>)\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html" class="struct" title="struct datafusion::common::arrow::array::StructArray">StructArray</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#method.from-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<(<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>, <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>)\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html" class="struct" title="struct datafusion::common::arrow::array::StructArray">StructArray</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#impl-Index%3C%26str%3E-for-StructArray" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html" class="trait" title="trait core::ops::index::Index">Index</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html" class="struct" title="struct datafusion::common::arrow::array::StructArray">StructArray</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#method.index" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index" class="fn">index</a>(&self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> &\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html" class="struct" title="struct datafusion::common::arrow::array::StructArray">StructArray</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html" class="trait" title="trait core::ops::index::Index">Index</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output" class="associatedtype" title="type core::ops::index::Index::Output">Output</a>

Get a reference to a column’s array by name.

Note: A schema can currently have duplicate field names, in which case the first field will always be selected. This issue will be addressed in [ARROW-11178](https://issues.apache.org/jira/browse/ARROW-11178)

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#panics-2" class="doc-anchor">§</a>Panics

Panics if the name is not in the schema.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#associatedtype.Output" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>

The returned type after indexing.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#impl-PartialEq-for-StructArray" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html" class="struct" title="struct datafusion::common::arrow::array::StructArray">StructArray</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html" class="struct" title="struct datafusion::common::arrow::array::StructArray">StructArray</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#impl-TryFrom%3CVec%3C(%26str,+Arc%3Cdyn+Array%3E)%3E%3E-for-StructArray" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<(&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>)\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html" class="struct" title="struct datafusion::common::arrow::array::StructArray">StructArray</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#method.try_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from" class="fn">try_from</a>( values: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<(&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>)\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html" class="struct" title="struct datafusion::common::arrow::array::StructArray">StructArray</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

builds a StructArray from a vector of names and arrays.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#associatedtype.Error" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>

The type returned in the event of a conversion error.

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html#blanket-implementations" class="anchor">§</a>
