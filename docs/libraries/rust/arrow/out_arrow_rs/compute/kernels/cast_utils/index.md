# Module cast_utils Copy item path

<a href="https://docs.rs/arrow-cast/56.2.0/x86_64-unknown-linux-gnu/src/arrow_cast/lib.rs.html#29" class="src">Source</a>

Expand description

[`Parser`](https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/trait.Parser.html "trait arrow::compute::kernels::cast_utils::Parser") implementations for converting strings to Arrow types

Used by the CSV and JSON readers to convert strings to Arrow types

## Structs<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/struct.IntervalParseConfig.html" class="struct" title="struct arrow::compute::kernels::cast_utils::IntervalParseConfig">IntervalParseConfig</a>  
Config to parse interval strings

## Enums<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/enum.IntervalUnit.html" class="enum" title="enum arrow::compute::kernels::cast_utils::IntervalUnit">IntervalUnit</a>  
Represents the units of an interval, with each variant corresponding to a bit in the interval’s bitfield representation

## Traits<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/trait.Parser.html" class="trait" title="trait arrow::compute::kernels::cast_utils::Parser">Parser</a>  
Specialized parsing implementations to convert strings to Arrow types.

## Functions<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/fn.parse_decimal.html" class="fn" title="fn arrow::compute::kernels::cast_utils::parse_decimal">parse_decimal</a>  
Parse the string format decimal value to i128/i256 format and checking the precision and scale. The result value can’t be out of bounds.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/fn.parse_interval_day_time.html" class="fn" title="fn arrow::compute::kernels::cast_utils::parse_interval_day_time">parse_interval_day_time</a>  
Parse human-readable interval string to Arrow [IntervalDayTimeType](https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalDayTimeType.html "struct arrow::datatypes::IntervalDayTimeType")

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/fn.parse_interval_month_day_nano.html" class="fn" title="fn arrow::compute::kernels::cast_utils::parse_interval_month_day_nano">parse_interval_month_day_nano</a>  
Parse human-readable interval string to Arrow [IntervalMonthDayNanoType](https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalMonthDayNanoType.html "struct arrow::datatypes::IntervalMonthDayNanoType")

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/fn.parse_interval_month_day_nano_config.html" class="fn" title="fn arrow::compute::kernels::cast_utils::parse_interval_month_day_nano_config">parse_interval_month_day_nano_config</a>  
Parse human-readable interval string to Arrow [IntervalMonthDayNanoType](https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalMonthDayNanoType.html "struct arrow::datatypes::IntervalMonthDayNanoType")

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/fn.parse_interval_year_month.html" class="fn" title="fn arrow::compute::kernels::cast_utils::parse_interval_year_month">parse_interval_year_month</a>  
Parse human-readable interval string to Arrow [IntervalYearMonthType](https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalYearMonthType.html "struct arrow::datatypes::IntervalYearMonthType")

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/fn.string_to_datetime.html" class="fn" title="fn arrow::compute::kernels::cast_utils::string_to_datetime">string_to_datetime</a>  
Accepts a string and parses it relative to the provided `timezone`

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/fn.string_to_time_nanoseconds.html" class="fn" title="fn arrow::compute::kernels::cast_utils::string_to_time_nanoseconds">string_to_time_nanoseconds</a>  
Accepts a string in ISO8601 standard format and some variants and converts it to nanoseconds since midnight.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/fn.string_to_timestamp_nanos.html" class="fn" title="fn arrow::compute::kernels::cast_utils::string_to_timestamp_nanos">string_to_timestamp_nanos</a>  
Accepts a string in RFC3339 / ISO8601 standard format and some variants and converts it to a nanosecond precision timestamp.

## Type Aliases<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/type.MonthDayNano.html" class="type" title="type arrow::compute::kernels::cast_utils::MonthDayNano">MonthDayNano</a>  
A tuple representing (months, days, nanoseconds) in an interval
