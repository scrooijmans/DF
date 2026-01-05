# Function as_union_array Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/cast.rs.html#777" class="src">Source</a>

``` rust
pub fn as_union_array(arr: &dyn Array) -> &UnionArray
```

Expand description

Force downcast of an [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array"), such as an [`ArrayRef`](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef") to [`UnionArray`](https://docs.rs/arrow/latest/arrow/array/struct.UnionArray.html "struct arrow::array::UnionArray"), panicking on failure.
