# Type Alias TimestampSecondArray Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/array/primitive_array.rs.html#301" class="src">Source</a>

``` rust
pub type TimestampSecondArray = PrimitiveArray<TimestampSecondType>;
```

Expand description

A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of seconds since UNIX epoch stored as `i64`

This type is similar to the [`chrono::DateTime`](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/datetime/struct.DateTime.html "struct chrono::datetime::DateTime") type and can hold values such as `1970-05-09 14:25:11 +01:00`

See also [`Timestamp`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Timestamp "variant arrow::datatypes::DataType::Timestamp").

## <a href="https://docs.rs/arrow/latest/arrow/array/type.TimestampSecondArray.html#example-utc-timestamps-post-epoch" class="doc-anchor">§</a>Example: UTC timestamps post epoch

``` rust
use arrow_array::timezone::Tz;
// Corresponds to single element array with entry 1970-05-09T14:25:11+0:00
let arr = TimestampSecondArray::from(vec![11111111]);
// OR
let arr = TimestampSecondArray::from(vec![Some(11111111)]);
let utc_tz: Tz = "+00:00".parse().unwrap();

assert_eq!(arr.value_as_datetime_with_tz(0, utc_tz).map(|v| v.to_string()).unwrap(), "1970-05-09 14:25:11 +00:00")
```

## <a href="https://docs.rs/arrow/latest/arrow/array/type.TimestampSecondArray.html#example-utc-timestamps-pre-epoch" class="doc-anchor">§</a>Example: UTC timestamps pre epoch

``` rust
use arrow_array::timezone::Tz;
// Corresponds to single element array with entry 1969-08-25T09:34:49+0:00
let arr = TimestampSecondArray::from(vec![-11111111]);
// OR
let arr = TimestampSecondArray::from(vec![Some(-11111111)]);
let utc_tz: Tz = "+00:00".parse().unwrap();

assert_eq!(arr.value_as_datetime_with_tz(0, utc_tz).map(|v| v.to_string()).unwrap(), "1969-08-25 09:34:49 +00:00")
```

## <a href="https://docs.rs/arrow/latest/arrow/array/type.TimestampSecondArray.html#example-with-timezone-specified" class="doc-anchor">§</a>Example: With timezone specified

``` rust
use arrow_array::timezone::Tz;
// Corresponds to single element array with entry 1970-05-10T00:25:11+10:00
let arr = TimestampSecondArray::from(vec![11111111]).with_timezone("+10:00".to_string());
// OR
let arr = TimestampSecondArray::from(vec![Some(11111111)]).with_timezone("+10:00".to_string());
let sydney_tz: Tz = "+10:00".parse().unwrap();

assert_eq!(arr.value_as_datetime_with_tz(0, sydney_tz).map(|v| v.to_string()).unwrap(), "1970-05-10 00:25:11 +10:00")
```

See [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") for more information and examples

## Aliased Type<a href="https://docs.rs/arrow/latest/arrow/array/type.TimestampSecondArray.html#aliased-type" class="anchor">§</a>

``` rust
pub struct TimestampSecondArray { /* private fields */ }
```
