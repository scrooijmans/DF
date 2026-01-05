# Trait SerWriter Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/shared.rs.html#35" class="src">Source</a>

``` rust
pub trait SerWriter<W>where
    W: Write,{
    // Required methods
    fn new(writer: W) -> Self
       where Self: Sized;
    fn finish(&mut self, df: &mut DataFrame) -> Result<(), PolarsError>;
}
```

Available on **crate feature `polars-io`** only.

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.SerWriter.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SerWriter.html#tymethod.new" class="fn">new</a>(writer: W) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SerWriter.html#tymethod.finish" class="fn">finish</a>(&mut self, df: &mut <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.SerWriter.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.SerWriter.html#impl-SerWriter%3CW%3E-for-CsvWriter%3CW%3E" class="anchor">§</a>

### impl\<W\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.SerWriter.html" class="trait" title="trait polars::prelude::SerWriter">SerWriter</a>\<W\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvWriter.html" class="struct" title="struct polars::prelude::CsvWriter">CsvWriter</a>\<W\>

where W: <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html" class="trait" title="trait std::io::Write">Write</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.SerWriter.html#impl-SerWriter%3CW%3E-for-IpcStreamWriter%3CW%3E" class="anchor">§</a>

### impl\<W\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.SerWriter.html" class="trait" title="trait polars::prelude::SerWriter">SerWriter</a>\<W\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamWriter.html" class="struct" title="struct polars::prelude::IpcStreamWriter">IpcStreamWriter</a>\<W\>

where W: <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html" class="trait" title="trait std::io::Write">Write</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.SerWriter.html#impl-SerWriter%3CW%3E-for-IpcWriter%3CW%3E" class="anchor">§</a>

### impl\<W\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.SerWriter.html" class="trait" title="trait polars::prelude::SerWriter">SerWriter</a>\<W\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcWriter.html" class="struct" title="struct polars::prelude::IpcWriter">IpcWriter</a>\<W\>

where W: <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html" class="trait" title="trait std::io::Write">Write</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.SerWriter.html#impl-SerWriter%3CW%3E-for-JsonWriter%3CW%3E" class="anchor">§</a>

### impl\<W\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.SerWriter.html" class="trait" title="trait polars::prelude::SerWriter">SerWriter</a>\<W\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonWriter.html" class="struct" title="struct polars::prelude::JsonWriter">JsonWriter</a>\<W\>

where W: <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html" class="trait" title="trait std::io::Write">Write</a>,
