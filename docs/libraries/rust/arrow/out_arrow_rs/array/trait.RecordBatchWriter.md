# Trait RecordBatchWriter Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/record_batch.rs.html#45" class="src">Source</a>

``` rust
pub trait RecordBatchWriter {
    // Required methods
    fn write(&mut self, batch: &RecordBatch) -> Result<(), ArrowError>;
    fn close(self) -> Result<(), ArrowError>;
}
```

Expand description

Trait for types that can write `RecordBatch`’s.

## Required Methods<a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchWriter.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchWriter.html#tymethod.write" class="fn">write</a>(&mut self, batch: &<a href="https://docs.rs/arrow/latest/arrow/array/struct.RecordBatch.html" class="struct" title="struct arrow::array::RecordBatch">RecordBatch</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Write a single batch to the writer.

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchWriter.html#tymethod.close" class="fn">close</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Write footer or termination data, then mark the writer as done.

## Implementations on Foreign Types<a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchWriter.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchWriter.html#impl-RecordBatchWriter-for-Writer%3CW%3E" class="anchor">§</a>

### impl\<W\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchWriter.html" class="trait" title="trait arrow::array::RecordBatchWriter">RecordBatchWriter</a> for <a href="https://docs.rs/arrow-csv/56.2.0/x86_64-unknown-linux-gnu/arrow_csv/writer/struct.Writer.html" class="struct" title="struct arrow_csv::writer::Writer">Writer</a>\<W\>

where W: <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html" class="trait" title="trait std::io::Write">Write</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchWriter.html#method.write" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchWriter.html#tymethod.write" class="fn">write</a>(&mut self, batch: &<a href="https://docs.rs/arrow/latest/arrow/array/struct.RecordBatch.html" class="struct" title="struct arrow::array::RecordBatch">RecordBatch</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchWriter.html#method.close" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchWriter.html#tymethod.close" class="fn">close</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchWriter.html#impl-RecordBatchWriter-for-FileWriter%3CW%3E" class="anchor">§</a>

### impl\<W\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchWriter.html" class="trait" title="trait arrow::array::RecordBatchWriter">RecordBatchWriter</a> for <a href="https://docs.rs/arrow-ipc/56.2.0/x86_64-unknown-linux-gnu/arrow_ipc/writer/struct.FileWriter.html" class="struct" title="struct arrow_ipc::writer::FileWriter">FileWriter</a>\<W\>

where W: <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html" class="trait" title="trait std::io::Write">Write</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchWriter.html#method.write-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchWriter.html#tymethod.write" class="fn">write</a>(&mut self, batch: &<a href="https://docs.rs/arrow/latest/arrow/array/struct.RecordBatch.html" class="struct" title="struct arrow::array::RecordBatch">RecordBatch</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchWriter.html#method.close-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchWriter.html#tymethod.close" class="fn">close</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchWriter.html#impl-RecordBatchWriter-for-StreamWriter%3CW%3E" class="anchor">§</a>

### impl\<W\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchWriter.html" class="trait" title="trait arrow::array::RecordBatchWriter">RecordBatchWriter</a> for <a href="https://docs.rs/arrow-ipc/56.2.0/x86_64-unknown-linux-gnu/arrow_ipc/writer/struct.StreamWriter.html" class="struct" title="struct arrow_ipc::writer::StreamWriter">StreamWriter</a>\<W\>

where W: <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html" class="trait" title="trait std::io::Write">Write</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchWriter.html#method.write-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchWriter.html#tymethod.write" class="fn">write</a>(&mut self, batch: &<a href="https://docs.rs/arrow/latest/arrow/array/struct.RecordBatch.html" class="struct" title="struct arrow::array::RecordBatch">RecordBatch</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchWriter.html#method.close-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchWriter.html#tymethod.close" class="fn">close</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchWriter.html#impl-RecordBatchWriter-for-Writer%3CW,+F%3E" class="anchor">§</a>

### impl\<W, F\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchWriter.html" class="trait" title="trait arrow::array::RecordBatchWriter">RecordBatchWriter</a> for <a href="https://docs.rs/arrow-json/56.2.0/x86_64-unknown-linux-gnu/arrow_json/writer/struct.Writer.html" class="struct" title="struct arrow_json::writer::Writer">Writer</a>\<W, F\>

where W: <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html" class="trait" title="trait std::io::Write">Write</a>, F: <a href="https://docs.rs/arrow-json/56.2.0/x86_64-unknown-linux-gnu/arrow_json/writer/trait.JsonFormat.html" class="trait" title="trait arrow_json::writer::JsonFormat">JsonFormat</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchWriter.html#method.write-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchWriter.html#tymethod.write" class="fn">write</a>(&mut self, batch: &<a href="https://docs.rs/arrow/latest/arrow/array/struct.RecordBatch.html" class="struct" title="struct arrow::array::RecordBatch">RecordBatch</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchWriter.html#method.close-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchWriter.html#tymethod.close" class="fn">close</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

## Implementors<a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchWriter.html#implementors" class="anchor">§</a>
