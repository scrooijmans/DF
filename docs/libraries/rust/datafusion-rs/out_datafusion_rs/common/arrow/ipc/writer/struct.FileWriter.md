# Struct FileWriter Copy item path

<a href="https://docs.rs/arrow-ipc/56.0.0/x86_64-unknown-linux-gnu/src/arrow_ipc/writer.rs.html#815" class="src">Source</a>

``` rust
pub struct FileWriter<W> { /* private fields */ }
```

Expand description

Arrow File Writer

Writes Arrow [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch")es in the [IPC File Format](https://arrow.apache.org/docs/format/Columnar.html#ipc-file-format).

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.FileWriter.html#see-also" class="doc-anchor">§</a>See Also

- [`StreamWriter`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.StreamWriter.html "struct datafusion::common::arrow::ipc::writer::StreamWriter") for writing IPC Streams

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.FileWriter.html#example" class="doc-anchor">§</a>Example

``` rust
let batch = record_batch!(("a", Int32, [1, 2, 3])).unwrap();
// create a new writer, the schema must be known in advance
let mut writer = FileWriter::try_new(&mut file, &batch.schema()).unwrap();
// write each batch to the underlying writer
writer.write(&batch).unwrap();
// When all batches are written, call finish to flush all buffers
writer.finish().unwrap();
```

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.FileWriter.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.FileWriter.html#impl-FileWriter%3CBufWriter%3CW%3E%3E" class="anchor">§</a>

### impl\<W\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.FileWriter.html" class="struct" title="struct datafusion::common::arrow::ipc::writer::FileWriter">FileWriter</a>\<<a href="https://doc.rust-lang.org/nightly/std/io/buffered/bufwriter/struct.BufWriter.html" class="struct" title="struct std::io::buffered::bufwriter::BufWriter">BufWriter</a>\<W\>\>

where W: <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html" class="trait" title="trait std::io::Write">Write</a>,

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.FileWriter.html#method.try_new_buffered" class="fn">try_new_buffered</a>( writer: W, schema: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.FileWriter.html" class="struct" title="struct datafusion::common::arrow::ipc::writer::FileWriter">FileWriter</a>\<<a href="https://doc.rust-lang.org/nightly/std/io/buffered/bufwriter/struct.BufWriter.html" class="struct" title="struct std::io::buffered::bufwriter::BufWriter">BufWriter</a>\<W\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

Try to create a new file writer with the writer wrapped in a BufWriter.

See [`FileWriter::try_new`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.FileWriter.html#method.try_new "associated function datafusion::common::arrow::ipc::writer::FileWriter::try_new") for an unbuffered version.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.FileWriter.html#impl-FileWriter%3CW%3E" class="anchor">§</a>

### impl\<W\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.FileWriter.html" class="struct" title="struct datafusion::common::arrow::ipc::writer::FileWriter">FileWriter</a>\<W\>

where W: <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html" class="trait" title="trait std::io::Write">Write</a>,

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.FileWriter.html#method.try_new" class="fn">try_new</a>(writer: W, schema: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.FileWriter.html" class="struct" title="struct datafusion::common::arrow::ipc::writer::FileWriter">FileWriter</a>\<W\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

Try to create a new writer, with the schema written as part of the header

Note the created writer is not buffered. See [`FileWriter::try_new_buffered`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.FileWriter.html#method.try_new_buffered "associated function datafusion::common::arrow::ipc::writer::FileWriter::try_new_buffered") for details.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.FileWriter.html#errors" class="doc-anchor">§</a>Errors

An [‘Err’](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Err "variant core::result::Result::Err") may be returned if writing the header to the writer fails.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.FileWriter.html#method.try_new_with_options" class="fn">try_new_with_options</a>( writer: W, schema: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>, write_options: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.IpcWriteOptions.html" class="struct" title="struct datafusion::common::arrow::ipc::writer::IpcWriteOptions">IpcWriteOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.FileWriter.html" class="struct" title="struct datafusion::common::arrow::ipc::writer::FileWriter">FileWriter</a>\<W\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

Try to create a new writer with IpcWriteOptions

Note the created writer is not buffered. See [`FileWriter::try_new_buffered`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.FileWriter.html#method.try_new_buffered "associated function datafusion::common::arrow::ipc::writer::FileWriter::try_new_buffered") for details.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.FileWriter.html#errors-1" class="doc-anchor">§</a>Errors

An [‘Err’](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Err "variant core::result::Result::Err") may be returned if writing the header to the writer fails.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.FileWriter.html#method.write_metadata" class="fn">write_metadata</a>( &mut self, key: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, value: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, )

Adds a key-value pair to the [FileWriter](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.FileWriter.html "struct datafusion::common::arrow::ipc::writer::FileWriter")’s custom metadata

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.FileWriter.html#method.write" class="fn">write</a>(&mut self, batch: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

Write a record batch to the file

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.FileWriter.html#method.finish" class="fn">finish</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

Write footer and closing tag, then mark the writer as done

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.FileWriter.html#method.schema" class="fn">schema</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>

Returns the arrow [`SchemaRef`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/type.SchemaRef.html "type datafusion::common::arrow::datatypes::SchemaRef") for this arrow file.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.FileWriter.html#method.get_ref" class="fn">get_ref</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;W</a>

Gets a reference to the underlying writer.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.FileWriter.html#method.get_mut" class="fn">get_mut</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut W</a>

Gets a mutable reference to the underlying writer.

It is inadvisable to directly write to the underlying writer.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.FileWriter.html#method.flush" class="fn">flush</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

Flush the underlying writer.

Both the BufWriter and the underlying writer are flushed.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.FileWriter.html#method.into_inner" class="fn">into_inner</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<W, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

Unwraps the underlying writer.

The writer is flushed and the FileWriter is finished before returning.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.FileWriter.html#errors-2" class="doc-anchor">§</a>Errors

An [‘Err’](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Err "variant core::result::Result::Err") may be returned if an error occurs while finishing the StreamWriter or while flushing the writer.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.FileWriter.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.FileWriter.html#impl-RecordBatchWriter-for-FileWriter%3CW%3E" class="anchor">§</a>

### impl\<W\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.RecordBatchWriter.html" class="trait" title="trait datafusion::common::arrow::array::RecordBatchWriter">RecordBatchWriter</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.FileWriter.html" class="struct" title="struct datafusion::common::arrow::ipc::writer::FileWriter">FileWriter</a>\<W\>

where W: <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html" class="trait" title="trait std::io::Write">Write</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.FileWriter.html#method.write-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.RecordBatchWriter.html#tymethod.write" class="fn">write</a>(&mut self, batch: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

Write a single batch to the writer.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.FileWriter.html#method.close" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.RecordBatchWriter.html#tymethod.close" class="fn">close</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

Write footer or termination data, then mark the writer as done.

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.FileWriter.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.FileWriter.html#blanket-implementations" class="anchor">§</a>
