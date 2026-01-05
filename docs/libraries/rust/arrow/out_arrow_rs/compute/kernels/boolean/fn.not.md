# Function not Copy item path

<a href="https://docs.rs/arrow-arith/56.2.0/x86_64-unknown-linux-gnu/src/arrow_arith/boolean.rs.html#308" class="src">Source</a>

``` rust
pub fn not(left: &BooleanArray) -> Result<BooleanArray, ArrowError>
```

Expand description

Performs unary `NOT` operation on an arrays. If value is null then the result is also null.

## <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/boolean/fn.not.html#error" class="doc-anchor">§</a>Error

This function never errors. It returns an error for consistency.

## <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/boolean/fn.not.html#example" class="doc-anchor">§</a>Example

``` rust
let a = BooleanArray::from(vec![Some(false), Some(true), None]);
let not_a = not(&a).unwrap();
assert_eq!(not_a, BooleanArray::from(vec![Some(true), Some(false), None]));
```
