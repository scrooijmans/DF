# Function create_primitive_array Copy item path

<a href="https://docs.rs/arrow/latest/src/arrow/util/bench_util.rs.html#36-52" class="src">Source</a>

``` rust
pub fn create_primitive_array<T>(
    size: usize,
    null_density: f32,
) -> PrimitiveArray<T>where
    T: ArrowPrimitiveType,
    StandardUniform: Distribution<T::Native>,
```

Available on **crate feature `test_utils`** only.

Expand description

Creates an random (but fixed-seeded) array of a given size and null density
