# Function as_largestring_array Copy item path

<a href="https://docs.rs/arrow-array/56.0.0/x86_64-unknown-linux-gnu/src/arrow_array/cast.rs.html#774" class="src">Source</a>

``` rust
pub fn as_largestring_array(
    arr: &dyn Array,
) -> &GenericByteArray<GenericStringType<i64>>
```

Expand description

Force downcast of an [`Array`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html "trait datafusion::common::arrow::array::Array"), such as an [`ArrayRef`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.ArrayRef.html "type datafusion::common::arrow::array::ArrayRef") to [`LargeStringArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.LargeStringArray.html "type datafusion::common::arrow::array::LargeStringArray"), panicking on failure.
