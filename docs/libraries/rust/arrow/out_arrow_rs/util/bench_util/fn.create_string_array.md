# Function create_string_array Copy item path

<a href="https://docs.rs/arrow/latest/src/arrow/util/bench_util.rs.html#125-130" class="src">Source</a>

``` rust
pub fn create_string_array<Offset: OffsetSizeTrait>(
    size: usize,
    null_density: f32,
) -> GenericStringArray<Offset>
```

Available on **crate feature `test_utils`** only.

Expand description

Creates a random (but fixed-seeded) string array of a given size and null density.

Strings have a random length between 0 and 400 alphanumeric characters. `0..400` is chosen to cover a wide range of common string lengths, which have a dramatic impact on performance of some queries, e.g. LIKE/ILIKE/regex.
