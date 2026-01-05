# Struct FixedSizeBinaryDictionaryBuilder Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/builder/fixed_size_binary_dictionary_builder.rs.html#61" class="src">Source</a>

``` rust
pub struct FixedSizeBinaryDictionaryBuilder<K>where
    K: ArrowDictionaryKeyType,{ /* private fields */ }
```

Expand description

Builder for [`DictionaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html "struct arrow::array::DictionaryArray") of [`FixedSizeBinaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html "struct arrow::array::FixedSizeBinaryArray")

The output array has a dictionary of unique, fixed-size binary values. The builder handles deduplication.

## <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.FixedSizeBinaryDictionaryBuilder.html#example" class="doc-anchor">§</a>Example

``` rust
// Build 3 byte FixedBinaryArrays
let byte_width = 3;
let mut builder = FixedSizeBinaryDictionaryBuilder::<Int8Type>::new(3);
builder.append("abc").unwrap();
builder.append_null();
builder.append(b"def").unwrap();
builder.append(b"def").unwrap(); // duplicate value
// Result is a Dictionary Array
let array = builder.finish();
let dict_array = array.as_any().downcast_ref::<DictionaryArray<Int8Type>>().unwrap();
// The array represents "abc", null, "def", "def"
assert_eq!(array.keys().len(), 4);
// but there are only 2 unique values
assert_eq!(array.values().len(), 2);
let values = dict_array.values().as_any().downcast_ref::<FixedSizeBinaryArray>().unwrap();
assert_eq!(values.value(0), "abc".as_bytes());
assert_eq!(values.value(1), "def".as_bytes());
```

## Implementations<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.FixedSizeBinaryDictionaryBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.FixedSizeBinaryDictionaryBuilder.html#impl-FixedSizeBinaryDictionaryBuilder%3CK%3E" class="anchor">§</a>

### impl\<K\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryDictionaryBuilder.html" class="struct" title="struct arrow::array::FixedSizeBinaryDictionaryBuilder">FixedSizeBinaryDictionaryBuilder</a>\<K\>

where K: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowDictionaryKeyType.html" class="trait" title="trait arrow::datatypes::ArrowDictionaryKeyType">ArrowDictionaryKeyType</a>,

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.FixedSizeBinaryDictionaryBuilder.html#method.new" class="fn">new</a>(byte_width: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryDictionaryBuilder.html" class="struct" title="struct arrow::array::FixedSizeBinaryDictionaryBuilder">FixedSizeBinaryDictionaryBuilder</a>\<K\>

Creates a new `FixedSizeBinaryDictionaryBuilder`

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.FixedSizeBinaryDictionaryBuilder.html#method.with_capacity" class="fn">with_capacity</a>( keys_capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, value_capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, byte_width: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryDictionaryBuilder.html" class="struct" title="struct arrow::array::FixedSizeBinaryDictionaryBuilder">FixedSizeBinaryDictionaryBuilder</a>\<K\>

Creates a new `FixedSizeBinaryDictionaryBuilder` with the provided capacities

`keys_capacity`: the number of keys, i.e. length of array to build `value_capacity`: the number of distinct dictionary values, i.e. size of dictionary `byte_width`: the byte width for individual values in the values array

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.FixedSizeBinaryDictionaryBuilder.html#method.try_new_from_builder" class="fn">try_new_from_builder</a>\<K2\>( source: <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryDictionaryBuilder.html" class="struct" title="struct arrow::array::FixedSizeBinaryDictionaryBuilder">FixedSizeBinaryDictionaryBuilder</a>\<K2\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryDictionaryBuilder.html" class="struct" title="struct arrow::array::FixedSizeBinaryDictionaryBuilder">FixedSizeBinaryDictionaryBuilder</a>\<K\>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

where \<K as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>: <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/cast/trait.NumCast.html" class="trait" title="trait num_traits::cast::NumCast">NumCast</a>, K2: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowDictionaryKeyType.html" class="trait" title="trait arrow::datatypes::ArrowDictionaryKeyType">ArrowDictionaryKeyType</a>, \<K2 as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>: <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/cast/trait.NumCast.html" class="trait" title="trait num_traits::cast::NumCast">NumCast</a>,

Creates a new `FixedSizeBinaryDictionaryBuilder` from the existing builder with the same keys and values, but with a new data type for the keys.

##### <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.FixedSizeBinaryDictionaryBuilder.html#example-1" class="doc-anchor">§</a>Example

``` rust

let mut u8_keyed_builder = FixedSizeBinaryDictionaryBuilder::<UInt8Type>::new(2);
// appending too many values causes the dictionary to overflow
for i in 0..=255 {
    u8_keyed_builder.append_value(vec![0, i]);
}
let result = u8_keyed_builder.append(vec![1, 0]);
assert!(matches!(result, Err(ArrowError::DictionaryKeyOverflowError{})));

// we need to upgrade to a larger key type
let mut u16_keyed_builder = FixedSizeBinaryDictionaryBuilder::<UInt16Type>::try_new_from_builder(u8_keyed_builder).unwrap();
let dictionary_array = u16_keyed_builder.finish();
let keys = dictionary_array.keys();

assert_eq!(keys, &UInt16Array::from_iter(0..256));
```

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.FixedSizeBinaryDictionaryBuilder.html#impl-FixedSizeBinaryDictionaryBuilder%3CK%3E-1" class="anchor">§</a>

