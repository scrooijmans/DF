# Module temporal_conversions Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/lib.rs.html#258" class="src">Source</a>

Expand description

Conversion methods for dates and times.

## Constants<a href="https://docs.rs/arrow/latest/arrow/array/temporal_conversions/index.html#constants" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/temporal_conversions/constant.MICROSECONDS.html" class="constant" title="constant arrow::array::temporal_conversions::MICROSECONDS">MICROSECONDS</a>  
Number of microseconds in a second

<a href="https://docs.rs/arrow/latest/arrow/array/temporal_conversions/constant.MICROSECONDS_IN_DAY.html" class="constant" title="constant arrow::array::temporal_conversions::MICROSECONDS_IN_DAY">MICROSECONDS_IN_DAY</a>  
Number of microseconds in a day

<a href="https://docs.rs/arrow/latest/arrow/array/temporal_conversions/constant.MILLISECONDS.html" class="constant" title="constant arrow::array::temporal_conversions::MILLISECONDS">MILLISECONDS</a>  
Number of milliseconds in a second

<a href="https://docs.rs/arrow/latest/arrow/array/temporal_conversions/constant.MILLISECONDS_IN_DAY.html" class="constant" title="constant arrow::array::temporal_conversions::MILLISECONDS_IN_DAY">MILLISECONDS_IN_DAY</a>  
Number of milliseconds in a day

<a href="https://docs.rs/arrow/latest/arrow/array/temporal_conversions/constant.NANOSECONDS.html" class="constant" title="constant arrow::array::temporal_conversions::NANOSECONDS">NANOSECONDS</a>  
Number of nanoseconds in a second

<a href="https://docs.rs/arrow/latest/arrow/array/temporal_conversions/constant.NANOSECONDS_IN_DAY.html" class="constant" title="constant arrow::array::temporal_conversions::NANOSECONDS_IN_DAY">NANOSECONDS_IN_DAY</a>  
Number of nanoseconds in a day

<a href="https://docs.rs/arrow/latest/arrow/array/temporal_conversions/constant.SECONDS_IN_DAY.html" class="constant" title="constant arrow::array::temporal_conversions::SECONDS_IN_DAY">SECONDS_IN_DAY</a>  
Number of seconds in a day

<a href="https://docs.rs/arrow/latest/arrow/array/temporal_conversions/constant.UNIX_EPOCH_DAY.html" class="constant" title="constant arrow::array::temporal_conversions::UNIX_EPOCH_DAY">UNIX_EPOCH_DAY</a>  
Constant from chrono crate

## Functions<a href="https://docs.rs/arrow/latest/arrow/array/temporal_conversions/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/temporal_conversions/fn.as_date.html" class="fn" title="fn arrow::array::temporal_conversions::as_date">as_date</a>  
Converts an [`ArrowPrimitiveType`](https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html "trait arrow::array::ArrowPrimitiveType") to [`NaiveDate`](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/naive/date/struct.NaiveDate.html "struct chrono::naive::date::NaiveDate")

<a href="https://docs.rs/arrow/latest/arrow/array/temporal_conversions/fn.as_datetime.html" class="fn" title="fn arrow::array::temporal_conversions::as_datetime">as_datetime</a>  
Converts an [`ArrowPrimitiveType`](https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html "trait arrow::array::ArrowPrimitiveType") to [`NaiveDateTime`](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/naive/datetime/struct.NaiveDateTime.html "struct chrono::naive::datetime::NaiveDateTime")

<a href="https://docs.rs/arrow/latest/arrow/array/temporal_conversions/fn.as_datetime_with_timezone.html" class="fn" title="fn arrow::array::temporal_conversions::as_datetime_with_timezone">as_datetime_with_timezone</a>  
Converts an [`ArrowPrimitiveType`](https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html "trait arrow::array::ArrowPrimitiveType") to [`DateTime<Tz>`](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/datetime/struct.DateTime.html "struct chrono::datetime::DateTime")

<a href="https://docs.rs/arrow/latest/arrow/array/temporal_conversions/fn.as_duration.html" class="fn" title="fn arrow::array::temporal_conversions::as_duration">as_duration</a>  
Converts an [`ArrowPrimitiveType`](https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html "trait arrow::array::ArrowPrimitiveType") to [`Duration`](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/type.Duration.html "type chrono::Duration")

