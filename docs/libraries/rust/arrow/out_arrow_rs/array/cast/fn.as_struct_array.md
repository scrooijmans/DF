# Function as_struct_array Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/cast.rs.html#776" class="src">Source</a>

``` rust
pub fn as_struct_array(arr: &dyn Array) -> &StructArray
```

Expand description

Force downcast of an [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array"), such as an [`ArrayRef`](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef") to [`StructArray`](https://docs.rs/arrow/latest/arrow/array/struct.StructArray.html "struct arrow::array::StructArray"), panicking on failure.
