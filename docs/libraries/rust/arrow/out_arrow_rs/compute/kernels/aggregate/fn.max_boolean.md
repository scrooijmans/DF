# Function max_boolean Copy item path

<a href="https://docs.rs/arrow-arith/56.2.0/x86_64-unknown-linux-gnu/src/arrow_arith/aggregate.rs.html#404" class="src">Source</a>

``` rust
pub fn max_boolean(array: &BooleanArray) -> Option<bool>
```

Expand description

Returns the maximum value in the boolean array

``` rust

let a = BooleanArray::from(vec![Some(true), None, Some(false)]);
assert_eq!(max_boolean(&a), Some(true))
```
