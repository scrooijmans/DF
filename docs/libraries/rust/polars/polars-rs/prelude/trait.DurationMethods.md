# Trait DurationMethods Copy item path

<a href="https://docs.rs/polars-time/0.51.0/x86_64-unknown-linux-gnu/src/polars_time/chunkedarray/duration.rs.html#10" class="src">Source</a>

``` rust
pub trait DurationMethods {
    // Required methods
    fn hours(&self) -> ChunkedArray<Int64Type>;
    fn days(&self) -> ChunkedArray<Int64Type>;
    fn minutes(&self) -> ChunkedArray<Int64Type>;
    fn seconds(&self) -> ChunkedArray<Int64Type>;
    fn milliseconds(&self) -> ChunkedArray<Int64Type>;
    fn microseconds(&self) -> ChunkedArray<Int64Type>;
    fn nanoseconds(&self) -> ChunkedArray<Int64Type>;
}
```

Available on **crate feature `temporal`** only.

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.DurationMethods.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.DurationMethods.html#tymethod.hours" class="fn">hours</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

Extract the hours from a `Duration`

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.DurationMethods.html#tymethod.days" class="fn">days</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

Extract the days from a `Duration`

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.DurationMethods.html#tymethod.minutes" class="fn">minutes</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

Extract the minutes from a `Duration`

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.DurationMethods.html#tymethod.seconds" class="fn">seconds</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

Extract the seconds from a `Duration`

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.DurationMethods.html#tymethod.milliseconds" class="fn">milliseconds</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

Extract the milliseconds from a `Duration`

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.DurationMethods.html#tymethod.microseconds" class="fn">microseconds</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

Extract the microseconds from a `Duration`

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.DurationMethods.html#tymethod.nanoseconds" class="fn">nanoseconds</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

Extract the nanoseconds from a `Duration`

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.DurationMethods.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.DurationMethods.html#impl-DurationMethods-for-Logical%3CDurationType,+Int64Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.DurationMethods.html" class="trait" title="trait polars::prelude::DurationMethods">DurationMethods</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationType.html" class="struct" title="struct polars::prelude::DurationType">DurationType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>
