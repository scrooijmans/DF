# Trait TemporalMethods Copy item path

<a href="https://docs.rs/polars-time/0.51.0/x86_64-unknown-linux-gnu/src/polars_time/series/mod.rs.html#19" class="src">Source</a>

``` rust
pub trait TemporalMethods: AsSeries {
Show 20 methods    // Provided methods
    fn hour(&self) -> Result<ChunkedArray<Int8Type>, PolarsError> { ... }
    fn minute(&self) -> Result<ChunkedArray<Int8Type>, PolarsError> { ... }
    fn second(&self) -> Result<ChunkedArray<Int8Type>, PolarsError> { ... }
    fn nanosecond(&self) -> Result<ChunkedArray<Int32Type>, PolarsError> { ... }
    fn day(&self) -> Result<ChunkedArray<Int8Type>, PolarsError> { ... }
    fn weekday(&self) -> Result<ChunkedArray<Int8Type>, PolarsError> { ... }
    fn week(&self) -> Result<ChunkedArray<Int8Type>, PolarsError> { ... }
    fn ordinal_day(&self) -> Result<ChunkedArray<Int16Type>, PolarsError> { ... }
    fn millennium(&self) -> Result<ChunkedArray<Int32Type>, PolarsError> { ... }
    fn century(&self) -> Result<ChunkedArray<Int32Type>, PolarsError> { ... }
    fn year(&self) -> Result<ChunkedArray<Int32Type>, PolarsError> { ... }
    fn iso_year(&self) -> Result<ChunkedArray<Int32Type>, PolarsError> { ... }
    fn ordinal_year(&self) -> Result<ChunkedArray<Int32Type>, PolarsError> { ... }
    fn is_leap_year(&self) -> Result<ChunkedArray<BooleanType>, PolarsError> { ... }
    fn quarter(&self) -> Result<ChunkedArray<Int8Type>, PolarsError> { ... }
    fn month(&self) -> Result<ChunkedArray<Int8Type>, PolarsError> { ... }
    fn days_in_month(&self) -> Result<ChunkedArray<Int8Type>, PolarsError> { ... }
    fn to_string(&self, format: &str) -> Result<Series, PolarsError> { ... }
    fn strftime(&self, format: &str) -> Result<Series, PolarsError> { ... }
    fn timestamp(
        &self,
        tu: TimeUnit,
    ) -> Result<ChunkedArray<Int64Type>, PolarsError> { ... }
}
```

Available on **crate feature `temporal`** only.

## Provided Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.TemporalMethods.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.TemporalMethods.html#method.hour" class="fn">hour</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Extract hour from underlying NaiveDateTime representation. Returns the hour number from 0 to 23.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.TemporalMethods.html#method.minute" class="fn">minute</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Extract minute from underlying NaiveDateTime representation. Returns the minute number from 0 to 59.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.TemporalMethods.html#method.second" class="fn">second</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Extract second from underlying NaiveDateTime representation. Returns the second number from 0 to 59.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.TemporalMethods.html#method.nanosecond" class="fn">nanosecond</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Returns the number of nanoseconds since the whole non-leap second. The range from 1,000,000,000 to 1,999,999,999 represents the leap second.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.TemporalMethods.html#method.day" class="fn">day</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Extract day from underlying NaiveDateTime representation. Returns the day of month starting from 1.

The return value ranges from 1 to 31. (The last day of month differs by months.)

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.TemporalMethods.html#method.weekday" class="fn">weekday</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Returns the ISO weekday number where monday = 1 and sunday = 7

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.TemporalMethods.html#method.week" class="fn">week</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Returns the ISO week number starting from 1. The return value ranges from 1 to 53. (The last week of year differs by years.)

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.TemporalMethods.html#method.ordinal_day" class="fn">ordinal_day</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int16Type.html" class="struct" title="struct polars::prelude::Int16Type">Int16Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Returns the day of year starting from 1.

The return value ranges from 1 to 366. (The last day of year differs by years.)

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.TemporalMethods.html#method.millennium" class="fn">millennium</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Calculate the millennium from the underlying NaiveDateTime representation.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.TemporalMethods.html#method.century" class="fn">century</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Calculate the millennium from the underlying NaiveDateTime representation.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.TemporalMethods.html#method.year" class="fn">year</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Extract year from underlying NaiveDateTime representation. Returns the year number in the calendar date.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.TemporalMethods.html#method.iso_year" class="fn">iso_year</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.TemporalMethods.html#method.ordinal_year" class="fn">ordinal_year</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Extract ordinal year from underlying NaiveDateTime representation. Returns the year number in the calendar date.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.TemporalMethods.html#method.is_leap_year" class="fn">is_leap_year</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Extract year from underlying NaiveDateTime representation. Returns whether the year is a leap year.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.TemporalMethods.html#method.quarter" class="fn">quarter</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Extract quarter from underlying NaiveDateTime representation. Quarters range from 1 to 4.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.TemporalMethods.html#method.month" class="fn">month</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Extract month from underlying NaiveDateTime representation. Returns the month number starting from 1.

The return value ranges from 1 to 12.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.TemporalMethods.html#method.days_in_month" class="fn">days_in_month</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Returns the number of days in the month of the underlying NaiveDateTime representation.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.TemporalMethods.html#method.to_string" class="fn">to_string</a>(&self, format: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Convert Time into String with the given format. See [chrono strftime/strptime](https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html).

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.TemporalMethods.html#method.strftime" class="fn">strftime</a>(&self, format: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Convert from Time into String with the given format. See [chrono strftime/strptime](https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html).

Alias for `to_string`.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.TemporalMethods.html#method.timestamp" class="fn">timestamp</a>( &self, tu: <a href="https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html" class="enum" title="enum polars::prelude::TimeUnit">TimeUnit</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Convert date(time) object to timestamp in [`TimeUnit`](https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html "enum polars::prelude::TimeUnit").

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.TemporalMethods.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.TemporalMethods.html#impl-TemporalMethods-for-T" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.TemporalMethods.html" class="trait" title="trait polars::prelude::TemporalMethods">TemporalMethods</a> for T

where T: <a href="https://docs.rs/polars/latest/polars/prelude/series/trait.AsSeries.html" class="trait" title="trait polars::prelude::series::AsSeries">AsSeries</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,
