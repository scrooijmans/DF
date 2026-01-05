# Struct Tz Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/timezone.rs.html#80" class="src">Source</a>

``` rust
pub struct Tz(/* private fields */);
```

Expand description

An Arrow [`TimeZone`](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.TimeZone.html "trait chrono::offset::TimeZone")

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html#impl-Clone-for-Tz" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html" class="struct" title="struct arrow::array::timezone::Tz">Tz</a>

<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html" class="struct" title="struct arrow::array::timezone::Tz">Tz</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html#impl-Debug-for-Tz" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html" class="struct" title="struct arrow::array::timezone::Tz">Tz</a>

<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html#impl-Display-for-Tz" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html" class="struct" title="struct arrow::array::timezone::Tz">Tz</a>

<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html#impl-FromStr-for-Tz" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html" class="trait" title="trait core::str::traits::FromStr">FromStr</a> for <a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html" class="struct" title="struct arrow::array::timezone::Tz">Tz</a>

<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html#associatedtype.Err" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#associatedtype.Err" class="associatedtype">Err</a> = <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>

The associated error which can be returned from parsing.

<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html#method.from_str" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#tymethod.from_str" class="fn">from_str</a>(tz: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html" class="struct" title="struct arrow::array::timezone::Tz">Tz</a>, \<<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html" class="struct" title="struct arrow::array::timezone::Tz">Tz</a> as <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html" class="trait" title="trait core::str::traits::FromStr">FromStr</a>\>::<a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#associatedtype.Err" class="associatedtype" title="type core::str::traits::FromStr::Err">Err</a>\>

