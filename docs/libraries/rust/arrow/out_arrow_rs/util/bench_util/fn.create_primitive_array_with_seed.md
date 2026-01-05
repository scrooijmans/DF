# Function create_primitive_array_with_seed Copy item path

<a href="https://docs.rs/arrow/latest/src/arrow/util/bench_util.rs.html#56-76" class="src">Source</a>

``` rust
pub fn create_primitive_array_with_seed<T>(
    size: usize,
    null_density: f32,
    seed: u64,
) -> PrimitiveArray<T>where
    T: ArrowPrimitiveType,
    StandardUniform: Distribution<T::Native>,
```

Available on **crate feature `test_utils`** only.

Expand description

Creates a [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of a given `size` and `null_density` filling it with random numbers generated using the provided `seed`.
