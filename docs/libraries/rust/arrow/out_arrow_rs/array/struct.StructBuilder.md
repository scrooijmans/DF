# Struct StructBuilder Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/builder/struct_builder.rs.html#102" class="src">Source</a>

``` rust
pub struct StructBuilder { /* private fields */ }
```

Expand description

Builder for [`StructArray`](https://docs.rs/arrow/latest/arrow/array/struct.StructArray.html "struct arrow::array::StructArray")

Note that callers should make sure that methods of all the child field builders are properly called to maintain the consistency of the data structure.

Handling arrays with complex layouts, such as `List<Struct<List<Struct>>>`, in Rust can be challenging due to its strong typing system. To construct a collection builder ([`ListBuilder`](https://docs.rs/arrow/latest/arrow/array/type.ListBuilder.html "type arrow::array::ListBuilder"), [`LargeListBuilder`](https://docs.rs/arrow/latest/arrow/array/type.LargeListBuilder.html "type arrow::array::LargeListBuilder"), or [`MapBuilder`](https://docs.rs/arrow/latest/arrow/array/struct.MapBuilder.html "struct arrow::array::MapBuilder")) using [`make_builder`](https://docs.rs/arrow/latest/arrow/array/fn.make_builder.html "fn arrow::array::make_builder"), multiple calls are required. This complexity arises from the recursive approach utilized by [`StructBuilder::from_fields`](https://docs.rs/arrow/latest/arrow/array/struct.StructBuilder.html#method.from_fields "associated function arrow::array::StructBuilder::from_fields").

Initially, [`StructBuilder::from_fields`](https://docs.rs/arrow/latest/arrow/array/struct.StructBuilder.html#method.from_fields "associated function arrow::array::StructBuilder::from_fields") invokes [`make_builder`](https://docs.rs/arrow/latest/arrow/array/fn.make_builder.html "fn arrow::array::make_builder"), which returns a `Box<dyn ArrayBuilder>`. To obtain the specific collection builder, one must first use [`StructBuilder::field_builder`](https://docs.rs/arrow/latest/arrow/array/struct.StructBuilder.html#method.field_builder "method arrow::array::StructBuilder::field_builder") to get a `Collection<[Box<dyn ArrayBuilder>]>`. Subsequently, the `values()` result from this operation can be downcast to the desired builder type.

For example, when working with [`ListBuilder`](https://docs.rs/arrow/latest/arrow/array/type.ListBuilder.html "type arrow::array::ListBuilder"), you would first call [`StructBuilder::field_builder::<ListBuilder<Box<dyn ArrayBuilder>>>`](https://docs.rs/arrow/latest/arrow/array/struct.StructBuilder.html#method.field_builder "method arrow::array::StructBuilder::field_builder") and then downcast the [`Box<dyn ArrayBuilder>`](https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html "struct alloc::boxed::Box") to the specific [`StructBuilder`](https://docs.rs/arrow/latest/arrow/array/struct.StructBuilder.html "struct arrow::array::StructBuilder") you need.

For a practical example see the code below:

``` rust
   use arrow_array::builder::{ArrayBuilder, ListBuilder, StringBuilder, StructBuilder};
   use arrow_schema::{DataType, Field, Fields};
   use std::sync::Arc;

   // This is an example column that has a List<Struct<List<Struct>>> layout
   let mut example_col = ListBuilder::new(StructBuilder::from_fields(
       vec![Field::new(
           "value_list",
           DataType::List(Arc::new(Field::new_list_field(
               DataType::Struct(Fields::from(vec![
                   Field::new("key", DataType::Utf8, true),
                   Field::new("value", DataType::Utf8, true),
               ])), //In this example we are trying to get to this builder and insert key/value pairs
               true,
           ))),
           true,
       )],
       0,
   ));

  // We can obtain the StructBuilder without issues, because example_col was created with StructBuilder
  let col_struct_builder: &mut StructBuilder = example_col.values();

  // We can't obtain the ListBuilder<StructBuilder> with the expected generic types, because under the hood
  // the StructBuilder was returned as a Box<dyn ArrayBuilder> and passed as such to the ListBuilder constructor
   
  // This panics in runtime, even though we know that the builder is a ListBuilder<StructBuilder>.
  // let sb = col_struct_builder
  //     .field_builder::<ListBuilder<StructBuilder>>(0)
  //     .as_mut()
  //     .unwrap();

  //To keep in line with Rust's strong typing, we fetch a ListBuilder<Box<dyn ArrayBuilder>> from the column StructBuilder first...
  let mut list_builder_option =
      col_struct_builder.field_builder::<ListBuilder<Box<dyn ArrayBuilder>>>(0);

  let list_builder = list_builder_option.as_mut().unwrap();

  // ... and then downcast the key/value pair values to a StructBuilder
  let struct_builder = list_builder
      .values()
      .as_any_mut()
      .downcast_mut::<StructBuilder>()
      .unwrap();

  // We can now append values to the StructBuilder
  let key_builder = struct_builder.field_builder::<StringBuilder>(0).unwrap();
  key_builder.append_value("my key");

  let value_builder = struct_builder.field_builder::<StringBuilder>(1).unwrap();
  value_builder.append_value("my value");

  struct_builder.append(true);
  list_builder.append(true);
  col_struct_builder.append(true);
  example_col.append(true);

  let array = example_col.finish();

  println!("My array: {:?}", array);
```

## Implementations<a href="https://docs.rs/arrow/latest/arrow/array/struct.StructBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.StructBuilder.html#impl-StructBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/struct.StructBuilder.html" class="struct" title="struct arrow::array::StructBuilder">StructBuilder</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.StructBuilder.html#method.new" class="fn">new</a>( fields: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Fields.html" class="struct" title="struct arrow::datatypes::Fields">Fields</a>\>, field_builders: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a>\>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.StructBuilder.html" class="struct" title="struct arrow::array::StructBuilder">StructBuilder</a>

Creates a new `StructBuilder`

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.StructBuilder.html#method.from_fields" class="fn">from_fields</a>(fields: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Fields.html" class="struct" title="struct arrow::datatypes::Fields">Fields</a>\>, capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.StructBuilder.html" class="struct" title="struct arrow::array::StructBuilder">StructBuilder</a>

Creates a new `StructBuilder` from [`Fields`](https://docs.rs/arrow/latest/arrow/datatypes/struct.Fields.html "struct arrow::datatypes::Fields") and `capacity`

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.StructBuilder.html#method.field_builder" class="fn">field_builder</a>\<T\>(&mut self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut T</a>\>

where T: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a>,

Returns a mutable reference to the child field builder at index `i`. Result will be `None` if the input type `T` provided doesn’t match the actual field builder’s type.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.StructBuilder.html#method.field_builders" class="fn">field_builders</a>(&self) -\> &\[<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a>\>\]

Returns a reference to field builders

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.StructBuilder.html#method.field_builders_mut" class="fn">field_builders_mut</a>(&mut self) -\> &mut \[<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a>\>\]

Returns a mutable reference to field builders

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.StructBuilder.html#method.num_fields" class="fn">num_fields</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the number of fields for the struct this builder is building.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.StructBuilder.html#method.append" class="fn">append</a>(&mut self, is_valid: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>)

Appends an element (either null or non-null) to the struct. The actual elements should be appended for each child sub-array in a consistent way.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.StructBuilder.html#method.append_null" class="fn">append_null</a>(&mut self)

Appends a null element to the struct.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.StructBuilder.html#method.append_nulls" class="fn">append_nulls</a>(&mut self, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Appends `n` `null`s into the builder.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.StructBuilder.html#method.finish" class="fn">finish</a>(&mut self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.StructArray.html" class="struct" title="struct arrow::array::StructArray">StructArray</a>

Builds the `StructArray` and reset this builder.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.StructBuilder.html#method.finish_cloned" class="fn">finish_cloned</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.StructArray.html" class="struct" title="struct arrow::array::StructArray">StructArray</a>

Builds the `StructArray` without resetting the builder.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.StructBuilder.html#method.validity_slice" class="fn">validity_slice</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>

Returns the current null buffer as a slice

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/struct.StructBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.StructBuilder.html#impl-ArrayBuilder-for-StructBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.StructBuilder.html" class="struct" title="struct arrow::array::StructBuilder">StructBuilder</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.StructBuilder.html#method.len" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the number of array slots in the builder.

Note that this always return the first child field builder’s length, and it is the caller’s responsibility to maintain the consistency that all the child field builder should have the equal number of elements.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.StructBuilder.html#method.finish-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.finish" class="fn">finish</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

Builds the array.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.StructBuilder.html#method.finish_cloned-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.finish_cloned" class="fn">finish_cloned</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

Builds the array without resetting the builder.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.StructBuilder.html#method.as_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the builder as a non-mutable `Any` reference.

This is most useful when one wants to call non-mutable APIs on a specific builder type. In this case, one can first cast this into a `Any`, and then use `downcast_ref` to get a reference on the specific builder.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.StructBuilder.html#method.as_any_mut" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.as_any_mut" class="fn">as_any_mut</a>(&mut self) -\> &mut (dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the builder as a mutable `Any` reference.

This is most useful when one wants to call mutable APIs on a specific builder type. In this case, one can first cast this into a `Any`, and then use `downcast_mut` to get a reference on the specific builder.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.StructBuilder.html#method.into_box_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.into_box_any" class="fn">into_box_any</a>(self: <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.StructBuilder.html" class="struct" title="struct arrow::array::StructBuilder">StructBuilder</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a>\>

Returns the boxed builder as a box of `Any`.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.StructBuilder.html#method.is_empty" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether number of array slots is zero

<a href="https://docs.rs/arrow/latest/arrow/array/struct.StructBuilder.html#impl-Debug-for-StructBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.StructBuilder.html" class="struct" title="struct arrow::array::StructBuilder">StructBuilder</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.StructBuilder.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/struct.StructBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/array/struct.StructBuilder.html#blanket-implementations" class="anchor">§</a>
