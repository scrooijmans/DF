# Function create_f16_array Copy item path

<a href="https://docs.rs/arrow/latest/src/arrow/util/bench_util.rs.html#547-559" class="src">Source</a>

``` rust
pub fn create_f16_array(size: usize, nan_density: f32) -> Float16Array
```

Available on **crate feature `test_utils`** only.

Expand description

Creates a random (but fixed-seeded) f16 array of a given size and nan-value density
