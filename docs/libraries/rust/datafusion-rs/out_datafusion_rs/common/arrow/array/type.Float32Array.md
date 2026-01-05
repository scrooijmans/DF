# Type Alias Float32Array Copy item path

<a href="https://docs.rs/arrow-array/56.0.0/x86_64-unknown-linux-gnu/src/arrow_array/array/primitive_array.rs.html#233" class="src">Source</a>

``` rust
pub type Float32Array = PrimitiveArray<Float32Type>;
```

Expand description

A [`PrimitiveArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.PrimitiveArray.html "struct datafusion::common::arrow::array::PrimitiveArray") of `f32`

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.Float32Array.html#examples" class="doc-anchor">§</a>Examples

Construction

``` rust
// Create from Vec<Option<f32>>
let arr = Float32Array::from(vec![Some(1.0), None, Some(2.0)]);
// Create from Vec<f32>
let arr = Float32Array::from(vec![1.0, 2.0, 3.0]);
// Create iter/collect
let arr: Float32Array = std::iter::repeat(42.0).take(10).collect();
```

See [`PrimitiveArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.PrimitiveArray.html "struct datafusion::common::arrow::array::PrimitiveArray") for more information and examples

## Aliased Type<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.Float32Array.html#aliased-type" class="anchor">§</a>

``` rust
pub struct Float32Array { /* private fields */ }
```
