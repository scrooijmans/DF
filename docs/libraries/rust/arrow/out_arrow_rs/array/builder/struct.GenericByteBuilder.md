# Struct GenericByteBuilder Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/builder/generic_bytes_builder.rs.html#30" class="src">Source</a>

``` rust
pub struct GenericByteBuilder<T>where
    T: ByteArrayType,{ /* private fields */ }
```

Expand description

Builder for [`GenericByteArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteArray.html "struct arrow::array::GenericByteArray")

For building strings, see docs on [`GenericStringBuilder`](https://docs.rs/arrow/latest/arrow/array/type.GenericStringBuilder.html "type arrow::array::GenericStringBuilder"). For building binary, see docs on [`GenericBinaryBuilder`](https://docs.rs/arrow/latest/arrow/array/type.GenericBinaryBuilder.html "type arrow::array::GenericBinaryBuilder").

## Implementations<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteBuilder.html#impl-GenericByteBuilder%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteBuilder.html" class="struct" title="struct arrow::array::GenericByteBuilder">GenericByteBuilder</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait arrow::datatypes::ByteArrayType">ByteArrayType</a>,

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteBuilder.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteBuilder.html" class="struct" title="struct arrow::array::GenericByteBuilder">GenericByteBuilder</a>\<T\>

Creates a new [`GenericByteBuilder`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteBuilder.html "struct arrow::array::GenericByteBuilder").

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteBuilder.html#method.with_capacity" class="fn">with_capacity</a>( item_capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, data_capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteBuilder.html" class="struct" title="struct arrow::array::GenericByteBuilder">GenericByteBuilder</a>\<T\>

Creates a new [`GenericByteBuilder`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteBuilder.html "struct arrow::array::GenericByteBuilder").

- `item_capacity` is the number of items to pre-allocate. The size of the preallocated buffer of offsets is the number of items plus one.
- `data_capacity` is the total number of bytes of data to pre-allocate (for all items, not per item).

#### pub unsafe fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteBuilder.html#method.new_from_buffer" class="fn">new_from_buffer</a>( offsets_buffer: <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html" class="struct" title="struct arrow::buffer::MutableBuffer">MutableBuffer</a>, value_buffer: <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html" class="struct" title="struct arrow::buffer::MutableBuffer">MutableBuffer</a>, null_buffer: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html" class="struct" title="struct arrow::buffer::MutableBuffer">MutableBuffer</a>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteBuilder.html" class="struct" title="struct arrow::array::GenericByteBuilder">GenericByteBuilder</a>\<T\>

Creates a new [`GenericByteBuilder`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteBuilder.html "struct arrow::array::GenericByteBuilder") from buffers.

##### <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteBuilder.html#safety" class="doc-anchor">§</a>Safety

This doesn’t verify buffer contents as it assumes the buffers are from existing and valid [`GenericByteArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteArray.html "struct arrow::array::GenericByteArray").

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteBuilder.html#method.append_value" class="fn">append_value</a>(&mut self, value: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\<T as <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait arrow::datatypes::ByteArrayType">ByteArrayType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteArrayType.html#associatedtype.Native" class="associatedtype" title="type arrow::datatypes::ByteArrayType::Native">Native</a>\>)

Appends a value into the builder.

