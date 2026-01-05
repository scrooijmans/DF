# Function is_not_null Copy item path

<a href="https://docs.rs/arrow-arith/56.2.0/x86_64-unknown-linux-gnu/src/arrow_arith/boolean.rs.html#345" class="src">Source</a>

``` rust
pub fn is_not_null(input: &dyn Array) -> Result<BooleanArray, ArrowError>
```

Expand description

Returns a non-null [BooleanArray](https://docs.rs/arrow/latest/arrow/array/struct.BooleanArray.html "struct arrow::array::BooleanArray") with whether each value of the array is not null.

## <a href="https://docs.rs/arrow/latest/arrow/compute/fn.is_not_null.html#error" class="doc-anchor">§</a>Error

This function never errors.

## <a href="https://docs.rs/arrow/latest/arrow/compute/fn.is_not_null.html#example" class="doc-anchor">§</a>Example

``` rust
let a = BooleanArray::from(vec![Some(false), Some(true), None]);
let a_is_not_null = is_not_null(&a).unwrap();
assert_eq!(a_is_not_null, BooleanArray::from(vec![true, true, false]));
```
