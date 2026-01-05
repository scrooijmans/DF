# Trait DatetimeMethods Copy item path

<a href="https://docs.rs/polars-time/0.51.0/x86_64-unknown-linux-gnu/src/polars_time/chunkedarray/datetime.rs.html#33" class="src">Source</a>

``` rust
pub trait DatetimeMethods: AsDatetime {
Show 16 methods    // Provided methods
    fn year(&self) -> ChunkedArray<Int32Type> { ... }
    fn is_leap_year(&self) -> ChunkedArray<BooleanType> { ... }
    fn iso_year(&self) -> ChunkedArray<Int32Type> { ... }
    fn quarter(&self) -> ChunkedArray<Int8Type> { ... }
    fn month(&self) -> ChunkedArray<Int8Type> { ... }
    fn days_in_month(&self) -> ChunkedArray<Int8Type> { ... }
    fn weekday(&self) -> ChunkedArray<Int8Type> { ... }
    fn week(&self) -> ChunkedArray<Int8Type> { ... }
    fn day(&self) -> ChunkedArray<Int8Type> { ... }
    fn hour(&self) -> ChunkedArray<Int8Type> { ... }
    fn minute(&self) -> ChunkedArray<Int8Type> { ... }
    fn second(&self) -> ChunkedArray<Int8Type> { ... }
    fn nanosecond(&self) -> ChunkedArray<Int32Type> { ... }
    fn ordinal(&self) -> ChunkedArray<Int16Type> { ... }
    fn parse_from_str_slice(
        name: PlSmallStr,
        v: &[&str],
        fmt: &str,
        tu: TimeUnit,
    ) -> Logical<DatetimeType, Int64Type> { ... }
    fn new_from_parts(
        year: &ChunkedArray<Int32Type>,
        month: &ChunkedArray<Int8Type>,
        day: &ChunkedArray<Int8Type>,
        hour: &ChunkedArray<Int8Type>,
        minute: &ChunkedArray<Int8Type>,
        second: &ChunkedArray<Int8Type>,
        nanosecond: &ChunkedArray<Int32Type>,
        ambiguous: &ChunkedArray<StringType>,
        time_unit: &TimeUnit,
        time_zone: Option<TimeZone>,
        name: PlSmallStr,
    ) -> Result<Logical<DatetimeType, Int64Type>, PolarsError> { ... }
}
```

Available on **crate feature `temporal`** only.

## Provided Methods<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.DatetimeMethods.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.DatetimeMethods.html#method.year" class="fn">year</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>

Extract month from underlying NaiveDateTime representation. Returns the year number in the calendar date.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.DatetimeMethods.html#method.is_leap_year" class="fn">is_leap_year</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

Extract year from underlying NaiveDate representation. Returns whether the year is a leap year.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.DatetimeMethods.html#method.iso_year" class="fn">iso_year</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.DatetimeMethods.html#method.quarter" class="fn">quarter</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>

Extract quarter from underlying NaiveDateTime representation. Quarters range from 1 to 4.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.DatetimeMethods.html#method.month" class="fn">month</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>

Extract month from underlying NaiveDateTime representation. Returns the month number starting from 1.

The return value ranges from 1 to 12.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.DatetimeMethods.html#method.days_in_month" class="fn">days_in_month</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>

Returns the number of days in the month of the underlying NaiveDateTime representation.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.DatetimeMethods.html#method.weekday" class="fn">weekday</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>

Extract ISO weekday from underlying NaiveDateTime representation. Returns the weekday number where monday = 1 and sunday = 7

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.DatetimeMethods.html#method.week" class="fn">week</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>

Returns the ISO week number starting from 1. The return value ranges from 1 to 53. (The last week of year differs by years.)

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.DatetimeMethods.html#method.day" class="fn">day</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>

Extract day from underlying NaiveDateTime representation. Returns the day of month starting from 1.

The return value ranges from 1 to 31. (The last day of month differs by months.)

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.DatetimeMethods.html#method.hour" class="fn">hour</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>

Extract hour from underlying NaiveDateTime representation. Returns the hour number from 0 to 23.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.DatetimeMethods.html#method.minute" class="fn">minute</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>

Extract minute from underlying NaiveDateTime representation. Returns the minute number from 0 to 59.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.DatetimeMethods.html#method.second" class="fn">second</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>

Extract second from underlying NaiveDateTime representation. Returns the second number from 0 to 59.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.DatetimeMethods.html#method.nanosecond" class="fn">nanosecond</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>

Extract second from underlying NaiveDateTime representation. Returns the number of nanoseconds since the whole non-leap second. The range from 1,000,000,000 to 1,999,999,999 represents the leap second.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.DatetimeMethods.html#method.ordinal" class="fn">ordinal</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int16Type.html" class="struct" title="struct polars::prelude::Int16Type">Int16Type</a>\>

Returns the day of year starting from 1.

The return value ranges from 1 to 366. (The last day of year differs by years.)

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.DatetimeMethods.html#method.parse_from_str_slice" class="fn">parse_from_str_slice</a>( name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: &\[&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\], fmt: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, tu: <a href="https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html" class="enum" title="enum polars::prelude::TimeUnit">TimeUnit</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeType.html" class="struct" title="struct polars::prelude::DatetimeType">DatetimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.DatetimeMethods.html#method.new_from_parts" class="fn">new_from_parts</a>( year: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>, month: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>, day: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>, hour: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>, minute: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>, second: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>, nanosecond: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>, ambiguous: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>, time_unit: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html" class="enum" title="enum polars::prelude::TimeUnit">TimeUnit</a>, time_zone: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeZone.html" class="struct" title="struct polars::prelude::TimeZone">TimeZone</a>\>, name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeType.html" class="struct" title="struct polars::prelude::DatetimeType">DatetimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Construct a datetime ChunkedArray from individual time components.

## Dyn Compatibility<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.DatetimeMethods.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.DatetimeMethods.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.DatetimeMethods.html#impl-DatetimeMethods-for-Logical%3CDatetimeType,+Int64Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.DatetimeMethods.html" class="trait" title="trait polars::prelude::DatetimeMethods">DatetimeMethods</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeType.html" class="struct" title="struct polars::prelude::DatetimeType">DatetimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>
