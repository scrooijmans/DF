# Function is_null Copy item path

<a href="https://docs.rs/arrow-arith/56.2.0/x86_64-unknown-linux-gnu/src/arrow_arith/boolean.rs.html#325" class="src">Source</a>

``` rust
pub fn is_null(input: &dyn Array) -> Result<BooleanArray, ArrowError>
```

Expand description

Returns a non-null [BooleanArray](https://docs.rs/arrow/latest/arrow/array/struct.BooleanArray.html "struct arrow::array::BooleanArray") with whether each value of the array is null.

## <a href="https://docs.rs/arrow/latest/arrow/compute/fn.is_null.html#error" class="doc-anchor">§</a>Error

This function never errors.

## <a href="https://docs.rs/arrow/latest/arrow/compute/fn.is_null.html#example" class="doc-anchor">§</a>Example

``` rust
let a = BooleanArray::from(vec![Some(false), Some(true), None]);
let a_is_null = is_null(&a).unwrap();
assert_eq!(a_is_null, BooleanArray::from(vec![false, false, true]));
```
