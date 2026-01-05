# Struct Writer Copy item path

<a href="https://docs.rs/arrow-json/56.0.0/x86_64-unknown-linux-gnu/src/arrow_json/writer/mod.rs.html#309" class="src">Source</a>

``` rust
pub struct Writer<W, F>where
    W: Write,
    F: JsonFormat,{ /* private fields */ }
```

Expand description

A JSON writer which serializes [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch")es to a stream of `u8` encoded JSON objects.

See the module level documentation for detailed usage and examples. The specific format of the stream is controlled by the [`JsonFormat`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/trait.JsonFormat.html "trait datafusion::common::arrow::json::writer::JsonFormat") type parameter.

By default the writer will skip writing keys with null values for backward compatibility. See [`WriterBuilder`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.WriterBuilder.html "struct datafusion::common::arrow::json::WriterBuilder") on how to customize this behaviour when creating a new writer.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.Writer.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.Writer.html#impl-Writer%3CW,+F%3E" class="anchor">§</a>

### impl\<W, F\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.Writer.html" class="struct" title="struct datafusion::common::arrow::json::Writer">Writer</a>\<W, F\>

where W: <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html" class="trait" title="trait std::io::Write">Write</a>, F: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/trait.JsonFormat.html" class="trait" title="trait datafusion::common::arrow::json::writer::JsonFormat">JsonFormat</a>,

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.Writer.html#method.new" class="fn">new</a>(writer: W) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.Writer.html" class="struct" title="struct datafusion::common::arrow::json::Writer">Writer</a>\<W, F\>

Construct a new writer

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.Writer.html#method.write" class="fn">write</a>(&mut self, batch: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

Serialize `batch` to JSON output

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.Writer.html#method.write_batches" class="fn">write_batches</a>( &mut self, batches: &\[&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

Serialize `batches` to JSON output

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.Writer.html#method.finish" class="fn">finish</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

Finishes the output stream. This function must be called after all record batches have been produced. (e.g. producing the final `']'` if writing arrays.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.Writer.html#method.get_ref" class="fn">get_ref</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;W</a>

Gets a reference to the underlying writer.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.Writer.html#method.get_mut" class="fn">get_mut</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut W</a>

Gets a mutable reference to the underlying writer.

Writing to the underlying writer must be done with care to avoid corrupting the output JSON.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.Writer.html#method.into_inner" class="fn">into_inner</a>(self) -\> W

Unwraps this `Writer<W>`, returning the underlying writer

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.Writer.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.Writer.html#impl-Debug-for-Writer%3CW,+F%3E" class="anchor">§</a>

### impl\<W, F\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.Writer.html" class="struct" title="struct datafusion::common::arrow::json::Writer">Writer</a>\<W, F\>

where W: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> + <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html" class="trait" title="trait std::io::Write">Write</a>, F: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> + <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/trait.JsonFormat.html" class="trait" title="trait datafusion::common::arrow::json::writer::JsonFormat">JsonFormat</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.Writer.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.Writer.html#impl-RecordBatchWriter-for-Writer%3CW,+F%3E" class="anchor">§</a>

### impl\<W, F\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.RecordBatchWriter.html" class="trait" title="trait datafusion::common::arrow::array::RecordBatchWriter">RecordBatchWriter</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.Writer.html" class="struct" title="struct datafusion::common::arrow::json::Writer">Writer</a>\<W, F\>

where W: <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html" class="trait" title="trait std::io::Write">Write</a>, F: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/trait.JsonFormat.html" class="trait" title="trait datafusion::common::arrow::json::writer::JsonFormat">JsonFormat</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.Writer.html#method.write-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.RecordBatchWriter.html#tymethod.write" class="fn">write</a>(&mut self, batch: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

Write a single batch to the writer.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.Writer.html#method.close" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.RecordBatchWriter.html#tymethod.close" class="fn">close</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

Write footer or termination data, then mark the writer as done.

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.Writer.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.Writer.html#blanket-implementations" class="anchor">§</a>
