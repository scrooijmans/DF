# Struct DictionaryArray Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/array/dictionary_array.rs.html#243" class="src">Source</a>

``` rust
pub struct DictionaryArray<K>where
    K: ArrowDictionaryKeyType,{ /* private fields */ }
```

Expand description

An array of [dictionary encoded values](https://arrow.apache.org/docs/format/Columnar.html#dictionary-encoded-layout)

This is mostly used to represent strings or a limited set of primitive types as integers, for example when doing NLP analysis or representing chromosomes by name.

[`DictionaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html "struct arrow::array::DictionaryArray") are represented using a `keys` array and a `values` array, which may be different lengths. The `keys` array stores indexes in the `values` array which holds the corresponding logical value, as shown here:

``` text
┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
  ┌─────────────────┐  ┌─────────┐ │     ┌─────────────────┐
│ │        A        │  │    0    │       │        A        │     values[keys[0]]
  ├─────────────────┤  ├─────────┤ │     ├─────────────────┤
│ │        D        │  │    2    │       │        B        │     values[keys[1]]
  ├─────────────────┤  ├─────────┤ │     ├─────────────────┤
│ │        B        │  │    2    │       │        B        │     values[keys[2]]
  └─────────────────┘  ├─────────┤ │     ├─────────────────┤
│                      │    1    │       │        D        │     values[keys[3]]
                       ├─────────┤ │     ├─────────────────┤
│                      │    1    │       │        D        │     values[keys[4]]
                       ├─────────┤ │     ├─────────────────┤
│                      │    0    │       │        A        │     values[keys[5]]
                       └─────────┘ │     └─────────────────┘
│       values            keys
 ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┘
                                            Logical array
                                               Contents
          DictionaryArray
             length = 6
```

## <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#example-from-nullable-data" class="doc-anchor">§</a>Example: From Nullable Data

``` rust
let test = vec!["a", "a", "b", "c"];
let array : DictionaryArray<Int8Type> = test.iter().map(|&x| if x == "b" {None} else {Some(x)}).collect();
assert_eq!(array.keys(), &Int8Array::from(vec![Some(0), Some(0), None, Some(1)]));
```

## <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#example-from-non-nullable-data" class="doc-anchor">§</a>Example: From Non-Nullable Data

``` rust
let test = vec!["a", "a", "b", "c"];
let array : DictionaryArray<Int8Type> = test.into_iter().collect();
assert_eq!(array.keys(), &Int8Array::from(vec![0, 0, 1, 2]));
```

## <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#example-from-existing-arrays" class="doc-anchor">§</a>Example: From Existing Arrays

``` rust
// You can form your own DictionaryArray by providing the
// values (dictionary) and keys (indexes into the dictionary):
let values = StringArray::from_iter_values(["a", "b", "c"]);
let keys = Int8Array::from_iter_values([0, 0, 1, 2]);
let array = DictionaryArray::<Int8Type>::try_new(keys, Arc::new(values)).unwrap();
let expected: DictionaryArray::<Int8Type> = vec!["a", "a", "b", "c"].into_iter().collect();
assert_eq!(&array, &expected);
```

## <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#example-using-builder" class="doc-anchor">§</a>Example: Using Builder

``` rust
let mut builder = StringDictionaryBuilder::<Int32Type>::new();
builder.append_value("a");
builder.append_null();
builder.append_value("a");
builder.append_value("b");
let array = builder.finish();

let values: Vec<_> = array.downcast_dict::<StringArray>().unwrap().into_iter().collect();
assert_eq!(&values, &[Some("a"), None, Some("a"), Some("b")]);
```

## Implementations<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#impl-DictionaryArray%3CK%3E" class="anchor">§</a>

### impl\<K\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html" class="struct" title="struct arrow::array::DictionaryArray">DictionaryArray</a>\<K\>

where K: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowDictionaryKeyType.html" class="trait" title="trait arrow::datatypes::ArrowDictionaryKeyType">ArrowDictionaryKeyType</a>,

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#method.new" class="fn">new</a>( keys: <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<K\>, values: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html" class="struct" title="struct arrow::array::DictionaryArray">DictionaryArray</a>\<K\>

Attempt to create a new DictionaryArray with a specified keys (indexes into the dictionary) and values (dictionary) array.

##### <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#panics" class="doc-anchor">§</a>Panics

Panics if [`Self::try_new`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html#method.try_new "associated function arrow::array::DictionaryArray::try_new") returns an error

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#method.try_new" class="fn">try_new</a>( keys: <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<K\>, values: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html" class="struct" title="struct arrow::array::DictionaryArray">DictionaryArray</a>\<K\>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Attempt to create a new DictionaryArray with a specified keys (indexes into the dictionary) and values (dictionary) array.

##### <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#errors" class="doc-anchor">§</a>Errors

Returns an error if any `keys[i] >= values.len() || keys[i] < 0`

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#method.new_scalar" class="fn">new_scalar</a>\<T\>(value: <a href="https://docs.rs/arrow/latest/arrow/array/struct.Scalar.html" class="struct" title="struct arrow::array::Scalar">Scalar</a>\<T\>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.Scalar.html" class="struct" title="struct arrow::array::Scalar">Scalar</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html" class="struct" title="struct arrow::array::DictionaryArray">DictionaryArray</a>\<K\>\>

where T: <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a> + 'static,

Create a new [`Scalar`](https://docs.rs/arrow/latest/arrow/array/struct.Scalar.html "struct arrow::array::Scalar") from `value`

#### pub unsafe fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#method.new_unchecked" class="fn">new_unchecked</a>( keys: <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<K\>, values: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html" class="struct" title="struct arrow::array::DictionaryArray">DictionaryArray</a>\<K\>

Create a new [`DictionaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html "struct arrow::array::DictionaryArray") without performing validation

##### <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#safety" class="doc-anchor">§</a>Safety

Safe provided [`Self::try_new`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html#method.try_new "associated function arrow::array::DictionaryArray::try_new") would not return an error

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#method.into_parts" class="fn">into_parts</a>(self) -\> (<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<K\>, <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>)

Deconstruct this array into its constituent parts

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#method.keys" class="fn">keys</a>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<K\>

Return an array view of the keys of this dictionary as a PrimitiveArray.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#method.lookup_key" class="fn">lookup_key</a>( &self, value: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<K as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>

If `value` is present in `values` (aka the dictionary), returns the corresponding key (index into the `values` array). Otherwise returns `None`.

Panics if `values` is not a [`StringArray`](https://docs.rs/arrow/latest/arrow/array/type.StringArray.html "type arrow::array::StringArray").

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#method.values" class="fn">values</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

Returns a reference to the dictionary values array

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#method.value_type" class="fn">value_type</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a>

Returns a clone of the value type of this list.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#method.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

The length of the dictionary is the length of the keys array.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Whether this dictionary is empty

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#method.is_ordered" class="fn">is_ordered</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Currently exists for compatibility purposes with Arrow IPC.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#method.keys_iter" class="fn">keys_iter</a>(&self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\>

Return an iterator over the keys (indexes into the dictionary)

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#method.key" class="fn">key</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

Return the value of `keys` (the dictionary key) at index `i`, cast to `usize`, `None` if the value at `i` is `NULL`.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#method.slice" class="fn">slice</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html" class="struct" title="struct arrow::array::DictionaryArray">DictionaryArray</a>\<K\>

Returns a zero-copy slice of this array with the indicated offset and length.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#method.downcast_dict" class="fn">downcast_dict</a>\<V\>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.TypedDictionaryArray.html" class="struct" title="struct arrow::array::TypedDictionaryArray">TypedDictionaryArray</a>\<'\_, K, V\>\>

where V: 'static,

Downcast this dictionary to a [`TypedDictionaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.TypedDictionaryArray.html "struct arrow::array::TypedDictionaryArray")

``` rust
use arrow_array::{Array, ArrayAccessor, DictionaryArray, StringArray, types::Int32Type};

let orig = [Some("a"), Some("b"), None];
let dictionary = DictionaryArray::<Int32Type>::from_iter(orig);
let typed = dictionary.downcast_dict::<StringArray>().unwrap();
assert_eq!(typed.value(0), "a");
assert_eq!(typed.value(1), "b");
assert!(typed.is_null(2));
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#method.with_values" class="fn">with_values</a>(&self, values: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html" class="struct" title="struct arrow::array::DictionaryArray">DictionaryArray</a>\<K\>

Returns a new dictionary with the same keys as the current instance but with a different set of dictionary values

This can be used to perform an operation on the values of a dictionary

##### <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#panics-1" class="doc-anchor">§</a>Panics

Panics if `values` has a length less than the current values

``` rust

// Construct a Dict(Int32, Int8)
let mut builder = PrimitiveDictionaryBuilder::<Int32Type, Int8Type>::with_capacity(2, 200);
for i in 0..100 {
    builder.append(i % 2).unwrap();
}

let dictionary = builder.finish();

// Perform a widening cast of dictionary values
let typed_dictionary = dictionary.downcast_dict::<Int8Array>().unwrap();
let values: Int64Array = typed_dictionary.values().unary(|x| x as i64);

// Create a Dict(Int32,
let new = dictionary.with_values(Arc::new(values));

// Verify values are as expected
let new_typed = new.downcast_dict::<Int64Array>().unwrap();
for i in 0..100 {
    assert_eq!(new_typed.value(i), (i % 2) as i64)
}
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#method.into_primitive_dict_builder" class="fn">into_primitive_dict_builder</a>\<V\>( self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html" class="struct" title="struct arrow::array::PrimitiveDictionaryBuilder">PrimitiveDictionaryBuilder</a>\<K, V\>, <a href="https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html" class="struct" title="struct arrow::array::DictionaryArray">DictionaryArray</a>\<K\>\>

where V: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>,

Returns `PrimitiveDictionaryBuilder` of this dictionary array for mutating its keys and values if the underlying data buffer is not shared by others.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#method.unary_mut" class="fn">unary_mut</a>\<F, V\>( self, op: F, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html" class="struct" title="struct arrow::array::DictionaryArray">DictionaryArray</a>\<K\>, <a href="https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html" class="struct" title="struct arrow::array::DictionaryArray">DictionaryArray</a>\<K\>\>

where V: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(\<V as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>) -\> \<V as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>,

Applies an unary and infallible function to a mutable dictionary array. Mutable dictionary array means that the buffers are not shared with other arrays. As a result, this mutates the buffers directly without allocating new buffers.

##### <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#implementation" class="doc-anchor">§</a>Implementation

This will apply the function for all dictionary values, including those on null slots. This implies that the operation must be infallible for any value of the corresponding type or this function may panic.

##### <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#example" class="doc-anchor">§</a>Example

``` rust
let values = Int32Array::from(vec![Some(10), Some(20), None]);
let keys = Int8Array::from_iter_values([0, 0, 1, 2]);
let dictionary = DictionaryArray::<Int8Type>::try_new(keys, Arc::new(values)).unwrap();
let c = dictionary.unary_mut::<_, Int32Type>(|x| x + 1).unwrap();
let typed = c.downcast_dict::<Int32Array>().unwrap();
assert_eq!(typed.value(0), 11);
assert_eq!(typed.value(1), 11);
assert_eq!(typed.value(2), 21);
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#method.occupancy" class="fn">occupancy</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct arrow::buffer::BooleanBuffer">BooleanBuffer</a>

Computes an occupancy mask for this dictionary’s values

For each value in [`Self::values`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html#method.values "method arrow::array::DictionaryArray::values") the corresponding bit will be set in the returned mask if it is referenced by a key in this [`DictionaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html "struct arrow::array::DictionaryArray")

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#impl-AnyDictionaryArray-for-DictionaryArray%3CK%3E" class="anchor">§</a>

### impl\<K\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.AnyDictionaryArray.html" class="trait" title="trait arrow::array::AnyDictionaryArray">AnyDictionaryArray</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html" class="struct" title="struct arrow::array::DictionaryArray">DictionaryArray</a>\<K\>

where K: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowDictionaryKeyType.html" class="trait" title="trait arrow::datatypes::ArrowDictionaryKeyType">ArrowDictionaryKeyType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#method.keys-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AnyDictionaryArray.html#tymethod.keys" class="fn">keys</a>(&self) -\> &dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>

Returns the primitive keys of this dictionary as an [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array")

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#method.values-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AnyDictionaryArray.html#tymethod.values" class="fn">values</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

Returns the values of this dictionary

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#method.normalized_keys" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AnyDictionaryArray.html#tymethod.normalized_keys" class="fn">normalized_keys</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

Returns the keys of this dictionary as usize [Read more](https://docs.rs/arrow/latest/arrow/array/trait.AnyDictionaryArray.html#tymethod.normalized_keys)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#method.with_values-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AnyDictionaryArray.html#tymethod.with_values" class="fn">with_values</a>(&self, values: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

Create a new [`DictionaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html "struct arrow::array::DictionaryArray") replacing `values` with the new values [Read more](https://docs.rs/arrow/latest/arrow/array/trait.AnyDictionaryArray.html#tymethod.with_values)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#impl-Array-for-DictionaryArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html" class="struct" title="struct arrow::array::DictionaryArray">DictionaryArray</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowDictionaryKeyType.html" class="trait" title="trait arrow::datatypes::ArrowDictionaryKeyType">ArrowDictionaryKeyType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#method.as_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the array as [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") so that it can be downcasted to a specific implementation. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.as_any)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#method.to_data" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.to_data" class="fn">to_data</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

Returns the underlying data of this array

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#method.into_data" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.into_data" class="fn">into_data</a>(self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

Returns the underlying data of this array [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.into_data)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#method.data_type" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.data_type" class="fn">data_type</a>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a>

Returns a reference to the [`DataType`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html "enum arrow::datatypes::DataType") of this array. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.data_type)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#method.slice-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.slice" class="fn">slice</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

Returns a zero-copy slice of this array with the indicated offset and length. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.slice)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#method.len-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the length (i.e., number of elements) of this array. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.len)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#method.is_empty-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether this array is empty. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.is_empty)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#method.shrink_to_fit" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.shrink_to_fit" class="fn">shrink_to_fit</a>(&mut self)

Shrinks the capacity of any exclusively owned buffer as much as possible [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.shrink_to_fit)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#method.offset" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.offset" class="fn">offset</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the offset into the underlying data used by this array(-slice). Note that the underlying data can be shared by many arrays. This defaults to `0`. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.offset)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#method.nulls" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.nulls" class="fn">nulls</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>\>

Returns the null buffer of this array if any. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.nulls)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#method.logical_nulls" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.logical_nulls" class="fn">logical_nulls</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>\>

Returns a potentially computed [`NullBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html "struct arrow::buffer::NullBuffer") that represents the logical null values of this array, if any. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.logical_nulls)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#method.logical_null_count" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.logical_null_count" class="fn">logical_null_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of logical null values in this array. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.logical_null_count)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#method.is_nullable" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_nullable" class="fn">is_nullable</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns `false` if the array is guaranteed to not contain any logical nulls [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_nullable)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#method.get_buffer_memory_size" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.get_buffer_memory_size" class="fn">get_buffer_memory_size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of bytes of memory pointed to by this array. The buffers store bytes in the Arrow memory format, and include the data as well as the validity map. Note that this does not always correspond to the exact memory usage of an array, since multiple arrays can share the same buffers or slices thereof.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#method.get_array_memory_size" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.get_array_memory_size" class="fn">get_array_memory_size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of bytes of memory occupied physically by this array. This value will always be greater than returned by `get_buffer_memory_size()` and includes the overhead of the data structures that contain the pointers to the various buffers.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#method.is_null" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_null" class="fn">is_null</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether the element at `index` is null according to [`Array::nulls`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.nulls "method arrow::array::Array::nulls") [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_null)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#method.is_valid" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_valid" class="fn">is_valid</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether the element at `index` is *not* null, the opposite of [`Self::is_null`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_null "method arrow_array::array::Array::is_null::is_null"). [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_valid)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#method.null_count" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.null_count" class="fn">null_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of physical null values in this array. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.null_count)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#impl-Clone-for-DictionaryArray%3CK%3E" class="anchor">§</a>

### impl\<K\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html" class="struct" title="struct arrow::array::DictionaryArray">DictionaryArray</a>\<K\>

where K: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowDictionaryKeyType.html" class="trait" title="trait arrow::datatypes::ArrowDictionaryKeyType">ArrowDictionaryKeyType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html" class="struct" title="struct arrow::array::DictionaryArray">DictionaryArray</a>\<K\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#impl-Debug-for-DictionaryArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html" class="struct" title="struct arrow::array::DictionaryArray">DictionaryArray</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowDictionaryKeyType.html" class="trait" title="trait arrow::datatypes::ArrowDictionaryKeyType">ArrowDictionaryKeyType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#impl-From%3CArrayData%3E-for-DictionaryArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html" class="struct" title="struct arrow::array::DictionaryArray">DictionaryArray</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowDictionaryKeyType.html" class="trait" title="trait arrow::datatypes::ArrowDictionaryKeyType">ArrowDictionaryKeyType</a>,

Constructs a `DictionaryArray` from an array data reference.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(data: <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html" class="struct" title="struct arrow::array::DictionaryArray">DictionaryArray</a>\<T\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#impl-From%3CDictionaryArray%3CT%3E%3E-for-ArrayData" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html" class="struct" title="struct arrow::array::DictionaryArray">DictionaryArray</a>\<T\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowDictionaryKeyType.html" class="trait" title="trait arrow::datatypes::ArrowDictionaryKeyType">ArrowDictionaryKeyType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(array: <a href="https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html" class="struct" title="struct arrow::array::DictionaryArray">DictionaryArray</a>\<T\>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#impl-FromIterator%3C%26str%3E-for-DictionaryArray%3CT%3E" class="anchor">§</a>

### impl\<'a, T\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html" class="struct" title="struct arrow::array::DictionaryArray">DictionaryArray</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowDictionaryKeyType.html" class="trait" title="trait arrow::datatypes::ArrowDictionaryKeyType">ArrowDictionaryKeyType</a>,

Constructs a `DictionaryArray` from an iterator of strings.

#### <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#example-2" class="doc-anchor">§</a>Example:

``` rust
use arrow_array::{DictionaryArray, PrimitiveArray, StringArray, types::Int8Type};

let test = vec!["a", "a", "b", "c"];
let array: DictionaryArray<Int8Type> = test.into_iter().collect();
assert_eq!(
    "DictionaryArray {keys: PrimitiveArray<Int8>\n[\n  0,\n  0,\n  1,\n  2,\n] values: StringArray\n[\n  \"a\",\n  \"b\",\n  \"c\",\n]}\n",
    format!("{:?}", array)
);
```

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#method.from_iter-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html" class="struct" title="struct arrow::array::DictionaryArray">DictionaryArray</a>\<T\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#impl-FromIterator%3COption%3C%26str%3E%3E-for-DictionaryArray%3CT%3E" class="anchor">§</a>

### impl\<'a, T\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html" class="struct" title="struct arrow::array::DictionaryArray">DictionaryArray</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowDictionaryKeyType.html" class="trait" title="trait arrow::datatypes::ArrowDictionaryKeyType">ArrowDictionaryKeyType</a>,

Constructs a `DictionaryArray` from an iterator of optional strings.

#### <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#example-1" class="doc-anchor">§</a>Example:

``` rust
use arrow_array::{DictionaryArray, PrimitiveArray, StringArray, types::Int8Type};

let test = vec!["a", "a", "b", "c"];
let array: DictionaryArray<Int8Type> = test
    .iter()
    .map(|&x| if x == "b" { None } else { Some(x) })
    .collect();
assert_eq!(
    "DictionaryArray {keys: PrimitiveArray<Int8>\n[\n  0,\n  0,\n  null,\n  1,\n] values: StringArray\n[\n  \"a\",\n  \"c\",\n]}\n",
    format!("{:?}", array)
);
```

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#method.from_iter" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html" class="struct" title="struct arrow::array::DictionaryArray">DictionaryArray</a>\<T\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#impl-PartialEq-for-DictionaryArray%3CK%3E" class="anchor">§</a>

### impl\<K\> <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html" class="struct" title="struct arrow::array::DictionaryArray">DictionaryArray</a>\<K\>

where K: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowDictionaryKeyType.html" class="trait" title="trait arrow::datatypes::ArrowDictionaryKeyType">ArrowDictionaryKeyType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html" class="struct" title="struct arrow::array::DictionaryArray">DictionaryArray</a>\<K\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html#blanket-implementations" class="anchor">§</a>
