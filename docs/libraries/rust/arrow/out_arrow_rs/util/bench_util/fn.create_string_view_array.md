# Function create_string_view_array Copy item path

<a href="https://docs.rs/arrow/latest/src/arrow/util/bench_util.rs.html#275-277" class="src">Source</a>

``` rust
pub fn create_string_view_array(
    size: usize,
    null_density: f32,
) -> StringViewArray
```

Available on **crate feature `test_utils`** only.

Expand description

Creates a random (but fixed-seeded) string view array of a given size and null density.

See `create_string_array` above for more details.
