# Function as_null_array Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/cast.rs.html#775" class="src">Source</a>

``` rust
pub fn as_null_array(arr: &dyn Array) -> &NullArray
```

Expand description

Force downcast of an [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array"), such as an [`ArrayRef`](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef") to [`NullArray`](https://docs.rs/arrow/latest/arrow/array/struct.NullArray.html "struct arrow::array::NullArray"), panicking on failure.
