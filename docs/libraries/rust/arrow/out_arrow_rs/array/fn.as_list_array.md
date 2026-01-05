# Function as_list_array Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/cast.rs.html#685" class="src">Source</a>

``` rust
pub fn as_list_array(arr: &dyn Array) -> &GenericListArray<i32>
```

Expand description

Force downcast of an [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array"), such as an [`ArrayRef`](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef") to [`ListArray`](https://docs.rs/arrow/latest/arrow/array/type.ListArray.html "type arrow::array::ListArray"), panicking on failure.
