# Function as_generic_binary_array Copy item path

<a href="https://docs.rs/arrow-array/56.0.0/x86_64-unknown-linux-gnu/src/arrow_array/cast.rs.html#708" class="src">Source</a>

``` rust
pub fn as_generic_binary_array<S>(
    arr: &dyn Array,
) -> &GenericByteArray<GenericBinaryType<S>>where
    S: OffsetSizeTrait,
```

Expand description

Force downcast of an [`Array`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html "trait datafusion::common::arrow::array::Array"), such as an [`ArrayRef`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.ArrayRef.html "type datafusion::common::arrow::array::ArrayRef") to [`GenericBinaryArray<S>`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.GenericBinaryArray.html "type datafusion::common::arrow::array::GenericBinaryArray"), panicking on failure.
