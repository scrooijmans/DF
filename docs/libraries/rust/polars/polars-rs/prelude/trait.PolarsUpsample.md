# Trait PolarsUpsample Copy item path

<a href="https://docs.rs/polars-time/0.51.0/x86_64-unknown-linux-gnu/src/polars_time/upsample.rs.html#9" class="src">Source</a>

``` rust
pub trait PolarsUpsample {
    // Required methods
    fn upsample<I>(
        &self,
        by: I,
        time_column: &str,
        every: Duration,
    ) -> Result<DataFrame, PolarsError>
       where I: IntoVec<PlSmallStr>;
    fn upsample_stable<I>(
        &self,
        by: I,
        time_column: &str,
        every: Duration,
    ) -> Result<DataFrame, PolarsError>
       where I: IntoVec<PlSmallStr>;
}
```

Available on **crate feature `temporal`** only.

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsUpsample.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsUpsample.html#tymethod.upsample" class="fn">upsample</a>\<I\>( &self, by: I, time_column: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, every: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Duration.html" class="struct" title="struct polars::prelude::Duration">Duration</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where I: <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoVec.html" class="trait" title="trait polars::prelude::IntoVec">IntoVec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>,

Upsample a [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame") at a regular frequency.

##### <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsUpsample.html#arguments" class="doc-anchor">§</a>Arguments

- `by` - First group by these columns and then upsample for every group
- `time_column` - Will be used to determine a date_range. Note that this column has to be sorted for the output to make sense.
- `every` - interval will start ‘every’ duration
- `offset` - change the start of the date_range by this offset.

The `every` and `offset` arguments are created with the following string language:

- 1ns (1 nanosecond)
- 1us (1 microsecond)
- 1ms (1 millisecond)
- 1s (1 second)
- 1m (1 minute)
- 1h (1 hour)
- 1d (1 calendar day)
- 1w (1 calendar week)
- 1mo (1 calendar month)
- 1q (1 calendar quarter)
- 1y (1 calendar year)
- 1i (1 index count)

Or combine them: “3d12h4m25s” \# 3 days, 12 hours, 4 minutes, and 25 seconds

By “calendar day”, we mean the corresponding time on the next day (which may not be 24 hours, depending on daylight savings). Similarly for “calendar week”, “calendar month”, “calendar quarter”, and “calendar year”.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsUpsample.html#tymethod.upsample_stable" class="fn">upsample_stable</a>\<I\>( &self, by: I, time_column: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, every: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Duration.html" class="struct" title="struct polars::prelude::Duration">Duration</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where I: <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoVec.html" class="trait" title="trait polars::prelude::IntoVec">IntoVec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>,

Upsample a [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame") at a regular frequency.

Similar to [`upsample`](https://docs.rs/polars/latest/polars/prelude/trait.PolarsUpsample.html#tymethod.upsample "method polars::prelude::PolarsUpsample::upsample"), but order of the DataFrame is maintained when `by` is specified.

##### <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsUpsample.html#arguments-1" class="doc-anchor">§</a>Arguments

- `by` - First group by these columns and then upsample for every group
- `time_column` - Will be used to determine a date_range. Note that this column has to be sorted for the output to make sense.
- `every` - interval will start ‘every’ duration
- `offset` - change the start of the date_range by this offset.

The `every` and `offset` arguments are created with the following string language:

- 1ns (1 nanosecond)
- 1us (1 microsecond)
- 1ms (1 millisecond)
- 1s (1 second)
- 1m (1 minute)
- 1h (1 hour)
- 1d (1 calendar day)
- 1w (1 calendar week)
- 1mo (1 calendar month)
- 1q (1 calendar quarter)
- 1y (1 calendar year)
- 1i (1 index count)

Or combine them: “3d12h4m25s” \# 3 days, 12 hours, 4 minutes, and 25 seconds

By “calendar day”, we mean the corresponding time on the next day (which may not be 24 hours, depending on daylight savings). Similarly for “calendar week”, “calendar month”, “calendar quarter”, and “calendar year”.

## Dyn Compatibility<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsUpsample.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsUpsample.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsUpsample.html#impl-PolarsUpsample-for-DataFrame" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsUpsample.html" class="trait" title="trait polars::prelude::PolarsUpsample">PolarsUpsample</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>
