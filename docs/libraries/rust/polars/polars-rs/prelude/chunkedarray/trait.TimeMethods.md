# Trait TimeMethods Copy item path

<a href="https://docs.rs/polars-time/0.51.0/x86_64-unknown-linux-gnu/src/polars_time/chunkedarray/time.rs.html#5" class="src">Source</a>

``` rust
pub trait TimeMethods {
    // Required methods
    fn hour(&self) -> ChunkedArray<Int8Type>;
    fn minute(&self) -> ChunkedArray<Int8Type>;
    fn second(&self) -> ChunkedArray<Int8Type>;
    fn nanosecond(&self) -> ChunkedArray<Int32Type>;
    fn parse_from_str_slice(
        name: PlSmallStr,
        v: &[&str],
        fmt: &str,
    ) -> Logical<TimeType, Int64Type>;
}
```

Available on **crate feature `temporal`** only.

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.TimeMethods.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.TimeMethods.html#tymethod.hour" class="fn">hour</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>

Extract hour from underlying NaiveDateTime representation. Returns the hour number from 0 to 23.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.TimeMethods.html#tymethod.minute" class="fn">minute</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>

Extract minute from underlying NaiveDateTime representation. Returns the minute number from 0 to 59.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.TimeMethods.html#tymethod.second" class="fn">second</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>

Extract second from underlying NaiveDateTime representation. Returns the second number from 0 to 59.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.TimeMethods.html#tymethod.nanosecond" class="fn">nanosecond</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>

Extract second from underlying NaiveDateTime representation. Returns the number of nanoseconds since the whole non-leap second. The range from 1,000,000,000 to 1,999,999,999 represents the leap second.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.TimeMethods.html#tymethod.parse_from_str_slice" class="fn">parse_from_str_slice</a>( name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: &\[&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\], fmt: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeType.html" class="struct" title="struct polars::prelude::TimeType">TimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

## Dyn Compatibility<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.TimeMethods.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.TimeMethods.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.TimeMethods.html#impl-TimeMethods-for-Logical%3CTimeType,+Int64Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.TimeMethods.html" class="trait" title="trait polars::prelude::TimeMethods">TimeMethods</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeType.html" class="struct" title="struct polars::prelude::TimeType">TimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>
