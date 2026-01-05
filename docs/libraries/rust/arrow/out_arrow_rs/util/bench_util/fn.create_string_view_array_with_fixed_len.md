# Function create_string_view_array_with_fixed_len Copy item path

<a href="https://docs.rs/arrow/latest/src/arrow/util/bench_util.rs.html#301-318" class="src">Source</a>

``` rust
pub fn create_string_view_array_with_fixed_len(
    size: usize,
    null_density: f32,
    str_len: usize,
) -> StringViewArray
```

Available on **crate feature `test_utils`** only.

Expand description

Creates a random (but fixed-seeded) array of a given size, null density and length
