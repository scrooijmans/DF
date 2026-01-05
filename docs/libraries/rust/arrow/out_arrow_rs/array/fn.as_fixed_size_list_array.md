# Function as_fixed_size_list_array Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/cast.rs.html#692" class="src">Source</a>

``` rust
pub fn as_fixed_size_list_array(arr: &dyn Array) -> &FixedSizeListArray
```

Expand description

Force downcast of an [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array"), such as an [`ArrayRef`](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef") to [`FixedSizeListArray`](https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListArray.html "struct arrow::array::FixedSizeListArray"), panicking on failure.
