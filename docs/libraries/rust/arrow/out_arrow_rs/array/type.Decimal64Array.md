# Type Alias Decimal64Array Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/array/primitive_array.rs.html#449" class="src">Source</a>

``` rust
pub type Decimal64Array = PrimitiveArray<Decimal64Type>;
```

Expand description

A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of 64-bit fixed point decimals

## <a href="https://docs.rs/arrow/latest/arrow/array/type.Decimal64Array.html#examples" class="doc-anchor">§</a>Examples

Construction

``` rust
// Create from Vec<Option<i64>>
let arr = Decimal64Array::from(vec![Some(1), None, Some(2)]);
// Create from Vec<i64>
let arr = Decimal64Array::from(vec![1, 2, 3]);
// Create iter/collect
let arr: Decimal64Array = std::iter::repeat(42).take(10).collect();
```

See [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") for more information and examples

## Aliased Type<a href="https://docs.rs/arrow/latest/arrow/array/type.Decimal64Array.html#aliased-type" class="anchor">§</a>

``` rust
pub struct Decimal64Array { /* private fields */ }
```
