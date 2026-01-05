# Struct MapBuilder Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/builder/map_builder.rs.html#58" class="src">Source</a>

``` rust
pub struct MapBuilder<K, V>where
    K: ArrayBuilder,
    V: ArrayBuilder,{ /* private fields */ }
```

Expand description

Builder for [`MapArray`](https://docs.rs/arrow/latest/arrow/array/struct.MapArray.html "struct arrow::array::MapArray")

``` rust

let string_builder = StringBuilder::new();
let int_builder = Int32Builder::with_capacity(4);

// Construct `[{"joe": 1}, {"blogs": 2, "foo": 4}, {}, null]`
let mut builder = MapBuilder::new(None, string_builder, int_builder);

builder.keys().append_value("joe");
builder.values().append_value(1);
builder.append(true).unwrap();

builder.keys().append_value("blogs");
builder.values().append_value(2);
builder.keys().append_value("foo");
builder.values().append_value(4);
builder.append(true).unwrap();
builder.append(true).unwrap();
builder.append(false).unwrap();

let array = builder.finish();
assert_eq!(array.value_offsets(), &[0, 1, 3, 3, 3]);
assert_eq!(array.values().as_ref(), &Int32Array::from(vec![1, 2, 4]));
assert_eq!(array.keys().as_ref(), &StringArray::from(vec!["joe", "blogs", "foo"]));
```

## Implementations<a href="https://docs.rs/arrow/latest/arrow/array/struct.MapBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.MapBuilder.html#impl-MapBuilder%3CK,+V%3E" class="anchor">§</a>

### impl\<K, V\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.MapBuilder.html" class="struct" title="struct arrow::array::MapBuilder">MapBuilder</a>\<K, V\>

where K: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a>, V: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a>,

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.MapBuilder.html#method.new" class="fn">new</a>( field_names: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.MapFieldNames.html" class="struct" title="struct arrow::array::MapFieldNames">MapFieldNames</a>\>, key_builder: K, value_builder: V, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.MapBuilder.html" class="struct" title="struct arrow::array::MapBuilder">MapBuilder</a>\<K, V\>

Creates a new `MapBuilder`

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.MapBuilder.html#method.with_capacity" class="fn">with_capacity</a>( field_names: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.MapFieldNames.html" class="struct" title="struct arrow::array::MapFieldNames">MapFieldNames</a>\>, key_builder: K, value_builder: V, capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.MapBuilder.html" class="struct" title="struct arrow::array::MapBuilder">MapBuilder</a>\<K, V\>

Creates a new `MapBuilder` with capacity

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.MapBuilder.html#method.with_keys_field" class="fn">with_keys_field</a>(self, field: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>\>\>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.MapBuilder.html" class="struct" title="struct arrow::array::MapBuilder">MapBuilder</a>\<K, V\>

Override the field passed to [`MapBuilder::new`](https://docs.rs/arrow/latest/arrow/array/struct.MapBuilder.html#method.new "associated function arrow::array::MapBuilder::new")

By default, a non-nullable field is created with the name `keys`

Note: [`Self::finish`](https://docs.rs/arrow/latest/arrow/array/struct.MapBuilder.html#method.finish "method arrow::array::MapBuilder::finish") and [`Self::finish_cloned`](https://docs.rs/arrow/latest/arrow/array/struct.MapBuilder.html#method.finish_cloned "method arrow::array::MapBuilder::finish_cloned") will panic if the field’s data type does not match that of `K` or the field is nullable

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.MapBuilder.html#method.with_values_field" class="fn">with_values_field</a>(self, field: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>\>\>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.MapBuilder.html" class="struct" title="struct arrow::array::MapBuilder">MapBuilder</a>\<K, V\>

Override the field passed to [`MapBuilder::new`](https://docs.rs/arrow/latest/arrow/array/struct.MapBuilder.html#method.new "associated function arrow::array::MapBuilder::new")

By default, a nullable field is created with the name `values`

Note: [`Self::finish`](https://docs.rs/arrow/latest/arrow/array/struct.MapBuilder.html#method.finish "method arrow::array::MapBuilder::finish") and [`Self::finish_cloned`](https://docs.rs/arrow/latest/arrow/array/struct.MapBuilder.html#method.finish_cloned "method arrow::array::MapBuilder::finish_cloned") will panic if the field’s data type does not match that of `V`

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.MapBuilder.html#method.keys" class="fn">keys</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut K</a>

Returns the key array builder of the map

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.MapBuilder.html#method.values" class="fn">values</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut V</a>

Returns the value array builder of the map

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.MapBuilder.html#method.entries" class="fn">entries</a>(&mut self) -\> (<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut K</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut V</a>)

Returns both the key and value array builders of the map

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.MapBuilder.html#method.append" class="fn">append</a>(&mut self, is_valid: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Finish the current map array slot

Returns an error if the key and values builders are in an inconsistent state.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.MapBuilder.html#method.finish" class="fn">finish</a>(&mut self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.MapArray.html" class="struct" title="struct arrow::array::MapArray">MapArray</a>

Builds the [`MapArray`](https://docs.rs/arrow/latest/arrow/array/struct.MapArray.html "struct arrow::array::MapArray")

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.MapBuilder.html#method.finish_cloned" class="fn">finish_cloned</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.MapArray.html" class="struct" title="struct arrow::array::MapArray">MapArray</a>

Builds the [`MapArray`](https://docs.rs/arrow/latest/arrow/array/struct.MapArray.html "struct arrow::array::MapArray") without resetting the builder.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.MapBuilder.html#method.validity_slice" class="fn">validity_slice</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>

Returns the current null buffer as a slice

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/struct.MapBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.MapBuilder.html#impl-ArrayBuilder-for-MapBuilder%3CK,+V%3E" class="anchor">§</a>

### impl\<K, V\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.MapBuilder.html" class="struct" title="struct arrow::array::MapBuilder">MapBuilder</a>\<K, V\>

where K: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a>, V: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/struct.MapBuilder.html#method.finish_cloned-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.finish_cloned" class="fn">finish_cloned</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

Builds the array without resetting the builder.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.MapBuilder.html#method.len" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the number of array slots in the builder

<a href="https://docs.rs/arrow/latest/arrow/array/struct.MapBuilder.html#method.finish-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.finish" class="fn">finish</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

Builds the array

<a href="https://docs.rs/arrow/latest/arrow/array/struct.MapBuilder.html#method.as_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the builder as a non-mutable `Any` reference. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.as_any)

<a href="https://docs.rs/arrow/latest/arrow/array/struct.MapBuilder.html#method.as_any_mut" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.as_any_mut" class="fn">as_any_mut</a>(&mut self) -\> &mut (dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the builder as a mutable `Any` reference. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.as_any_mut)

<a href="https://docs.rs/arrow/latest/arrow/array/struct.MapBuilder.html#method.into_box_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.into_box_any" class="fn">into_box_any</a>(self: <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.MapBuilder.html" class="struct" title="struct arrow::array::MapBuilder">MapBuilder</a>\<K, V\>\>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a>\>

Returns the boxed builder as a box of `Any`.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.MapBuilder.html#method.is_empty" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether number of array slots is zero

<a href="https://docs.rs/arrow/latest/arrow/array/struct.MapBuilder.html#impl-Debug-for-MapBuilder%3CK,+V%3E" class="anchor">§</a>

### impl\<K, V\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.MapBuilder.html" class="struct" title="struct arrow::array::MapBuilder">MapBuilder</a>\<K, V\>

where K: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> + <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a>, V: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> + <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/struct.MapBuilder.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/struct.MapBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/array/struct.MapBuilder.html#blanket-implementations" class="anchor">§</a>
