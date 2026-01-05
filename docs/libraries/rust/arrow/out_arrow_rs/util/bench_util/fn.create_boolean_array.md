# Function create_boolean_array Copy item path

<a href="https://docs.rs/arrow/latest/src/arrow/util/bench_util.rs.html#103-118" class="src">Source</a>

``` rust
pub fn create_boolean_array(
    size: usize,
    null_density: f32,
    true_density: f32,
) -> BooleanArraywhere
    StandardUniform: Distribution<bool>,
```

Available on **crate feature `test_utils`** only.

Expand description

Creates a random (but fixed-seeded) array of a given size and null density
