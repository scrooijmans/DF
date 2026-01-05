# Type Alias UInt32Array Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/array/primitive_array.rs.html#168" class="src">Source</a>

``` rust
pub type UInt32Array = PrimitiveArray<UInt32Type>;
```

Expand description

A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of `u32`

## <a href="https://docs.rs/arrow/latest/arrow/array/array/type.UInt32Array.html#examples" class="doc-anchor">§</a>Examples

Construction

``` rust
// Create from Vec<Option<u32>>
let arr = UInt32Array::from(vec![Some(1), None, Some(2)]);
// Create from Vec<u32>
let arr = UInt32Array::from(vec![1, 2, 3]);
// Create iter/collect
let arr: UInt32Array = std::iter::repeat(42).take(10).collect();
```

See [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") for more information and examples

## Aliased Type<a href="https://docs.rs/arrow/latest/arrow/array/array/type.UInt32Array.html#aliased-type" class="anchor">§</a>

``` rust
pub struct UInt32Array { /* private fields */ }
```
