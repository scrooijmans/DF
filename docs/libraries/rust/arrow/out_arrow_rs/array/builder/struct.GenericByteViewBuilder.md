# Struct GenericByteViewBuilder Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/builder/generic_bytes_view_builder.rs.html#81" class="src">Source</a>

``` rust
pub struct GenericByteViewBuilder<T>where
    T: ByteViewType + ?Sized,{ /* private fields */ }
```

Expand description

A builder for [`GenericByteViewArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewArray.html "struct arrow::array::GenericByteViewArray")

A [`GenericByteViewArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewArray.html "struct arrow::array::GenericByteViewArray") consists of a list of data blocks containing string data, and a list of views into those buffers.

See examples on [`StringViewBuilder`](https://docs.rs/arrow/latest/arrow/array/type.StringViewBuilder.html "type arrow::array::StringViewBuilder") and [`BinaryViewBuilder`](https://docs.rs/arrow/latest/arrow/array/type.BinaryViewBuilder.html "type arrow::array::BinaryViewBuilder")

This builder can be used in two ways

## <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteViewBuilder.html#append-values" class="doc-anchor">§</a>Append Values

To avoid bump allocating, this builder allocates data in fixed size blocks, configurable using [`GenericByteViewBuilder::with_fixed_block_size`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewBuilder.html#method.with_fixed_block_size "method arrow::array::GenericByteViewBuilder::with_fixed_block_size"). [`GenericByteViewBuilder::append_value`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewBuilder.html#method.append_value "method arrow::array::GenericByteViewBuilder::append_value") writes values larger than [`MAX_INLINE_VIEW_LEN`](https://docs.rs/arrow-data/56.2.0/x86_64-unknown-linux-gnu/arrow_data/byte_view/constant.MAX_INLINE_VIEW_LEN.html "constant arrow_data::byte_view::MAX_INLINE_VIEW_LEN") bytes to the current in-progress block, with values smaller than [`MAX_INLINE_VIEW_LEN`](https://docs.rs/arrow-data/56.2.0/x86_64-unknown-linux-gnu/arrow_data/byte_view/constant.MAX_INLINE_VIEW_LEN.html "constant arrow_data::byte_view::MAX_INLINE_VIEW_LEN") bytes inlined into the views. If a value is appended that will not fit in the in-progress block, it will be closed, and a new block of sufficient size allocated

## <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteViewBuilder.html#append-views" class="doc-anchor">§</a>Append Views

Some use-cases may wish to reuse an existing allocation containing string data, for example, when parsing data from a parquet data page. In such a case entire blocks can be appended using [`GenericByteViewBuilder::append_block`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewBuilder.html#method.append_block "method arrow::array::GenericByteViewBuilder::append_block") and then views into this block appended using [`GenericByteViewBuilder::try_append_view`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewBuilder.html#method.try_append_view "method arrow::array::GenericByteViewBuilder::try_append_view")

## Implementations<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteViewBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteViewBuilder.html#impl-GenericByteViewBuilder%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewBuilder.html" class="struct" title="struct arrow::array::GenericByteViewBuilder">GenericByteViewBuilder</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html" class="trait" title="trait arrow::datatypes::ByteViewType">ByteViewType</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteViewBuilder.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewBuilder.html" class="struct" title="struct arrow::array::GenericByteViewBuilder">GenericByteViewBuilder</a>\<T\>

Creates a new [`GenericByteViewBuilder`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewBuilder.html "struct arrow::array::GenericByteViewBuilder").

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteViewBuilder.html#method.with_capacity" class="fn">with_capacity</a>(capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewBuilder.html" class="struct" title="struct arrow::array::GenericByteViewBuilder">GenericByteViewBuilder</a>\<T\>

Creates a new [`GenericByteViewBuilder`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewBuilder.html "struct arrow::array::GenericByteViewBuilder") with space for `capacity` string values.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteViewBuilder.html#method.with_fixed_block_size" class="fn">with_fixed_block_size</a>(self, block_size: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewBuilder.html" class="struct" title="struct arrow::array::GenericByteViewBuilder">GenericByteViewBuilder</a>\<T\>

Set a fixed buffer size for variable length strings

The block size is the size of the buffer used to store values greater than [`MAX_INLINE_VIEW_LEN`](https://docs.rs/arrow-data/56.2.0/x86_64-unknown-linux-gnu/arrow_data/byte_view/constant.MAX_INLINE_VIEW_LEN.html "constant arrow_data::byte_view::MAX_INLINE_VIEW_LEN") bytes. The builder allocates new buffers when the current buffer is full.

By default the builder balances buffer size and buffer count by growing buffer size exponentially from 8KB up to 2MB. The first buffer allocated is 8KB, then 16KB, then 32KB, etc up to 2MB.

If this method is used, any new buffers allocated are  
exactly this size. This can be useful for advanced users that want to control the memory usage and buffer count.

See <https://github.com/apache/arrow-rs/issues/6094> for more details on the implications.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteViewBuilder.html#method.with_deduplicate_strings" class="fn">with_deduplicate_strings</a>(self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewBuilder.html" class="struct" title="struct arrow::array::GenericByteViewBuilder">GenericByteViewBuilder</a>\<T\>

Deduplicate strings while building the array

This will potentially decrease the memory usage if the array have repeated strings It will also increase the time to build the array as it needs to hash the strings

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteViewBuilder.html#method.append_block" class="fn">append_block</a>(&mut self, buffer: <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

Append a new data block returning the new block offset

Note: this will first flush any in-progress block

This allows appending views from blocks added using [`Self::append_block`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewBuilder.html#method.append_block "method arrow::array::GenericByteViewBuilder::append_block"). See [`Self::append_value`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewBuilder.html#method.append_value "method arrow::array::GenericByteViewBuilder::append_value") for appending individual values

``` rust
let mut builder = StringViewBuilder::new();

let block = builder.append_block(b"helloworldbingobongo".into());

builder.try_append_view(block, 0, 5).unwrap();
builder.try_append_view(block, 5, 5).unwrap();
builder.try_append_view(block, 10, 5).unwrap();
builder.try_append_view(block, 15, 5).unwrap();
builder.try_append_view(block, 0, 15).unwrap();
let array = builder.finish();

let actual: Vec<_> = array.iter().flatten().collect();
let expected = &["hello", "world", "bingo", "bongo", "helloworldbingo"];
assert_eq!(actual, expected);
```

#### pub unsafe fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteViewBuilder.html#method.append_view_unchecked" class="fn">append_view_unchecked</a>( &mut self, block: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, len: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, )

Append a view of the given `block`, `offset` and `length`

##### <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteViewBuilder.html#safety" class="doc-anchor">§</a>Safety

\(1\) The block must have been added using [`Self::append_block`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewBuilder.html#method.append_block "method arrow::array::GenericByteViewBuilder::append_block") (2) The range `offset..offset+length` must be within the bounds of the block (3) The data in the block must be valid of type `T`

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteViewBuilder.html#method.append_array" class="fn">append_array</a>(&mut self, array: &<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<T\>)

Appends an array to the builder. This will flush any in-progress block and append the data buffers and add the (adapted) views.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteViewBuilder.html#method.try_append_view" class="fn">try_append_view</a>( &mut self, block: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, len: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Try to append a view of the given `block`, `offset` and `length`

See [`Self::append_block`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewBuilder.html#method.append_block "method arrow::array::GenericByteViewBuilder::append_block")

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteViewBuilder.html#method.get_value" class="fn">get_value</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\] <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteViewBuilder.html#" class="tooltip" data-notable-ty="&amp;[u8]">ⓘ</a>

Returns the value at the given index Useful if we want to know what value has been inserted to the builder The index has to be smaller than `self.len()`, otherwise it will panic

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteViewBuilder.html#method.append_value" class="fn">append_value</a>(&mut self, value: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\<T as <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html" class="trait" title="trait arrow::datatypes::ByteViewType">ByteViewType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html#associatedtype.Native" class="associatedtype" title="type arrow::datatypes::ByteViewType::Native">Native</a>\>)

Appends a value into the builder

##### <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteViewBuilder.html#panics" class="doc-anchor">§</a>Panics

Panics if

- String buffer count exceeds `u32::MAX`
- String length exceeds `u32::MAX`

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteViewBuilder.html#method.append_option" class="fn">append_option</a>( &mut self, value: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\<T as <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html" class="trait" title="trait arrow::datatypes::ByteViewType">ByteViewType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html#associatedtype.Native" class="associatedtype" title="type arrow::datatypes::ByteViewType::Native">Native</a>\>\>, )

Append an `Option` value into the builder

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteViewBuilder.html#method.append_null" class="fn">append_null</a>(&mut self)

Append a null value into the builder

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteViewBuilder.html#method.finish" class="fn">finish</a>(&mut self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<T\>

Builds the [`GenericByteViewArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewArray.html "struct arrow::array::GenericByteViewArray") and reset this builder

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteViewBuilder.html#method.finish_cloned" class="fn">finish_cloned</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<T\>

Builds the [`GenericByteViewArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewArray.html "struct arrow::array::GenericByteViewArray") without resetting the builder

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteViewBuilder.html#method.validity_slice" class="fn">validity_slice</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>

Returns the current null buffer as a slice

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteViewBuilder.html#method.allocated_size" class="fn">allocated_size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Return the allocated size of this builder in bytes, useful for memory accounting.

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteViewBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteViewBuilder.html#impl-ArrayBuilder-for-GenericByteViewBuilder%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewBuilder.html" class="struct" title="struct arrow::array::GenericByteViewBuilder">GenericByteViewBuilder</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html" class="trait" title="trait arrow::datatypes::ByteViewType">ByteViewType</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteViewBuilder.html#method.len" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the number of array slots in the builder

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteViewBuilder.html#method.finish-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.finish" class="fn">finish</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

Builds the array

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteViewBuilder.html#method.finish_cloned-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.finish_cloned" class="fn">finish_cloned</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

Builds the array without resetting the underlying builder.

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteViewBuilder.html#method.as_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the builder as a non-mutable `Any` reference. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.as_any)

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteViewBuilder.html#method.as_any_mut" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.as_any_mut" class="fn">as_any_mut</a>(&mut self) -\> &mut (dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the builder as a mutable `Any` reference. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.as_any_mut)

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteViewBuilder.html#method.into_box_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.into_box_any" class="fn">into_box_any</a>(self: <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewBuilder.html" class="struct" title="struct arrow::array::GenericByteViewBuilder">GenericByteViewBuilder</a>\<T\>\>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a>\>

Returns the boxed builder as a box of `Any`.

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteViewBuilder.html#method.is_empty" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether number of array slots is zero

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteViewBuilder.html#impl-Debug-for-GenericByteViewBuilder%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewBuilder.html" class="struct" title="struct arrow::array::GenericByteViewBuilder">GenericByteViewBuilder</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html" class="trait" title="trait arrow::datatypes::ByteViewType">ByteViewType</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteViewBuilder.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteViewBuilder.html#impl-Default-for-GenericByteViewBuilder%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewBuilder.html" class="struct" title="struct arrow::array::GenericByteViewBuilder">GenericByteViewBuilder</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html" class="trait" title="trait arrow::datatypes::ByteViewType">ByteViewType</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteViewBuilder.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewBuilder.html" class="struct" title="struct arrow::array::GenericByteViewBuilder">GenericByteViewBuilder</a>\<T\>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteViewBuilder.html#impl-Extend%3COption%3CV%3E%3E-for-GenericByteViewBuilder%3CT%3E" class="anchor">§</a>

### impl\<T, V\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html" class="trait" title="trait core::iter::traits::collect::Extend">Extend</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<V\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewBuilder.html" class="struct" title="struct arrow::array::GenericByteViewBuilder">GenericByteViewBuilder</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html" class="trait" title="trait arrow::datatypes::ByteViewType">ByteViewType</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, V: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\<T as <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html" class="trait" title="trait arrow::datatypes::ByteViewType">ByteViewType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html#associatedtype.Native" class="associatedtype" title="type arrow::datatypes::ByteViewType::Native">Native</a>\>,

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteViewBuilder.html#method.extend" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend" class="fn">extend</a>\<I\>(&mut self, iter: I)

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<V\>\>,

Extends a collection with the contents of an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend)

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteViewBuilder.html#method.extend_one" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_one" class="fn">extend_one</a>(&mut self, item: A)

🔬This is a nightly-only experimental API. (`extend_one`)

Extends a collection with exactly one element.

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteViewBuilder.html#method.extend_reserve" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve" class="fn">extend_reserve</a>(&mut self, additional: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

🔬This is a nightly-only experimental API. (`extend_one`)

Reserves capacity in a collection for the given number of additional elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve)

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteViewBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteViewBuilder.html#blanket-implementations" class="anchor">§</a>
