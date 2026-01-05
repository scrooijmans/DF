# Function create_fsb_array Copy item path

<a href="https://docs.rs/arrow/latest/src/arrow/util/bench_util.rs.html#471-489" class="src">Source</a>

``` rust
pub fn create_fsb_array(
    size: usize,
    null_density: f32,
    value_len: usize,
) -> FixedSizeBinaryArray
```

Available on **crate feature `test_utils`** only.

Expand description

Creates an random (but fixed-seeded) array of a given size and null density
