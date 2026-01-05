# Struct GenericListBuilder Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/builder/generic_list_builder.rs.html#88" class="src">Source</a>

``` rust
pub struct GenericListBuilder<OffsetSize, T>where
    OffsetSize: OffsetSizeTrait,
    T: ArrayBuilder,{ /* private fields */ }
```

Expand description

Builder for [`GenericListArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html "struct arrow::array::GenericListArray")

Use [`ListBuilder`](https://docs.rs/arrow/latest/arrow/array/type.ListBuilder.html "type arrow::array::ListBuilder") to build [`ListArray`](https://docs.rs/arrow/latest/arrow/array/type.ListArray.html "type arrow::array::ListArray")s and [`LargeListBuilder`](https://docs.rs/arrow/latest/arrow/array/type.LargeListBuilder.html "type arrow::array::LargeListBuilder") to build [`LargeListArray`](https://docs.rs/arrow/latest/arrow/array/type.LargeListArray.html "type arrow::array::LargeListArray")s.

## <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericListBuilder.html#example" class="doc-anchor">§</a>Example

Here is code that constructs a ListArray with the contents: `[[A,B,C], [], NULL, [D], [NULL, F]]`

``` rust
let values_builder = StringBuilder::new();
let mut builder = ListBuilder::new(values_builder);

// [A, B, C]
builder.values().append_value("A");
builder.values().append_value("B");
builder.values().append_value("C");
builder.append(true);

// [ ] (empty list)
builder.append(true);

// Null
builder.append(false);

// [D]
builder.values().append_value("D");
builder.append(true);

// [NULL, F]
builder.values().append_null();
builder.values().append_value("F");
builder.append(true);

// Build the array
let array = builder.finish();

// Values is a string array
// "A", "B" "C", "?", "D", NULL, "F"
assert_eq!(
  array.values().as_ref(),
  &StringArray::from(vec![
    Some("A"), Some("B"), Some("C"),
    Some("D"), None, Some("F")
  ])
);

// Offsets are indexes into the values array
assert_eq!(
  array.value_offsets(),
  &[0, 3, 3, 3, 4, 6]
);
```

## Implementations<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericListBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericListBuilder.html#impl-GenericListBuilder%3COffsetSize,+T%3E" class="anchor">§</a>

### impl\<OffsetSize, T\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListBuilder.html" class="struct" title="struct arrow::array::GenericListBuilder">GenericListBuilder</a>\<OffsetSize, T\>

where OffsetSize: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>, T: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a>,

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericListBuilder.html#method.new" class="fn">new</a>(values_builder: T) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListBuilder.html" class="struct" title="struct arrow::array::GenericListBuilder">GenericListBuilder</a>\<OffsetSize, T\>

Creates a new [`GenericListBuilder`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListBuilder.html "struct arrow::array::GenericListBuilder") from a given values array builder

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericListBuilder.html#method.with_capacity" class="fn">with_capacity</a>( values_builder: T, capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListBuilder.html" class="struct" title="struct arrow::array::GenericListBuilder">GenericListBuilder</a>\<OffsetSize, T\>

Creates a new [`GenericListBuilder`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListBuilder.html "struct arrow::array::GenericListBuilder") from a given values array builder `capacity` is the number of items to pre-allocate space for in this builder

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericListBuilder.html#method.with_field" class="fn">with_field</a>( self, field: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>\>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListBuilder.html" class="struct" title="struct arrow::array::GenericListBuilder">GenericListBuilder</a>\<OffsetSize, T\>

Override the field passed to [`GenericListArray::new`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html#method.new "associated function arrow::array::GenericListArray::new")

By default a nullable field is created with the name `item`

Note: [`Self::finish`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListBuilder.html#method.finish "method arrow::array::GenericListBuilder::finish") and [`Self::finish_cloned`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListBuilder.html#method.finish_cloned "method arrow::array::GenericListBuilder::finish_cloned") will panic if the field’s data type does not match that of `T`

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericListBuilder.html#impl-GenericListBuilder%3COffsetSize,+T%3E-1" class="anchor">§</a>

### impl\<OffsetSize, T\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListBuilder.html" class="struct" title="struct arrow::array::GenericListBuilder">GenericListBuilder</a>\<OffsetSize, T\>

where OffsetSize: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>, T: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a> + 'static,

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericListBuilder.html#method.values" class="fn">values</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut T</a>

Returns the child array builder as a mutable reference.

This mutable reference can be used to append values into the child array builder, but you must call [`append`](https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericListBuilder.html#method.append) to delimit each distinct list value.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericListBuilder.html#method.values_ref" class="fn">values_ref</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>

Returns the child array builder as an immutable reference

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericListBuilder.html#method.append" class="fn">append</a>(&mut self, is_valid: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>)

Finish the current variable-length list array slot

##### <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericListBuilder.html#panics" class="doc-anchor">§</a>Panics

Panics if the length of [`Self::values`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListBuilder.html#method.values "method arrow::array::GenericListBuilder::values") exceeds `OffsetSize::MAX`

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericListBuilder.html#method.append_value" class="fn">append_value</a>\<I, V\>(&mut self, i: I)

where T: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html" class="trait" title="trait core::iter::traits::collect::Extend">Extend</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<V\>\>, I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<V\>\>,

Append a value to this [`GenericListBuilder`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListBuilder.html "struct arrow::array::GenericListBuilder")

``` rust
let mut builder = ListBuilder::new(Int32Builder::new());

builder.append_value([Some(1), Some(2), Some(3)]);
builder.append_value([]);
builder.append_value([None]);

let array = builder.finish();
assert_eq!(array.len(), 3);

assert_eq!(array.value_offsets(), &[0, 3, 3, 4]);
let values = array.values().as_primitive::<Int32Type>();
assert_eq!(values, &Int32Array::from(vec![Some(1), Some(2), Some(3), None]));
```

This is an alternative API to appending directly to [`Self::values`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListBuilder.html#method.values "method arrow::array::GenericListBuilder::values") and delimiting the result with [`Self::append`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListBuilder.html#method.append "method arrow::array::GenericListBuilder::append")

``` rust
let mut builder = ListBuilder::new(Int32Builder::new());

builder.values().append_value(1);
builder.values().append_value(2);
builder.values().append_value(3);
builder.append(true);
builder.append(true);
builder.values().append_null();
builder.append(true);

let array = builder.finish();
assert_eq!(array.len(), 3);

assert_eq!(array.value_offsets(), &[0, 3, 3, 4]);
let values = array.values().as_primitive::<Int32Type>();
assert_eq!(values, &Int32Array::from(vec![Some(1), Some(2), Some(3), None]));
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericListBuilder.html#method.append_null" class="fn">append_null</a>(&mut self)

Append a null to this [`GenericListBuilder`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListBuilder.html "struct arrow::array::GenericListBuilder")

See [`Self::append_value`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListBuilder.html#method.append_value "method arrow::array::GenericListBuilder::append_value") for an example use.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericListBuilder.html#method.append_nulls" class="fn">append_nulls</a>(&mut self, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Appends `n` `null`s into the builder.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericListBuilder.html#method.append_option" class="fn">append_option</a>\<I, V\>(&mut self, i: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<I\>)

where T: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html" class="trait" title="trait core::iter::traits::collect::Extend">Extend</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<V\>\>, I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<V\>\>,

Appends an optional value into this [`GenericListBuilder`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListBuilder.html "struct arrow::array::GenericListBuilder")

If `Some` calls [`Self::append_value`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListBuilder.html#method.append_value "method arrow::array::GenericListBuilder::append_value") otherwise calls [`Self::append_null`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListBuilder.html#method.append_null "method arrow::array::GenericListBuilder::append_null")

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericListBuilder.html#method.finish" class="fn">finish</a>(&mut self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html" class="struct" title="struct arrow::array::GenericListArray">GenericListArray</a>\<OffsetSize\>

Builds the [`GenericListArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html "struct arrow::array::GenericListArray") and reset this builder.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericListBuilder.html#method.finish_cloned" class="fn">finish_cloned</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html" class="struct" title="struct arrow::array::GenericListArray">GenericListArray</a>\<OffsetSize\>

Builds the [`GenericListArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html "struct arrow::array::GenericListArray") without resetting the builder.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericListBuilder.html#method.offsets_slice" class="fn">offsets_slice</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[OffsetSize]</a>

Returns the current offsets buffer as a slice

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericListBuilder.html#method.validity_slice" class="fn">validity_slice</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>

Returns the current null buffer as a slice

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericListBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericListBuilder.html#impl-ArrayBuilder-for-GenericListBuilder%3COffsetSize,+T%3E" class="anchor">§</a>

### impl\<OffsetSize, T\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListBuilder.html" class="struct" title="struct arrow::array::GenericListBuilder">GenericListBuilder</a>\<OffsetSize, T\>

where OffsetSize: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>, T: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a> + 'static,

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericListBuilder.html#method.as_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the builder as a non-mutable `Any` reference.

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericListBuilder.html#method.as_any_mut" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.as_any_mut" class="fn">as_any_mut</a>(&mut self) -\> &mut (dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the builder as a mutable `Any` reference.

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericListBuilder.html#method.into_box_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.into_box_any" class="fn">into_box_any</a>(self: <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListBuilder.html" class="struct" title="struct arrow::array::GenericListBuilder">GenericListBuilder</a>\<OffsetSize, T\>\>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a>\>

Returns the boxed builder as a box of `Any`.

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericListBuilder.html#method.len" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the number of array slots in the builder

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericListBuilder.html#method.finish-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.finish" class="fn">finish</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

Builds the array and reset this builder.

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericListBuilder.html#method.finish_cloned-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.finish_cloned" class="fn">finish_cloned</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

Builds the array without resetting the builder.

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericListBuilder.html#method.is_empty" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether number of array slots is zero

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericListBuilder.html#impl-Debug-for-GenericListBuilder%3COffsetSize,+T%3E" class="anchor">§</a>

### impl\<OffsetSize, T\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListBuilder.html" class="struct" title="struct arrow::array::GenericListBuilder">GenericListBuilder</a>\<OffsetSize, T\>

where OffsetSize: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> + <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>, T: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> + <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericListBuilder.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericListBuilder.html#impl-Default-for-GenericListBuilder%3CO,+T%3E" class="anchor">§</a>

### impl\<O, T\> <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListBuilder.html" class="struct" title="struct arrow::array::GenericListBuilder">GenericListBuilder</a>\<O, T\>

where O: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>, T: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a> + <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericListBuilder.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListBuilder.html" class="struct" title="struct arrow::array::GenericListBuilder">GenericListBuilder</a>\<O, T\>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericListBuilder.html#impl-Extend%3COption%3CV%3E%3E-for-GenericListBuilder%3CO,+B%3E" class="anchor">§</a>

### impl\<O, B, V, E\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html" class="trait" title="trait core::iter::traits::collect::Extend">Extend</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<V\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListBuilder.html" class="struct" title="struct arrow::array::GenericListBuilder">GenericListBuilder</a>\<O, B\>

where O: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>, B: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a> + <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html" class="trait" title="trait core::iter::traits::collect::Extend">Extend</a>\<E\>, V: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = E\>,

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericListBuilder.html#method.extend" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend" class="fn">extend</a>\<T\>(&mut self, iter: T)

where T: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<V\>\>,

Extends a collection with the contents of an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend)

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericListBuilder.html#method.extend_one" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_one" class="fn">extend_one</a>(&mut self, item: A)

🔬This is a nightly-only experimental API. (`extend_one`)

Extends a collection with exactly one element.

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericListBuilder.html#method.extend_reserve" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve" class="fn">extend_reserve</a>(&mut self, additional: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

🔬This is a nightly-only experimental API. (`extend_one`)

Reserves capacity in a collection for the given number of additional elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve)

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericListBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericListBuilder.html#blanket-implementations" class="anchor">§</a>
