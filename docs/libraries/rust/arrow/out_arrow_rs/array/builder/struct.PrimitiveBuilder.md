# Struct PrimitiveBuilder Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/builder/primitive_builder.rs.html#100" class="src">Source</a>

``` rust
pub struct PrimitiveBuilder<T>where
    T: ArrowPrimitiveType,{ /* private fields */ }
```

Expand description

Builder for [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray")

## Implementations<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.PrimitiveBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.PrimitiveBuilder.html#impl-PrimitiveBuilder%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveBuilder.html" class="struct" title="struct arrow::array::PrimitiveBuilder">PrimitiveBuilder</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>,

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.PrimitiveBuilder.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveBuilder.html" class="struct" title="struct arrow::array::PrimitiveBuilder">PrimitiveBuilder</a>\<T\>

Creates a new primitive array builder

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.PrimitiveBuilder.html#method.with_capacity" class="fn">with_capacity</a>(capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveBuilder.html" class="struct" title="struct arrow::array::PrimitiveBuilder">PrimitiveBuilder</a>\<T\>

Creates a new primitive array builder with capacity no of items

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.PrimitiveBuilder.html#method.new_from_buffer" class="fn">new_from_buffer</a>( values_buffer: <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html" class="struct" title="struct arrow::buffer::MutableBuffer">MutableBuffer</a>, null_buffer: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html" class="struct" title="struct arrow::buffer::MutableBuffer">MutableBuffer</a>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveBuilder.html" class="struct" title="struct arrow::array::PrimitiveBuilder">PrimitiveBuilder</a>\<T\>

Creates a new primitive array builder from buffers

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.PrimitiveBuilder.html#method.with_data_type" class="fn">with_data_type</a>(self, data_type: <a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveBuilder.html" class="struct" title="struct arrow::array::PrimitiveBuilder">PrimitiveBuilder</a>\<T\>

By default [`PrimitiveBuilder`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveBuilder.html "struct arrow::array::PrimitiveBuilder") uses [`ArrowPrimitiveType::DATA_TYPE`](https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE "associated constant arrow::array::ArrowPrimitiveType::DATA_TYPE") as the data type of the generated array.

This method allows overriding the data type, to allow specifying timezones for [`DataType::Timestamp`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Timestamp "variant arrow::datatypes::DataType::Timestamp") or precision and scale for [`DataType::Decimal32`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Decimal32 "variant arrow::datatypes::DataType::Decimal32"), [`DataType::Decimal64`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Decimal64 "variant arrow::datatypes::DataType::Decimal64"), [`DataType::Decimal128`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Decimal128 "variant arrow::datatypes::DataType::Decimal128") and [`DataType::Decimal256`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Decimal256 "variant arrow::datatypes::DataType::Decimal256")

##### <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.PrimitiveBuilder.html#panics" class="doc-anchor">§</a>Panics

This method panics if `data_type` is not [PrimitiveArray::is_compatible](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html#method.is_compatible "associated function arrow::array::PrimitiveArray::is_compatible")

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.PrimitiveBuilder.html#method.capacity" class="fn">capacity</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the capacity of this builder measured in slots of type `T`

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.PrimitiveBuilder.html#method.append_value" class="fn">append_value</a>(&mut self, v: \<T as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>)

Appends a value of type `T` into the builder

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.PrimitiveBuilder.html#method.append_value_n" class="fn">append_value_n</a>(&mut self, v: \<T as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Appends a value of type `T` into the builder `n` times

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.PrimitiveBuilder.html#method.append_null" class="fn">append_null</a>(&mut self)

Appends a null slot into the builder

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.PrimitiveBuilder.html#method.append_nulls" class="fn">append_nulls</a>(&mut self, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Appends `n` no. of null’s into the builder

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.PrimitiveBuilder.html#method.append_option" class="fn">append_option</a>(&mut self, v: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<T as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>)

Appends an `Option<T>` into the builder

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.PrimitiveBuilder.html#method.append_slice" class="fn">append_slice</a>(&mut self, v: &\[\<T as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\])

Appends a slice of type `T` into the builder

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.PrimitiveBuilder.html#method.append_values" class="fn">append_values</a>( &mut self, values: &\[\<T as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\], is_valid: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\], )

Appends values from a slice of type `T` and a validity boolean slice

##### <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.PrimitiveBuilder.html#panics-1" class="doc-anchor">§</a>Panics

Panics if `values` and `is_valid` have different lengths

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.PrimitiveBuilder.html#method.append_array" class="fn">append_array</a>(&mut self, array: &<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>)

Appends array values and null to this builder as is (this means that underlying null values are copied as is).

##### <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.PrimitiveBuilder.html#panics-2" class="doc-anchor">§</a>Panics

Panics if `array` and `self` data types are different

#### pub unsafe fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.PrimitiveBuilder.html#method.append_trusted_len_iter" class="fn">append_trusted_len_iter</a>( &mut self, iter: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = \<T as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>, )

Appends values from a trusted length iterator.

##### <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.PrimitiveBuilder.html#safety" class="doc-anchor">§</a>Safety

This requires the iterator be a trusted length. This could instead require the iterator implement `TrustedLen` once that is stabilized.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.PrimitiveBuilder.html#method.finish" class="fn">finish</a>(&mut self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>

Builds the [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") and reset this builder.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.PrimitiveBuilder.html#method.finish_cloned" class="fn">finish_cloned</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>

Builds the [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") without resetting the builder.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.PrimitiveBuilder.html#method.values_slice" class="fn">values_slice</a>(&self) -\> &\[\<T as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\]

Returns the current values buffer as a slice

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.PrimitiveBuilder.html#method.values_slice_mut" class="fn">values_slice_mut</a>(&mut self) -\> &mut \[\<T as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\]

Returns the current values buffer as a mutable slice

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.PrimitiveBuilder.html#method.validity_slice" class="fn">validity_slice</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>

Returns the current null buffer as a slice

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.PrimitiveBuilder.html#method.validity_slice_mut" class="fn">validity_slice_mut</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&mut \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>

Returns the current null buffer as a mutable slice

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.PrimitiveBuilder.html#method.slices_mut" class="fn">slices_mut</a>( &mut self, ) -\> (&mut \[\<T as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\], <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&mut \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>)

Returns the current values buffer and null buffer as a slice

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.PrimitiveBuilder.html#impl-PrimitiveBuilder%3CP%3E" class="anchor">§</a>

### impl\<P\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveBuilder.html" class="struct" title="struct arrow::array::PrimitiveBuilder">PrimitiveBuilder</a>\<P\>

where P: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.DecimalType.html" class="trait" title="trait arrow::datatypes::DecimalType">DecimalType</a>,

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.PrimitiveBuilder.html#method.with_precision_and_scale" class="fn">with_precision_and_scale</a>( self, precision: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>, scale: <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveBuilder.html" class="struct" title="struct arrow::array::PrimitiveBuilder">PrimitiveBuilder</a>\<P\>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Sets the precision and scale

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.PrimitiveBuilder.html#impl-PrimitiveBuilder%3CP%3E-1" class="anchor">§</a>

### impl\<P\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveBuilder.html" class="struct" title="struct arrow::array::PrimitiveBuilder">PrimitiveBuilder</a>\<P\>

where P: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTimestampType.html" class="trait" title="trait arrow::datatypes::ArrowTimestampType">ArrowTimestampType</a>,

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.PrimitiveBuilder.html#method.with_timezone" class="fn">with_timezone</a>(self, timezone: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveBuilder.html" class="struct" title="struct arrow::array::PrimitiveBuilder">PrimitiveBuilder</a>\<P\>

Sets the timezone

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.PrimitiveBuilder.html#method.with_timezone_opt" class="fn">with_timezone_opt</a>\<S\>(self, timezone: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<S\>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveBuilder.html" class="struct" title="struct arrow::array::PrimitiveBuilder">PrimitiveBuilder</a>\<P\>

where S: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>,

Sets an optional timezone

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.PrimitiveBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.PrimitiveBuilder.html#impl-ArrayBuilder-for-PrimitiveBuilder%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveBuilder.html" class="struct" title="struct arrow::array::PrimitiveBuilder">PrimitiveBuilder</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.PrimitiveBuilder.html#method.as_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the builder as a non-mutable `Any` reference.

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.PrimitiveBuilder.html#method.as_any_mut" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.as_any_mut" class="fn">as_any_mut</a>(&mut self) -\> &mut (dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the builder as a mutable `Any` reference.

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.PrimitiveBuilder.html#method.into_box_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.into_box_any" class="fn">into_box_any</a>(self: <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveBuilder.html" class="struct" title="struct arrow::array::PrimitiveBuilder">PrimitiveBuilder</a>\<T\>\>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a>\>

Returns the boxed builder as a box of `Any`.

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.PrimitiveBuilder.html#method.len" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the number of array slots in the builder

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.PrimitiveBuilder.html#method.finish-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.finish" class="fn">finish</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

Builds the array and reset this builder.

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.PrimitiveBuilder.html#method.finish_cloned-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.finish_cloned" class="fn">finish_cloned</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

Builds the array without resetting the builder.

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.PrimitiveBuilder.html#method.is_empty" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether number of array slots is zero

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.PrimitiveBuilder.html#impl-Debug-for-PrimitiveBuilder%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveBuilder.html" class="struct" title="struct arrow::array::PrimitiveBuilder">PrimitiveBuilder</a>\<T\>

where T: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> + <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>, \<T as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.PrimitiveBuilder.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.PrimitiveBuilder.html#impl-Default-for-PrimitiveBuilder%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveBuilder.html" class="struct" title="struct arrow::array::PrimitiveBuilder">PrimitiveBuilder</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.PrimitiveBuilder.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveBuilder.html" class="struct" title="struct arrow::array::PrimitiveBuilder">PrimitiveBuilder</a>\<T\>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.PrimitiveBuilder.html#impl-Extend%3COption%3C%3CP+as+ArrowPrimitiveType%3E::Native%3E%3E-for-PrimitiveBuilder%3CP%3E" class="anchor">§</a>

### impl\<P\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html" class="trait" title="trait core::iter::traits::collect::Extend">Extend</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<P as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveBuilder.html" class="struct" title="struct arrow::array::PrimitiveBuilder">PrimitiveBuilder</a>\<P\>

where P: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.PrimitiveBuilder.html#method.extend" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend" class="fn">extend</a>\<T\>(&mut self, iter: T)

where T: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<P as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>,

Extends a collection with the contents of an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend)

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.PrimitiveBuilder.html#method.extend_one" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_one" class="fn">extend_one</a>(&mut self, item: A)

🔬This is a nightly-only experimental API. (`extend_one`)

Extends a collection with exactly one element.

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.PrimitiveBuilder.html#method.extend_reserve" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve" class="fn">extend_reserve</a>(&mut self, additional: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

🔬This is a nightly-only experimental API. (`extend_one`)

Reserves capacity in a collection for the given number of additional elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve)

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.PrimitiveBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.PrimitiveBuilder.html#blanket-implementations" class="anchor">§</a>
