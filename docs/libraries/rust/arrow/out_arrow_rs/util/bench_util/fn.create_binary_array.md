# Function create_binary_array Copy item path

<a href="https://docs.rs/arrow/latest/src/arrow/util/bench_util.rs.html#448-468" class="src">Source</a>

``` rust
pub fn create_binary_array<Offset: OffsetSizeTrait>(
    size: usize,
    null_density: f32,
) -> GenericBinaryArray<Offset>
```

Available on **crate feature `test_utils`** only.

Expand description

Creates an random (but fixed-seeded) binary array of a given size and null density