### impl\<K\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryDictionaryBuilder.html" class="struct" title="struct arrow::array::FixedSizeBinaryDictionaryBuilder">FixedSizeBinaryDictionaryBuilder</a>\<K\>

where K: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowDictionaryKeyType.html" class="trait" title="trait arrow::datatypes::ArrowDictionaryKeyType">ArrowDictionaryKeyType</a>,

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.FixedSizeBinaryDictionaryBuilder.html#method.append" class="fn">append</a>( &mut self, value: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\<K as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Append a value to the array. Return an existing index if already present in the values array or a new index if the value is appended to the values array.

Returns an error if the new index would overflow the key type.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.FixedSizeBinaryDictionaryBuilder.html#method.append_null" class="fn">append_null</a>(&mut self)

Appends a null slot into the builder

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.FixedSizeBinaryDictionaryBuilder.html#method.append_nulls" class="fn">append_nulls</a>(&mut self, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Appends `n` `null`s into the builder.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.FixedSizeBinaryDictionaryBuilder.html#method.append_value" class="fn">append_value</a>(&mut self, value: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>)

Infallibly append a value to this builder

##### <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.FixedSizeBinaryDictionaryBuilder.html#panics" class="doc-anchor">§</a>Panics

Panics if the resulting length of the dictionary values array would exceed `T::Native::MAX`

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.FixedSizeBinaryDictionaryBuilder.html#method.finish" class="fn">finish</a>(&mut self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html" class="struct" title="struct arrow::array::DictionaryArray">DictionaryArray</a>\<K\>

Builds the `DictionaryArray` and reset this builder.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.FixedSizeBinaryDictionaryBuilder.html#method.finish_cloned" class="fn">finish_cloned</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html" class="struct" title="struct arrow::array::DictionaryArray">DictionaryArray</a>\<K\>

Builds the `DictionaryArray` without resetting the builder.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.FixedSizeBinaryDictionaryBuilder.html#method.finish_preserve_values" class="fn">finish_preserve_values</a>(&mut self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html" class="struct" title="struct arrow::array::DictionaryArray">DictionaryArray</a>\<K\>

Builds the `DictionaryArray` without resetting the values builder or the internal de-duplication map.

The advantage of doing this is that the values will represent the entire set of what has been built so-far by this builder and ensures consistency in the assignment of keys to values across multiple calls to `finish_preserve_values`. This enables ipc writers to efficiently emit delta dictionaries.

The downside to this is that building the record requires creating a copy of the values, which can become slowly more expensive if the dictionary grows.

Additionally, if record batches from multiple different dictionary builders for the same column are fed into a single ipc writer, beware that entire dictionaries are likely to be re-sent frequently even when the majority of the values are not used by the current record batch.

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.FixedSizeBinaryDictionaryBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.FixedSizeBinaryDictionaryBuilder.html#impl-ArrayBuilder-for-FixedSizeBinaryDictionaryBuilder%3CK%3E" class="anchor">§</a>

### impl\<K\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryDictionaryBuilder.html" class="struct" title="struct arrow::array::FixedSizeBinaryDictionaryBuilder">FixedSizeBinaryDictionaryBuilder</a>\<K\>

where K: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowDictionaryKeyType.html" class="trait" title="trait arrow::datatypes::ArrowDictionaryKeyType">ArrowDictionaryKeyType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.FixedSizeBinaryDictionaryBuilder.html#method.as_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the builder as an non-mutable `Any` reference.

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.FixedSizeBinaryDictionaryBuilder.html#method.as_any_mut" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.as_any_mut" class="fn">as_any_mut</a>(&mut self) -\> &mut (dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the builder as an mutable `Any` reference.

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.FixedSizeBinaryDictionaryBuilder.html#method.into_box_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.into_box_any" class="fn">into_box_any</a>(self: <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryDictionaryBuilder.html" class="struct" title="struct arrow::array::FixedSizeBinaryDictionaryBuilder">FixedSizeBinaryDictionaryBuilder</a>\<K\>\>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a>\>

Returns the boxed builder as a box of `Any`.

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.FixedSizeBinaryDictionaryBuilder.html#method.len" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the number of array slots in the builder

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.FixedSizeBinaryDictionaryBuilder.html#method.finish-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.finish" class="fn">finish</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

Builds the array and reset this builder.

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.FixedSizeBinaryDictionaryBuilder.html#method.finish_cloned-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.finish_cloned" class="fn">finish_cloned</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

Builds the array without resetting the builder.

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.FixedSizeBinaryDictionaryBuilder.html#method.is_empty" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether number of array slots is zero

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.FixedSizeBinaryDictionaryBuilder.html#impl-Debug-for-FixedSizeBinaryDictionaryBuilder%3CK%3E" class="anchor">§</a>

### impl\<K\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryDictionaryBuilder.html" class="struct" title="struct arrow::array::FixedSizeBinaryDictionaryBuilder">FixedSizeBinaryDictionaryBuilder</a>\<K\>

where K: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> + <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowDictionaryKeyType.html" class="trait" title="trait arrow::datatypes::ArrowDictionaryKeyType">ArrowDictionaryKeyType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.FixedSizeBinaryDictionaryBuilder.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.FixedSizeBinaryDictionaryBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.FixedSizeBinaryDictionaryBuilder.html#blanket-implementations" class="anchor">§</a>
