# Type Alias IntervalYearMonthArray Copy item path

<a href="https://docs.rs/arrow-array/56.0.0/x86_64-unknown-linux-gnu/src/arrow_array/array/primitive_array.rs.html#367" class="src">Source</a>

``` rust
pub type IntervalYearMonthArray = PrimitiveArray<IntervalYearMonthType>;
```

Expand description

A [`PrimitiveArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.PrimitiveArray.html "struct datafusion::common::arrow::array::PrimitiveArray") of “calendar” intervals in whole months

See [`IntervalYearMonthType`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalYearMonthType.html "struct datafusion::common::arrow::datatypes::IntervalYearMonthType") for details on representation and caveats.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.IntervalYearMonthArray.html#example" class="doc-anchor">§</a>Example

``` rust
let array = IntervalYearMonthArray::from(vec![
  2,  // 2 months
  25, // 2 years and 1 month
  -1  // -1 months
]);
```

## Aliased Type<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.IntervalYearMonthArray.html#aliased-type" class="anchor">§</a>

``` rust
pub struct IntervalYearMonthArray { /* private fields */ }
```