See the [GenericStringBuilder](https://docs.rs/arrow/latest/arrow/array/type.GenericStringBuilder.html "type arrow::array::GenericStringBuilder") documentation for examples of incrementally building string values with multiple `write!` calls.

##### <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteBuilder.html#panics" class="doc-anchor">§</a>Panics

Panics if the resulting length of [`Self::values_slice`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteBuilder.html#method.values_slice "method arrow::array::GenericByteBuilder::values_slice") would exceed `T::Offset::MAX` bytes.

For example, this can happen with [`StringArray`](https://docs.rs/arrow/latest/arrow/array/type.StringArray.html "type arrow::array::StringArray") or [`BinaryArray`](https://docs.rs/arrow/latest/arrow/array/type.BinaryArray.html "type arrow::array::BinaryArray") where the total length of all values exceeds 2GB

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteBuilder.html#method.append_option" class="fn">append_option</a>( &mut self, value: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\<T as <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait arrow::datatypes::ByteArrayType">ByteArrayType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteArrayType.html#associatedtype.Native" class="associatedtype" title="type arrow::datatypes::ByteArrayType::Native">Native</a>\>\>, )

Append an `Option` value into the builder.

- A `None` value will append a null value.
- A `Some` value will append the value.

See [`Self::append_value`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteBuilder.html#method.append_value "method arrow::array::GenericByteBuilder::append_value") for more panic information.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteBuilder.html#method.append_null" class="fn">append_null</a>(&mut self)

Append a null value into the builder.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteBuilder.html#method.append_nulls" class="fn">append_nulls</a>(&mut self, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Appends `n` `null`s into the builder.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteBuilder.html#method.append_array" class="fn">append_array</a>(&mut self, array: &<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteArray.html" class="struct" title="struct arrow::array::GenericByteArray">GenericByteArray</a>\<T\>)

Appends array values and null to this builder as is (this means that underlying null values are copied as is).

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteBuilder.html#method.finish" class="fn">finish</a>(&mut self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteArray.html" class="struct" title="struct arrow::array::GenericByteArray">GenericByteArray</a>\<T\>

Builds the [`GenericByteArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteArray.html "struct arrow::array::GenericByteArray") and reset this builder.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteBuilder.html#method.finish_cloned" class="fn">finish_cloned</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteArray.html" class="struct" title="struct arrow::array::GenericByteArray">GenericByteArray</a>\<T\>

Builds the [`GenericByteArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteArray.html "struct arrow::array::GenericByteArray") without resetting the builder.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteBuilder.html#method.values_slice" class="fn">values_slice</a>(&self) -\> &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\] <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteBuilder.html#" class="tooltip" data-notable-ty="&amp;[u8]">ⓘ</a>

Returns the current values buffer as a slice

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteBuilder.html#method.offsets_slice" class="fn">offsets_slice</a>(&self) -\> &\[\<T as <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait arrow::datatypes::ByteArrayType">ByteArrayType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteArrayType.html#associatedtype.Offset" class="associatedtype" title="type arrow::datatypes::ByteArrayType::Offset">Offset</a>\]

Returns the current offsets buffer as a slice

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteBuilder.html#method.validity_slice" class="fn">validity_slice</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>

Returns the current null buffer as a slice

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteBuilder.html#method.validity_slice_mut" class="fn">validity_slice_mut</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&mut \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>

Returns the current null buffer as a mutable slice

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteBuilder.html#impl-ArrayBuilder-for-GenericByteBuilder%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteBuilder.html" class="struct" title="struct arrow::array::GenericByteBuilder">GenericByteBuilder</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait arrow::datatypes::ByteArrayType">ByteArrayType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteBuilder.html#method.len" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the number of binary slots in the builder

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteBuilder.html#method.finish-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.finish" class="fn">finish</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

Builds the array and reset this builder.

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteBuilder.html#method.finish_cloned-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.finish_cloned" class="fn">finish_cloned</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

Builds the array without resetting the builder.

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteBuilder.html#method.as_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the builder as a non-mutable `Any` reference.

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteBuilder.html#method.as_any_mut" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.as_any_mut" class="fn">as_any_mut</a>(&mut self) -\> &mut (dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the builder as a mutable `Any` reference.

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteBuilder.html#method.into_box_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.into_box_any" class="fn">into_box_any</a>(self: <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteBuilder.html" class="struct" title="struct arrow::array::GenericByteBuilder">GenericByteBuilder</a>\<T\>\>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a>\>

Returns the boxed builder as a box of `Any`.

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteBuilder.html#method.is_empty" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether number of array slots is zero

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteBuilder.html#impl-Debug-for-GenericByteBuilder%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteBuilder.html" class="struct" title="struct arrow::array::GenericByteBuilder">GenericByteBuilder</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait arrow::datatypes::ByteArrayType">ByteArrayType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteBuilder.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteBuilder.html#impl-Default-for-GenericByteBuilder%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteBuilder.html" class="struct" title="struct arrow::array::GenericByteBuilder">GenericByteBuilder</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait arrow::datatypes::ByteArrayType">ByteArrayType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteBuilder.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteBuilder.html" class="struct" title="struct arrow::array::GenericByteBuilder">GenericByteBuilder</a>\<T\>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteBuilder.html#impl-Extend%3COption%3CV%3E%3E-for-GenericByteBuilder%3CT%3E" class="anchor">§</a>

### impl\<T, V\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html" class="trait" title="trait core::iter::traits::collect::Extend">Extend</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<V\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteBuilder.html" class="struct" title="struct arrow::array::GenericByteBuilder">GenericByteBuilder</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait arrow::datatypes::ByteArrayType">ByteArrayType</a>, V: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\<T as <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait arrow::datatypes::ByteArrayType">ByteArrayType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteArrayType.html#associatedtype.Native" class="associatedtype" title="type arrow::datatypes::ByteArrayType::Native">Native</a>\>,

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteBuilder.html#method.extend" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend" class="fn">extend</a>\<I\>(&mut self, iter: I)

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<V\>\>,

Extends a collection with the contents of an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend)

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteBuilder.html#method.extend_one" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_one" class="fn">extend_one</a>(&mut self, item: A)

🔬This is a nightly-only experimental API. (`extend_one`)

Extends a collection with exactly one element.

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteBuilder.html#method.extend_reserve" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve" class="fn">extend_reserve</a>(&mut self, additional: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

🔬This is a nightly-only experimental API. (`extend_one`)

Reserves capacity in a collection for the given number of additional elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve)

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteBuilder.html#impl-Write-for-GenericByteBuilder%3CGenericBinaryType%3CO%3E%3E" class="anchor">§</a>

### impl\<O\> <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html" class="trait" title="trait std::io::Write">Write</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteBuilder.html" class="struct" title="struct arrow::array::GenericByteBuilder">GenericByteBuilder</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.GenericBinaryType.html" class="struct" title="struct arrow::datatypes::GenericBinaryType">GenericBinaryType</a>\<O\>\>

where O: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteBuilder.html#method.write" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html#tymethod.write" class="fn">write</a>(&mut self, bs: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

Writes a buffer into this writer, returning how many bytes were written. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#tymethod.write)

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteBuilder.html#method.flush" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html#tymethod.flush" class="fn">flush</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

Flushes this output stream, ensuring that all intermediately buffered contents reach their destination. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#tymethod.flush)

1.36.0 · <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1758" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteBuilder.html#method.write_vectored" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_vectored" class="fn">write_vectored</a>(&mut self, bufs: &\[<a href="https://doc.rust-lang.org/nightly/std/io/struct.IoSlice.html" class="struct" title="struct std::io::IoSlice">IoSlice</a>\<'\_\>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

Like [`write`](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#tymethod.write "method std::io::Write::write"), except that it writes from a slice of buffers. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_vectored)

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteBuilder.html#method.is_write_vectored" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.is_write_vectored" class="fn">is_write_vectored</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

🔬This is a nightly-only experimental API. (`can_vector`)

Determines if this `Write`r has an efficient [`write_vectored`](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_vectored "method std::io::Write::write_vectored") implementation. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.is_write_vectored)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1835" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteBuilder.html#method.write_all" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_all" class="fn">write_all</a>(&mut self, buf: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

Attempts to write an entire buffer into this writer. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_all)

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteBuilder.html#method.write_all_vectored" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_all_vectored" class="fn">write_all_vectored</a>(&mut self, bufs: &mut \[<a href="https://doc.rust-lang.org/nightly/std/io/struct.IoSlice.html" class="struct" title="struct std::io::IoSlice">IoSlice</a>\<'\_\>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

🔬This is a nightly-only experimental API. (`write_all_vectored`)

Attempts to write multiple buffers into this writer. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_all_vectored)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1950" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteBuilder.html#method.write_fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_fmt" class="fn">write_fmt</a>(&mut self, args: <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Arguments.html" class="struct" title="struct core::fmt::Arguments">Arguments</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

Writes a formatted string into this writer, returning any error encountered. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_fmt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1980-1982" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteBuilder.html#method.by_ref" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.by_ref" class="fn">by_ref</a>(&mut self) -\> &mut Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Creates a “by reference” adapter for this instance of `Write`. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.by_ref)

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteBuilder.html#impl-Write-for-GenericByteBuilder%3CGenericStringType%3CO%3E%3E" class="anchor">§</a>

### impl\<O\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Write.html" class="trait" title="trait core::fmt::Write">Write</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteBuilder.html" class="struct" title="struct arrow::array::GenericByteBuilder">GenericByteBuilder</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.GenericStringType.html" class="struct" title="struct arrow::datatypes::GenericStringType">GenericStringType</a>\<O\>\>

where O: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteBuilder.html#method.write_str" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Write.html#tymethod.write_str" class="fn">write_str</a>(&mut self, s: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Writes a string slice into this writer, returning whether the write succeeded. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Write.html#tymethod.write_str)

1.1.0 · <a href="https://doc.rust-lang.org/nightly/src/core/fmt/mod.rs.html#180" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteBuilder.html#method.write_char" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Write.html#method.write_char" class="fn">write_char</a>(&mut self, c: <a href="https://doc.rust-lang.org/nightly/std/primitive.char.html" class="primitive">char</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Writes a [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char") into this writer, returning whether the write succeeded. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Write.html#method.write_char)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/fmt/mod.rs.html#209" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteBuilder.html#method.write_fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Write.html#method.write_fmt" class="fn">write_fmt</a>(&mut self, args: <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Arguments.html" class="struct" title="struct core::fmt::Arguments">Arguments</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Glue for usage of the [`write!`](https://doc.rust-lang.org/nightly/core/macro.write.html "macro core::write") macro with implementors of this trait. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Write.html#method.write_fmt)

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteBuilder.html#blanket-implementations" class="anchor">§</a>
