# Function max_fixed_size_binary Copy item path

<a href="https://docs.rs/arrow-arith/56.2.0/x86_64-unknown-linux-gnu/src/arrow_arith/aggregate.rs.html#517" class="src">Source</a>

``` rust
pub fn max_fixed_size_binary(array: &FixedSizeBinaryArray) -> Option<&[u8]>
```

Expand description

Returns the maximum value in the fixed size binary array, according to the natural order.
