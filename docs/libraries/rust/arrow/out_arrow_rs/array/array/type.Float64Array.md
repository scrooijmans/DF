# Type Alias Float64Array Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/array/primitive_array.rs.html#252" class="src">Source</a>

``` rust
pub type Float64Array = PrimitiveArray<Float64Type>;
```

Expand description

A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of `f64`

## <a href="https://docs.rs/arrow/latest/arrow/array/array/type.Float64Array.html#examples" class="doc-anchor">§</a>Examples

Construction

``` rust
// Create from Vec<Option<f32>>
let arr = Float64Array::from(vec![Some(1.0), None, Some(2.0)]);
// Create from Vec<f32>
let arr = Float64Array::from(vec![1.0, 2.0, 3.0]);
// Create iter/collect
let arr: Float64Array = std::iter::repeat(42.0).take(10).collect();
```

See [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") for more information and examples

## Aliased Type<a href="https://docs.rs/arrow/latest/arrow/array/array/type.Float64Array.html#aliased-type" class="anchor">§</a>

``` rust
pub struct Float64Array { /* private fields */ }
```
