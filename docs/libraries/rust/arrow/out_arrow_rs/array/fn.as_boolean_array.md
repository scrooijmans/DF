# Function as_boolean_array Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/cast.rs.html#746" class="src">Source</a>

``` rust
pub fn as_boolean_array(arr: &dyn Array) -> &BooleanArray
```

Expand description

Force downcast of an [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array"), such as an [`ArrayRef`](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef") to [`BooleanArray`](https://docs.rs/arrow/latest/arrow/array/struct.BooleanArray.html "struct arrow::array::BooleanArray"), panicking on failure.

## <a href="https://docs.rs/arrow/latest/arrow/array/fn.as_boolean_array.html#example" class="doc-anchor">§</a>Example

``` rust

let arr: ArrayRef = Arc::new(BooleanArray::from_iter(vec![Some(true)]));
let boolean_array = as_boolean_array(&arr);
```
