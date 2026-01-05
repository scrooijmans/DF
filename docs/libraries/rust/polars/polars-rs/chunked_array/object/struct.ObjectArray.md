# Struct ObjectArray Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/object/mod.rs.html#23" class="src">Source</a>

``` rust
pub struct ObjectArray<T>where
    T: PolarsObject,{ /* private fields */ }
```

## Implementations<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#impl-ObjectArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html" class="struct" title="struct polars::chunked_array::object::ObjectArray">ObjectArray</a>\<T\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsObject.html" class="trait" title="trait polars::prelude::PolarsObject">PolarsObject</a>,

#### pub fn <a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.values_iter" class="fn">values_iter</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/slice/iter/struct.Iter.html" class="struct" title="struct core::slice::iter::Iter">Iter</a>\<'\_, T\> <a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#" class="tooltip" data-notable-ty="Iter&lt;&#39;_, T&gt;">ⓘ</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.iter" class="fn">iter</a>(&self) -\> <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/bitmap/utils/zip_validity/enum.ZipValidity.html" class="enum" title="enum polars_arrow::bitmap::utils::zip_validity::ZipValidity">ZipValidity</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>, <a href="https://doc.rust-lang.org/nightly/core/slice/iter/struct.Iter.html" class="struct" title="struct core::slice::iter::Iter">Iter</a>\<'\_, T\>, <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/bitmap/utils/iterator/struct.BitmapIter.html" class="struct" title="struct polars_arrow::bitmap::utils::iterator::BitmapIter">BitmapIter</a>\<'\_\>\>

Returns an iterator of `Option<&T>` over every element of this array.

#### pub fn <a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.value" class="fn">value</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>

Get a value at a certain index location

#### pub fn <a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.get" class="fn">get</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>\>

#### pub unsafe fn <a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.value_unchecked" class="fn">value_unchecked</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>

Get a value at a certain index location

##### <a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#safety" class="doc-anchor">§</a>Safety

This does not any bound checks. The caller needs to ensure the index is within the size of the array.

#### pub unsafe fn <a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.is_valid_unchecked" class="fn">is_valid_unchecked</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Check validity

##### <a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#safety-1" class="doc-anchor">§</a>Safety

No bounds checks

#### pub unsafe fn <a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.is_null_unchecked" class="fn">is_null_unchecked</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Check validity

##### <a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#safety-2" class="doc-anchor">§</a>Safety

No bounds checks

#### pub fn <a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.with_validity" class="fn">with_validity</a>(self, validity: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/bitmap/immutable/struct.Bitmap.html" class="struct" title="struct polars_arrow::bitmap::immutable::Bitmap">Bitmap</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html" class="struct" title="struct polars::chunked_array::object::ObjectArray">ObjectArray</a>\<T\>

Returns this array with a new validity.

##### <a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#panic" class="doc-anchor">§</a>Panic

Panics iff `validity.len() != self.len()`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.set_validity" class="fn">set_validity</a>(&mut self, validity: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/bitmap/immutable/struct.Bitmap.html" class="struct" title="struct polars_arrow::bitmap::immutable::Bitmap">Bitmap</a>\>)

Sets the validity of this array.

##### <a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#panics" class="doc-anchor">§</a>Panics

