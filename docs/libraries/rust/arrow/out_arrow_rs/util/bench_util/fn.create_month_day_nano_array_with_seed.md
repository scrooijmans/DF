# Function create_month_day_nano_array_with_seed Copy item path

<a href="https://docs.rs/arrow/latest/src/arrow/util/bench_util.rs.html#80-100" class="src">Source</a>

``` rust
pub fn create_month_day_nano_array_with_seed(
    size: usize,
    null_density: f32,
    seed: u64,
) -> IntervalMonthDayNanoArray
```

Available on **crate feature `test_utils`** only.

Expand description

Creates a [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of a given `size` and `null_density` filling it with random [`IntervalMonthDayNano`](https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalMonthDayNano.html "struct arrow::datatypes::IntervalMonthDayNano") generated using the provided `seed`.
