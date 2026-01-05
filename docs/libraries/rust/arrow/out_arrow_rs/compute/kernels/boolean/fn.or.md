# Function or Copy item path

<a href="https://docs.rs/arrow-arith/56.2.0/x86_64-unknown-linux-gnu/src/arrow_arith/boolean.rs.html#271" class="src">Source</a>

``` rust
pub fn or(
    left: &BooleanArray,
    right: &BooleanArray,
) -> Result<BooleanArray, ArrowError>
```

Expand description

Performs `OR` operation on two arrays. If either left or right value is null then the result is also null.

## <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/boolean/fn.or.html#error" class="doc-anchor">§</a>Error

This function errors when the arrays have different lengths.

## <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/boolean/fn.or.html#example" class="doc-anchor">§</a>Example

``` rust
let a = BooleanArray::from(vec![Some(false), Some(true), None]);
let b = BooleanArray::from(vec![Some(true), Some(true), Some(false)]);
let or_ab = or(&a, &b).unwrap();
assert_eq!(or_ab, BooleanArray::from(vec![Some(true), Some(true), None]));
```