Parses a string `s` to return a value of this type. [Read more](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#tymethod.from_str)

<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html#impl-TimeZone-for-Tz" class="anchor">§</a>

### impl <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.TimeZone.html" class="trait" title="trait chrono::offset::TimeZone">TimeZone</a> for <a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html" class="struct" title="struct arrow::array::timezone::Tz">Tz</a>

<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html#associatedtype.Offset" class="anchor">§</a>

#### type <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.TimeZone.html#associatedtype.Offset" class="associatedtype">Offset</a> = <a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.TzOffset.html" class="struct" title="struct arrow::array::timezone::TzOffset">TzOffset</a>

An associated offset type. This type is used to store the actual offset in date and time types. The original `TimeZone` value can be recovered via `TimeZone::from_offset`.

<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html#method.from_offset" class="anchor">§</a>

#### fn <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.TimeZone.html#tymethod.from_offset" class="fn">from_offset</a>(offset: &\<<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html" class="struct" title="struct arrow::array::timezone::Tz">Tz</a> as <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.TimeZone.html" class="trait" title="trait chrono::offset::TimeZone">TimeZone</a>\>::<a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.TimeZone.html#associatedtype.Offset" class="associatedtype" title="type chrono::offset::TimeZone::Offset">Offset</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html" class="struct" title="struct arrow::array::timezone::Tz">Tz</a>

Reconstructs the time zone from the offset.

<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html#method.offset_from_local_date" class="anchor">§</a>

#### fn <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.TimeZone.html#tymethod.offset_from_local_date" class="fn">offset_from_local_date</a>( &self, local: &<a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/naive/date/struct.NaiveDate.html" class="struct" title="struct chrono::naive::date::NaiveDate">NaiveDate</a>, ) -\> <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/enum.LocalResult.html" class="enum" title="enum chrono::offset::LocalResult">LocalResult</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html" class="struct" title="struct arrow::array::timezone::Tz">Tz</a> as <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.TimeZone.html" class="trait" title="trait chrono::offset::TimeZone">TimeZone</a>\>::<a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.TimeZone.html#associatedtype.Offset" class="associatedtype" title="type chrono::offset::TimeZone::Offset">Offset</a>\>

Creates the offset(s) for given local `NaiveDate` if possible.

<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html#method.offset_from_local_datetime" class="anchor">§</a>

#### fn <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.TimeZone.html#tymethod.offset_from_local_datetime" class="fn">offset_from_local_datetime</a>( &self, local: &<a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/naive/datetime/struct.NaiveDateTime.html" class="struct" title="struct chrono::naive::datetime::NaiveDateTime">NaiveDateTime</a>, ) -\> <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/enum.LocalResult.html" class="enum" title="enum chrono::offset::LocalResult">LocalResult</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html" class="struct" title="struct arrow::array::timezone::Tz">Tz</a> as <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.TimeZone.html" class="trait" title="trait chrono::offset::TimeZone">TimeZone</a>\>::<a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.TimeZone.html#associatedtype.Offset" class="associatedtype" title="type chrono::offset::TimeZone::Offset">Offset</a>\>

Creates the offset(s) for given local `NaiveDateTime` if possible.

<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html#method.offset_from_utc_date" class="anchor">§</a>

#### fn <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.TimeZone.html#tymethod.offset_from_utc_date" class="fn">offset_from_utc_date</a>(&self, utc: &<a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/naive/date/struct.NaiveDate.html" class="struct" title="struct chrono::naive::date::NaiveDate">NaiveDate</a>) -\> \<<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html" class="struct" title="struct arrow::array::timezone::Tz">Tz</a> as <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.TimeZone.html" class="trait" title="trait chrono::offset::TimeZone">TimeZone</a>\>::<a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.TimeZone.html#associatedtype.Offset" class="associatedtype" title="type chrono::offset::TimeZone::Offset">Offset</a>

Creates the offset for given UTC `NaiveDate`. This cannot fail.

<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html#method.offset_from_utc_datetime" class="anchor">§</a>

#### fn <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.TimeZone.html#tymethod.offset_from_utc_datetime" class="fn">offset_from_utc_datetime</a>( &self, utc: &<a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/naive/datetime/struct.NaiveDateTime.html" class="struct" title="struct chrono::naive::datetime::NaiveDateTime">NaiveDateTime</a>, ) -\> \<<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html" class="struct" title="struct arrow::array::timezone::Tz">Tz</a> as <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.TimeZone.html" class="trait" title="trait chrono::offset::TimeZone">TimeZone</a>\>::<a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.TimeZone.html#associatedtype.Offset" class="associatedtype" title="type chrono::offset::TimeZone::Offset">Offset</a>

Creates the offset for given UTC `NaiveDateTime`. This cannot fail.

<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html#method.with_ymd_and_hms" class="anchor">§</a>

#### fn <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.TimeZone.html#method.with_ymd_and_hms" class="fn">with_ymd_and_hms</a>( &self, year: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, month: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, day: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, hour: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, min: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, sec: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, ) -\> <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/enum.LocalResult.html" class="enum" title="enum chrono::offset::LocalResult">LocalResult</a>\<<a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/datetime/struct.DateTime.html" class="struct" title="struct chrono::datetime::DateTime">DateTime</a>\<Self\>\>

Make a new `DateTime` from year, month, day, time components and current time zone. [Read more](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.TimeZone.html#method.with_ymd_and_hms)

<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html#method.ymd" class="anchor">§</a>

#### fn <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.TimeZone.html#method.ymd" class="fn">ymd</a>(&self, year: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, month: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, day: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/date/struct.Date.html" class="struct" title="struct chrono::date::Date">Date</a>\<Self\>

👎Deprecated since 0.4.23: use `with_ymd_and_hms()` instead

Makes a new `Date` from year, month, day and the current time zone. This assumes the proleptic Gregorian calendar, with the year 0 being 1 BCE. [Read more](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.TimeZone.html#method.ymd)

<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html#method.ymd_opt" class="anchor">§</a>

#### fn <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.TimeZone.html#method.ymd_opt" class="fn">ymd_opt</a>(&self, year: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, month: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, day: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/enum.LocalResult.html" class="enum" title="enum chrono::offset::LocalResult">LocalResult</a>\<<a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/date/struct.Date.html" class="struct" title="struct chrono::date::Date">Date</a>\<Self\>\>

👎Deprecated since 0.4.23: use `with_ymd_and_hms()` instead

Makes a new `Date` from year, month, day and the current time zone. This assumes the proleptic Gregorian calendar, with the year 0 being 1 BCE. [Read more](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.TimeZone.html#method.ymd_opt)

<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html#method.yo" class="anchor">§</a>

#### fn <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.TimeZone.html#method.yo" class="fn">yo</a>(&self, year: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, ordinal: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/date/struct.Date.html" class="struct" title="struct chrono::date::Date">Date</a>\<Self\>

👎Deprecated since 0.4.23: use `from_local_datetime()` with a `NaiveDateTime` instead

Makes a new `Date` from year, day of year (DOY or “ordinal”) and the current time zone. This assumes the proleptic Gregorian calendar, with the year 0 being 1 BCE. [Read more](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.TimeZone.html#method.yo)

<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html#method.yo_opt" class="anchor">§</a>

#### fn <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.TimeZone.html#method.yo_opt" class="fn">yo_opt</a>(&self, year: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, ordinal: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/enum.LocalResult.html" class="enum" title="enum chrono::offset::LocalResult">LocalResult</a>\<<a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/date/struct.Date.html" class="struct" title="struct chrono::date::Date">Date</a>\<Self\>\>

👎Deprecated since 0.4.23: use `from_local_datetime()` with a `NaiveDateTime` instead

Makes a new `Date` from year, day of year (DOY or “ordinal”) and the current time zone. This assumes the proleptic Gregorian calendar, with the year 0 being 1 BCE. [Read more](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.TimeZone.html#method.yo_opt)

<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html#method.isoywd" class="anchor">§</a>

#### fn <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.TimeZone.html#method.isoywd" class="fn">isoywd</a>(&self, year: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, week: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, weekday: <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/weekday/enum.Weekday.html" class="enum" title="enum chrono::weekday::Weekday">Weekday</a>) -\> <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/date/struct.Date.html" class="struct" title="struct chrono::date::Date">Date</a>\<Self\>

👎Deprecated since 0.4.23: use `from_local_datetime()` with a `NaiveDateTime` instead

Makes a new `Date` from ISO week date (year and week number), day of the week (DOW) and the current time zone. This assumes the proleptic Gregorian calendar, with the year 0 being 1 BCE. The resulting `Date` may have a different year from the input year. [Read more](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.TimeZone.html#method.isoywd)

<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html#method.isoywd_opt" class="anchor">§</a>

#### fn <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.TimeZone.html#method.isoywd_opt" class="fn">isoywd_opt</a>( &self, year: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, week: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, weekday: <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/weekday/enum.Weekday.html" class="enum" title="enum chrono::weekday::Weekday">Weekday</a>, ) -\> <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/enum.LocalResult.html" class="enum" title="enum chrono::offset::LocalResult">LocalResult</a>\<<a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/date/struct.Date.html" class="struct" title="struct chrono::date::Date">Date</a>\<Self\>\>

👎Deprecated since 0.4.23: use `from_local_datetime()` with a `NaiveDateTime` instead

Makes a new `Date` from ISO week date (year and week number), day of the week (DOW) and the current time zone. This assumes the proleptic Gregorian calendar, with the year 0 being 1 BCE. The resulting `Date` may have a different year from the input year. [Read more](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.TimeZone.html#method.isoywd_opt)

<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html#method.timestamp" class="anchor">§</a>

#### fn <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.TimeZone.html#method.timestamp" class="fn">timestamp</a>(&self, secs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, nsecs: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/datetime/struct.DateTime.html" class="struct" title="struct chrono::datetime::DateTime">DateTime</a>\<Self\>

👎Deprecated since 0.4.23: use `timestamp_opt()` instead

Makes a new `DateTime` from the number of non-leap seconds since January 1, 1970 0:00:00 UTC (aka “UNIX timestamp”) and the number of nanoseconds since the last whole non-leap second. [Read more](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.TimeZone.html#method.timestamp)

<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html#method.timestamp_opt" class="anchor">§</a>

#### fn <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.TimeZone.html#method.timestamp_opt" class="fn">timestamp_opt</a>(&self, secs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, nsecs: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/enum.LocalResult.html" class="enum" title="enum chrono::offset::LocalResult">LocalResult</a>\<<a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/datetime/struct.DateTime.html" class="struct" title="struct chrono::datetime::DateTime">DateTime</a>\<Self\>\>

Makes a new `DateTime` from the number of non-leap seconds since January 1, 1970 0:00:00 UTC (aka “UNIX timestamp”) and the number of nanoseconds since the last whole non-leap second. [Read more](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.TimeZone.html#method.timestamp_opt)

<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html#method.timestamp_millis" class="anchor">§</a>

#### fn <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.TimeZone.html#method.timestamp_millis" class="fn">timestamp_millis</a>(&self, millis: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/datetime/struct.DateTime.html" class="struct" title="struct chrono::datetime::DateTime">DateTime</a>\<Self\>

👎Deprecated since 0.4.23: use `timestamp_millis_opt()` instead

Makes a new `DateTime` from the number of non-leap milliseconds since January 1, 1970 0:00:00 UTC (aka “UNIX timestamp”). [Read more](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.TimeZone.html#method.timestamp_millis)

<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html#method.timestamp_millis_opt" class="anchor">§</a>

#### fn <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.TimeZone.html#method.timestamp_millis_opt" class="fn">timestamp_millis_opt</a>(&self, millis: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/enum.LocalResult.html" class="enum" title="enum chrono::offset::LocalResult">LocalResult</a>\<<a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/datetime/struct.DateTime.html" class="struct" title="struct chrono::datetime::DateTime">DateTime</a>\<Self\>\>

Makes a new `DateTime` from the number of non-leap milliseconds since January 1, 1970 0:00:00 UTC (aka “UNIX timestamp”). [Read more](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.TimeZone.html#method.timestamp_millis_opt)

<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html#method.timestamp_nanos" class="anchor">§</a>

#### fn <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.TimeZone.html#method.timestamp_nanos" class="fn">timestamp_nanos</a>(&self, nanos: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/datetime/struct.DateTime.html" class="struct" title="struct chrono::datetime::DateTime">DateTime</a>\<Self\>

Makes a new `DateTime` from the number of non-leap nanoseconds since January 1, 1970 0:00:00 UTC (aka “UNIX timestamp”). [Read more](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.TimeZone.html#method.timestamp_nanos)

<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html#method.timestamp_micros" class="anchor">§</a>

#### fn <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.TimeZone.html#method.timestamp_micros" class="fn">timestamp_micros</a>(&self, micros: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/enum.LocalResult.html" class="enum" title="enum chrono::offset::LocalResult">LocalResult</a>\<<a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/datetime/struct.DateTime.html" class="struct" title="struct chrono::datetime::DateTime">DateTime</a>\<Self\>\>

Makes a new `DateTime` from the number of non-leap microseconds since January 1, 1970 0:00:00 UTC (aka “UNIX timestamp”). [Read more](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.TimeZone.html#method.timestamp_micros)

<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html#method.datetime_from_str" class="anchor">§</a>

#### fn <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.TimeZone.html#method.datetime_from_str" class="fn">datetime_from_str</a>( &self, s: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, fmt: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/datetime/struct.DateTime.html" class="struct" title="struct chrono::datetime::DateTime">DateTime</a>\<Self\>, <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/format/struct.ParseError.html" class="struct" title="struct chrono::format::ParseError">ParseError</a>\>

👎Deprecated since 0.4.29: use `DateTime::parse_from_str` or `NaiveDateTime::parse_from_str` with `and_utc()` or `and_local_timezone()` instead

Parses a string with the specified format string and returns a `DateTime` with the current offset. [Read more](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.TimeZone.html#method.datetime_from_str)

<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html#method.from_local_date" class="anchor">§</a>

#### fn <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.TimeZone.html#method.from_local_date" class="fn">from_local_date</a>(&self, local: &<a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/naive/date/struct.NaiveDate.html" class="struct" title="struct chrono::naive::date::NaiveDate">NaiveDate</a>) -\> <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/enum.LocalResult.html" class="enum" title="enum chrono::offset::LocalResult">LocalResult</a>\<<a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/date/struct.Date.html" class="struct" title="struct chrono::date::Date">Date</a>\<Self\>\>

👎Deprecated since 0.4.23: use `from_local_datetime()` instead

Converts the local `NaiveDate` to the timezone-aware `Date` if possible.

<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html#method.from_local_datetime" class="anchor">§</a>

#### fn <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.TimeZone.html#method.from_local_datetime" class="fn">from_local_datetime</a>( &self, local: &<a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/naive/datetime/struct.NaiveDateTime.html" class="struct" title="struct chrono::naive::datetime::NaiveDateTime">NaiveDateTime</a>, ) -\> <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/enum.LocalResult.html" class="enum" title="enum chrono::offset::LocalResult">LocalResult</a>\<<a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/datetime/struct.DateTime.html" class="struct" title="struct chrono::datetime::DateTime">DateTime</a>\<Self\>\>

Converts the local `NaiveDateTime` to the timezone-aware `DateTime` if possible.

<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html#method.from_utc_date" class="anchor">§</a>

#### fn <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.TimeZone.html#method.from_utc_date" class="fn">from_utc_date</a>(&self, utc: &<a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/naive/date/struct.NaiveDate.html" class="struct" title="struct chrono::naive::date::NaiveDate">NaiveDate</a>) -\> <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/date/struct.Date.html" class="struct" title="struct chrono::date::Date">Date</a>\<Self\>

👎Deprecated since 0.4.23: use `from_utc_datetime()` instead

Converts the UTC `NaiveDate` to the local time. The UTC is continuous and thus this cannot fail (but can give the duplicate local time).

<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html#method.from_utc_datetime" class="anchor">§</a>

#### fn <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.TimeZone.html#method.from_utc_datetime" class="fn">from_utc_datetime</a>(&self, utc: &<a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/naive/datetime/struct.NaiveDateTime.html" class="struct" title="struct chrono::naive::datetime::NaiveDateTime">NaiveDateTime</a>) -\> <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/datetime/struct.DateTime.html" class="struct" title="struct chrono::datetime::DateTime">DateTime</a>\<Self\>

Converts the UTC `NaiveDateTime` to the local time. The UTC is continuous and thus this cannot fail (but can give the duplicate local time).

<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html#impl-Copy-for-Tz" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html" class="struct" title="struct arrow::array::timezone::Tz">Tz</a>

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html#blanket-implementations" class="anchor">§</a>
