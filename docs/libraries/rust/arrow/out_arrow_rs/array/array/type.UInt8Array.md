# Type Alias UInt8Array Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/array/primitive_array.rs.html#130" class="src">Source</a>

``` rust
pub type UInt8Array = PrimitiveArray<UInt8Type>;
```

Expand description

A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of `u8`

## <a href="https://docs.rs/arrow/latest/arrow/array/array/type.UInt8Array.html#examples" class="doc-anchor">§</a>Examples

Construction

``` rust
// Create from Vec<Option<u8>>
let arr = UInt8Array::from(vec![Some(1), None, Some(2)]);
// Create from Vec<u8>
let arr = UInt8Array::from(vec![1, 2, 3]);
// Create iter/collect
let arr: UInt8Array = std::iter::repeat(42).take(10).collect();
```

See [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") for more information and examples

## Aliased Type<a href="https://docs.rs/arrow/latest/arrow/array/array/type.UInt8Array.html#aliased-type" class="anchor">§</a>

``` rust
pub struct UInt8Array { /* private fields */ }
```
