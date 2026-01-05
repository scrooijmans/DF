# Type Alias IntervalDayTimeArray Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/array/primitive_array.rs.html#383" class="src">Source</a>

``` rust
pub type IntervalDayTimeArray = PrimitiveArray<IntervalDayTimeType>;
```

Expand description

A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of “calendar” intervals in days and milliseconds

See [`IntervalDayTime`](https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalDayTime.html "struct arrow::datatypes::IntervalDayTime") for details on representation and caveats.

## <a href="https://docs.rs/arrow/latest/arrow/array/array/type.IntervalDayTimeArray.html#example" class="doc-anchor">§</a>Example

``` rust
use arrow_array::types::IntervalDayTime;
let array = IntervalDayTimeArray::from(vec![
  IntervalDayTime::new(1, 1000),                 // 1 day, 1000 milliseconds
  IntervalDayTime::new(33, 0),                  // 33 days, 0 milliseconds
  IntervalDayTime::new(0, 12 * 60 * 60 * 1000), // 0 days, 12 hours
]);
```

## Aliased Type<a href="https://docs.rs/arrow/latest/arrow/array/array/type.IntervalDayTimeArray.html#aliased-type" class="anchor">§</a>

``` rust
pub struct IntervalDayTimeArray { /* private fields */ }
```
