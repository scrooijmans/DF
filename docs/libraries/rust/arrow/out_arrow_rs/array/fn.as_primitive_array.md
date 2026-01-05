# Function as_primitive_array Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/cast.rs.html#495-497" class="src">Source</a>

``` rust
pub fn as_primitive_array<T>(arr: &dyn Array) -> &PrimitiveArray<T>where
    T: ArrowPrimitiveType,
```

Expand description

Force downcast of an [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array"), such as an [`ArrayRef`](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef"), to [`PrimitiveArray<T>`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray"), panic’ing on failure.

## <a href="https://docs.rs/arrow/latest/arrow/array/fn.as_primitive_array.html#example" class="doc-anchor">§</a>Example

``` rust

let arr: ArrayRef = Arc::new(Int32Array::from(vec![Some(1)]));

// Downcast an `ArrayRef` to Int32Array / PrimitiveArray<Int32>:
let primitive_array: &Int32Array = as_primitive_array(&arr);

// Equivalently:
let primitive_array = as_primitive_array::<Int32Type>(&arr);

// This is the equivalent of:
let primitive_array = arr
    .as_any()
    .downcast_ref::<Int32Array>()
    .unwrap();
```