<a href="https://docs.rs/arrow/latest/arrow/array/temporal_conversions/fn.as_time.html" class="fn" title="fn arrow::array::temporal_conversions::as_time">as_time</a>  
Converts an [`ArrowPrimitiveType`](https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html "trait arrow::array::ArrowPrimitiveType") to [`NaiveTime`](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/naive/time/struct.NaiveTime.html "struct chrono::naive::time::NaiveTime")

<a href="https://docs.rs/arrow/latest/arrow/array/temporal_conversions/fn.date32_to_datetime.html" class="fn" title="fn arrow::array::temporal_conversions::date32_to_datetime">date32_to_datetime</a>  
converts a `i32` representing a `date32` to [`NaiveDateTime`](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/naive/datetime/struct.NaiveDateTime.html "struct chrono::naive::datetime::NaiveDateTime")

<a href="https://docs.rs/arrow/latest/arrow/array/temporal_conversions/fn.date64_to_datetime.html" class="fn" title="fn arrow::array::temporal_conversions::date64_to_datetime">date64_to_datetime</a>  
converts a `i64` representing a `date64` to [`NaiveDateTime`](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/naive/datetime/struct.NaiveDateTime.html "struct chrono::naive::datetime::NaiveDateTime")

<a href="https://docs.rs/arrow/latest/arrow/array/temporal_conversions/fn.duration_ms_to_duration.html" class="fn" title="fn arrow::array::temporal_conversions::duration_ms_to_duration">duration_ms_to_duration</a>Deprecated  
converts a `i64` representing a `duration(ms)` to [`Duration`](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/type.Duration.html "type chrono::Duration")

<a href="https://docs.rs/arrow/latest/arrow/array/temporal_conversions/fn.duration_ns_to_duration.html" class="fn" title="fn arrow::array::temporal_conversions::duration_ns_to_duration">duration_ns_to_duration</a>  
converts a `i64` representing a `duration(ns)` to [`Duration`](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/type.Duration.html "type chrono::Duration")

<a href="https://docs.rs/arrow/latest/arrow/array/temporal_conversions/fn.duration_s_to_duration.html" class="fn" title="fn arrow::array::temporal_conversions::duration_s_to_duration">duration_s_to_duration</a>Deprecated  
converts a `i64` representing a `duration(s)` to [`Duration`](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/type.Duration.html "type chrono::Duration")

<a href="https://docs.rs/arrow/latest/arrow/array/temporal_conversions/fn.duration_us_to_duration.html" class="fn" title="fn arrow::array::temporal_conversions::duration_us_to_duration">duration_us_to_duration</a>  
converts a `i64` representing a `duration(us)` to [`Duration`](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/type.Duration.html "type chrono::Duration")

<a href="https://docs.rs/arrow/latest/arrow/array/temporal_conversions/fn.time32ms_to_time.html" class="fn" title="fn arrow::array::temporal_conversions::time32ms_to_time">time32ms_to_time</a>  
converts a `i32` representing a `time32(ms)` to [`NaiveDateTime`](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/naive/datetime/struct.NaiveDateTime.html "struct chrono::naive::datetime::NaiveDateTime")

<a href="https://docs.rs/arrow/latest/arrow/array/temporal_conversions/fn.time32s_to_time.html" class="fn" title="fn arrow::array::temporal_conversions::time32s_to_time">time32s_to_time</a>  
converts a `i32` representing a `time32(s)` to [`NaiveDateTime`](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/naive/datetime/struct.NaiveDateTime.html "struct chrono::naive::datetime::NaiveDateTime")

<a href="https://docs.rs/arrow/latest/arrow/array/temporal_conversions/fn.time64ns_to_time.html" class="fn" title="fn arrow::array::temporal_conversions::time64ns_to_time">time64ns_to_time</a>  
converts a `i64` representing a `time64(ns)` to [`NaiveDateTime`](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/naive/datetime/struct.NaiveDateTime.html "struct chrono::naive::datetime::NaiveDateTime")

<a href="https://docs.rs/arrow/latest/arrow/array/temporal_conversions/fn.time64us_to_time.html" class="fn" title="fn arrow::array::temporal_conversions::time64us_to_time">time64us_to_time</a>  
converts a `i64` representing a `time64(us)` to [`NaiveDateTime`](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/naive/datetime/struct.NaiveDateTime.html "struct chrono::naive::datetime::NaiveDateTime")

<a href="https://docs.rs/arrow/latest/arrow/array/temporal_conversions/fn.time_to_time32ms.html" class="fn" title="fn arrow::array::temporal_conversions::time_to_time32ms">time_to_time32ms</a>  
converts [`NaiveTime`](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/naive/time/struct.NaiveTime.html "struct chrono::naive::time::NaiveTime") to a `i32` representing a `time32(ms)`

