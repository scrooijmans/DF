# Struct FixedSizeBinaryBuilder Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/builder/fixed_size_binary_builder.rs.html#44" class="src">Source</a>

``` rust
pub struct FixedSizeBinaryBuilder { /* private fields */ }
```

Expand description

Builder for [`FixedSizeBinaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html "struct arrow::array::FixedSizeBinaryArray")

``` rust
let mut builder = FixedSizeBinaryBuilder::with_capacity(3, 5);
// [b"hello", null, b"arrow"]
builder.append_value(b"hello").unwrap();
builder.append_null();
builder.append_value(b"arrow").unwrap();

let array = builder.finish();
assert_eq!(array.value(0), b"hello");
assert!(array.is_null(1));
assert_eq!(array.value(2), b"arrow");
```

## Implementations<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.FixedSizeBinaryBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.FixedSizeBinaryBuilder.html#impl-FixedSizeBinaryBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryBuilder.html" class="struct" title="struct arrow::array::FixedSizeBinaryBuilder">FixedSizeBinaryBuilder</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.FixedSizeBinaryBuilder.html#method.new" class="fn">new</a>(byte_width: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryBuilder.html" class="struct" title="struct arrow::array::FixedSizeBinaryBuilder">FixedSizeBinaryBuilder</a>

Creates a new [`FixedSizeBinaryBuilder`](https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryBuilder.html "struct arrow::array::FixedSizeBinaryBuilder")

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.FixedSizeBinaryBuilder.html#method.with_capacity" class="fn">with_capacity</a>(capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, byte_width: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryBuilder.html" class="struct" title="struct arrow::array::FixedSizeBinaryBuilder">FixedSizeBinaryBuilder</a>

Creates a new [`FixedSizeBinaryBuilder`](https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryBuilder.html "struct arrow::array::FixedSizeBinaryBuilder"), `capacity` is the number of byte slices that can be appended without reallocating

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.FixedSizeBinaryBuilder.html#method.append_value" class="fn">append_value</a>( &mut self, value: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Appends a byte slice into the builder.

Automatically update the null buffer to delimit the slice appended in as a distinct value element.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.FixedSizeBinaryBuilder.html#method.append_null" class="fn">append_null</a>(&mut self)

Append a null value to the array.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.FixedSizeBinaryBuilder.html#method.append_nulls" class="fn">append_nulls</a>(&mut self, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Appends `n` `null`s into the builder.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.FixedSizeBinaryBuilder.html#method.values_slice" class="fn">values_slice</a>(&self) -\> &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\] <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.FixedSizeBinaryBuilder.html#" class="tooltip" data-notable-ty="&amp;[u8]">ⓘ</a>

Returns the current values buffer as a slice

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.FixedSizeBinaryBuilder.html#method.finish" class="fn">finish</a>(&mut self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html" class="struct" title="struct arrow::array::FixedSizeBinaryArray">FixedSizeBinaryArray</a>

Builds the [`FixedSizeBinaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html "struct arrow::array::FixedSizeBinaryArray") and reset this builder.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.FixedSizeBinaryBuilder.html#method.finish_cloned" class="fn">finish_cloned</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html" class="struct" title="struct arrow::array::FixedSizeBinaryArray">FixedSizeBinaryArray</a>

Builds the [`FixedSizeBinaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html "struct arrow::array::FixedSizeBinaryArray") without resetting the builder.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.FixedSizeBinaryBuilder.html#method.validity_slice" class="fn">validity_slice</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>

Returns the current null buffer as a slice

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.FixedSizeBinaryBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.FixedSizeBinaryBuilder.html#impl-ArrayBuilder-for-FixedSizeBinaryBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryBuilder.html" class="struct" title="struct arrow::array::FixedSizeBinaryBuilder">FixedSizeBinaryBuilder</a>

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.FixedSizeBinaryBuilder.html#method.as_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the builder as a non-mutable `Any` reference.

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.FixedSizeBinaryBuilder.html#method.as_any_mut" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.as_any_mut" class="fn">as_any_mut</a>(&mut self) -\> &mut (dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the builder as a mutable `Any` reference.

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.FixedSizeBinaryBuilder.html#method.into_box_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.into_box_any" class="fn">into_box_any</a>(self: <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryBuilder.html" class="struct" title="struct arrow::array::FixedSizeBinaryBuilder">FixedSizeBinaryBuilder</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a>\>

Returns the boxed builder as a box of `Any`.

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.FixedSizeBinaryBuilder.html#method.len" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the number of array slots in the builder

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.FixedSizeBinaryBuilder.html#method.finish-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.finish" class="fn">finish</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

Builds the array and reset this builder.

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.FixedSizeBinaryBuilder.html#method.finish_cloned-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.finish_cloned" class="fn">finish_cloned</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

Builds the array without resetting the builder.

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.FixedSizeBinaryBuilder.html#method.is_empty" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether number of array slots is zero

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.FixedSizeBinaryBuilder.html#impl-Debug-for-FixedSizeBinaryBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryBuilder.html" class="struct" title="struct arrow::array::FixedSizeBinaryBuilder">FixedSizeBinaryBuilder</a>

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.FixedSizeBinaryBuilder.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.FixedSizeBinaryBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.FixedSizeBinaryBuilder.html#blanket-implementations" class="anchor">§</a>
