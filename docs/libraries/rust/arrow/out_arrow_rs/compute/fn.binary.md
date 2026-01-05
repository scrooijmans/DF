# Function binary Copy item path

<a href="https://docs.rs/arrow-arith/56.2.0/x86_64-unknown-linux-gnu/src/arrow_arith/arity.rs.html#104-113" class="src">Source</a>

``` rust
pub fn binary<A, B, F, O>(
    a: &PrimitiveArray<A>,
    b: &PrimitiveArray<B>,
    op: F,
) -> Result<PrimitiveArray<O>, ArrowError>where
    A: ArrowPrimitiveType,
    B: ArrowPrimitiveType,
    O: ArrowPrimitiveType,
    F: Fn(<A as ArrowPrimitiveType>::Native, <B as ArrowPrimitiveType>::Native) -> <O as ArrowPrimitiveType>::Native,
```

Expand description

Allies a binary infallable function to two [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray")s, producing a new [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray")

## <a href="https://docs.rs/arrow/latest/arrow/compute/fn.binary.html#details" class="doc-anchor">§</a>Details

Given two arrays of length `len`, calls `op(a[i], b[i])` for `i` in `0..len`, collecting the results in a [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray").

If any index is null in either `a` or `b`, the corresponding index in the result will also be null

Like [`unary`](https://docs.rs/arrow/latest/arrow/compute/fn.unary.html "fn arrow::compute::unary"), the `op` is evaluated for every element in the two arrays, including those elements which are NULL. This is beneficial as the cost of the operation is low compared to the cost of branching, and especially when the operation can be vectorised, however, requires `op` to be infallible for all possible values of its inputs

## <a href="https://docs.rs/arrow/latest/arrow/compute/fn.binary.html#errors" class="doc-anchor">§</a>Errors

- if the arrays have different lengths.

## <a href="https://docs.rs/arrow/latest/arrow/compute/fn.binary.html#example" class="doc-anchor">§</a>Example

``` rust
let a = Float32Array::from(vec![Some(5.1f32), None, Some(6.8), Some(7.2)]);
let b = Int32Array::from(vec![1, 2, 4, 9]);
// compute int(a) + b for each element
let c = binary(&a, &b, |a, b| a as i32 + b).unwrap();
assert_eq!(c, Int32Array::from(vec![Some(6), None, Some(10), Some(16)]));
```
