# Function nullif Copy item path

<a href="https://docs.rs/arrow-select/56.2.0/x86_64-unknown-linux-gnu/src/arrow_select/nullif.rs.html#44" class="src">Source</a>

``` rust
pub fn nullif(
    left: &dyn Array,
    right: &BooleanArray,
) -> Result<Arc<dyn Array>, ArrowError>
```

Expand description

Returns a new array with the same values and the validity bit to false where the corresponding element of`right` is true.

This can be used to implement SQL `NULLIF`

## <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/nullif/fn.nullif.html#example" class="doc-anchor">§</a>Example

``` rust
// input is [null, 8, 1, 9]
let a = Int32Array::from(vec![None, Some(8), Some(1), Some(9)]);
// use nullif to set index 1 to null
let bool_array = BooleanArray::from(vec![Some(false), Some(true), Some(false), None]);
let nulled = nullif(&a, &bool_array).unwrap();
// The resulting array is [null, null, 1, 9]
assert_eq!(nulled.as_primitive(), &Int32Array::from(vec![None, None, Some(1), Some(9)]));
```
