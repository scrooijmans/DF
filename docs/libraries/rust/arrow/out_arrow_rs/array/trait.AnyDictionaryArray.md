# Trait AnyDictionaryArray Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/array/dictionary_array.rs.html#1006" class="src">Source</a>

``` rust
pub trait AnyDictionaryArray: Array {
    // Required methods
    fn keys(&self) -> &dyn Array;
    fn values(&self) -> &Arc<dyn Array>;
    fn normalized_keys(&self) -> Vec<usize>;
    fn with_values(&self, values: Arc<dyn Array>) -> Arc<dyn Array>;
}
```

Expand description

A [`DictionaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html "struct arrow::array::DictionaryArray") with the key type erased

This can be used to efficiently implement kernels for all possible dictionary keys without needing to create specialized implementations for each key type

For example

``` rust

fn to_string(a: &dyn Array) -> Result<ArrayRef, ArrowError> {
    if let Some(d) = a.as_any_dictionary_opt() {
        // Recursively handle dictionary input
        let r = to_string(d.values().as_ref())?;
        return Ok(d.with_values(r));
    }
    downcast_primitive_array! {
        a => Ok(Arc::new(a.iter().map(|x| x.map(|x| format!("{x:?}"))).collect::<StringArray>())),
        d => Err(ArrowError::InvalidArgumentError(format!("{d:?} not supported")))
    }
}

let result = to_string(&Int32Array::from(vec![1, 2, 3])).unwrap();
let actual = result.as_string::<i32>().iter().map(Option::unwrap).collect::<Vec<_>>();
assert_eq!(actual, &["1", "2", "3"]);

let mut dict = PrimitiveDictionaryBuilder::<Int32Type, UInt16Type>::new();
dict.extend([Some(1), Some(1), Some(2), Some(3), Some(2)]);
let dict = dict.finish();

let r = to_string(&dict).unwrap();
let r = r.as_dictionary::<Int32Type>().downcast_dict::<StringArray>().unwrap();
assert_eq!(r.keys(), dict.keys()); // Keys are the same

let actual = r.into_iter().map(Option::unwrap).collect::<Vec<_>>();
assert_eq!(actual, &["1", "1", "2", "3", "2"]);
```

See [`AsArray::as_any_dictionary_opt`](https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#tymethod.as_any_dictionary_opt "method arrow::array::AsArray::as_any_dictionary_opt") and [`AsArray::as_any_dictionary`](https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_any_dictionary "method arrow::array::AsArray::as_any_dictionary")

## Required Methods<a href="https://docs.rs/arrow/latest/arrow/array/trait.AnyDictionaryArray.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AnyDictionaryArray.html#tymethod.keys" class="fn">keys</a>(&self) -\> &dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>

Returns the primitive keys of this dictionary as an [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array")

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AnyDictionaryArray.html#tymethod.values" class="fn">values</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

Returns the values of this dictionary

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AnyDictionaryArray.html#tymethod.normalized_keys" class="fn">normalized_keys</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

Returns the keys of this dictionary as usize

The values for nulls will be arbitrary, but are guaranteed to be in the range `0..self.values.len()`

##### <a href="https://docs.rs/arrow/latest/arrow/array/trait.AnyDictionaryArray.html#panic" class="doc-anchor">§</a>Panic

Panics if `values.len() == 0`

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AnyDictionaryArray.html#tymethod.with_values" class="fn">with_values</a>(&self, values: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

Create a new [`DictionaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html "struct arrow::array::DictionaryArray") replacing `values` with the new values

See [`DictionaryArray::with_values`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html#method.with_values "method arrow::array::DictionaryArray::with_values")

## Implementors<a href="https://docs.rs/arrow/latest/arrow/array/trait.AnyDictionaryArray.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.AnyDictionaryArray.html#impl-AnyDictionaryArray-for-DictionaryArray%3CK%3E" class="anchor">§</a>

### impl\<K\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.AnyDictionaryArray.html" class="trait" title="trait arrow::array::AnyDictionaryArray">AnyDictionaryArray</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html" class="struct" title="struct arrow::array::DictionaryArray">DictionaryArray</a>\<K\>

where K: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowDictionaryKeyType.html" class="trait" title="trait arrow::datatypes::ArrowDictionaryKeyType">ArrowDictionaryKeyType</a>,
