# Function bool_and Copy item path

<a href="https://docs.rs/arrow-arith/56.2.0/x86_64-unknown-linux-gnu/src/arrow_arith/aggregate.rs.html#754" class="src">Source</a>

``` rust
pub fn bool_and(array: &BooleanArray) -> Option<bool>
```

Expand description

Returns true if all non-null input values are true, otherwise false.

Returns `None` if the array is empty or only contains null values.
