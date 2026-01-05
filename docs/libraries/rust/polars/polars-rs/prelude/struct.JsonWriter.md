# Struct JsonWriter Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/json/mod.rs.html#118" class="src">Source</a>

``` rust
pub struct JsonWriter<W>where
    W: Write,{ /* private fields */ }
```

Available on **crate feature `polars-io`** only.

Expand description

Writes a DataFrame to JSON.

Under the hood, this uses [`arrow2::io::json`](https://docs.rs/arrow2/latest/arrow2/io/json/write/fn.write.html). `arrow2` generally serializes types that are not JSON primitives, such as Date and DateTime, as their `Display`-formatted versions. For instance, a (naive) DateTime column is formatted as the String `"yyyy-mm-dd HH:MM:SS"`. To control how non-primitive columns are serialized, convert them to String or another primitive type before serializing.

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonWriter.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonWriter.html#impl-JsonWriter%3CW%3E" class="anchor">§</a>

### impl\<W\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonWriter.html" class="struct" title="struct polars::prelude::JsonWriter">JsonWriter</a>\<W\>

where W: <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html" class="trait" title="trait std::io::Write">Write</a>,

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonWriter.html#method.with_json_format" class="fn">with_json_format</a>(self, format: <a href="https://docs.rs/polars/latest/polars/prelude/enum.JsonFormat.html" class="enum" title="enum polars::prelude::JsonFormat">JsonFormat</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonWriter.html" class="struct" title="struct polars::prelude::JsonWriter">JsonWriter</a>\<W\>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonWriter.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonWriter.html#impl-SerWriter%3CW%3E-for-JsonWriter%3CW%3E" class="anchor">§</a>

### impl\<W\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.SerWriter.html" class="trait" title="trait polars::prelude::SerWriter">SerWriter</a>\<W\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonWriter.html" class="struct" title="struct polars::prelude::JsonWriter">JsonWriter</a>\<W\>

where W: <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html" class="trait" title="trait std::io::Write">Write</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonWriter.html#method.new" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SerWriter.html#tymethod.new" class="fn">new</a>(buffer: W) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonWriter.html" class="struct" title="struct polars::prelude::JsonWriter">JsonWriter</a>\<W\>

Create a new `JsonWriter` writing to `buffer` with format `JsonFormat::JsonLines`. To specify a different format, use e.g., [`JsonWriter::new(buffer).with_json_format(JsonFormat::Json)`](https://docs.rs/polars/latest/polars/prelude/struct.JsonWriter.html#method.with_json_format "method polars::prelude::JsonWriter::with_json_format").

<a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonWriter.html#method.finish" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SerWriter.html#tymethod.finish" class="fn">finish</a>(&mut self, df: &mut <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonWriter.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonWriter.html#blanket-implementations" class="anchor">§</a>
