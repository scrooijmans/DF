# Struct StructArray Copy item path

<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/src/polars_arrow/array/struct_/mod.rs.html#31" class="src">Source</a>

``` rust
pub struct StructArray { /* private fields */ }
```

Available on **crate feature `polars-io`** only.

Expand description

A [`StructArray`](https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html "struct polars::prelude::StructArray") is a nested [`Array`](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html "trait polars_arrow::array::Array") with an optional validity representing multiple [`Array`](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html "trait polars_arrow::array::Array") with the same number of rows.

## <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#example" class="doc-anchor">§</a>Example

``` rust
use polars_arrow::array::*;
use polars_arrow::datatypes::*;
let boolean = BooleanArray::from_slice(&[false, false, true, true]).boxed();
let int = Int32Array::from_slice(&[42, 28, 19, 31]).boxed();

let fields = vec![
    Field::new("b".into(), ArrowDataType::Boolean, false),
    Field::new("c".into(), ArrowDataType::Int32, false),
];

let array = StructArray::new(ArrowDataType::Struct(fields), 4, vec![boolean, int], None);
```

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#impl-StructArray" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.iter" class="fn">iter</a>( &'a self, ) -\> <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/bitmap/utils/zip_validity/enum.ZipValidity.html" class="enum" title="enum polars_arrow::bitmap::utils::zip_validity::ZipValidity">ZipValidity</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/scalar/trait.Scalar.html" class="trait" title="trait polars_arrow::scalar::Scalar">Scalar</a>\>\>, StructValueIter\<'a\>, <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/bitmap/utils/iterator/struct.BitmapIter.html" class="struct" title="struct polars_arrow::bitmap::utils::iterator::BitmapIter">BitmapIter</a>\<'a\>\>

Returns an iterator of `Option<Box<dyn Array>>`

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.values_iter" class="fn">values_iter</a>(&'a self) -\> StructValueIter\<'a\>

Returns an iterator of `Box<dyn Array>`

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#impl-StructArray-1" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.try_new" class="fn">try_new</a>( dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, values: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>\>, validity: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/bitmap/immutable/struct.Bitmap.html" class="struct" title="struct polars_arrow::bitmap::immutable::Bitmap">Bitmap</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Returns a new [`StructArray`](https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html "struct polars::prelude::StructArray").

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#errors" class="doc-anchor">§</a>Errors

This function errors iff:

