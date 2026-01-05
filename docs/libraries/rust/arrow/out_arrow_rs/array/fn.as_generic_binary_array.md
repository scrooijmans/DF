# Function as_generic_binary_array Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/cast.rs.html#708" class="src">Source</a>

``` rust
pub fn as_generic_binary_array<S>(
    arr: &dyn Array,
) -> &GenericByteArray<GenericBinaryType<S>>where
    S: OffsetSizeTrait,
```

Expand description

Force downcast of an [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array"), such as an [`ArrayRef`](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef") to [`GenericBinaryArray<S>`](https://docs.rs/arrow/latest/arrow/array/type.GenericBinaryArray.html "type arrow::array::GenericBinaryArray"), panicking on failure.
