# Struct FixedSizeListBuilder Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/builder/fixed_size_list_builder.rs.html#65" class="src">Source</a>

``` rust
pub struct FixedSizeListBuilder<T>where
    T: ArrayBuilder,{ /* private fields */ }
```

Expand description

Builder for [`FixedSizeListArray`](https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListArray.html "struct arrow::array::FixedSizeListArray")

``` rust
use arrow_array::{builder::{Int32Builder, FixedSizeListBuilder}, Array, Int32Array};
let values_builder = Int32Builder::new();
let mut builder = FixedSizeListBuilder::new(values_builder, 3);

//  [[0, 1, 2], null, [3, null, 5], [6, 7, null]]
builder.values().append_value(0);
builder.values().append_value(1);
builder.values().append_value(2);
builder.append(true);
builder.values().append_null();
builder.values().append_null();
builder.values().append_null();
builder.append(false);
builder.values().append_value(3);
builder.values().append_null();
builder.values().append_value(5);
builder.append(true);
builder.values().append_value(6);
builder.values().append_value(7);
builder.values().append_null();
builder.append(true);
let list_array = builder.finish();
assert_eq!(
    *list_array.value(0),
    Int32Array::from(vec![Some(0), Some(1), Some(2)])
);
assert!(list_array.is_null(1));
assert_eq!(
    *list_array.value(2),
    Int32Array::from(vec![Some(3), None, Some(5)])
);
assert_eq!(
    *list_array.value(3),
    Int32Array::from(vec![Some(6), Some(7), None])
)
```

## Implementations<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListBuilder.html#impl-FixedSizeListBuilder%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListBuilder.html" class="struct" title="struct arrow::array::FixedSizeListBuilder">FixedSizeListBuilder</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a>,

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListBuilder.html#method.new" class="fn">new</a>(values_builder: T, value_length: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListBuilder.html" class="struct" title="struct arrow::array::FixedSizeListBuilder">FixedSizeListBuilder</a>\<T\>

Creates a new [`FixedSizeListBuilder`](https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListBuilder.html "struct arrow::array::FixedSizeListBuilder") from a given values array builder `value_length` is the number of values within each array

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListBuilder.html#method.with_capacity" class="fn">with_capacity</a>( values_builder: T, value_length: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListBuilder.html" class="struct" title="struct arrow::array::FixedSizeListBuilder">FixedSizeListBuilder</a>\<T\>

Creates a new [`FixedSizeListBuilder`](https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListBuilder.html "struct arrow::array::FixedSizeListBuilder") from a given values array builder `value_length` is the number of values within each array `capacity` is the number of items to pre-allocate space for in this builder

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListBuilder.html#method.with_field" class="fn">with_field</a>(self, field: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>\>\>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListBuilder.html" class="struct" title="struct arrow::array::FixedSizeListBuilder">FixedSizeListBuilder</a>\<T\>

Override the field passed to [`FixedSizeListArray::new`](https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListArray.html#method.new "associated function arrow::array::FixedSizeListArray::new")

By default, a nullable field is created with the name `item`

Note: [`Self::finish`](https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListBuilder.html#method.finish "method arrow::array::FixedSizeListBuilder::finish") and [`Self::finish_cloned`](https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListBuilder.html#method.finish_cloned "method arrow::array::FixedSizeListBuilder::finish_cloned") will panic if the field’s data type does not match that of `T`

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListBuilder.html#impl-FixedSizeListBuilder%3CT%3E-1" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListBuilder.html" class="struct" title="struct arrow::array::FixedSizeListBuilder">FixedSizeListBuilder</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a> + 'static,

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListBuilder.html#method.values" class="fn">values</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut T</a>

Returns the child array builder as a mutable reference.

This mutable reference can be used to append values into the child array builder, but you must call [`append`](https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListBuilder.html#method.append) to delimit each distinct list value.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListBuilder.html#method.value_length" class="fn">value_length</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

Returns the length of the list

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListBuilder.html#method.append" class="fn">append</a>(&mut self, is_valid: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>)

Finish the current fixed-length list array slot

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListBuilder.html#method.finish" class="fn">finish</a>(&mut self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListArray.html" class="struct" title="struct arrow::array::FixedSizeListArray">FixedSizeListArray</a>

Builds the [`FixedSizeListBuilder`](https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListBuilder.html "struct arrow::array::FixedSizeListBuilder") and reset this builder.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListBuilder.html#method.finish_cloned" class="fn">finish_cloned</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListArray.html" class="struct" title="struct arrow::array::FixedSizeListArray">FixedSizeListArray</a>

Builds the [`FixedSizeListBuilder`](https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListBuilder.html "struct arrow::array::FixedSizeListBuilder") without resetting the builder.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListBuilder.html#method.validity_slice" class="fn">validity_slice</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>

Returns the current null buffer as a slice

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListBuilder.html#impl-ArrayBuilder-for-FixedSizeListBuilder%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListBuilder.html" class="struct" title="struct arrow::array::FixedSizeListBuilder">FixedSizeListBuilder</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a> + 'static,

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListBuilder.html#method.as_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the builder as a non-mutable `Any` reference.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListBuilder.html#method.as_any_mut" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.as_any_mut" class="fn">as_any_mut</a>(&mut self) -\> &mut (dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the builder as a mutable `Any` reference.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListBuilder.html#method.into_box_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.into_box_any" class="fn">into_box_any</a>(self: <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListBuilder.html" class="struct" title="struct arrow::array::FixedSizeListBuilder">FixedSizeListBuilder</a>\<T\>\>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a>\>

Returns the boxed builder as a box of `Any`.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListBuilder.html#method.len" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the number of array slots in the builder

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListBuilder.html#method.finish-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.finish" class="fn">finish</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

Builds the array and reset this builder.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListBuilder.html#method.finish_cloned-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.finish_cloned" class="fn">finish_cloned</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

Builds the array without resetting the builder.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListBuilder.html#method.is_empty" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether number of array slots is zero

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListBuilder.html#impl-Debug-for-FixedSizeListBuilder%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListBuilder.html" class="struct" title="struct arrow::array::FixedSizeListBuilder">FixedSizeListBuilder</a>\<T\>

where T: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> + <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListBuilder.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListBuilder.html#blanket-implementations" class="anchor">§</a>
