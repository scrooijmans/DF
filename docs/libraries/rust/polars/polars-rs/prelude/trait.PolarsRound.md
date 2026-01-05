# Trait PolarsRound Copy item path

<a href="https://docs.rs/polars-time/0.51.0/x86_64-unknown-linux-gnu/src/polars_time/round.rs.html#15" class="src">Source</a>

``` rust
pub trait PolarsRound {
    // Required method
    fn round(
        &self,
        every: &ChunkedArray<StringType>,
        tz: Option<&Tz>,
    ) -> Result<Self, PolarsError>
       where Self: Sized;
}
```

Available on **crate feature `temporal`** only.

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsRound.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsRound.html#tymethod.round" class="fn">round</a>( &self, every: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>, tz: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/chrono-tz/0.10.3/x86_64-unknown-linux-gnu/chrono_tz/timezones/enum.Tz.html" class="enum" title="enum chrono_tz::timezones::Tz">Tz</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsRound.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsRound.html#impl-PolarsRound-for-Logical%3CDateType,+Int32Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsRound.html" class="trait" title="trait polars::prelude::PolarsRound">PolarsRound</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DateType.html" class="struct" title="struct polars::prelude::DateType">DateType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsRound.html#impl-PolarsRound-for-Logical%3CDatetimeType,+Int64Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsRound.html" class="trait" title="trait polars::prelude::PolarsRound">PolarsRound</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeType.html" class="struct" title="struct polars::prelude::DatetimeType">DatetimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>
