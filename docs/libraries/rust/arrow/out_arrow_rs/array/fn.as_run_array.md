# Function as_run_array Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/cast.rs.html#603-605" class="src">Source</a>

``` rust
pub fn as_run_array<T>(arr: &dyn Array) -> &RunArray<T>where
    T: RunEndIndexType,
```

Expand description

Force downcast of an [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array"), such as an [`ArrayRef`](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef") to [`RunArray<T>`](https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html "struct arrow::array::RunArray"), panic’ing on failure.

## <a href="https://docs.rs/arrow/latest/arrow/array/fn.as_run_array.html#example" class="doc-anchor">§</a>Example

``` rust

let arr: RunArray<Int32Type> = vec![Some("foo")].into_iter().collect();
let arr: ArrayRef = std::sync::Arc::new(arr);
let run_array: &RunArray<Int32Type> = as_run_array::<Int32Type>(&arr);
```
