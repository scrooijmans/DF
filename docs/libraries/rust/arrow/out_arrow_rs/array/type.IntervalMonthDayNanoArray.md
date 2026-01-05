# Type Alias IntervalMonthDayNanoArray Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/array/primitive_array.rs.html#399" class="src">Source</a>

``` rust
pub type IntervalMonthDayNanoArray = PrimitiveArray<IntervalMonthDayNanoType>;
```

Expand description

A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of “calendar” intervals in months, days, and nanoseconds.

See [`IntervalMonthDayNano`](https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalMonthDayNano.html "struct arrow::datatypes::IntervalMonthDayNano") for details on representation and caveats.

## <a href="https://docs.rs/arrow/latest/arrow/array/type.IntervalMonthDayNanoArray.html#example" class="doc-anchor">§</a>Example

``` rust
use arrow_array::types::IntervalMonthDayNano;
let array = IntervalMonthDayNanoArray::from(vec![
  IntervalMonthDayNano::new(1, 2, 1000),             // 1 month, 2 days, 1 nanosecond
  IntervalMonthDayNano::new(12, 1, 0),               // 12 months, 1 days, 0 nanoseconds
  IntervalMonthDayNano::new(0, 0, 12 * 1000 * 1000), // 0 days, 12 milliseconds
]);
```

## Aliased Type<a href="https://docs.rs/arrow/latest/arrow/array/type.IntervalMonthDayNanoArray.html#aliased-type" class="anchor">§</a>

``` rust
pub struct IntervalMonthDayNanoArray { /* private fields */ }
```
