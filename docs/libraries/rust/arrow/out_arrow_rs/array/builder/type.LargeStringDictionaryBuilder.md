# Type Alias LargeStringDictionaryBuilder Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/builder/generic_bytes_dictionary_builder.rs.html#560" class="src">Source</a>

``` rust
pub type LargeStringDictionaryBuilder<K> = GenericByteDictionaryBuilder<K, GenericStringType<i64>>;
```

Expand description

Builder for [`DictionaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html "struct arrow::array::DictionaryArray") of [`LargeStringArray`](https://docs.rs/arrow/latest/arrow/array/type.LargeStringArray.html "type arrow::array::LargeStringArray")

## Aliased Type<a href="https://docs.rs/arrow/latest/arrow/array/builder/type.LargeStringDictionaryBuilder.html#aliased-type" class="anchor">§</a>

``` rust
pub struct LargeStringDictionaryBuilder<K> { /* private fields */ }
```
