# Struct Writer Copy item path

<a href="https://docs.rs/arrow-csv/56.0.0/x86_64-unknown-linux-gnu/src/arrow_csv/writer.rs.html#77" class="src">Source</a>

``` rust
pub struct Writer<W>where
    W: Write,{ /* private fields */ }
```

Expand description

A CSV writer

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/struct.Writer.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/struct.Writer.html#impl-Writer%3CW%3E" class="anchor">§</a>

### impl\<W\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/struct.Writer.html" class="struct" title="struct datafusion::common::arrow::csv::Writer">Writer</a>\<W\>

where W: <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html" class="trait" title="trait std::io::Write">Write</a>,

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/struct.Writer.html#method.new" class="fn">new</a>(writer: W) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/struct.Writer.html" class="struct" title="struct datafusion::common::arrow::csv::Writer">Writer</a>\<W\>

Create a new CsvWriter from a writable object, with default options

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/struct.Writer.html#method.write" class="fn">write</a>(&mut self, batch: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

Write a vector of record batches to a writable object

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/struct.Writer.html#method.into_inner" class="fn">into_inner</a>(self) -\> W

Unwraps this `Writer<W>`, returning the underlying writer.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/struct.Writer.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/struct.Writer.html#impl-Debug-for-Writer%3CW%3E" class="anchor">§</a>

### impl\<W\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/struct.Writer.html" class="struct" title="struct datafusion::common::arrow::csv::Writer">Writer</a>\<W\>

where W: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> + <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html" class="trait" title="trait std::io::Write">Write</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/struct.Writer.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/struct.Writer.html#impl-RecordBatchWriter-for-Writer%3CW%3E" class="anchor">§</a>

### impl\<W\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.RecordBatchWriter.html" class="trait" title="trait datafusion::common::arrow::array::RecordBatchWriter">RecordBatchWriter</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/struct.Writer.html" class="struct" title="struct datafusion::common::arrow::csv::Writer">Writer</a>\<W\>

where W: <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html" class="trait" title="trait std::io::Write">Write</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/struct.Writer.html#method.write-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.RecordBatchWriter.html#tymethod.write" class="fn">write</a>(&mut self, batch: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

Write a single batch to the writer.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/struct.Writer.html#method.close" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.RecordBatchWriter.html#tymethod.close" class="fn">close</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

Write footer or termination data, then mark the writer as done.

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/struct.Writer.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/struct.Writer.html#blanket-implementations" class="anchor">§</a>
