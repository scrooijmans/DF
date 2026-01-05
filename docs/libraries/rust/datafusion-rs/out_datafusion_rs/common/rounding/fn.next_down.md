# Function next_down Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/rounding.rs.html#209" class="src">Source</a>

``` rust
pub fn next_down<F>(float: F) -> Fwhere
    F: FloatBits + Copy,
```

Expand description

Returns the next representable floating-point value smaller than the input value.

This function takes a floating-point value that implements the FloatBits trait, calculates the next representable value smaller than the input, and returns it.

If the input value is NaN or negative infinity, the function returns the input value.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/rounding/fn.next_down.html#examples" class="doc-anchor">§</a>Examples

``` rust
use datafusion_common::rounding::next_down;

let f: f32 = 1.0;
let next_f = next_down(f);
assert_eq!(next_f, 0.99999994);
```
