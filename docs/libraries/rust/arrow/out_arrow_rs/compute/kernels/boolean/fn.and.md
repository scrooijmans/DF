# Function and Copy item path

<a href="https://docs.rs/arrow-arith/56.2.0/x86_64-unknown-linux-gnu/src/arrow_arith/boolean.rs.html#254" class="src">Source</a>

``` rust
pub fn and(
    left: &BooleanArray,
    right: &BooleanArray,
) -> Result<BooleanArray, ArrowError>
```

Expand description

Performs `AND` operation on two arrays. If either left or right value is null then the result is also null.

## <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/boolean/fn.and.html#error" class="doc-anchor">§</a>Error

This function errors when the arrays have different lengths.

## <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/boolean/fn.and.html#example" class="doc-anchor">§</a>Example

``` rust
let a = BooleanArray::from(vec![Some(false), Some(true), None]);
let b = BooleanArray::from(vec![Some(true), Some(true), Some(false)]);
let and_ab = and(&a, &b).unwrap();
assert_eq!(and_ab, BooleanArray::from(vec![Some(false), Some(true), None]));
```
