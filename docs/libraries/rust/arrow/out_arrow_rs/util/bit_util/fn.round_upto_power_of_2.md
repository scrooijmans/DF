# Function round_upto_power_of_2 Copy item path

<a href="https://docs.rs/arrow-buffer/56.2.0/x86_64-unknown-linux-gnu/src/arrow_buffer/util/bit_util.rs.html#29" class="src">Source</a>

``` rust
pub fn round_upto_power_of_2(num: usize, factor: usize) -> usize
```

Expand description

Returns the nearest multiple of `factor` that is `>=` than `num`. Here `factor` must be a power of 2.