<a href="https://docs.rs/arrow/latest/arrow/array/temporal_conversions/fn.time_to_time32s.html" class="fn" title="fn arrow::array::temporal_conversions::time_to_time32s">time_to_time32s</a>  
converts [`NaiveTime`](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/naive/time/struct.NaiveTime.html "struct chrono::naive::time::NaiveTime") to a `i32` representing a `time32(s)`

<a href="https://docs.rs/arrow/latest/arrow/array/temporal_conversions/fn.time_to_time64ns.html" class="fn" title="fn arrow::array::temporal_conversions::time_to_time64ns">time_to_time64ns</a>  
converts [`NaiveTime`](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/naive/time/struct.NaiveTime.html "struct chrono::naive::time::NaiveTime") to a `i64` representing a `time64(ns)`

<a href="https://docs.rs/arrow/latest/arrow/array/temporal_conversions/fn.time_to_time64us.html" class="fn" title="fn arrow::array::temporal_conversions::time_to_time64us">time_to_time64us</a>  
converts [`NaiveTime`](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/naive/time/struct.NaiveTime.html "struct chrono::naive::time::NaiveTime") to a `i64` representing a `time64(us)`

<a href="https://docs.rs/arrow/latest/arrow/array/temporal_conversions/fn.timestamp_ms_to_datetime.html" class="fn" title="fn arrow::array::temporal_conversions::timestamp_ms_to_datetime">timestamp_ms_to_datetime</a>  
converts a `i64` representing a `timestamp(ms)` to [`NaiveDateTime`](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/naive/datetime/struct.NaiveDateTime.html "struct chrono::naive::datetime::NaiveDateTime")

<a href="https://docs.rs/arrow/latest/arrow/array/temporal_conversions/fn.timestamp_ns_to_datetime.html" class="fn" title="fn arrow::array::temporal_conversions::timestamp_ns_to_datetime">timestamp_ns_to_datetime</a>  
converts a `i64` representing a `timestamp(ns)` to [`NaiveDateTime`](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/naive/datetime/struct.NaiveDateTime.html "struct chrono::naive::datetime::NaiveDateTime")

<a href="https://docs.rs/arrow/latest/arrow/array/temporal_conversions/fn.timestamp_s_to_date.html" class="fn" title="fn arrow::array::temporal_conversions::timestamp_s_to_date">timestamp_s_to_date</a>  
Similar to timestamp_s_to_datetime but only compute `date`

<a href="https://docs.rs/arrow/latest/arrow/array/temporal_conversions/fn.timestamp_s_to_datetime.html" class="fn" title="fn arrow::array::temporal_conversions::timestamp_s_to_datetime">timestamp_s_to_datetime</a>  
converts a `i64` representing a `timestamp(s)` to [`NaiveDateTime`](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/naive/datetime/struct.NaiveDateTime.html "struct chrono::naive::datetime::NaiveDateTime")

<a href="https://docs.rs/arrow/latest/arrow/array/temporal_conversions/fn.timestamp_s_to_time.html" class="fn" title="fn arrow::array::temporal_conversions::timestamp_s_to_time">timestamp_s_to_time</a>  
Similar to timestamp_s_to_datetime but only compute `time`

<a href="https://docs.rs/arrow/latest/arrow/array/temporal_conversions/fn.timestamp_us_to_datetime.html" class="fn" title="fn arrow::array::temporal_conversions::timestamp_us_to_datetime">timestamp_us_to_datetime</a>  
converts a `i64` representing a `timestamp(us)` to [`NaiveDateTime`](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/naive/datetime/struct.NaiveDateTime.html "struct chrono::naive::datetime::NaiveDateTime")

<a href="https://docs.rs/arrow/latest/arrow/array/temporal_conversions/fn.try_duration_ms_to_duration.html" class="fn" title="fn arrow::array::temporal_conversions::try_duration_ms_to_duration">try_duration_ms_to_duration</a>  
converts a `i64` representing a `duration(ms)` to [`Option<Duration>`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")

<a href="https://docs.rs/arrow/latest/arrow/array/temporal_conversions/fn.try_duration_s_to_duration.html" class="fn" title="fn arrow::array::temporal_conversions::try_duration_s_to_duration">try_duration_s_to_duration</a>  
converts a `i64` representing a `duration(s)` to [`Option<Duration>`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")
