# Type Alias UInt32DictionaryArray Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/array/dictionary_array.rs.html#144" class="src">Source</a>

``` rust
pub type UInt32DictionaryArray = DictionaryArray<UInt32Type>;
```

Expand description

A [`DictionaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html "struct arrow::array::DictionaryArray") indexed by `u32`

## <a href="https://docs.rs/arrow/latest/arrow/array/type.UInt32DictionaryArray.html#example-using-collect" class="doc-anchor">§</a>Example: Using `collect`

``` rust

let array: UInt32DictionaryArray = vec!["a", "a", "b", "c"].into_iter().collect();
let values: Arc<dyn Array> = Arc::new(StringArray::from(vec!["a", "b", "c"]));
assert_eq!(array.keys(), &UInt32Array::from(vec![0, 0, 1, 2]));
assert_eq!(array.values(), &values);
```

See [`DictionaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html "struct arrow::array::DictionaryArray") for more information and examples

## Aliased Type<a href="https://docs.rs/arrow/latest/arrow/array/type.UInt32DictionaryArray.html#aliased-type" class="anchor">§</a>

``` rust
pub struct UInt32DictionaryArray { /* private fields */ }
```
