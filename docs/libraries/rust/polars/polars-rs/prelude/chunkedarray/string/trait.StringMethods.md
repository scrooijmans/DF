# Trait StringMethods Copy item path

<a href="https://docs.rs/polars-time/0.51.0/x86_64-unknown-linux-gnu/src/polars_time/chunkedarray/string/mod.rs.html#82" class="src">Source</a>

``` rust
pub trait StringMethods: AsString {
    // Provided methods
    fn as_time(
        &self,
        fmt: Option<&str>,
        use_cache: bool,
    ) -> Result<Logical<TimeType, Int64Type>, PolarsError> { ... }
    fn as_date_not_exact(
        &self,
        fmt: Option<&str>,
    ) -> Result<Logical<DateType, Int32Type>, PolarsError> { ... }
    fn as_datetime_not_exact(
        &self,
        fmt: Option<&str>,
        tu: TimeUnit,
        tz_aware: bool,
        tz: Option<&TimeZone>,
        _ambiguous: &ChunkedArray<StringType>,
        ensure_matching_tz: bool,
    ) -> Result<Logical<DatetimeType, Int64Type>, PolarsError> { ... }
    fn as_date(
        &self,
        fmt: Option<&str>,
        use_cache: bool,
    ) -> Result<Logical<DateType, Int32Type>, PolarsError> { ... }
    fn as_datetime(
        &self,
        fmt: Option<&str>,
        tu: TimeUnit,
        use_cache: bool,
        tz_aware: bool,
        tz: Option<&TimeZone>,
        ambiguous: &ChunkedArray<StringType>,
    ) -> Result<Logical<DatetimeType, Int64Type>, PolarsError> { ... }
}
```

Available on **crate feature `temporal`** only.

## Provided Methods<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/trait.StringMethods.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/trait.StringMethods.html#method.as_time" class="fn">as_time</a>( &self, fmt: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>, use_cache: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeType.html" class="struct" title="struct polars::prelude::TimeType">TimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Parsing string values and return a [`TimeChunked`](https://docs.rs/polars/latest/polars/prelude/type.TimeChunked.html "type polars::prelude::TimeChunked")

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/trait.StringMethods.html#method.as_date_not_exact" class="fn">as_date_not_exact</a>( &self, fmt: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DateType.html" class="struct" title="struct polars::prelude::DateType">DateType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Parsing string values and return a [`DateChunked`](https://docs.rs/polars/latest/polars/prelude/type.DateChunked.html "type polars::prelude::DateChunked") Different from `as_date` this function allows matches that not contain the whole string e.g. “foo-2021-01-01-bar” could match “2021-01-01”

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/trait.StringMethods.html#method.as_datetime_not_exact" class="fn">as_datetime_not_exact</a>( &self, fmt: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>, tu: <a href="https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html" class="enum" title="enum polars::prelude::TimeUnit">TimeUnit</a>, tz_aware: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, tz: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeZone.html" class="struct" title="struct polars::prelude::TimeZone">TimeZone</a>\>, \_ambiguous: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>, ensure_matching_tz: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeType.html" class="struct" title="struct polars::prelude::DatetimeType">DatetimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Parsing string values and return a [`DatetimeChunked`](https://docs.rs/polars/latest/polars/prelude/type.DatetimeChunked.html "type polars::prelude::DatetimeChunked") Different from `as_datetime` this function allows matches that not contain the whole string e.g. “foo-2021-01-01-bar” could match “2021-01-01”

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/trait.StringMethods.html#method.as_date" class="fn">as_date</a>( &self, fmt: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>, use_cache: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DateType.html" class="struct" title="struct polars::prelude::DateType">DateType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Parsing string values and return a [`DateChunked`](https://docs.rs/polars/latest/polars/prelude/type.DateChunked.html "type polars::prelude::DateChunked")

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/trait.StringMethods.html#method.as_datetime" class="fn">as_datetime</a>( &self, fmt: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>, tu: <a href="https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html" class="enum" title="enum polars::prelude::TimeUnit">TimeUnit</a>, use_cache: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, tz_aware: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, tz: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeZone.html" class="struct" title="struct polars::prelude::TimeZone">TimeZone</a>\>, ambiguous: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeType.html" class="struct" title="struct polars::prelude::DatetimeType">DatetimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Parsing string values and return a [`DatetimeChunked`](https://docs.rs/polars/latest/polars/prelude/type.DatetimeChunked.html "type polars::prelude::DatetimeChunked").

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/trait.StringMethods.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/trait.StringMethods.html#impl-StringMethods-for-ChunkedArray%3CStringType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.StringMethods.html" class="trait" title="trait polars::prelude::StringMethods">StringMethods</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>
