# Function binary_mut Copy item path

<a href="https://docs.rs/arrow-arith/56.2.0/x86_64-unknown-linux-gnu/src/arrow_arith/arity.rs.html#201-209" class="src">Source</a>

``` rust
pub fn binary_mut<T, U, F>(
    a: PrimitiveArray<T>,
    b: &PrimitiveArray<U>,
    op: F,
) -> Result<Result<PrimitiveArray<T>, ArrowError>, PrimitiveArray<T>>where
    T: ArrowPrimitiveType,
    U: ArrowPrimitiveType,
    F: Fn(<T as ArrowPrimitiveType>::Native, <U as ArrowPrimitiveType>::Native) -> <T as ArrowPrimitiveType>::Native,
```

Expand description

Applies a binary and infallible function to values in two arrays, replacing the values in the first array in place.

## <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/arity/fn.binary_mut.html#details" class="doc-anchor">§</a>Details

Given two arrays of length `len`, calls `op(a[i], b[i])` for `i` in `0..len`, modifying the [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") `a` in place, if possible.

If any index is null in either `a` or `b`, the corresponding index in the result will also be null.

## <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/arity/fn.binary_mut.html#buffer-reuse" class="doc-anchor">§</a>Buffer Reuse

If the underlying buffers in `a` are not shared with other arrays, mutates the underlying buffer in place, without allocating.

If the underlying buffer in `a` are shared, returns Err(self)

Like [`unary`](https://docs.rs/arrow/latest/arrow/compute/fn.unary.html "fn arrow::compute::unary") the provided function is evaluated for every index, ignoring validity. This is beneficial when the cost of the operation is low compared to the cost of branching, and especially when the operation can be vectorised, however, requires `op` to be infallible for all possible values of its inputs

## <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/arity/fn.binary_mut.html#errors" class="doc-anchor">§</a>Errors

- If the arrays have different lengths
- If the array is not mutable (see “Buffer Reuse”)

## <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/arity/fn.binary_mut.html#see-also" class="doc-anchor">§</a>See Also

- Documentation on [`PrimitiveArray::unary_mut`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html#method.unary_mut "method arrow::array::PrimitiveArray::unary_mut") for operating on [`ArrayRef`](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef").

## <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/arity/fn.binary_mut.html#example" class="doc-anchor">§</a>Example

``` rust
// compute a + b for each element
let a = Float32Array::from(vec![Some(5.1f32), None, Some(6.8)]);
let b = Int32Array::from(vec![Some(1), None, Some(2)]);
// compute a + b, updating the value in a in place if possible
let a = binary_mut(a, &b, |a, b| a + b as f32).unwrap().unwrap();
// a is updated in place
assert_eq!(a, Float32Array::from(vec![Some(6.1), None, Some(8.8)]));
```

## <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/arity/fn.binary_mut.html#example-with-shared-buffers" class="doc-anchor">§</a>Example with shared buffers

``` rust
let a = Float32Array::from(vec![Some(5.1f32), None, Some(6.8)]);
let b = Float32Array::from(vec![Some(1.0f32), None, Some(2.0)]);
// a_clone shares the buffer with a
let a_cloned = a.clone();
// try to update a in place, but it is shared. Returns Err(a)
let a = binary_mut(a, &b, |a, b| a + b).unwrap_err();
assert_eq!(a_cloned, a);
// drop shared reference
drop(a_cloned);
// now a is not shared, so we can update it in place
let a = binary_mut(a, &b, |a, b| a + b).unwrap().unwrap();
assert_eq!(a, Float32Array::from(vec![Some(6.1), None, Some(8.8)]));
```
