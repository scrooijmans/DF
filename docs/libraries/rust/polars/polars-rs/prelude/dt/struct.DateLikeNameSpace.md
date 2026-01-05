# Struct DateLikeNameSpace Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/dt.rs.html#4" class="src">Source</a>

``` rust
pub struct DateLikeNameSpace(/* private fields */);
```

Available on **crate feature `lazy`** only.

Expand description

Specialized expressions for [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") with dates/datetimes.

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/dt/struct.DateLikeNameSpace.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/dt/struct.DateLikeNameSpace.html#impl-DateLikeNameSpace" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/dt/struct.DateLikeNameSpace.html" class="struct" title="struct polars::prelude::dt::DateLikeNameSpace">DateLikeNameSpace</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/dt/struct.DateLikeNameSpace.html#method.to_string" class="fn">to_string</a>(self, format: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Convert from Date/Time/Datetime into String with the given format. See [chrono strftime/strptime](https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html).

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/dt/struct.DateLikeNameSpace.html#method.strftime" class="fn">strftime</a>(self, format: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Convert from Date/Time/Datetime into String with the given format. See [chrono strftime/strptime](https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html).

Alias for `to_string`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/dt/struct.DateLikeNameSpace.html#method.cast_time_unit" class="fn">cast_time_unit</a>(self, tu: <a href="https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html" class="enum" title="enum polars::prelude::TimeUnit">TimeUnit</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Change the underlying [`TimeUnit`](https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html "enum polars::prelude::TimeUnit"). And update the data accordingly.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/dt/struct.DateLikeNameSpace.html#method.with_time_unit" class="fn">with_time_unit</a>(self, tu: <a href="https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html" class="enum" title="enum polars::prelude::TimeUnit">TimeUnit</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Change the underlying [`TimeUnit`](https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html "enum polars::prelude::TimeUnit") of the [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series"). This does not modify the data.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/dt/struct.DateLikeNameSpace.html#method.convert_time_zone" class="fn">convert_time_zone</a>(self, time_zone: <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeZone.html" class="struct" title="struct polars::prelude::TimeZone">TimeZone</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Change the underlying [`TimeZone`](https://docs.rs/polars/latest/polars/prelude/struct.TimeZone.html "struct polars::prelude::TimeZone") of the [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series"). This does not modify the data.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/dt/struct.DateLikeNameSpace.html#method.millennium" class="fn">millennium</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Get the millennium of a Date/Datetime

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/dt/struct.DateLikeNameSpace.html#method.century" class="fn">century</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Get the century of a Date/Datetime

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/dt/struct.DateLikeNameSpace.html#method.year" class="fn">year</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Get the year of a Date/Datetime

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/dt/struct.DateLikeNameSpace.html#method.is_leap_year" class="fn">is_leap_year</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/dt/struct.DateLikeNameSpace.html#method.iso_year" class="fn">iso_year</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Get the iso-year of a Date/Datetime. This may not correspond with a calendar year.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/dt/struct.DateLikeNameSpace.html#method.month" class="fn">month</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Get the month of a Date/Datetime.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/dt/struct.DateLikeNameSpace.html#method.days_in_month" class="fn">days_in_month</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Get the number of days in the month of a Date/Datetime.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/dt/struct.DateLikeNameSpace.html#method.quarter" class="fn">quarter</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Extract quarter from underlying NaiveDateTime representation. Quarters range from 1 to 4.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/dt/struct.DateLikeNameSpace.html#method.week" class="fn">week</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Extract the week from the underlying Date representation. Can be performed on Date and Datetime

Returns the ISO week number starting from 1. The return value ranges from 1 to 53. (The last week of year differs by years.)

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/dt/struct.DateLikeNameSpace.html#method.weekday" class="fn">weekday</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Extract the ISO week day from the underlying Date representation. Can be performed on Date and Datetime.

Returns the weekday number where monday = 1 and sunday = 7

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/dt/struct.DateLikeNameSpace.html#method.day" class="fn">day</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Get the month of a Date/Datetime.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/dt/struct.DateLikeNameSpace.html#method.ordinal_day" class="fn">ordinal_day</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Get the ordinal_day of a Date/Datetime.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/dt/struct.DateLikeNameSpace.html#method.time" class="fn">time</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Get the (local) time of a Date/Datetime/Time.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/dt/struct.DateLikeNameSpace.html#method.date" class="fn">date</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Get the (local) date of a Date/Datetime.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/dt/struct.DateLikeNameSpace.html#method.datetime" class="fn">datetime</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Get the (local) datetime of a Datetime.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/dt/struct.DateLikeNameSpace.html#method.hour" class="fn">hour</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Get the hour of a Datetime/Time64.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/dt/struct.DateLikeNameSpace.html#method.minute" class="fn">minute</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Get the minute of a Datetime/Time64.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/dt/struct.DateLikeNameSpace.html#method.second" class="fn">second</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Get the second of a Datetime/Time64.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/dt/struct.DateLikeNameSpace.html#method.millisecond" class="fn">millisecond</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Get the millisecond of a Time64 (scaled from nanosecs).

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/dt/struct.DateLikeNameSpace.html#method.microsecond" class="fn">microsecond</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Get the microsecond of a Time64 (scaled from nanosecs).

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/dt/struct.DateLikeNameSpace.html#method.nanosecond" class="fn">nanosecond</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Get the nanosecond part of a Time64.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/dt/struct.DateLikeNameSpace.html#method.timestamp" class="fn">timestamp</a>(self, tu: <a href="https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html" class="enum" title="enum polars::prelude::TimeUnit">TimeUnit</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Return the timestamp (UNIX epoch) of a Datetime/Date.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/dt/struct.DateLikeNameSpace.html#method.truncate" class="fn">truncate</a>(self, every: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Truncate the Datetime/Date range into buckets.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/dt/struct.DateLikeNameSpace.html#method.base_utc_offset" class="fn">base_utc_offset</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Get the base offset from UTC.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/dt/struct.DateLikeNameSpace.html#method.dst_offset" class="fn">dst_offset</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Get the additional offset from UTC currently in effect (usually due to daylight saving time).

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/dt/struct.DateLikeNameSpace.html#method.round" class="fn">round</a>(self, every: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Round the Datetime/Date range into buckets.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/dt/struct.DateLikeNameSpace.html#method.replace_time_zone" class="fn">replace_time_zone</a>( self, time_zone: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeZone.html" class="struct" title="struct polars::prelude::TimeZone">TimeZone</a>\>, ambiguous: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, non_existent: <a href="https://docs.rs/polars/latest/polars/prelude/enum.NonExistent.html" class="enum" title="enum polars::prelude::NonExistent">NonExistent</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/dt/struct.DateLikeNameSpace.html#method.combine" class="fn">combine</a>(self, time: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, tu: <a href="https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html" class="enum" title="enum polars::prelude::TimeUnit">TimeUnit</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Combine an existing Date/Datetime with a Time, creating a new Datetime value.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/dt/struct.DateLikeNameSpace.html#method.total_days" class="fn">total_days</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Express a Duration in terms of its total number of integer days.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/dt/struct.DateLikeNameSpace.html#method.total_hours" class="fn">total_hours</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Express a Duration in terms of its total number of integer hours.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/dt/struct.DateLikeNameSpace.html#method.total_minutes" class="fn">total_minutes</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Express a Duration in terms of its total number of integer minutes.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/dt/struct.DateLikeNameSpace.html#method.total_seconds" class="fn">total_seconds</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Express a Duration in terms of its total number of integer seconds.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/dt/struct.DateLikeNameSpace.html#method.total_milliseconds" class="fn">total_milliseconds</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Express a Duration in terms of its total number of milliseconds.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/dt/struct.DateLikeNameSpace.html#method.total_microseconds" class="fn">total_microseconds</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Express a Duration in terms of its total number of microseconds.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/dt/struct.DateLikeNameSpace.html#method.total_nanoseconds" class="fn">total_nanoseconds</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Express a Duration in terms of its total number of nanoseconds.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/dt/struct.DateLikeNameSpace.html#method.replace" class="fn">replace</a>( self, year: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, month: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, day: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, hour: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, minute: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, second: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, microsecond: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, ambiguous: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Replace the time units of a value

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/dt/struct.DateLikeNameSpace.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/dt/struct.DateLikeNameSpace.html#blanket-implementations" class="anchor">§</a>
