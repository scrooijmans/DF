# Type Alias Decimal32Array Copy item path

<a href="https://docs.rs/arrow-array/56.0.0/x86_64-unknown-linux-gnu/src/arrow_array/array/primitive_array.rs.html#430" class="src">Source</a>

``` rust
pub type Decimal32Array = PrimitiveArray<Decimal32Type>;
```

Expand description

A [`PrimitiveArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.PrimitiveArray.html "struct datafusion::common::arrow::array::PrimitiveArray") of 32-bit fixed point decimals

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/type.Decimal32Array.html#examples" class="doc-anchor">§</a>Examples

Construction

``` rust
// Create from Vec<Option<i32>>
let arr = Decimal32Array::from(vec![Some(1), None, Some(2)]);
// Create from Vec<i32>
let arr = Decimal32Array::from(vec![1, 2, 3]);
// Create iter/collect
let arr: Decimal32Array = std::iter::repeat(42).take(10).collect();
```

See [`PrimitiveArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.PrimitiveArray.html "struct datafusion::common::arrow::array::PrimitiveArray") for more information and examples

## Aliased Type<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/type.Decimal32Array.html#aliased-type" class="anchor">§</a>

``` rust
pub struct Decimal32Array { /* private fields */ }
```
