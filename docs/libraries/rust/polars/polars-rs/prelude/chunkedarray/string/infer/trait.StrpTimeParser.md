# Trait StrpTimeParser Copy item path

<a href="https://docs.rs/polars-time/0.51.0/x86_64-unknown-linux-gnu/src/polars_time/chunkedarray/string/infer.rs.html#137" class="src">Source</a>

``` rust
pub trait StrpTimeParser<T> {
    // Required method
    fn parse_bytes(
        &mut self,
        val: &[u8],
        time_unit: Option<TimeUnit>,
    ) -> Option<T>;
}
```

Available on **crate feature `temporal`** only.

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/trait.StrpTimeParser.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/trait.StrpTimeParser.html#tymethod.parse_bytes" class="fn">parse_bytes</a>(&mut self, val: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\], time_unit: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html" class="enum" title="enum polars::prelude::TimeUnit">TimeUnit</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<T\>

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/trait.StrpTimeParser.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/trait.StrpTimeParser.html#impl-StrpTimeParser%3Ci32%3E-for-DatetimeInfer%3CInt32Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/trait.StrpTimeParser.html" class="trait" title="trait polars::prelude::chunkedarray::string::infer::StrpTimeParser">StrpTimeParser</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/struct.DatetimeInfer.html" class="struct" title="struct polars::prelude::chunkedarray::string::infer::DatetimeInfer">DatetimeInfer</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/trait.StrpTimeParser.html#impl-StrpTimeParser%3Ci64%3E-for-DatetimeInfer%3CInt64Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/trait.StrpTimeParser.html" class="trait" title="trait polars::prelude::chunkedarray::string::infer::StrpTimeParser">StrpTimeParser</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/struct.DatetimeInfer.html" class="struct" title="struct polars::prelude::chunkedarray::string::infer::DatetimeInfer">DatetimeInfer</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>
