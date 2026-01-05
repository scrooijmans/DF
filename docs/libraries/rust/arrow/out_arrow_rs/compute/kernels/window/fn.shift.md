# Function shift Copy item path

<a href="https://docs.rs/arrow-select/56.2.0/x86_64-unknown-linux-gnu/src/arrow_select/window.rs.html#55" class="src">Source</a>

``` rust
pub fn shift(
    array: &dyn Array,
    offset: i64,
) -> Result<Arc<dyn Array>, ArrowError>
```

Expand description

Shifts array by defined number of items (to left or right) A positive value for `offset` shifts the array to the right a negative value shifts the array to the left.

## <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/window/fn.shift.html#examples" class="doc-anchor">§</a>Examples

``` rust

let a: Int32Array = vec![Some(1), None, Some(4)].into();

// shift array 1 element to the right
let res = shift(&a, 1).unwrap();
let expected: Int32Array = vec![None, Some(1), None].into();
assert_eq!(res.as_ref(), &expected);

// shift array 1 element to the left
let res = shift(&a, -1).unwrap();
let expected: Int32Array = vec![None, Some(4), None].into();
assert_eq!(res.as_ref(), &expected);

// shift array 0 element, although not recommended
let res = shift(&a, 0).unwrap();
let expected: Int32Array = vec![Some(1), None, Some(4)].into();
assert_eq!(res.as_ref(), &expected);

// shift array 3 element to the right
let res = shift(&a, 3).unwrap();
let expected: Int32Array = vec![None, None, None].into();
assert_eq!(res.as_ref(), &expected);
```
