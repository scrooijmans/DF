# Struct GenericByteViewArray Copy item path

<a href="https://docs.rs/arrow-array/56.0.0/x86_64-unknown-linux-gnu/src/arrow_array/array/byte_view_array.rs.html#165" class="src">Source</a>

``` rust
pub struct GenericByteViewArray<T>where
    T: ByteViewType + ?Sized,{ /* private fields */ }
```

Expand description

[Variable-size Binary View Layout](https://arrow.apache.org/docs/format/Columnar.html#variable-size-binary-view-layout): An array of variable length bytes views.

This array type is used to store variable length byte data (e.g. Strings, Binary) and has efficient operations such as `take`, `filter`, and comparison.

This is different from [`GenericByteArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html "struct datafusion::common::arrow::array::GenericByteArray"), which also stores variable length byte data, as it represents strings with an offset and length. `take` and `filter` like operations are implemented by manipulating the “views” (`u128`) without modifying the bytes. Each view also stores an inlined prefix which speed up comparisons.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#see-also" class="doc-anchor">§</a>See Also

- [`StringViewArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.StringViewArray.html "type datafusion::common::arrow::array::StringViewArray") for storing utf8 encoded string data
- [`BinaryViewArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.BinaryViewArray.html "type datafusion::common::arrow::array::BinaryViewArray") for storing bytes
- [`ByteView`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.ByteView.html "struct datafusion::common::arrow::array::ByteView") to interpret `u128`s layout of the views.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#layout-views-and-buffers" class="doc-anchor">§</a>Layout: “views” and buffers

A `GenericByteViewArray` stores variable length byte strings. An array of `N` elements is stored as `N` fixed length “views” and a variable number of variable length “buffers”.

Each view is a `u128` value whose layout is different depending on the length of the string stored at that location:

``` text
                        ┌──────┬────────────────────────┐
                        │length│      string value      │
   Strings (len <= 12)  │      │    (padded with 0)     │
                        └──────┴────────────────────────┘
                         0    31                      127

                        ┌───────┬───────┬───────┬───────┐
                        │length │prefix │  buf  │offset │
   Strings (len > 12)   │       │       │ index │       │
                        └───────┴───────┴───────┴───────┘
                         0    31       63      95    127
```

- Strings with length \<= 12 ([`MAX_INLINE_VIEW_LEN`](https://docs.rs/arrow-data/56.0.0/x86_64-unknown-linux-gnu/arrow_data/byte_view/constant.MAX_INLINE_VIEW_LEN.html "constant arrow_data::byte_view::MAX_INLINE_VIEW_LEN")) are stored directly in the view. See [`Self::inline_value`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html#method.inline_value "associated function datafusion::common::arrow::array::GenericByteViewArray::inline_value") to access the inlined prefix from a short view.

- Strings with length \> 12: The first four bytes are stored inline in the view and the entire string is stored in one of the buffers. See [`ByteView`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.ByteView.html "struct datafusion::common::arrow::array::ByteView") to access the fields of the these views.

As with other arrays, the optimized kernels in [`arrow_compute`](https://docs.rs/arrow/latest/arrow/compute/index.html) are likely the easiest and fastest way to work with this data. However, it is possible to access the views and buffers directly for more control.

For example

``` rust
use arrow_data::ByteView;
let array = StringViewArray::from(vec![
  "hello",
  "this string is longer than 12 bytes",
  "this string is also longer than 12 bytes"
]);

// ** Examine the first view (short string) **
assert!(array.is_valid(0)); // Check for nulls
let short_view: u128 = array.views()[0]; // "hello"
// get length of the string
let len = short_view as u32;
assert_eq!(len, 5); // strings less than 12 bytes are stored in the view
// SAFETY: `view` is a valid view
let value = unsafe {
  StringViewArray::inline_value(&short_view, len as usize)
};
assert_eq!(value, b"hello");

// ** Examine the third view (long string) **
assert!(array.is_valid(12)); // Check for nulls
let long_view: u128 = array.views()[2]; // "this string is also longer than 12 bytes"
let len = long_view as u32;
assert_eq!(len, 40); // strings longer than 12 bytes are stored in the buffer
let view = ByteView::from(long_view); // use ByteView to access the fields
assert_eq!(view.length, 40);
assert_eq!(view.buffer_index, 0);
assert_eq!(view.offset, 35); // data starts after the first long string
// Views for long strings store a 4 byte prefix
let prefix = view.prefix.to_le_bytes();
assert_eq!(&prefix, b"this");
let value = array.value(2); // get the string value (see `value` implementation for how to access the bytes directly)
assert_eq!(value, "this string is also longer than 12 bytes");
```

Unlike [`GenericByteArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html "struct datafusion::common::arrow::array::GenericByteArray"), there are no constraints on the offsets other than they must point into a valid buffer. However, they can be out of order, non continuous and overlapping.

For example, in the following diagram, the strings “FishWasInTownToday” and “CrumpleFacedFish” are both longer than 12 bytes and thus are stored in a separate buffer while the string “LavaMonster” is stored inlined in the view. In this case, the same bytes for “Fish” are used to store both strings.

``` text
                                                                           ┌───┐
                        ┌──────┬──────┬──────┬──────┐               offset │...│
"FishWasInTownTodayYay" │  21  │ Fish │  0   │ 115  │─ ─              103  │Mr.│
                        └──────┴──────┴──────┴──────┘   │      ┌ ─ ─ ─ ─ ▶ │Cru│
                        ┌──────┬──────┬──────┬──────┐                      │mpl│
"CrumpleFacedFish"      │  16  │ Crum │  0   │ 103  │─ ─│─ ─ ─ ┘           │eFa│
                        └──────┴──────┴──────┴──────┘                      │ced│
                        ┌──────┬────────────────────┐   └ ─ ─ ─ ─ ─ ─ ─ ─ ▶│Fis│
"LavaMonster"           │  11  │   LavaMonster      │                      │hWa│
                        └──────┴────────────────────┘               offset │sIn│
                                                                      115  │Tow│
                                                                           │nTo│
                                                                           │day│
                                 u128 "views"                              │Yay│
                                                                  buffer 0 │...│
                                                                           └───┘
```

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#impl-GenericByteViewArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<T\>

where T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteViewType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteViewType">ByteViewType</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.new" class="fn">new</a>( views: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.ScalarBuffer.html" class="struct" title="struct datafusion::common::arrow::buffer::ScalarBuffer">ScalarBuffer</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\>, buffers: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.Buffer.html" class="struct" title="struct datafusion::common::arrow::buffer::Buffer">Buffer</a>\>, nulls: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct datafusion::common::arrow::buffer::NullBuffer">NullBuffer</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<T\>

Create a new [`GenericByteViewArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html "struct datafusion::common::arrow::array::GenericByteViewArray") from the provided parts, panicking on failure

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#panics" class="doc-anchor">§</a>Panics

Panics if [`GenericByteViewArray::try_new`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html#method.try_new "associated function datafusion::common::arrow::array::GenericByteViewArray::try_new") returns an error

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.try_new" class="fn">try_new</a>( views: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.ScalarBuffer.html" class="struct" title="struct datafusion::common::arrow::buffer::ScalarBuffer">ScalarBuffer</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\>, buffers: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.Buffer.html" class="struct" title="struct datafusion::common::arrow::buffer::Buffer">Buffer</a>\>, nulls: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct datafusion::common::arrow::buffer::NullBuffer">NullBuffer</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<T\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

Create a new [`GenericByteViewArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html "struct datafusion::common::arrow::array::GenericByteViewArray") from the provided parts, returning an error on failure

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#errors" class="doc-anchor">§</a>Errors

- `views.len() != nulls.len()`
- [ByteViewType::validate](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteViewType.html#tymethod.validate "associated function datafusion::common::arrow::datatypes::ByteViewType::validate") fails

#### pub unsafe fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.new_unchecked" class="fn">new_unchecked</a>( views: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.ScalarBuffer.html" class="struct" title="struct datafusion::common::arrow::buffer::ScalarBuffer">ScalarBuffer</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\>, buffers: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.Buffer.html" class="struct" title="struct datafusion::common::arrow::buffer::Buffer">Buffer</a>\>, nulls: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct datafusion::common::arrow::buffer::NullBuffer">NullBuffer</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<T\>

Create a new [`GenericByteViewArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html "struct datafusion::common::arrow::array::GenericByteViewArray") from the provided parts, without validation

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#safety" class="doc-anchor">§</a>Safety

Safe if [`Self::try_new`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html#method.try_new "associated function datafusion::common::arrow::array::GenericByteViewArray::try_new") would not error

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.new_null" class="fn">new_null</a>(len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<T\>

Create a new [`GenericByteViewArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html "struct datafusion::common::arrow::array::GenericByteViewArray") of length `len` where all values are null

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.new_scalar" class="fn">new_scalar</a>( value: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\<T as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteViewType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteViewType">ByteViewType</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteViewType.html#associatedtype.Native" class="associatedtype" title="type datafusion::common::arrow::datatypes::ByteViewType::Native">Native</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.Scalar.html" class="struct" title="struct datafusion::common::arrow::array::Scalar">Scalar</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<T\>\>

Create a new [`Scalar`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.Scalar.html "struct datafusion::common::arrow::array::Scalar") from `value`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.from_iter_values" class="fn">from_iter_values</a>\<Ptr, I\>(iter: I) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<T\>

where Ptr: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\<T as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteViewType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteViewType">ByteViewType</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteViewType.html#associatedtype.Native" class="associatedtype" title="type datafusion::common::arrow::datatypes::ByteViewType::Native">Native</a>\>, I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = Ptr\>,

Creates a [`GenericByteViewArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html "struct datafusion::common::arrow::array::GenericByteViewArray") based on an iterator of values without nulls

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.into_parts" class="fn">into_parts</a>(self) -\> (<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.ScalarBuffer.html" class="struct" title="struct datafusion::common::arrow::buffer::ScalarBuffer">ScalarBuffer</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\>, <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.Buffer.html" class="struct" title="struct datafusion::common::arrow::buffer::Buffer">Buffer</a>\>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct datafusion::common::arrow::buffer::NullBuffer">NullBuffer</a>\>)

Deconstruct this array into its constituent parts

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.views" class="fn">views</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.ScalarBuffer.html" class="struct" title="struct datafusion::common::arrow::buffer::ScalarBuffer">ScalarBuffer</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\>

Returns the views buffer

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.data_buffers" class="fn">data_buffers</a>(&self) -\> &\[<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.Buffer.html" class="struct" title="struct datafusion::common::arrow::buffer::Buffer">Buffer</a>\]

Returns the buffers storing string data

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.value" class="fn">value</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> &\<T as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteViewType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteViewType">ByteViewType</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteViewType.html#associatedtype.Native" class="associatedtype" title="type datafusion::common::arrow::datatypes::ByteViewType::Native">Native</a>

Returns the element at index `i`

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#panics-1" class="doc-anchor">§</a>Panics

Panics if index `i` is out of bounds.

#### pub unsafe fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.value_unchecked" class="fn">value_unchecked</a>(&self, idx: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> &\<T as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteViewType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteViewType">ByteViewType</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteViewType.html#associatedtype.Native" class="associatedtype" title="type datafusion::common::arrow::datatypes::ByteViewType::Native">Native</a>

Returns the element at index `i` without bounds checking

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#safety-1" class="doc-anchor">§</a>Safety

Caller is responsible for ensuring that the index is within the bounds of the array

#### pub unsafe fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.inline_value" class="fn">inline_value</a>(view: &<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>, len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\] <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#" class="tooltip" data-notable-ty="&amp;[u8]">ⓘ</a>

Returns the first `len` bytes the inline value of the view.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#safety-2" class="doc-anchor">§</a>Safety

- The `view` must be a valid element from `Self::views()` that adheres to the view layout.
- The `len` must be the length of the inlined value. It should never be larger than [`MAX_INLINE_VIEW_LEN`](https://docs.rs/arrow-data/56.0.0/x86_64-unknown-linux-gnu/arrow_data/byte_view/constant.MAX_INLINE_VIEW_LEN.html "constant arrow_data::byte_view::MAX_INLINE_VIEW_LEN").

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.iter" class="fn">iter</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.ArrayIter.html" class="struct" title="struct datafusion::common::arrow::array::ArrayIter">ArrayIter</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<T\>\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#" class="tooltip" data-notable-ty="ArrayIter&lt;&amp;GenericByteViewArray&lt;T&gt;&gt;">ⓘ</a>

Constructs a new iterator for iterating over the values of this array

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.bytes_iter" class="fn">bytes_iter</a>(&self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>

Returns an iterator over the bytes of this array, including null values

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.prefix_bytes_iter" class="fn">prefix_bytes_iter</a>( &self, prefix_len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>

Returns an iterator over the first `prefix_len` bytes of each array element, including null values.

If `prefix_len` is larger than the element’s length, the iterator will return an empty slice (`&[]`).

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.suffix_bytes_iter" class="fn">suffix_bytes_iter</a>( &self, suffix_len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>

Returns an iterator over the last `suffix_len` bytes of each array element, including null values.

Note that for [`StringViewArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.StringViewArray.html "type datafusion::common::arrow::array::StringViewArray") the last bytes may start in the middle of a UTF-8 codepoint, and thus may not be a valid `&str`.

If `suffix_len` is larger than the element’s length, the iterator will return an empty slice (`&[]`).

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.slice" class="fn">slice</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<T\>

Returns a zero-copy slice of this array with the indicated offset and length.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.gc" class="fn">gc</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<T\>

Returns a “compacted” version of this array

The original array will *not* be modified

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#garbage-collection" class="doc-anchor">§</a>Garbage Collection

Before GC:

``` text
                                       ┌──────┐
                                       │......│
                                       │......│
┌────────────────────┐       ┌ ─ ─ ─ ▶ │Data1 │   Large buffer
│       View 1       │─ ─ ─ ─          │......│  with data that
├────────────────────┤                 │......│ is not referred
│       View 2       │─ ─ ─ ─ ─ ─ ─ ─▶ │Data2 │ to by View 1 or
└────────────────────┘                 │......│      View 2
                                       │......│
   2 views, refer to                   │......│
  small portions of a                  └──────┘
     large buffer
```

After GC:

``` text
┌────────────────────┐                 ┌─────┐    After gc, only
│       View 1       │─ ─ ─ ─ ─ ─ ─ ─▶ │Data1│     data that is
├────────────────────┤       ┌ ─ ─ ─ ▶ │Data2│    pointed to by
│       View 2       │─ ─ ─ ─          └─────┘     the views is
└────────────────────┘                                 left


        2 views
```

This method will compact the data buffers by recreating the view array and only include the data that is pointed to by the views.

Note that it will copy the array regardless of whether the original array is compact. Use with caution as this can be an expensive operation, only use it when you are sure that the view array is significantly smaller than when it is originally created, e.g., after filtering or slicing.

Note: this function does not attempt to canonicalize / deduplicate values. For this feature see [`GenericByteViewBuilder::with_deduplicate_strings`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewBuilder.html#method.with_deduplicate_strings "method datafusion::common::arrow::array::GenericByteViewBuilder::with_deduplicate_strings").

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.total_buffer_bytes_used" class="fn">total_buffer_bytes_used</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of bytes used by all non inlined views in all buffers.

Note this does not account for views that point at the same underlying data in buffers

For example, if the array has three strings views:

- View with length = 9 (inlined)
- View with length = 32 (non inlined)
- View with length = 16 (non inlined)

Then this method would report 48

#### pub unsafe fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.compare_unchecked" class="fn">compare_unchecked</a>( left: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<T\>, left_idx: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, right: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<T\>, right_idx: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>

Compare two [`GenericByteViewArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html "struct datafusion::common::arrow::array::GenericByteViewArray") at index `left_idx` and `right_idx`

Comparing two ByteView types are non-trivial. It takes a bit of patience to understand why we don’t just compare two &[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html "primitive u8") directly.

ByteView types give us the following two advantages, and we need to be careful not to lose them: (1) For string/byte smaller than [`MAX_INLINE_VIEW_LEN`](https://docs.rs/arrow-data/56.0.0/x86_64-unknown-linux-gnu/arrow_data/byte_view/constant.MAX_INLINE_VIEW_LEN.html "constant arrow_data::byte_view::MAX_INLINE_VIEW_LEN") bytes, the entire data is inlined in the view. Meaning that reading one array element requires only one memory access (two memory access required for StringArray, one for offset buffer, the other for value buffer).

\(2\) For string/byte larger than [`MAX_INLINE_VIEW_LEN`](https://docs.rs/arrow-data/56.0.0/x86_64-unknown-linux-gnu/arrow_data/byte_view/constant.MAX_INLINE_VIEW_LEN.html "constant arrow_data::byte_view::MAX_INLINE_VIEW_LEN") bytes, we can still be faster than (for certain operations) StringArray/ByteArray, thanks to the inlined 4 bytes. Consider equality check: If the first four bytes of the two strings are different, we can return false immediately (with just one memory access).

If we directly compare two &[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html "primitive u8"), we materialize the entire string (i.e., make multiple memory accesses), which might be unnecessary.

- Most of the time (eq, ord), we only need to look at the first 4 bytes to know the answer, e.g., if the inlined 4 bytes are different, we can directly return unequal without looking at the full string.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#order-check-flow" class="doc-anchor">§</a>Order check flow

\(1\) if both string are smaller than [`MAX_INLINE_VIEW_LEN`](https://docs.rs/arrow-data/56.0.0/x86_64-unknown-linux-gnu/arrow_data/byte_view/constant.MAX_INLINE_VIEW_LEN.html "constant arrow_data::byte_view::MAX_INLINE_VIEW_LEN") bytes, we can directly compare the data inlined to the view. (2) if any of the string is larger than [`MAX_INLINE_VIEW_LEN`](https://docs.rs/arrow-data/56.0.0/x86_64-unknown-linux-gnu/arrow_data/byte_view/constant.MAX_INLINE_VIEW_LEN.html "constant arrow_data::byte_view::MAX_INLINE_VIEW_LEN") bytes, we need to compare the full string. (2.1) if the inlined 4 bytes are different, we can return the result immediately. (2.2) o.w., we need to compare the full string.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#safety-3" class="doc-anchor">§</a>Safety

The left/right_idx must within range of each array

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.inline_key_fast" class="fn">inline_key_fast</a>(raw: <a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>

Builds a 128-bit composite key for an inline value:

- High 96 bits: the inline data in big-endian byte order (for correct lexicographical sorting).
- Low 32 bits: the length in big-endian byte order, acting as a tiebreaker so shorter strings (or those with fewer meaningful bytes) always numerically sort before longer ones.

This function extracts the length and the 12-byte inline string data from the raw little-endian `u128` representation, converts them to big-endian ordering, and packs them into a single `u128` value suitable for fast, branchless comparisons.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#why-include-length" class="doc-anchor">§</a>Why include length?

A pure 96-bit content comparison can’t distinguish between two values whose inline bytes compare equal—either because one is a true prefix of the other or because zero-padding hides extra bytes. By tucking the 32-bit length into the lower bits, a single `u128` compare handles both content and length in one go.

Example: comparing “bar” (3 bytes) vs “bar\0” (4 bytes)

| String    | Bytes 0–4 (length LE) | Bytes 4–16 (data + padding) |
|-----------|-----------------------|-----------------------------|
| `"bar"`   | `03 00 00 00`         | `62 61 72` + 9 × `00`       |
| `"bar\0"` | `04 00 00 00`         | `62 61 72 00` + 8 × `00`    |

Both inline parts become `62 61 72 00…00`, so they tie on content. The length field then differentiates:

``` text
key("bar")   = 0x0000000000000000000062617200000003
key("bar\0") = 0x0000000000000000000062617200000004
⇒ key("bar") < key("bar\0")
```

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#inlining-and-endianness" class="doc-anchor">§</a>Inlining and Endianness

- We start by calling `.to_le_bytes()` on the `raw` `u128`, because Rust’s native in‑memory representation is little‑endian on x86/ARM.
- We extract the low 32 bits numerically (`raw as u32`)—this step is endianness‑free.
- We copy the 12 bytes of inline data (original order) into `buf[0..12]`.
- We serialize `length` as big‑endian into `buf[12..16]`.
- Finally, `u128::from_be_bytes(buf)` treats `buf[0]` as the most significant byte and `buf[15]` as the least significant, producing a `u128` whose integer value directly encodes “inline data then length” in big‑endian form.

This ensures that a simple `u128` comparison is equivalent to the desired lexicographical comparison of the inline bytes followed by length.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#impl-GenericByteViewArray%3CBinaryViewType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.BinaryViewType.html" class="struct" title="struct datafusion::common::arrow::datatypes::BinaryViewType">BinaryViewType</a>\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.to_string_view" class="fn">to_string_view</a>( self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.StringViewType.html" class="struct" title="struct datafusion::common::arrow::datatypes::StringViewType">StringViewType</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

Convert the [`BinaryViewArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.BinaryViewArray.html "type datafusion::common::arrow::array::BinaryViewArray") to [`StringViewArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.StringViewArray.html "type datafusion::common::arrow::array::StringViewArray") If items not utf8 data, validate will fail and error returned.

#### pub unsafe fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.to_string_view_unchecked" class="fn">to_string_view_unchecked</a>( self, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.StringViewType.html" class="struct" title="struct datafusion::common::arrow::datatypes::StringViewType">StringViewType</a>\>

Convert the [`BinaryViewArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.BinaryViewArray.html "type datafusion::common::arrow::array::BinaryViewArray") to [`StringViewArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.StringViewArray.html "type datafusion::common::arrow::array::StringViewArray")

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#safety-4" class="doc-anchor">§</a>Safety

Caller is responsible for ensuring that items in array are utf8 data.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#impl-GenericByteViewArray%3CStringViewType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.StringViewType.html" class="struct" title="struct datafusion::common::arrow::datatypes::StringViewType">StringViewType</a>\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.to_binary_view" class="fn">to_binary_view</a>(self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.BinaryViewType.html" class="struct" title="struct datafusion::common::arrow::datatypes::BinaryViewType">BinaryViewType</a>\>

Convert the [`StringViewArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.StringViewArray.html "type datafusion::common::arrow::array::StringViewArray") to [`BinaryViewArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.BinaryViewArray.html "type datafusion::common::arrow::array::BinaryViewArray")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.is_ascii" class="fn">is_ascii</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if all data within this array is ASCII

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#impl-Array-for-GenericByteViewArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<T\>

where T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteViewType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteViewType">ByteViewType</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.as_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the array as [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") so that it can be downcasted to a specific implementation. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.as_any)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.to_data" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.to_data" class="fn">to_data</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.ArrayData.html" class="struct" title="struct datafusion::common::arrow::array::ArrayData">ArrayData</a>

Returns the underlying data of this array

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.into_data" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.into_data" class="fn">into_data</a>(self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.ArrayData.html" class="struct" title="struct datafusion::common::arrow::array::ArrayData">ArrayData</a>

Returns the underlying data of this array [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.into_data)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.data_type" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.data_type" class="fn">data_type</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

Returns a reference to the [`DataType`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html "enum datafusion::common::arrow::datatypes::DataType") of this array. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.data_type)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.slice-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.slice" class="fn">slice</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>

Returns a zero-copy slice of this array with the indicated offset and length. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.slice)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.len" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the length (i.e., number of elements) of this array. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.len)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.is_empty" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether this array is empty. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.is_empty)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.shrink_to_fit" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.shrink_to_fit" class="fn">shrink_to_fit</a>(&mut self)

Shrinks the capacity of any exclusively owned buffer as much as possible [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.shrink_to_fit)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.offset" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.offset" class="fn">offset</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the offset into the underlying data used by this array(-slice). Note that the underlying data can be shared by many arrays. This defaults to `0`. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.offset)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.nulls" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.nulls" class="fn">nulls</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct datafusion::common::arrow::buffer::NullBuffer">NullBuffer</a>\>

Returns the null buffer of this array if any. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.nulls)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.logical_null_count" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.logical_null_count" class="fn">logical_null_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of logical null values in this array. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.logical_null_count)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.get_buffer_memory_size" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.get_buffer_memory_size" class="fn">get_buffer_memory_size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of bytes of memory pointed to by this array. The buffers store bytes in the Arrow memory format, and include the data as well as the validity map. Note that this does not always correspond to the exact memory usage of an array, since multiple arrays can share the same buffers or slices thereof.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.get_array_memory_size" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.get_array_memory_size" class="fn">get_array_memory_size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of bytes of memory occupied physically by this array. This value will always be greater than returned by `get_buffer_memory_size()` and includes the overhead of the data structures that contain the pointers to the various buffers.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.logical_nulls" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.logical_nulls" class="fn">logical_nulls</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct datafusion::common::arrow::buffer::NullBuffer">NullBuffer</a>\>

Returns a potentially computed [`NullBuffer`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.NullBuffer.html "struct datafusion::common::arrow::buffer::NullBuffer") that represents the logical null values of this array, if any. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.logical_nulls)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.is_null" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.is_null" class="fn">is_null</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether the element at `index` is null according to [`Array::nulls`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.nulls "method datafusion::common::arrow::array::Array::nulls") [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.is_null)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.is_valid" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.is_valid" class="fn">is_valid</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether the element at `index` is *not* null, the opposite of [`Self::is_null`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.is_null "method arrow_array::array::Array::is_null::is_null"). [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.is_valid)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.null_count" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.null_count" class="fn">null_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of physical null values in this array. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.null_count)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.is_nullable" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.is_nullable" class="fn">is_nullable</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns `false` if the array is guaranteed to not contain any logical nulls [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.is_nullable)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#impl-ArrayAccessor-for-%26GenericByteViewArray%3CT%3E" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html" class="trait" title="trait datafusion::common::arrow::array::ArrayAccessor">ArrayAccessor</a> for &'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<T\>

where T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteViewType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteViewType">ByteViewType</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#associatedtype.Item" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#associatedtype.Item" class="associatedtype">Item</a> = &'a \<T as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteViewType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteViewType">ByteViewType</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteViewType.html#associatedtype.Native" class="associatedtype" title="type datafusion::common::arrow::datatypes::ByteViewType::Native">Native</a>

The Arrow type of the element being accessed.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.value-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#tymethod.value" class="fn">value</a>( &self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> \<&'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<T\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html" class="trait" title="trait datafusion::common::arrow::array::ArrayAccessor">ArrayAccessor</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#associatedtype.Item" class="associatedtype" title="type datafusion::common::arrow::array::ArrayAccessor::Item">Item</a>

Returns the element at index `i` [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#tymethod.value)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.value_unchecked-1" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#tymethod.value_unchecked" class="fn">value_unchecked</a>( &self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> \<&'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<T\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html" class="trait" title="trait datafusion::common::arrow::array::ArrayAccessor">ArrayAccessor</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#associatedtype.Item" class="associatedtype" title="type datafusion::common::arrow::array::ArrayAccessor::Item">Item</a>

Returns the element at index `i` [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#tymethod.value_unchecked)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#impl-BinaryArrayType%3C&#39;a%3E-for-%26GenericByteViewArray%3CBinaryViewType%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.BinaryArrayType.html" class="trait" title="trait datafusion::common::arrow::array::BinaryArrayType">BinaryArrayType</a>\<'a\> for &'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.BinaryViewType.html" class="struct" title="struct datafusion::common::arrow::datatypes::BinaryViewType">BinaryViewType</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.iter-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.BinaryArrayType.html#tymethod.iter" class="fn">iter</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.ArrayIter.html" class="struct" title="struct datafusion::common::arrow::array::ArrayIter">ArrayIter</a>\<&'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.BinaryViewType.html" class="struct" title="struct datafusion::common::arrow::datatypes::BinaryViewType">BinaryViewType</a>\>\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#" class="tooltip" data-notable-ty="ArrayIter&lt;&amp;&#39;a GenericByteViewArray&lt;BinaryViewType&gt;&gt;">ⓘ</a>

Constructs a new iterator

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#impl-Clone-for-GenericByteViewArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<T\>

where T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteViewType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteViewType">ByteViewType</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<T\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#impl-Debug-for-GenericByteViewArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<T\>

where T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteViewType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteViewType">ByteViewType</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#impl-From%3C%26GenericByteArray%3CFROM%3E%3E-for-GenericByteViewArray%3CV%3E" class="anchor">§</a>

### impl\<FROM, V\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<FROM\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<V\>

where FROM: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteArrayType">ByteArrayType</a>, \<FROM as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteArrayType">ByteArrayType</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteArrayType.html#associatedtype.Offset" class="associatedtype" title="type datafusion::common::arrow::datatypes::ByteArrayType::Offset">Offset</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait datafusion::common::arrow::array::OffsetSizeTrait">OffsetSizeTrait</a> + <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/cast/trait.ToPrimitive.html" class="trait" title="trait num_traits::cast::ToPrimitive">ToPrimitive</a>, V: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteViewType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteViewType">ByteViewType</a>\<Native = \<FROM as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteArrayType">ByteArrayType</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteArrayType.html#associatedtype.Native" class="associatedtype" title="type datafusion::common::arrow::datatypes::ByteArrayType::Native">Native</a>\>,

Efficiently convert a [`GenericByteArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html "struct datafusion::common::arrow::array::GenericByteArray") to a [`GenericByteViewArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html "struct datafusion::common::arrow::array::GenericByteViewArray")

For example this method can convert a [`StringArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.StringArray.html "type datafusion::common::arrow::array::StringArray") to a [`StringViewArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.StringViewArray.html "type datafusion::common::arrow::array::StringViewArray").

If the offsets are all less than u32::MAX, the new [`GenericByteViewArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html "struct datafusion::common::arrow::array::GenericByteViewArray") is built without copying the underlying string data (views are created directly into the existing buffer)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(byte_array: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<FROM\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<V\>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#impl-From%3CArrayData%3E-for-GenericByteViewArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.ArrayData.html" class="struct" title="struct datafusion::common::arrow::array::ArrayData">ArrayData</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<T\>

where T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteViewType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteViewType">ByteViewType</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.ArrayData.html" class="struct" title="struct datafusion::common::arrow::array::ArrayData">ArrayData</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<T\>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#impl-From%3CGenericByteViewArray%3CT%3E%3E-for-ArrayData" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<T\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.ArrayData.html" class="struct" title="struct datafusion::common::arrow::array::ArrayData">ArrayData</a>

where T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteViewType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteViewType">ByteViewType</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.from-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(array: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<T\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.ArrayData.html" class="struct" title="struct datafusion::common::arrow::array::ArrayData">ArrayData</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#impl-From%3CVec%3C%26%5Bu8%5D%3E%3E-for-GenericByteViewArray%3CBinaryViewType%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<&\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.BinaryViewType.html" class="struct" title="struct datafusion::common::arrow::datatypes::BinaryViewType">BinaryViewType</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.from-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<&\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.BinaryViewType.html" class="struct" title="struct datafusion::common::arrow::datatypes::BinaryViewType">BinaryViewType</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#impl-From%3CVec%3C%26str%3E%3E-for-GenericByteViewArray%3CStringViewType%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.StringViewType.html" class="struct" title="struct datafusion::common::arrow::datatypes::StringViewType">StringViewType</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.from-5" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.StringViewType.html" class="struct" title="struct datafusion::common::arrow::datatypes::StringViewType">StringViewType</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#impl-From%3CVec%3COption%3C%26%5Bu8%5D%3E%3E%3E-for-GenericByteViewArray%3CBinaryViewType%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.BinaryViewType.html" class="struct" title="struct datafusion::common::arrow::datatypes::BinaryViewType">BinaryViewType</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.from-4" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.BinaryViewType.html" class="struct" title="struct datafusion::common::arrow::datatypes::BinaryViewType">BinaryViewType</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#impl-From%3CVec%3COption%3C%26str%3E%3E%3E-for-GenericByteViewArray%3CStringViewType%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.StringViewType.html" class="struct" title="struct datafusion::common::arrow::datatypes::StringViewType">StringViewType</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.from-6" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.StringViewType.html" class="struct" title="struct datafusion::common::arrow::datatypes::StringViewType">StringViewType</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#impl-From%3CVec%3COption%3CString%3E%3E%3E-for-GenericByteViewArray%3CStringViewType%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.StringViewType.html" class="struct" title="struct datafusion::common::arrow::datatypes::StringViewType">StringViewType</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.from-8" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.StringViewType.html" class="struct" title="struct datafusion::common::arrow::datatypes::StringViewType">StringViewType</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#impl-From%3CVec%3CString%3E%3E-for-GenericByteViewArray%3CStringViewType%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.StringViewType.html" class="struct" title="struct datafusion::common::arrow::datatypes::StringViewType">StringViewType</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.from-7" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.StringViewType.html" class="struct" title="struct datafusion::common::arrow::datatypes::StringViewType">StringViewType</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#impl-FromIterator%3C%26Option%3CPtr%3E%3E-for-GenericByteViewArray%3CT%3E" class="anchor">§</a>

### impl\<'a, Ptr, T\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<&'a <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Ptr\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<T\>

where Ptr: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\<T as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteViewType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteViewType">ByteViewType</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteViewType.html#associatedtype.Native" class="associatedtype" title="type datafusion::common::arrow::datatypes::ByteViewType::Native">Native</a>\> + 'a, T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteViewType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteViewType">ByteViewType</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.from_iter" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<T\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = &'a <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Ptr\>\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#impl-FromIterator%3COption%3CPtr%3E%3E-for-GenericByteViewArray%3CT%3E" class="anchor">§</a>

### impl\<Ptr, T\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Ptr\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<T\>

where T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteViewType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteViewType">ByteViewType</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, Ptr: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\<T as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteViewType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteViewType">ByteViewType</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteViewType.html#associatedtype.Native" class="associatedtype" title="type datafusion::common::arrow::datatypes::ByteViewType::Native">Native</a>\>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.from_iter-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<T\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Ptr\>\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#impl-IntoIterator-for-%26GenericByteViewArray%3CT%3E" class="anchor">§</a>

### impl\<'a, T\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a> for &'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<T\>

where T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteViewType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteViewType">ByteViewType</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#associatedtype.Item-1" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a \<T as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteViewType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteViewType">ByteViewType</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteViewType.html#associatedtype.Native" class="associatedtype" title="type datafusion::common::arrow::datatypes::ByteViewType::Native">Native</a>\>

The type of the elements being iterated over.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#associatedtype.IntoIter" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype">IntoIter</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.ArrayIter.html" class="struct" title="struct datafusion::common::arrow::array::ArrayIter">ArrayIter</a>\<&'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<T\>\>

Which kind of iterator are we turning this into?

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.into_iter" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter" class="fn">into_iter</a>(self) -\> \<&'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<T\> as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::IntoIter">IntoIter</a>

Creates an iterator from a value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#impl-PartialEq-for-GenericByteViewArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<T\>

where T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteViewType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteViewType">ByteViewType</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<T\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#impl-StringArrayType%3C&#39;a%3E-for-%26GenericByteViewArray%3CStringViewType%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.StringArrayType.html" class="trait" title="trait datafusion::common::arrow::array::StringArrayType">StringArrayType</a>\<'a\> for &'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.StringViewType.html" class="struct" title="struct datafusion::common::arrow::datatypes::StringViewType">StringViewType</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.is_ascii-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.StringArrayType.html#tymethod.is_ascii" class="fn">is_ascii</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if all data within this string array is ASCII

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#method.iter-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.StringArrayType.html#tymethod.iter" class="fn">iter</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.ArrayIter.html" class="struct" title="struct datafusion::common::arrow::array::ArrayIter">ArrayIter</a>\<&'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.StringViewType.html" class="struct" title="struct datafusion::common::arrow::datatypes::StringViewType">StringViewType</a>\>\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#" class="tooltip" data-notable-ty="ArrayIter&lt;&amp;&#39;a GenericByteViewArray&lt;StringViewType&gt;&gt;">ⓘ</a>

Constructs a new iterator

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/struct.GenericByteViewArray.html#blanket-implementations" class="anchor">§</a>
