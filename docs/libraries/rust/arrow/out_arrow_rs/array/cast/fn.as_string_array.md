# Function as_string_array Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/cast.rs.html#727" class="src">Source</a>

``` rust
pub fn as_string_array(
    arr: &dyn Array,
) -> &GenericByteArray<GenericStringType<i32>>
```

Expand description

Force downcast of an [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array"), such as an [`ArrayRef`](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef") to [`StringArray`](https://docs.rs/arrow/latest/arrow/array/type.StringArray.html "type arrow::array::StringArray"), panicking on failure.

## <a href="https://docs.rs/arrow/latest/arrow/array/cast/fn.as_string_array.html#example" class="doc-anchor">§</a>Example

``` rust

let arr: ArrayRef = Arc::new(StringArray::from_iter(vec![Some("foo")]));
let string_array = as_string_array(&arr);
```
