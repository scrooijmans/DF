# Struct CsvWriter Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/csv/write/writer.rs.html#17" class="src">Source</a>

``` rust
pub struct CsvWriter<W>where
    W: Write,{ /* private fields */ }
```

Available on **crate feature `polars-io`** only.

Expand description

Write a DataFrame to csv.

Don’t use a `Buffered` writer, the `CsvWriter` internally already buffers writes.

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvWriter.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvWriter.html#impl-CsvWriter%3CW%3E" class="anchor">§</a>

### impl\<W\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvWriter.html" class="struct" title="struct polars::prelude::CsvWriter">CsvWriter</a>\<W\>

where W: <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html" class="trait" title="trait std::io::Write">Write</a>,

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvWriter.html#method.include_bom" class="fn">include_bom</a>(self, include_bom: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvWriter.html" class="struct" title="struct polars::prelude::CsvWriter">CsvWriter</a>\<W\>

Set whether to write UTF-8 BOM.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvWriter.html#method.include_header" class="fn">include_header</a>(self, include_header: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvWriter.html" class="struct" title="struct polars::prelude::CsvWriter">CsvWriter</a>\<W\>

Set whether to write headers.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvWriter.html#method.with_separator" class="fn">with_separator</a>(self, separator: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvWriter.html" class="struct" title="struct polars::prelude::CsvWriter">CsvWriter</a>\<W\>

Set the CSV file’s column separator as a byte character.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvWriter.html#method.with_batch_size" class="fn">with_batch_size</a>(self, batch_size: <a href="https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html" class="struct" title="struct core::num::nonzero::NonZero">NonZero</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvWriter.html" class="struct" title="struct polars::prelude::CsvWriter">CsvWriter</a>\<W\>

Set the batch size to use while writing the CSV.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvWriter.html#method.with_date_format" class="fn">with_date_format</a>(self, format: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvWriter.html" class="struct" title="struct polars::prelude::CsvWriter">CsvWriter</a>\<W\>

Set the CSV file’s date format.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvWriter.html#method.with_time_format" class="fn">with_time_format</a>(self, format: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvWriter.html" class="struct" title="struct polars::prelude::CsvWriter">CsvWriter</a>\<W\>

Set the CSV file’s time format.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvWriter.html#method.with_datetime_format" class="fn">with_datetime_format</a>(self, format: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvWriter.html" class="struct" title="struct polars::prelude::CsvWriter">CsvWriter</a>\<W\>

Set the CSV file’s datetime format.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvWriter.html#method.with_float_scientific" class="fn">with_float_scientific</a>(self, scientific: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvWriter.html" class="struct" title="struct polars::prelude::CsvWriter">CsvWriter</a>\<W\>

Set the CSV file’s forced scientific notation for floats.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvWriter.html#method.with_float_precision" class="fn">with_float_precision</a>(self, precision: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvWriter.html" class="struct" title="struct polars::prelude::CsvWriter">CsvWriter</a>\<W\>

Set the CSV file’s float precision.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvWriter.html#method.with_decimal_comma" class="fn">with_decimal_comma</a>(self, decimal_comma: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvWriter.html" class="struct" title="struct polars::prelude::CsvWriter">CsvWriter</a>\<W\>

Set the CSV decimal separator.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvWriter.html#method.with_quote_char" class="fn">with_quote_char</a>(self, char: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvWriter.html" class="struct" title="struct polars::prelude::CsvWriter">CsvWriter</a>\<W\>

Set the single byte character used for quoting.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvWriter.html#method.with_null_value" class="fn">with_null_value</a>(self, null_value: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvWriter.html" class="struct" title="struct polars::prelude::CsvWriter">CsvWriter</a>\<W\>

Set the CSV file’s null value representation.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvWriter.html#method.with_line_terminator" class="fn">with_line_terminator</a>(self, line_terminator: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvWriter.html" class="struct" title="struct polars::prelude::CsvWriter">CsvWriter</a>\<W\>

Set the CSV file’s line terminator.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvWriter.html#method.with_quote_style" class="fn">with_quote_style</a>(self, quote_style: <a href="https://docs.rs/polars/latest/polars/prelude/enum.QuoteStyle.html" class="enum" title="enum polars::prelude::QuoteStyle">QuoteStyle</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvWriter.html" class="struct" title="struct polars::prelude::CsvWriter">CsvWriter</a>\<W\>

Set the CSV file’s quoting behavior. See more on [`QuoteStyle`](https://docs.rs/polars/latest/polars/prelude/enum.QuoteStyle.html "enum polars::prelude::QuoteStyle").

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvWriter.html#method.n_threads" class="fn">n_threads</a>(self, n_threads: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvWriter.html" class="struct" title="struct polars::prelude::CsvWriter">CsvWriter</a>\<W\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvWriter.html#method.batched" class="fn">batched</a>( self, schema: &<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/polars_io/csv/write/writer/struct.BatchedWriter.html" class="struct" title="struct polars_io::csv::write::writer::BatchedWriter">BatchedWriter</a>\<W\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvWriter.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvWriter.html#impl-SerWriter%3CW%3E-for-CsvWriter%3CW%3E" class="anchor">§</a>

### impl\<W\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.SerWriter.html" class="trait" title="trait polars::prelude::SerWriter">SerWriter</a>\<W\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvWriter.html" class="struct" title="struct polars::prelude::CsvWriter">CsvWriter</a>\<W\>

where W: <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html" class="trait" title="trait std::io::Write">Write</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvWriter.html#method.new" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SerWriter.html#tymethod.new" class="fn">new</a>(buffer: W) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvWriter.html" class="struct" title="struct polars::prelude::CsvWriter">CsvWriter</a>\<W\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvWriter.html#method.finish" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SerWriter.html#tymethod.finish" class="fn">finish</a>(&mut self, df: &mut <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvWriter.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvWriter.html#blanket-implementations" class="anchor">§</a>
