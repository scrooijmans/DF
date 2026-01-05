# Function as_dictionary_array Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/cast.rs.html#580-582" class="src">Source</a>

``` rust
pub fn as_dictionary_array<T>(arr: &dyn Array) -> &DictionaryArray<T>where
    T: ArrowDictionaryKeyType,
```

Expand description

Force downcast of an [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array"), such as an [`ArrayRef`](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef") to [`DictionaryArray<T>`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html "struct arrow::array::DictionaryArray"), panic’ing on failure.

## <a href="https://docs.rs/arrow/latest/arrow/array/fn.as_dictionary_array.html#example" class="doc-anchor">§</a>Example

``` rust

let arr: DictionaryArray<Int32Type> = vec![Some("foo")].into_iter().collect();
let arr: ArrayRef = std::sync::Arc::new(arr);
let dict_array: &DictionaryArray<Int32Type> = as_dictionary_array::<Int32Type>(&arr);
```
