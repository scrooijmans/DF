# Type Alias Int8Array Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/array/primitive_array.rs.html#54" class="src">Source</a>

``` rust
pub type Int8Array = PrimitiveArray<Int8Type>;
```

Expand description

A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of `i8`

## <a href="https://docs.rs/arrow/latest/arrow/array/array/type.Int8Array.html#examples" class="doc-anchor">§</a>Examples

Construction

``` rust
// Create from Vec<Option<i8>>
let arr = Int8Array::from(vec![Some(1), None, Some(2)]);
// Create from Vec<i8>
let arr = Int8Array::from(vec![1, 2, 3]);
// Create iter/collect
let arr: Int8Array = std::iter::repeat(42).take(10).collect();
```

See [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") for more information and examples

## Aliased Type<a href="https://docs.rs/arrow/latest/arrow/array/array/type.Int8Array.html#aliased-type" class="anchor">§</a>

``` rust
pub struct Int8Array { /* private fields */ }
```
