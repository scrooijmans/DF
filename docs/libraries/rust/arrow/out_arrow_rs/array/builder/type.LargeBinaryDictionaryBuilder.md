# Type Alias LargeBinaryDictionaryBuilder Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/builder/generic_bytes_dictionary_builder.rs.html#598" class="src">Source</a>

``` rust
pub type LargeBinaryDictionaryBuilder<K> = GenericByteDictionaryBuilder<K, GenericBinaryType<i64>>;
```

Expand description

Builder for [`DictionaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html "struct arrow::array::DictionaryArray") of [`LargeBinaryArray`](https://docs.rs/arrow/latest/arrow/array/type.LargeBinaryArray.html "type arrow::array::LargeBinaryArray")

## Aliased Type<a href="https://docs.rs/arrow/latest/arrow/array/builder/type.LargeBinaryDictionaryBuilder.html#aliased-type" class="anchor">§</a>

``` rust
pub struct LargeBinaryDictionaryBuilder<K> { /* private fields */ }
```
