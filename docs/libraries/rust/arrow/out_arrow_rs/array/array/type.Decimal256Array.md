# Type Alias Decimal256Array Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/array/primitive_array.rs.html#488" class="src">Source</a>

``` rust
pub type Decimal256Array = PrimitiveArray<Decimal256Type>;
```

Expand description

A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of 256-bit fixed point decimals

## <a href="https://docs.rs/arrow/latest/arrow/array/array/type.Decimal256Array.html#examples" class="doc-anchor">§</a>Examples

Construction

``` rust
use arrow_buffer::i256;
// Create from Vec<Option<i256>>
let arr = Decimal256Array::from(vec![Some(i256::from(1)), None, Some(i256::from(2))]);
// Create from Vec<i256>
let arr = Decimal256Array::from(vec![i256::from(1), i256::from(2), i256::from(3)]);
// Create iter/collect
let arr: Decimal256Array = std::iter::repeat(i256::from(42)).take(10).collect();
```

See [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") for more information and examples

## Aliased Type<a href="https://docs.rs/arrow/latest/arrow/array/array/type.Decimal256Array.html#aliased-type" class="anchor">§</a>

``` rust
pub struct Decimal256Array { /* private fields */ }
```