- `dtype`’s physical type is not [`crate::datatypes::PhysicalType::Struct`](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/datatypes/physical_type/enum.PhysicalType.html#variant.Struct "variant polars_arrow::datatypes::physical_type::PhysicalType::Struct").
- the children of `dtype` are empty
- the values’s len is different from children’s length
- any of the values’s data type is different from its corresponding children’ data type
- any element of values has a different length than the first element
- the validity’s length is not equal to the length of the first element

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.new" class="fn">new</a>( dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, values: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>\>, validity: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/bitmap/immutable/struct.Bitmap.html" class="struct" title="struct polars_arrow::bitmap::immutable::Bitmap">Bitmap</a>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a>

Returns a new [`StructArray`](https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html "struct polars::prelude::StructArray")

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#panics" class="doc-anchor">§</a>Panics

This function panics iff:

- `dtype`’s physical type is not [`crate::datatypes::PhysicalType::Struct`](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/datatypes/physical_type/enum.PhysicalType.html#variant.Struct "variant polars_arrow::datatypes::physical_type::PhysicalType::Struct").
- the children of `dtype` are empty
- the values’s len is different from children’s length
- any of the values’s data type is different from its corresponding children’ data type
- any element of values has a different length than the first element
- the validity’s length is not equal to the length of the first element

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.new_empty" class="fn">new_empty</a>(dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a>

Creates an empty [`StructArray`](https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html "struct polars::prelude::StructArray").

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.new_null" class="fn">new_null</a>(dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a>

Creates a null [`StructArray`](https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html "struct polars::prelude::StructArray") of length `length`.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#impl-StructArray-2" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.into_data" class="fn">into_data</a>( self, ) -\> (<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrowField.html" class="struct" title="struct polars::prelude::ArrowField">Field</a>\>, <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>\>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/bitmap/immutable/struct.Bitmap.html" class="struct" title="struct polars_arrow::bitmap::immutable::Bitmap">Bitmap</a>\>)

Deconstructs the [`StructArray`](https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html "struct polars::prelude::StructArray") into its individual components.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.slice" class="fn">slice</a>(&mut self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Slices this [`StructArray`](https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html "struct polars::prelude::StructArray").

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#panics-1" class="doc-anchor">§</a>Panics

panics iff `offset + length > self.len()`

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#implementation" class="doc-anchor">§</a>Implementation

This operation is `O(F)` where `F` is the number of fields.

#### pub unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.slice_unchecked" class="fn">slice_unchecked</a>(&mut self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Slices this [`StructArray`](https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html "struct polars::prelude::StructArray").

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#implementation-1" class="doc-anchor">§</a>Implementation

This operation is `O(F)` where `F` is the number of fields.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#safety" class="doc-anchor">§</a>Safety

The caller must ensure that `offset + length <= self.len()`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.sliced" class="fn">sliced</a>(self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a>

Returns this array sliced.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#implementation-2" class="doc-anchor">§</a>Implementation

This function is `O(1)`.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#panics-2" class="doc-anchor">§</a>Panics

iff `offset + length > self.len()`.

#### pub unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.sliced_unchecked" class="fn">sliced_unchecked</a>( self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a>

Returns this array sliced.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#implementation-3" class="doc-anchor">§</a>Implementation

This function is `O(1)`.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#safety-1" class="doc-anchor">§</a>Safety

The caller must ensure that `offset + length <= self.len()`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.with_validity" class="fn">with_validity</a>(self, validity: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/bitmap/immutable/struct.Bitmap.html" class="struct" title="struct polars_arrow::bitmap::immutable::Bitmap">Bitmap</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a>

Returns this array with a new validity.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#panic" class="doc-anchor">§</a>Panic

Panics iff `validity.len() != self.len()`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.set_validity" class="fn">set_validity</a>(&mut self, validity: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/bitmap/immutable/struct.Bitmap.html" class="struct" title="struct polars_arrow::bitmap::immutable::Bitmap">Bitmap</a>\>)

Sets the validity of this array.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#panics-3" class="doc-anchor">§</a>Panics

This function panics iff `values.len() != self.len()`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.take_validity" class="fn">take_validity</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/bitmap/immutable/struct.Bitmap.html" class="struct" title="struct polars_arrow::bitmap::immutable::Bitmap">Bitmap</a>\>

Takes the validity of this array, leaving it without a validity mask.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.boxed" class="fn">boxed</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>

Boxes this array into a [`Box<dyn Array>`](https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html "struct alloc::boxed::Box").

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.arced" class="fn">arced</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>

Arcs this array into a [`std::sync::Arc<dyn Array>`](https://docs.rs/polars/latest/polars/prelude/struct.Arc.html "struct polars::prelude::Arc").

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#impl-StructArray-3" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.validity" class="fn">validity</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/bitmap/immutable/struct.Bitmap.html" class="struct" title="struct polars_arrow::bitmap::immutable::Bitmap">Bitmap</a>\>

The optional validity.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.values" class="fn">values</a>(&self) -\> &\[<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>\]

Returns the values of this [`StructArray`](https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html "struct polars::prelude::StructArray").

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.fields" class="fn">fields</a>(&self) -\> &\[<a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrowField.html" class="struct" title="struct polars::prelude::ArrowField">Field</a>\]

Returns the fields of this [`StructArray`](https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html "struct polars::prelude::StructArray").

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#impl-StructArray-4" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.get_fields" class="fn">get_fields</a>(dtype: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>) -\> &\[<a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrowField.html" class="struct" title="struct polars::prelude::ArrowField">Field</a>\]

Returns the fields the `DataType::Struct`.

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#impl-Array-for-StructArray" class="anchor">§</a>

### impl <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.as_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Converts itself to a reference of [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"), which enables downcasting to concrete types.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.as_any_mut" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#tymethod.as_any_mut" class="fn">as_any_mut</a>(&mut self) -\> &mut (dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Converts itself to a mutable reference of [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"), which enables mutable downcasting to concrete types.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.len-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#tymethod.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

The length of the [`Array`](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html "trait polars_arrow::array::Array"). Every array has a length corresponding to the number of elements (slots).

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.dtype" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#tymethod.dtype" class="fn">dtype</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>

The [`ArrowDataType`](https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html "enum polars::prelude::ArrowDataType") of the [`Array`](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html "trait polars_arrow::array::Array"). In combination with [`Array::as_any`](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#tymethod.as_any "method polars_arrow::array::Array::as_any"), this can be used to downcast trait objects (`dyn Array`) to concrete arrays.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.split_at_boxed" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#tymethod.split_at_boxed" class="fn">split_at_boxed</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> (<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>, <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>)

Split [`Self`](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html "trait polars_arrow::array::Array") at `offset` into two boxed [`Array`](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html "trait polars_arrow::array::Array")s where `offset <= self.len()`.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.split_at_boxed_unchecked" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#tymethod.split_at_boxed_unchecked" class="fn">split_at_boxed_unchecked</a>( &self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> (<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>, <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>)

Split [`Self`](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html "trait polars_arrow::array::Array") at `offset` into two boxed [`Array`](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html "trait polars_arrow::array::Array")s without checking `offset <= self.len()`. [Read more](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#tymethod.split_at_boxed_unchecked)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.slice-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#tymethod.slice" class="fn">slice</a>(&mut self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Slices this [`Array`](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html "trait polars_arrow::array::Array"). [Read more](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#tymethod.slice)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.slice_unchecked-1" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#tymethod.slice_unchecked" class="fn">slice_unchecked</a>(&mut self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Slices the [`Array`](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html "trait polars_arrow::array::Array"). [Read more](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#tymethod.slice_unchecked)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.to_boxed" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#tymethod.to_boxed" class="fn">to_boxed</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>

Clone a `&dyn Array` to an owned `Box<dyn Array>`.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.validity-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#tymethod.validity" class="fn">validity</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/bitmap/immutable/struct.Bitmap.html" class="struct" title="struct polars_arrow::bitmap::immutable::Bitmap">Bitmap</a>\>

The validity of the [`Array`](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html "trait polars_arrow::array::Array"): every array has an optional [`Bitmap`](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/bitmap/immutable/struct.Bitmap.html "struct polars_arrow::bitmap::immutable::Bitmap") that, when available specifies whether the array slot is valid or not (null). When the validity is [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None"), all slots are valid.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.with_validity-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#tymethod.with_validity" class="fn">with_validity</a>(&self, validity: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/bitmap/immutable/struct.Bitmap.html" class="struct" title="struct polars_arrow::bitmap::immutable::Bitmap">Bitmap</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>

Clones this [`Array`](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html "trait polars_arrow::array::Array") with a new assigned bitmap. [Read more](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#tymethod.with_validity)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.is_empty" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

whether the array is empty

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.null_count" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#method.null_count" class="fn">null_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

The number of null slots on this [`Array`](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html "trait polars_arrow::array::Array"). [Read more](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#method.null_count)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.has_nulls" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#method.has_nulls" class="fn">has_nulls</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.is_null" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#method.is_null" class="fn">is_null</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether slot `i` is null. [Read more](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#method.is_null)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.is_null_unchecked" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#method.is_null_unchecked" class="fn">is_null_unchecked</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether slot `i` is null. [Read more](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#method.is_null_unchecked)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.is_valid" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#method.is_valid" class="fn">is_valid</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether slot `i` is valid. [Read more](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#method.is_valid)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.sliced-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#method.sliced" class="fn">sliced</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>

Returns a slice of this [`Array`](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html "trait polars_arrow::array::Array"). [Read more](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#method.sliced)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.sliced_unchecked-1" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#method.sliced_unchecked" class="fn">sliced_unchecked</a>( &self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>

Returns a slice of this [`Array`](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html "trait polars_arrow::array::Array"). [Read more](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#method.sliced_unchecked)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#impl-ArrayFromIter%3C()%3E-for-StructArray" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIter.html" class="trait" title="trait polars::prelude::ArrayFromIter">ArrayFromIter</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.arr_from_iter-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIter.html#tymethod.arr_from_iter" class="fn">arr_from_iter</a>\<I\>(\_iter: I) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.try_arr_from_iter-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIter.html#tymethod.try_arr_from_iter" class="fn">try_arr_from_iter</a>\<E, I\>(\_iter: I) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a>, E\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, E\>\>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.arr_from_iter_trusted-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIter.html#method.arr_from_iter_trusted" class="fn">arr_from_iter_trusted</a>\<I\>(iter: I) -\> Self

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = T\>, \<I as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::IntoIter">IntoIter</a>: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/trusted_len/trait.TrustedLen.html" class="trait" title="trait polars_arrow::trusted_len::TrustedLen">TrustedLen</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.try_arr_from_iter_trusted-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIter.html#method.try_arr_from_iter_trusted" class="fn">try_arr_from_iter_trusted</a>\<E, I\>(iter: I) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, E\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<T, E\>\>, \<I as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::IntoIter">IntoIter</a>: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/trusted_len/trait.TrustedLen.html" class="trait" title="trait polars_arrow::trusted_len::TrustedLen">TrustedLen</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#impl-ArrayFromIter%3COption%3C()%3E%3E-for-StructArray" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIter.html" class="trait" title="trait polars::prelude::ArrayFromIter">ArrayFromIter</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.arr_from_iter" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIter.html#tymethod.arr_from_iter" class="fn">arr_from_iter</a>\<I\>(\_iter: I) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.try_arr_from_iter" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIter.html#tymethod.try_arr_from_iter" class="fn">try_arr_from_iter</a>\<E, I\>(\_iter: I) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a>, E\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>, E\>\>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.arr_from_iter_trusted" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIter.html#method.arr_from_iter_trusted" class="fn">arr_from_iter_trusted</a>\<I\>(iter: I) -\> Self

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = T\>, \<I as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::IntoIter">IntoIter</a>: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/trusted_len/trait.TrustedLen.html" class="trait" title="trait polars_arrow::trusted_len::TrustedLen">TrustedLen</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.try_arr_from_iter_trusted" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIter.html#method.try_arr_from_iter_trusted" class="fn">try_arr_from_iter_trusted</a>\<E, I\>(iter: I) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, E\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<T, E\>\>, \<I as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::IntoIter">IntoIter</a>: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/trusted_len/trait.TrustedLen.html" class="trait" title="trait polars_arrow::trusted_len::TrustedLen">TrustedLen</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#impl-ArrayFromIterDtype%3C()%3E-for-StructArray" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIterDtype.html" class="trait" title="trait polars::prelude::ArrayFromIterDtype">ArrayFromIterDtype</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.arr_from_iter_with_dtype" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIterDtype.html#tymethod.arr_from_iter_with_dtype" class="fn">arr_from_iter_with_dtype</a>\<I\>(\_dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>, \_iter: I) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.try_arr_from_iter_with_dtype" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIterDtype.html#tymethod.try_arr_from_iter_with_dtype" class="fn">try_arr_from_iter_with_dtype</a>\<E, I\>( \_dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>, \_iter: I, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a>, E\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, E\>\>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.arr_from_iter_trusted_with_dtype" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIterDtype.html#method.arr_from_iter_trusted_with_dtype" class="fn">arr_from_iter_trusted_with_dtype</a>\<I\>(dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>, iter: I) -\> Self

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = T\>, \<I as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::IntoIter">IntoIter</a>: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/trusted_len/trait.TrustedLen.html" class="trait" title="trait polars_arrow::trusted_len::TrustedLen">TrustedLen</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.try_arr_from_iter_trusted_with_dtype" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIterDtype.html#method.try_arr_from_iter_trusted_with_dtype" class="fn">try_arr_from_iter_trusted_with_dtype</a>\<E, I\>( dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>, iter: I, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, E\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<T, E\>\>, \<I as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::IntoIter">IntoIter</a>: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/trusted_len/trait.TrustedLen.html" class="trait" title="trait polars_arrow::trusted_len::TrustedLen">TrustedLen</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#impl-ArrayFromIterDtype%3COption%3C()%3E%3E-for-StructArray" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIterDtype.html" class="trait" title="trait polars::prelude::ArrayFromIterDtype">ArrayFromIterDtype</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.arr_from_iter_with_dtype-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIterDtype.html#tymethod.arr_from_iter_with_dtype" class="fn">arr_from_iter_with_dtype</a>\<I\>(\_dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>, \_iter: I) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.try_arr_from_iter_with_dtype-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIterDtype.html#tymethod.try_arr_from_iter_with_dtype" class="fn">try_arr_from_iter_with_dtype</a>\<E, I\>( \_dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>, \_iter: I, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a>, E\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>, E\>\>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.arr_from_iter_trusted_with_dtype-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIterDtype.html#method.arr_from_iter_trusted_with_dtype" class="fn">arr_from_iter_trusted_with_dtype</a>\<I\>(dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>, iter: I) -\> Self

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = T\>, \<I as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::IntoIter">IntoIter</a>: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/trusted_len/trait.TrustedLen.html" class="trait" title="trait polars_arrow::trusted_len::TrustedLen">TrustedLen</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.try_arr_from_iter_trusted_with_dtype-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIterDtype.html#method.try_arr_from_iter_trusted_with_dtype" class="fn">try_arr_from_iter_trusted_with_dtype</a>\<E, I\>( dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>, iter: I, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, E\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<T, E\>\>, \<I as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::IntoIter">IntoIter</a>: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/trusted_len/trait.TrustedLen.html" class="trait" title="trait polars_arrow::trusted_len::TrustedLen">TrustedLen</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#impl-Clone-for-StructArray" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#impl-Debug-for-StructArray" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#impl-IntoIterator-for-%26StructArray" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a> for &'a <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#associatedtype.Item" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/scalar/trait.Scalar.html" class="trait" title="trait polars_arrow::scalar::Scalar">Scalar</a>\>\>\>

The type of the elements being iterated over.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#associatedtype.IntoIter" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype">IntoIter</a> = <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/bitmap/utils/zip_validity/enum.ZipValidity.html" class="enum" title="enum polars_arrow::bitmap::utils::zip_validity::ZipValidity">ZipValidity</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/scalar/trait.Scalar.html" class="trait" title="trait polars_arrow::scalar::Scalar">Scalar</a>\>\>, StructValueIter\<'a\>, <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/bitmap/utils/iterator/struct.BitmapIter.html" class="struct" title="struct polars_arrow::bitmap::utils::iterator::BitmapIter">BitmapIter</a>\<'a\>\>

Which kind of iterator are we turning this into?

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.into_iter" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter" class="fn">into_iter</a>(self) -\> \<&'a <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a> as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::IntoIter">IntoIter</a>

Creates an iterator from a value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#impl-PartialEq%3C%26dyn+Array%3E-for-StructArray" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>\<&(dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a> + 'static)\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.eq-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &&(dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a> + 'static)) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.ne-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#impl-PartialEq-for-StructArray" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#impl-Splitable-for-StructArray" class="anchor">§</a>

### impl <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Splitable.html" class="trait" title="trait polars_arrow::array::Splitable">Splitable</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.check_bound" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Splitable.html#tymethod.check_bound" class="fn">check_bound</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method._split_at_unchecked" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Splitable.html#tymethod._split_at_unchecked" class="fn">_split_at_unchecked</a>( &self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> (<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a>)

Internal implementation of `split_at_unchecked`. For any usage, prefer the using `split_at` or `split_at_unchecked`. [Read more](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Splitable.html#tymethod._split_at_unchecked)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.split_at" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Splitable.html#method.split_at" class="fn">split_at</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> (Self, Self)

Split [`Self`](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Splitable.html "trait polars_arrow::array::Splitable") at `offset` where `offset <= self.len()`.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.split_at_unchecked" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Splitable.html#method.split_at_unchecked" class="fn">split_at_unchecked</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> (Self, Self)

Split [`Self`](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Splitable.html "trait polars_arrow::array::Splitable") at `offset` without checking `offset <= self.len()`. [Read more](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Splitable.html#method.split_at_unchecked)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#impl-StaticArray-for-StructArray" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html" class="trait" title="trait polars::prelude::StaticArray">StaticArray</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#associatedtype.ValueT" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueT" class="associatedtype">ValueT</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#associatedtype.ZeroableValueT" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ZeroableValueT" class="associatedtype">ZeroableValueT</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#associatedtype.ValueIterT" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueIterT" class="associatedtype">ValueIterT</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/core/iter/sources/repeat/struct.Repeat.html" class="struct" title="struct core::iter::sources::repeat::Repeat">Repeat</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.with_validity_typed" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#tymethod.with_validity_typed" class="fn">with_validity_typed</a>(self, validity: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/bitmap/immutable/struct.Bitmap.html" class="struct" title="struct polars_arrow::bitmap::immutable::Bitmap">Bitmap</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.full_null" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#tymethod.full_null" class="fn">full_null</a>(length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.get" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#method.get" class="fn">get</a>(&self, idx: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueT" class="associatedtype" title="type polars::prelude::StaticArray::ValueT">ValueT</a>\<'\_\>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.get_unchecked" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#method.get_unchecked" class="fn">get_unchecked</a>(&self, idx: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueT" class="associatedtype" title="type polars::prelude::StaticArray::ValueT">ValueT</a>\<'\_\>\>

Safety [Read more](https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#method.get_unchecked)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.last" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#method.last" class="fn">last</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueT" class="associatedtype" title="type polars::prelude::StaticArray::ValueT">ValueT</a>\<'\_\>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.value" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#method.value" class="fn">value</a>(&self, idx: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueT" class="associatedtype" title="type polars::prelude::StaticArray::ValueT">ValueT</a>\<'\_\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.value_unchecked" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#method.value_unchecked" class="fn">value_unchecked</a>(&self, idx: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueT" class="associatedtype" title="type polars::prelude::StaticArray::ValueT">ValueT</a>\<'\_\>

Safety [Read more](https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#method.value_unchecked)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.as_slice" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#method.as_slice" class="fn">as_slice</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&\[Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueT" class="associatedtype" title="type polars::prelude::StaticArray::ValueT">ValueT</a>\<'\_\>\]\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.iter-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#method.iter" class="fn">iter</a>( &self, ) -\> <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/bitmap/utils/zip_validity/enum.ZipValidity.html" class="enum" title="enum polars_arrow::bitmap::utils::zip_validity::ZipValidity">ZipValidity</a>\<Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueT" class="associatedtype" title="type polars::prelude::StaticArray::ValueT">ValueT</a>\<'\_\>, Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueIterT" class="associatedtype" title="type polars::prelude::StaticArray::ValueIterT">ValueIterT</a>\<'\_\>, <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/bitmap/utils/iterator/struct.BitmapIter.html" class="struct" title="struct polars_arrow::bitmap::utils::iterator::BitmapIter">BitmapIter</a>\<'\_\>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.values_iter-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#method.values_iter" class="fn">values_iter</a>(&self) -\> Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueIterT" class="associatedtype" title="type polars::prelude::StaticArray::ValueIterT">ValueIterT</a>\<'\_\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.from_vec" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#method.from_vec" class="fn">from_vec</a>(v: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueT" class="associatedtype" title="type polars::prelude::StaticArray::ValueT">ValueT</a>\<'\_\>\>, dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>) -\> Self

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.from_zeroable_vec" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#method.from_zeroable_vec" class="fn">from_zeroable_vec</a>( v: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ZeroableValueT" class="associatedtype" title="type polars::prelude::StaticArray::ZeroableValueT">ZeroableValueT</a>\<'\_\>\>, dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>, ) -\> Self

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.full" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#method.full" class="fn">full</a>(length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, value: Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueT" class="associatedtype" title="type polars::prelude::StaticArray::ValueT">ValueT</a>\<'\_\>, dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>) -\> Self

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#impl-TotalEqKernel-for-StructArray" class="anchor">§</a>

### impl <a href="https://docs.rs/polars-compute/0.51.0/x86_64-unknown-linux-gnu/polars_compute/comparisons/trait.TotalEqKernel.html" class="trait" title="trait polars_compute::comparisons::TotalEqKernel">TotalEqKernel</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#associatedtype.Scalar" class="anchor">§</a>

#### type <a href="https://docs.rs/polars-compute/0.51.0/x86_64-unknown-linux-gnu/polars_compute/comparisons/trait.TotalEqKernel.html#associatedtype.Scalar" class="associatedtype">Scalar</a> = <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.tot_eq_kernel" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-compute/0.51.0/x86_64-unknown-linux-gnu/polars_compute/comparisons/trait.TotalEqKernel.html#tymethod.tot_eq_kernel" class="fn">tot_eq_kernel</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a>) -\> <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/bitmap/immutable/struct.Bitmap.html" class="struct" title="struct polars_arrow::bitmap::immutable::Bitmap">Bitmap</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.tot_ne_kernel" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-compute/0.51.0/x86_64-unknown-linux-gnu/polars_compute/comparisons/trait.TotalEqKernel.html#tymethod.tot_ne_kernel" class="fn">tot_ne_kernel</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a>) -\> <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/bitmap/immutable/struct.Bitmap.html" class="struct" title="struct polars_arrow::bitmap::immutable::Bitmap">Bitmap</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.tot_eq_kernel_broadcast" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-compute/0.51.0/x86_64-unknown-linux-gnu/polars_compute/comparisons/trait.TotalEqKernel.html#tymethod.tot_eq_kernel_broadcast" class="fn">tot_eq_kernel_broadcast</a>( &self, \_other: &\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a> as <a href="https://docs.rs/polars-compute/0.51.0/x86_64-unknown-linux-gnu/polars_compute/comparisons/trait.TotalEqKernel.html" class="trait" title="trait polars_compute::comparisons::TotalEqKernel">TotalEqKernel</a>\>::<a href="https://docs.rs/polars-compute/0.51.0/x86_64-unknown-linux-gnu/polars_compute/comparisons/trait.TotalEqKernel.html#associatedtype.Scalar" class="associatedtype" title="type polars_compute::comparisons::TotalEqKernel::Scalar">Scalar</a>, ) -\> <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/bitmap/immutable/struct.Bitmap.html" class="struct" title="struct polars_arrow::bitmap::immutable::Bitmap">Bitmap</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.tot_ne_kernel_broadcast" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-compute/0.51.0/x86_64-unknown-linux-gnu/polars_compute/comparisons/trait.TotalEqKernel.html#tymethod.tot_ne_kernel_broadcast" class="fn">tot_ne_kernel_broadcast</a>( &self, \_other: &\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a> as <a href="https://docs.rs/polars-compute/0.51.0/x86_64-unknown-linux-gnu/polars_compute/comparisons/trait.TotalEqKernel.html" class="trait" title="trait polars_compute::comparisons::TotalEqKernel">TotalEqKernel</a>\>::<a href="https://docs.rs/polars-compute/0.51.0/x86_64-unknown-linux-gnu/polars_compute/comparisons/trait.TotalEqKernel.html#associatedtype.Scalar" class="associatedtype" title="type polars_compute::comparisons::TotalEqKernel::Scalar">Scalar</a>, ) -\> <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/bitmap/immutable/struct.Bitmap.html" class="struct" title="struct polars_arrow::bitmap::immutable::Bitmap">Bitmap</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.tot_eq_missing_kernel" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-compute/0.51.0/x86_64-unknown-linux-gnu/polars_compute/comparisons/trait.TotalEqKernel.html#method.tot_eq_missing_kernel" class="fn">tot_eq_missing_kernel</a>(&self, other: &Self) -\> <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/bitmap/immutable/struct.Bitmap.html" class="struct" title="struct polars_arrow::bitmap::immutable::Bitmap">Bitmap</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.tot_ne_missing_kernel" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-compute/0.51.0/x86_64-unknown-linux-gnu/polars_compute/comparisons/trait.TotalEqKernel.html#method.tot_ne_missing_kernel" class="fn">tot_ne_missing_kernel</a>(&self, other: &Self) -\> <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/bitmap/immutable/struct.Bitmap.html" class="struct" title="struct polars_arrow::bitmap::immutable::Bitmap">Bitmap</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.tot_eq_missing_kernel_broadcast" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-compute/0.51.0/x86_64-unknown-linux-gnu/polars_compute/comparisons/trait.TotalEqKernel.html#method.tot_eq_missing_kernel_broadcast" class="fn">tot_eq_missing_kernel_broadcast</a>(&self, other: &Self::<a href="https://docs.rs/polars-compute/0.51.0/x86_64-unknown-linux-gnu/polars_compute/comparisons/trait.TotalEqKernel.html#associatedtype.Scalar" class="associatedtype" title="type polars_compute::comparisons::TotalEqKernel::Scalar">Scalar</a>) -\> <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/bitmap/immutable/struct.Bitmap.html" class="struct" title="struct polars_arrow::bitmap::immutable::Bitmap">Bitmap</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.tot_ne_missing_kernel_broadcast" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-compute/0.51.0/x86_64-unknown-linux-gnu/polars_compute/comparisons/trait.TotalEqKernel.html#method.tot_ne_missing_kernel_broadcast" class="fn">tot_ne_missing_kernel_broadcast</a>(&self, other: &Self::<a href="https://docs.rs/polars-compute/0.51.0/x86_64-unknown-linux-gnu/polars_compute/comparisons/trait.TotalEqKernel.html#associatedtype.Scalar" class="associatedtype" title="type polars_compute::comparisons::TotalEqKernel::Scalar">Scalar</a>) -\> <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/bitmap/immutable/struct.Bitmap.html" class="struct" title="struct polars_arrow::bitmap::immutable::Bitmap">Bitmap</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#impl-TryFrom%3CStructArray%3E-for-DataFrame" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#associatedtype.Error" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>

The type returned in the event of a conversion error.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#method.try_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from" class="fn">try_from</a>(arr: <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Performs the conversion.

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html#blanket-implementations" class="anchor">§</a>
