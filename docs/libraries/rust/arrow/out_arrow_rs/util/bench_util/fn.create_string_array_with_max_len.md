# Function create_string_array_with_max_len Copy item path

<a href="https://docs.rs/arrow/latest/src/arrow/util/bench_util.rs.html#231-249" class="src">Source</a>

``` rust
pub fn create_string_array_with_max_len<Offset: OffsetSizeTrait>(
    size: usize,
    null_density: f32,
    max_str_len: usize,
) -> GenericStringArray<Offset>
```

Available on **crate feature `test_utils`** only.

Expand description

Creates a random (but fixed-seeded) array of rand size with a given max size, null density and length
