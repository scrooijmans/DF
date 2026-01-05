# Type Alias Float16Array Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/array/primitive_array.rs.html#214" class="src">Source</a>

``` rust
pub type Float16Array = PrimitiveArray<Float16Type>;
```

Expand description

A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of `f16`

## <a href="https://docs.rs/arrow/latest/arrow/array/type.Float16Array.html#examples" class="doc-anchor">§</a>Examples

Construction

``` rust
use half::f16;
// Create from Vec<Option<f16>>
let arr = Float16Array::from(vec![Some(f16::from_f64(1.0)), Some(f16::from_f64(2.0))]);
// Create from Vec<i8>
let arr = Float16Array::from(vec![f16::from_f64(1.0), f16::from_f64(2.0), f16::from_f64(3.0)]);
// Create iter/collect
let arr: Float16Array = std::iter::repeat(f16::from_f64(1.0)).take(10).collect();
```

## <a href="https://docs.rs/arrow/latest/arrow/array/type.Float16Array.html#example-using-collect" class="doc-anchor">§</a>Example: Using `collect`

``` rust
use half::f16;
let arr : Float16Array = [Some(f16::from_f64(1.0)), Some(f16::from_f64(2.0))].into_iter().collect();
```

See [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") for more information and examples

## Aliased Type<a href="https://docs.rs/arrow/latest/arrow/array/type.Float16Array.html#aliased-type" class="anchor">§</a>

``` rust
pub struct Float16Array { /* private fields */ }
```
