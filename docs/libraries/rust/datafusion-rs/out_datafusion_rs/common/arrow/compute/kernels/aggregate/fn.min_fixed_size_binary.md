# Function min_fixed_size_binary Copy item path

<a href="https://docs.rs/arrow-arith/56.0.0/x86_64-unknown-linux-gnu/src/arrow_arith/aggregate.rs.html#532" class="src">Source</a>

``` rust
pub fn min_fixed_size_binary(array: &FixedSizeBinaryArray) -> Option<&[u8]>
```

Expand description

Returns the minimum value in the fixed size binary array, according to the natural order.