This function panics iff `validity.len() != self.len()`.

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#impl-Array-for-ObjectArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a> for <a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html" class="struct" title="struct polars::chunked_array::object::ObjectArray">ObjectArray</a>\<T\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsObject.html" class="trait" title="trait polars::prelude::PolarsObject">PolarsObject</a>,

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.as_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Converts itself to a reference of [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"), which enables downcasting to concrete types.

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.dtype" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#tymethod.dtype" class="fn">dtype</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>

The [`ArrowDataType`](https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html "enum polars::prelude::ArrowDataType") of the [`Array`](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html "trait polars_arrow::array::Array"). In combination with [`Array::as_any`](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#tymethod.as_any "method polars_arrow::array::Array::as_any"), this can be used to downcast trait objects (`dyn Array`) to concrete arrays.

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.slice" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#tymethod.slice" class="fn">slice</a>(&mut self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Slices this [`Array`](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html "trait polars_arrow::array::Array"). [Read more](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#tymethod.slice)

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.slice_unchecked" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#tymethod.slice_unchecked" class="fn">slice_unchecked</a>(&mut self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Slices the [`Array`](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html "trait polars_arrow::array::Array"). [Read more](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#tymethod.slice_unchecked)

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.split_at_boxed" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#tymethod.split_at_boxed" class="fn">split_at_boxed</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> (<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>, <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>)

Split [`Self`](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html "trait polars_arrow::array::Array") at `offset` into two boxed [`Array`](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html "trait polars_arrow::array::Array")s where `offset <= self.len()`.

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.split_at_boxed_unchecked" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#tymethod.split_at_boxed_unchecked" class="fn">split_at_boxed_unchecked</a>( &self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> (<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>, <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>)

Split [`Self`](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html "trait polars_arrow::array::Array") at `offset` into two boxed [`Array`](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html "trait polars_arrow::array::Array")s without checking `offset <= self.len()`. [Read more](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#tymethod.split_at_boxed_unchecked)

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.len" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#tymethod.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

The length of the [`Array`](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html "trait polars_arrow::array::Array"). Every array has a length corresponding to the number of elements (slots).

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.validity" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#tymethod.validity" class="fn">validity</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/bitmap/immutable/struct.Bitmap.html" class="struct" title="struct polars_arrow::bitmap::immutable::Bitmap">Bitmap</a>\>

The validity of the [`Array`](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html "trait polars_arrow::array::Array"): every array has an optional [`Bitmap`](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/bitmap/immutable/struct.Bitmap.html "struct polars_arrow::bitmap::immutable::Bitmap") that, when available specifies whether the array slot is valid or not (null). When the validity is [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None"), all slots are valid.

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.with_validity-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#tymethod.with_validity" class="fn">with_validity</a>(&self, validity: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/bitmap/immutable/struct.Bitmap.html" class="struct" title="struct polars_arrow::bitmap::immutable::Bitmap">Bitmap</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>

Clones this [`Array`](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html "trait polars_arrow::array::Array") with a new assigned bitmap. [Read more](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#tymethod.with_validity)

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.to_boxed" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#tymethod.to_boxed" class="fn">to_boxed</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>

Clone a `&dyn Array` to an owned `Box<dyn Array>`.

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.as_any_mut" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#tymethod.as_any_mut" class="fn">as_any_mut</a>(&mut self) -\> &mut (dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Converts itself to a mutable reference of [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"), which enables mutable downcasting to concrete types.

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.null_count" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#method.null_count" class="fn">null_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

The number of null slots on this [`Array`](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html "trait polars_arrow::array::Array"). [Read more](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#method.null_count)

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.is_empty" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

whether the array is empty

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.has_nulls" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#method.has_nulls" class="fn">has_nulls</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.is_null" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#method.is_null" class="fn">is_null</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether slot `i` is null. [Read more](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#method.is_null)

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.is_null_unchecked-1" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#method.is_null_unchecked" class="fn">is_null_unchecked</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether slot `i` is null. [Read more](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#method.is_null_unchecked)

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.is_valid" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#method.is_valid" class="fn">is_valid</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether slot `i` is valid. [Read more](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#method.is_valid)

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.sliced" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#method.sliced" class="fn">sliced</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>

Returns a slice of this [`Array`](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html "trait polars_arrow::array::Array"). [Read more](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#method.sliced)

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.sliced_unchecked" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#method.sliced_unchecked" class="fn">sliced_unchecked</a>( &self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>

Returns a slice of this [`Array`](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html "trait polars_arrow::array::Array"). [Read more](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html#method.sliced_unchecked)

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#impl-ArrayFromIter%3C%26T%3E-for-ObjectArray%3CT%3E" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIter.html" class="trait" title="trait polars::prelude::ArrayFromIter">ArrayFromIter</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>\> for <a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html" class="struct" title="struct polars::chunked_array::object::ObjectArray">ObjectArray</a>\<T\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsObject.html" class="trait" title="trait polars::prelude::PolarsObject">PolarsObject</a>,

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.arr_from_iter" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIter.html#tymethod.arr_from_iter" class="fn">arr_from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html" class="struct" title="struct polars::chunked_array::object::ObjectArray">ObjectArray</a>\<T\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>\>,

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.try_arr_from_iter" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIter.html#tymethod.try_arr_from_iter" class="fn">try_arr_from_iter</a>\<E, I\>(iter: I) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html" class="struct" title="struct polars::chunked_array::object::ObjectArray">ObjectArray</a>\<T\>, E\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>, E\>\>,

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.arr_from_iter_trusted" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIter.html#method.arr_from_iter_trusted" class="fn">arr_from_iter_trusted</a>\<I\>(iter: I) -\> Self

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = T\>, \<I as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::IntoIter">IntoIter</a>: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/trusted_len/trait.TrustedLen.html" class="trait" title="trait polars_arrow::trusted_len::TrustedLen">TrustedLen</a>,

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.try_arr_from_iter_trusted" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIter.html#method.try_arr_from_iter_trusted" class="fn">try_arr_from_iter_trusted</a>\<E, I\>(iter: I) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, E\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<T, E\>\>, \<I as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::IntoIter">IntoIter</a>: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/trusted_len/trait.TrustedLen.html" class="trait" title="trait polars_arrow::trusted_len::TrustedLen">TrustedLen</a>,

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#impl-ArrayFromIter%3COption%3C%26T%3E%3E-for-ObjectArray%3CT%3E" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIter.html" class="trait" title="trait polars::prelude::ArrayFromIter">ArrayFromIter</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>\>\> for <a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html" class="struct" title="struct polars::chunked_array::object::ObjectArray">ObjectArray</a>\<T\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsObject.html" class="trait" title="trait polars::prelude::PolarsObject">PolarsObject</a>,

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.arr_from_iter-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIter.html#tymethod.arr_from_iter" class="fn">arr_from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html" class="struct" title="struct polars::chunked_array::object::ObjectArray">ObjectArray</a>\<T\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>\>\>,

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.try_arr_from_iter-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIter.html#tymethod.try_arr_from_iter" class="fn">try_arr_from_iter</a>\<E, I\>(iter: I) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html" class="struct" title="struct polars::chunked_array::object::ObjectArray">ObjectArray</a>\<T\>, E\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>\>, E\>\>,

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.arr_from_iter_trusted-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIter.html#method.arr_from_iter_trusted" class="fn">arr_from_iter_trusted</a>\<I\>(iter: I) -\> Self

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = T\>, \<I as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::IntoIter">IntoIter</a>: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/trusted_len/trait.TrustedLen.html" class="trait" title="trait polars_arrow::trusted_len::TrustedLen">TrustedLen</a>,

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.try_arr_from_iter_trusted-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIter.html#method.try_arr_from_iter_trusted" class="fn">try_arr_from_iter_trusted</a>\<E, I\>(iter: I) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, E\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<T, E\>\>, \<I as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::IntoIter">IntoIter</a>: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/trusted_len/trait.TrustedLen.html" class="trait" title="trait polars_arrow::trusted_len::TrustedLen">TrustedLen</a>,

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#impl-Clone-for-ObjectArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html" class="struct" title="struct polars::chunked_array::object::ObjectArray">ObjectArray</a>\<T\>

where T: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> + <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsObject.html" class="trait" title="trait polars::prelude::PolarsObject">PolarsObject</a>,

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html" class="struct" title="struct polars::chunked_array::object::ObjectArray">ObjectArray</a>\<T\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#impl-Debug-for-ObjectArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html" class="struct" title="struct polars::chunked_array::object::ObjectArray">ObjectArray</a>\<T\>

where T: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> + <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsObject.html" class="trait" title="trait polars::prelude::PolarsObject">PolarsObject</a>,

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#impl-From%3CVec%3CT%3E%3E-for-ObjectArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<T\>\> for <a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html" class="struct" title="struct polars::chunked_array::object::ObjectArray">ObjectArray</a>\<T\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsObject.html" class="trait" title="trait polars::prelude::PolarsObject">PolarsObject</a>,

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(values: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<T\>) -\> <a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html" class="struct" title="struct polars::chunked_array::object::ObjectArray">ObjectArray</a>\<T\>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#impl-IfThenElseKernel-for-ObjectArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars-compute/0.51.0/x86_64-unknown-linux-gnu/polars_compute/if_then_else/trait.IfThenElseKernel.html" class="trait" title="trait polars_compute::if_then_else::IfThenElseKernel">IfThenElseKernel</a> for <a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html" class="struct" title="struct polars::chunked_array::object::ObjectArray">ObjectArray</a>\<T\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsObject.html" class="trait" title="trait polars::prelude::PolarsObject">PolarsObject</a>,

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#associatedtype.Scalar" class="anchor">§</a>

#### type <a href="https://docs.rs/polars-compute/0.51.0/x86_64-unknown-linux-gnu/polars_compute/if_then_else/trait.IfThenElseKernel.html#associatedtype.Scalar" class="associatedtype">Scalar</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.if_then_else" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-compute/0.51.0/x86_64-unknown-linux-gnu/polars_compute/if_then_else/trait.IfThenElseKernel.html#tymethod.if_then_else" class="fn">if_then_else</a>( mask: &<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/bitmap/immutable/struct.Bitmap.html" class="struct" title="struct polars_arrow::bitmap::immutable::Bitmap">Bitmap</a>, if_true: &<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html" class="struct" title="struct polars::chunked_array::object::ObjectArray">ObjectArray</a>\<T\>, if_false: &<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html" class="struct" title="struct polars::chunked_array::object::ObjectArray">ObjectArray</a>\<T\>, ) -\> <a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html" class="struct" title="struct polars::chunked_array::object::ObjectArray">ObjectArray</a>\<T\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.if_then_else_broadcast_true" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-compute/0.51.0/x86_64-unknown-linux-gnu/polars_compute/if_then_else/trait.IfThenElseKernel.html#tymethod.if_then_else_broadcast_true" class="fn">if_then_else_broadcast_true</a>( mask: &<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/bitmap/immutable/struct.Bitmap.html" class="struct" title="struct polars_arrow::bitmap::immutable::Bitmap">Bitmap</a>, if_true: \<<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html" class="struct" title="struct polars::chunked_array::object::ObjectArray">ObjectArray</a>\<T\> as <a href="https://docs.rs/polars-compute/0.51.0/x86_64-unknown-linux-gnu/polars_compute/if_then_else/trait.IfThenElseKernel.html" class="trait" title="trait polars_compute::if_then_else::IfThenElseKernel">IfThenElseKernel</a>\>::<a href="https://docs.rs/polars-compute/0.51.0/x86_64-unknown-linux-gnu/polars_compute/if_then_else/trait.IfThenElseKernel.html#associatedtype.Scalar" class="associatedtype" title="type polars_compute::if_then_else::IfThenElseKernel::Scalar">Scalar</a>\<'\_\>, if_false: &<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html" class="struct" title="struct polars::chunked_array::object::ObjectArray">ObjectArray</a>\<T\>, ) -\> <a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html" class="struct" title="struct polars::chunked_array::object::ObjectArray">ObjectArray</a>\<T\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.if_then_else_broadcast_false" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-compute/0.51.0/x86_64-unknown-linux-gnu/polars_compute/if_then_else/trait.IfThenElseKernel.html#tymethod.if_then_else_broadcast_false" class="fn">if_then_else_broadcast_false</a>( mask: &<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/bitmap/immutable/struct.Bitmap.html" class="struct" title="struct polars_arrow::bitmap::immutable::Bitmap">Bitmap</a>, if_true: &<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html" class="struct" title="struct polars::chunked_array::object::ObjectArray">ObjectArray</a>\<T\>, if_false: \<<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html" class="struct" title="struct polars::chunked_array::object::ObjectArray">ObjectArray</a>\<T\> as <a href="https://docs.rs/polars-compute/0.51.0/x86_64-unknown-linux-gnu/polars_compute/if_then_else/trait.IfThenElseKernel.html" class="trait" title="trait polars_compute::if_then_else::IfThenElseKernel">IfThenElseKernel</a>\>::<a href="https://docs.rs/polars-compute/0.51.0/x86_64-unknown-linux-gnu/polars_compute/if_then_else/trait.IfThenElseKernel.html#associatedtype.Scalar" class="associatedtype" title="type polars_compute::if_then_else::IfThenElseKernel::Scalar">Scalar</a>\<'\_\>, ) -\> <a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html" class="struct" title="struct polars::chunked_array::object::ObjectArray">ObjectArray</a>\<T\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.if_then_else_broadcast_both" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-compute/0.51.0/x86_64-unknown-linux-gnu/polars_compute/if_then_else/trait.IfThenElseKernel.html#tymethod.if_then_else_broadcast_both" class="fn">if_then_else_broadcast_both</a>( \_dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>, mask: &<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/bitmap/immutable/struct.Bitmap.html" class="struct" title="struct polars_arrow::bitmap::immutable::Bitmap">Bitmap</a>, if_true: \<<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html" class="struct" title="struct polars::chunked_array::object::ObjectArray">ObjectArray</a>\<T\> as <a href="https://docs.rs/polars-compute/0.51.0/x86_64-unknown-linux-gnu/polars_compute/if_then_else/trait.IfThenElseKernel.html" class="trait" title="trait polars_compute::if_then_else::IfThenElseKernel">IfThenElseKernel</a>\>::<a href="https://docs.rs/polars-compute/0.51.0/x86_64-unknown-linux-gnu/polars_compute/if_then_else/trait.IfThenElseKernel.html#associatedtype.Scalar" class="associatedtype" title="type polars_compute::if_then_else::IfThenElseKernel::Scalar">Scalar</a>\<'\_\>, if_false: \<<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html" class="struct" title="struct polars::chunked_array::object::ObjectArray">ObjectArray</a>\<T\> as <a href="https://docs.rs/polars-compute/0.51.0/x86_64-unknown-linux-gnu/polars_compute/if_then_else/trait.IfThenElseKernel.html" class="trait" title="trait polars_compute::if_then_else::IfThenElseKernel">IfThenElseKernel</a>\>::<a href="https://docs.rs/polars-compute/0.51.0/x86_64-unknown-linux-gnu/polars_compute/if_then_else/trait.IfThenElseKernel.html#associatedtype.Scalar" class="associatedtype" title="type polars_compute::if_then_else::IfThenElseKernel::Scalar">Scalar</a>\<'\_\>, ) -\> <a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html" class="struct" title="struct polars::chunked_array::object::ObjectArray">ObjectArray</a>\<T\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#impl-IntoIterator-for-%26ObjectArray%3CT%3E" class="anchor">§</a>

### impl\<'a, T\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a> for &'a <a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html" class="struct" title="struct polars::chunked_array::object::ObjectArray">ObjectArray</a>\<T\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsObject.html" class="trait" title="trait polars::prelude::PolarsObject">PolarsObject</a>,

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#associatedtype.Item" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>\>

The type of the elements being iterated over.

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#associatedtype.IntoIter" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype">IntoIter</a> = ObjectIter\<'a, T\>

Which kind of iterator are we turning this into?

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.into_iter" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter" class="fn">into_iter</a>(self) -\> \<&'a <a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html" class="struct" title="struct polars::chunked_array::object::ObjectArray">ObjectArray</a>\<T\> as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::IntoIter">IntoIter</a>

Creates an iterator from a value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#impl-ParameterFreeDtypeStaticArray-for-ObjectArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/static_array/trait.ParameterFreeDtypeStaticArray.html" class="trait" title="trait polars_arrow::array::static_array::ParameterFreeDtypeStaticArray">ParameterFreeDtypeStaticArray</a> for <a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html" class="struct" title="struct polars::chunked_array::object::ObjectArray">ObjectArray</a>\<T\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsObject.html" class="trait" title="trait polars::prelude::PolarsObject">PolarsObject</a>,

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.get_dtype" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/static_array/trait.ParameterFreeDtypeStaticArray.html#tymethod.get_dtype" class="fn">get_dtype</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#impl-Splitable-for-ObjectArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Splitable.html" class="trait" title="trait polars_arrow::array::Splitable">Splitable</a> for <a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html" class="struct" title="struct polars::chunked_array::object::ObjectArray">ObjectArray</a>\<T\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsObject.html" class="trait" title="trait polars::prelude::PolarsObject">PolarsObject</a>,

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.check_bound" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Splitable.html#tymethod.check_bound" class="fn">check_bound</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method._split_at_unchecked" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Splitable.html#tymethod._split_at_unchecked" class="fn">_split_at_unchecked</a>( &self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> (<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html" class="struct" title="struct polars::chunked_array::object::ObjectArray">ObjectArray</a>\<T\>, <a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html" class="struct" title="struct polars::chunked_array::object::ObjectArray">ObjectArray</a>\<T\>)

Internal implementation of `split_at_unchecked`. For any usage, prefer the using `split_at` or `split_at_unchecked`. [Read more](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Splitable.html#tymethod._split_at_unchecked)

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.split_at" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Splitable.html#method.split_at" class="fn">split_at</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> (Self, Self)

Split [`Self`](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Splitable.html "trait polars_arrow::array::Splitable") at `offset` where `offset <= self.len()`.

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.split_at_unchecked" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Splitable.html#method.split_at_unchecked" class="fn">split_at_unchecked</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> (Self, Self)

Split [`Self`](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Splitable.html "trait polars_arrow::array::Splitable") at `offset` without checking `offset <= self.len()`. [Read more](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Splitable.html#method.split_at_unchecked)

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#impl-StaticArray-for-ObjectArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html" class="trait" title="trait polars::prelude::StaticArray">StaticArray</a> for <a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html" class="struct" title="struct polars::chunked_array::object::ObjectArray">ObjectArray</a>\<T\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsObject.html" class="trait" title="trait polars::prelude::PolarsObject">PolarsObject</a>,

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#associatedtype.ValueT" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueT" class="associatedtype">ValueT</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#associatedtype.ZeroableValueT" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ZeroableValueT" class="associatedtype">ZeroableValueT</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#associatedtype.ValueIterT" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueIterT" class="associatedtype">ValueIterT</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/core/slice/iter/struct.Iter.html" class="struct" title="struct core::slice::iter::Iter">Iter</a>\<'a, T\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.value_unchecked-1" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#method.value_unchecked" class="fn">value_unchecked</a>( &self, idx: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> \<<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html" class="struct" title="struct polars::chunked_array::object::ObjectArray">ObjectArray</a>\<T\> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html" class="trait" title="trait polars::prelude::StaticArray">StaticArray</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueT" class="associatedtype" title="type polars::prelude::StaticArray::ValueT">ValueT</a>\<'\_\>

Safety [Read more](https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#method.value_unchecked)

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.values_iter-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#method.values_iter" class="fn">values_iter</a>(&self) -\> \<<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html" class="struct" title="struct polars::chunked_array::object::ObjectArray">ObjectArray</a>\<T\> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html" class="trait" title="trait polars::prelude::StaticArray">StaticArray</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueIterT" class="associatedtype" title="type polars::prelude::StaticArray::ValueIterT">ValueIterT</a>\<'\_\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.iter-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#method.iter" class="fn">iter</a>( &self, ) -\> <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/bitmap/utils/zip_validity/enum.ZipValidity.html" class="enum" title="enum polars_arrow::bitmap::utils::zip_validity::ZipValidity">ZipValidity</a>\<\<<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html" class="struct" title="struct polars::chunked_array::object::ObjectArray">ObjectArray</a>\<T\> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html" class="trait" title="trait polars::prelude::StaticArray">StaticArray</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueT" class="associatedtype" title="type polars::prelude::StaticArray::ValueT">ValueT</a>\<'\_\>, \<<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html" class="struct" title="struct polars::chunked_array::object::ObjectArray">ObjectArray</a>\<T\> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html" class="trait" title="trait polars::prelude::StaticArray">StaticArray</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueIterT" class="associatedtype" title="type polars::prelude::StaticArray::ValueIterT">ValueIterT</a>\<'\_\>, <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/bitmap/utils/iterator/struct.BitmapIter.html" class="struct" title="struct polars_arrow::bitmap::utils::iterator::BitmapIter">BitmapIter</a>\<'\_\>\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.with_validity_typed" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#tymethod.with_validity_typed" class="fn">with_validity_typed</a>(self, validity: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/bitmap/immutable/struct.Bitmap.html" class="struct" title="struct polars_arrow::bitmap::immutable::Bitmap">Bitmap</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html" class="struct" title="struct polars::chunked_array::object::ObjectArray">ObjectArray</a>\<T\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.full_null" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#tymethod.full_null" class="fn">full_null</a>(length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, \_dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>) -\> <a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html" class="struct" title="struct polars::chunked_array::object::ObjectArray">ObjectArray</a>\<T\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.get-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#method.get" class="fn">get</a>(&self, idx: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueT" class="associatedtype" title="type polars::prelude::StaticArray::ValueT">ValueT</a>\<'\_\>\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.get_unchecked" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#method.get_unchecked" class="fn">get_unchecked</a>(&self, idx: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueT" class="associatedtype" title="type polars::prelude::StaticArray::ValueT">ValueT</a>\<'\_\>\>

Safety [Read more](https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#method.get_unchecked)

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.last" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#method.last" class="fn">last</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueT" class="associatedtype" title="type polars::prelude::StaticArray::ValueT">ValueT</a>\<'\_\>\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.value-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#method.value" class="fn">value</a>(&self, idx: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueT" class="associatedtype" title="type polars::prelude::StaticArray::ValueT">ValueT</a>\<'\_\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.as_slice" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#method.as_slice" class="fn">as_slice</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&\[Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueT" class="associatedtype" title="type polars::prelude::StaticArray::ValueT">ValueT</a>\<'\_\>\]\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.from_vec" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#method.from_vec" class="fn">from_vec</a>(v: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueT" class="associatedtype" title="type polars::prelude::StaticArray::ValueT">ValueT</a>\<'\_\>\>, dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>) -\> Self

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.from_zeroable_vec" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#method.from_zeroable_vec" class="fn">from_zeroable_vec</a>( v: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ZeroableValueT" class="associatedtype" title="type polars::prelude::StaticArray::ZeroableValueT">ZeroableValueT</a>\<'\_\>\>, dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>, ) -\> Self

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#method.full" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#method.full" class="fn">full</a>(length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, value: Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueT" class="associatedtype" title="type polars::prelude::StaticArray::ValueT">ValueT</a>\<'\_\>, dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>) -\> Self

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#impl-ArrowArray-for-ObjectArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/legacy/is_valid/trait.ArrowArray.html" class="trait" title="trait polars_arrow::legacy::is_valid::ArrowArray">ArrowArray</a> for <a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html" class="struct" title="struct polars::chunked_array::object::ObjectArray">ObjectArray</a>\<T\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsObject.html" class="trait" title="trait polars::prelude::PolarsObject">PolarsObject</a>,

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html#blanket-implementations" class="anchor">§</a>
