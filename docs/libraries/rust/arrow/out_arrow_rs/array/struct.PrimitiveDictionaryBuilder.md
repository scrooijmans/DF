# Struct PrimitiveDictionaryBuilder Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/builder/primitive_dictionary_builder.rs.html#83" class="src">Source</a>

``` rust
pub struct PrimitiveDictionaryBuilder<K, V>where
    K: ArrowPrimitiveType,
    V: ArrowPrimitiveType,{ /* private fields */ }
```

Expand description

Builder for [`DictionaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html "struct arrow::array::DictionaryArray") of [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray")

## <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html#example" class="doc-anchor">§</a>Example:

``` rust


let mut builder = PrimitiveDictionaryBuilder::<UInt8Type, UInt32Type>::new();
 builder.append(12345678).unwrap();
 builder.append_null();
 builder.append(22345678).unwrap();
 let array = builder.finish();

 assert_eq!(
     array.keys(),
     &UInt8Array::from(vec![Some(0), None, Some(1)])
 );

 // Values are polymorphic and so require a downcast.
 let av = array.values();
 let ava: &UInt32Array = av.as_any().downcast_ref::<UInt32Array>().unwrap();
 let avs: &[u32] = ava.values();

 assert!(!array.is_null(0));
 assert!(array.is_null(1));
 assert!(!array.is_null(2));

 assert_eq!(avs, &[12345678, 22345678]);
```

## Implementations<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html#impl-PrimitiveDictionaryBuilder%3CK,+V%3E" class="anchor">§</a>

### impl\<K, V\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html" class="struct" title="struct arrow::array::PrimitiveDictionaryBuilder">PrimitiveDictionaryBuilder</a>\<K, V\>

where K: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>, V: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>,

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html" class="struct" title="struct arrow::array::PrimitiveDictionaryBuilder">PrimitiveDictionaryBuilder</a>\<K, V\>

Creates a new `PrimitiveDictionaryBuilder`.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html#method.new_from_empty_builders" class="fn">new_from_empty_builders</a>( keys_builder: <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveBuilder.html" class="struct" title="struct arrow::array::PrimitiveBuilder">PrimitiveBuilder</a>\<K\>, values_builder: <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveBuilder.html" class="struct" title="struct arrow::array::PrimitiveBuilder">PrimitiveBuilder</a>\<V\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html" class="struct" title="struct arrow::array::PrimitiveDictionaryBuilder">PrimitiveDictionaryBuilder</a>\<K, V\>

Creates a new `PrimitiveDictionaryBuilder` from the provided keys and values builders.

##### <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html#panics" class="doc-anchor">§</a>Panics

This method panics if `keys_builder` or `values_builder` is not empty.

#### pub unsafe fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html#method.new_from_builders" class="fn">new_from_builders</a>( keys_builder: <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveBuilder.html" class="struct" title="struct arrow::array::PrimitiveBuilder">PrimitiveBuilder</a>\<K\>, values_builder: <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveBuilder.html" class="struct" title="struct arrow::array::PrimitiveBuilder">PrimitiveBuilder</a>\<V\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html" class="struct" title="struct arrow::array::PrimitiveDictionaryBuilder">PrimitiveDictionaryBuilder</a>\<K, V\>

Creates a new `PrimitiveDictionaryBuilder` from existing `PrimitiveBuilder`s of keys and values.

##### <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html#safety" class="doc-anchor">§</a>Safety

caller must ensure that the passed in builders are valid for DictionaryArray.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html#method.with_capacity" class="fn">with_capacity</a>( keys_capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, values_capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html" class="struct" title="struct arrow::array::PrimitiveDictionaryBuilder">PrimitiveDictionaryBuilder</a>\<K, V\>

Creates a new `PrimitiveDictionaryBuilder` with the provided capacities

`keys_capacity`: the number of keys, i.e. length of array to build `values_capacity`: the number of distinct dictionary values, i.e. size of dictionary

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html#method.try_new_from_builder" class="fn">try_new_from_builder</a>\<K2\>( source: <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html" class="struct" title="struct arrow::array::PrimitiveDictionaryBuilder">PrimitiveDictionaryBuilder</a>\<K2, V\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html" class="struct" title="struct arrow::array::PrimitiveDictionaryBuilder">PrimitiveDictionaryBuilder</a>\<K, V\>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

where \<K as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>: <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/cast/trait.NumCast.html" class="trait" title="trait num_traits::cast::NumCast">NumCast</a>, K2: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowDictionaryKeyType.html" class="trait" title="trait arrow::datatypes::ArrowDictionaryKeyType">ArrowDictionaryKeyType</a>, \<K2 as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>: <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/cast/trait.NumCast.html" class="trait" title="trait num_traits::cast::NumCast">NumCast</a>,

Creates a new `PrimitiveDictionaryBuilder` from the existing builder with the same keys and values, but with a new data type for the keys.

##### <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html#example-1" class="doc-anchor">§</a>Example

``` rust

let mut u8_keyed_builder = PrimitiveDictionaryBuilder::<UInt8Type, UInt64Type>::new();

// appending too many values causes the dictionary to overflow
for i in 0..256 {
    u8_keyed_builder.append_value(i);
}
let result = u8_keyed_builder.append(256);
assert!(matches!(result, Err(ArrowError::DictionaryKeyOverflowError{})));

// we need to upgrade to a larger key type
let mut u16_keyed_builder = PrimitiveDictionaryBuilder::<UInt16Type, UInt64Type>::try_new_from_builder(u8_keyed_builder).unwrap();
let dictionary_array = u16_keyed_builder.finish();
let keys = dictionary_array.keys();

assert_eq!(keys, &UInt16Array::from_iter(0..256));
```

<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html#impl-PrimitiveDictionaryBuilder%3CK,+V%3E-1" class="anchor">§</a>

### impl\<K, V\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html" class="struct" title="struct arrow::array::PrimitiveDictionaryBuilder">PrimitiveDictionaryBuilder</a>\<K, V\>

where K: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowDictionaryKeyType.html" class="trait" title="trait arrow::datatypes::ArrowDictionaryKeyType">ArrowDictionaryKeyType</a>, V: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>,

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html#method.append" class="fn">append</a>( &mut self, value: \<V as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\<K as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Append a primitive value to the array. Return an existing index if already present in the values array or a new index if the value is appended to the values array.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html#method.append_n" class="fn">append_n</a>( &mut self, value: \<V as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>, count: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\<K as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Append a value multiple times to the array. This is the same as `append` but allows to append the same value multiple times without doing multiple lookups.

Returns an error if the new index would overflow the key type.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html#method.append_value" class="fn">append_value</a>(&mut self, value: \<V as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>)

Infallibly append a value to this builder

##### <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html#panics-1" class="doc-anchor">§</a>Panics

Panics if the resulting length of the dictionary values array would exceed `T::Native::MAX`

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html#method.append_values" class="fn">append_values</a>( &mut self, value: \<V as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>, count: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, )

Infallibly append a value to this builder repeatedly `count` times. This is the same as `append_value` but allows to append the same value multiple times without doing multiple lookups.

##### <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html#panics-2" class="doc-anchor">§</a>Panics

Panics if the resulting length of the dictionary values array would exceed `T::Native::MAX`

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html#method.append_null" class="fn">append_null</a>(&mut self)

Appends a null slot into the builder

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html#method.append_nulls" class="fn">append_nulls</a>(&mut self, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Append `n` null slots into the builder

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html#method.append_option" class="fn">append_option</a>( &mut self, value: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<V as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>, )

Append an `Option` value into the builder

##### <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html#panics-3" class="doc-anchor">§</a>Panics

Panics if the resulting length of the dictionary values array would exceed `T::Native::MAX`

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html#method.append_options" class="fn">append_options</a>( &mut self, value: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<V as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>, count: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, )

Append an `Option` value into the builder repeatedly `count` times. This is the same as `append_option` but allows to append the same value multiple times without doing multiple lookups.

##### <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html#panics-4" class="doc-anchor">§</a>Panics

Panics if the resulting length of the dictionary values array would exceed `T::Native::MAX`

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html#method.extend_dictionary" class="fn">extend_dictionary</a>( &mut self, dictionary: &<a href="https://docs.rs/arrow/latest/arrow/array/struct.TypedDictionaryArray.html" class="struct" title="struct arrow::array::TypedDictionaryArray">TypedDictionaryArray</a>\<'\_, K, <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<V\>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Extends builder with dictionary

This is the same as [`Self::extend`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html#method.extend "method arrow::array::PrimitiveDictionaryBuilder::extend") but is faster as it translates the dictionary values once rather than doing a lookup for each item in the iterator

when dictionary values are null (the actual mapped values) the keys are null

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html#method.finish" class="fn">finish</a>(&mut self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html" class="struct" title="struct arrow::array::DictionaryArray">DictionaryArray</a>\<K\>

Builds the `DictionaryArray` and reset this builder.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html#method.finish_cloned" class="fn">finish_cloned</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html" class="struct" title="struct arrow::array::DictionaryArray">DictionaryArray</a>\<K\>

Builds the `DictionaryArray` without resetting the builder.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html#method.finish_preserve_values" class="fn">finish_preserve_values</a>(&mut self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html" class="struct" title="struct arrow::array::DictionaryArray">DictionaryArray</a>\<K\>

Builds the `DictionaryArray` without resetting the values builder or the internal de-duplication map.

The advantage of doing this is that the values will represent the entire set of what has been built so-far by this builder and ensures consistency in the assignment of keys to values across multiple calls to `finish_preserve_values`. This enables ipc writers to efficiently emit delta dictionaries.

The downside to this is that building the record requires creating a copy of the values, which can become slowly more expensive if the dictionary grows.

Additionally, if record batches from multiple different dictionary builders for the same column are fed into a single ipc writer, beware that entire dictionaries are likely to be re-sent frequently even when the majority of the values are not used by the current record batch.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html#method.values_slice" class="fn">values_slice</a>(&self) -\> &\[\<V as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\]

Returns the current dictionary values buffer as a slice

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html#method.values_slice_mut" class="fn">values_slice_mut</a>(&mut self) -\> &mut \[\<V as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\]

Returns the current dictionary values buffer as a mutable slice

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html#method.validity_slice" class="fn">validity_slice</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>

Returns the current null buffer as a slice

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html#impl-ArrayBuilder-for-PrimitiveDictionaryBuilder%3CK,+V%3E" class="anchor">§</a>

### impl\<K, V\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html" class="struct" title="struct arrow::array::PrimitiveDictionaryBuilder">PrimitiveDictionaryBuilder</a>\<K, V\>

where K: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowDictionaryKeyType.html" class="trait" title="trait arrow::datatypes::ArrowDictionaryKeyType">ArrowDictionaryKeyType</a>, V: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html#method.as_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the builder as an non-mutable `Any` reference.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html#method.as_any_mut" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.as_any_mut" class="fn">as_any_mut</a>(&mut self) -\> &mut (dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the builder as an mutable `Any` reference.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html#method.into_box_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.into_box_any" class="fn">into_box_any</a>(self: <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html" class="struct" title="struct arrow::array::PrimitiveDictionaryBuilder">PrimitiveDictionaryBuilder</a>\<K, V\>\>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a>\>

Returns the boxed builder as a box of `Any`.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html#method.len" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the number of array slots in the builder

<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html#method.finish-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.finish" class="fn">finish</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

Builds the array and reset this builder.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html#method.finish_cloned-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.finish_cloned" class="fn">finish_cloned</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

Builds the array without resetting the builder.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html#method.is_empty" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether number of array slots is zero

<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html#impl-Debug-for-PrimitiveDictionaryBuilder%3CK,+V%3E" class="anchor">§</a>

### impl\<K, V\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html" class="struct" title="struct arrow::array::PrimitiveDictionaryBuilder">PrimitiveDictionaryBuilder</a>\<K, V\>

where K: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> + <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>, V: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> + <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>, \<V as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html#impl-Default-for-PrimitiveDictionaryBuilder%3CK,+V%3E" class="anchor">§</a>

### impl\<K, V\> <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html" class="struct" title="struct arrow::array::PrimitiveDictionaryBuilder">PrimitiveDictionaryBuilder</a>\<K, V\>

where K: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>, V: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html" class="struct" title="struct arrow::array::PrimitiveDictionaryBuilder">PrimitiveDictionaryBuilder</a>\<K, V\>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html#impl-Extend%3COption%3C%3CP+as+ArrowPrimitiveType%3E::Native%3E%3E-for-PrimitiveDictionaryBuilder%3CK,+P%3E" class="anchor">§</a>

### impl\<K, P\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html" class="trait" title="trait core::iter::traits::collect::Extend">Extend</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<P as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html" class="struct" title="struct arrow::array::PrimitiveDictionaryBuilder">PrimitiveDictionaryBuilder</a>\<K, P\>

where K: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowDictionaryKeyType.html" class="trait" title="trait arrow::datatypes::ArrowDictionaryKeyType">ArrowDictionaryKeyType</a>, P: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html#method.extend" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend" class="fn">extend</a>\<T\>(&mut self, iter: T)

where T: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<P as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>,

Extends a collection with the contents of an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend)

<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html#method.extend_one" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_one" class="fn">extend_one</a>(&mut self, item: A)

🔬This is a nightly-only experimental API. (`extend_one`)

Extends a collection with exactly one element.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html#method.extend_reserve" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve" class="fn">extend_reserve</a>(&mut self, additional: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

🔬This is a nightly-only experimental API. (`extend_one`)

Reserves capacity in a collection for the given number of additional elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve)

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html#blanket-implementations" class="anchor">§</a>
