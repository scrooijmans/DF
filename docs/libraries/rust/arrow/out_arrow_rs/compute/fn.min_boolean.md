# Function min_boolean Copy item path

<a href="https://docs.rs/arrow-arith/56.2.0/x86_64-unknown-linux-gnu/src/arrow_arith/aggregate.rs.html#346" class="src">Source</a>

``` rust
pub fn min_boolean(array: &BooleanArray) -> Option<bool>
```

Expand description

Returns the minimum value in the boolean array.

``` rust

let a = BooleanArray::from(vec![Some(true), None, Some(false)]);
assert_eq!(min_boolean(&a), Some(false))
```
