# Struct ParquetWriter Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/writer/file_writer/parquet_writer.rs.html#251-258" class="src">Source</a>

``` rust
pub struct ParquetWriter { /* private fields */ }
```

Expand description

\`ParquetWriter\`\` is used to write arrow data into parquet file on storage.

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/struct.ParquetWriter.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/struct.ParquetWriter.html#impl-CurrentFileStatus-for-ParquetWriter" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/trait.CurrentFileStatus.html" class="trait" title="trait iceberg::writer::CurrentFileStatus">CurrentFileStatus</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/struct.ParquetWriter.html" class="struct" title="struct iceberg::writer::file_writer::ParquetWriter">ParquetWriter</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/struct.ParquetWriter.html#method.current_file_path" class="anchor">§</a>

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/trait.CurrentFileStatus.html#tymethod.current_file_path" class="fn">current_file_path</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

Get the current file path.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/struct.ParquetWriter.html#method.current_row_num" class="anchor">§</a>

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/trait.CurrentFileStatus.html#tymethod.current_row_num" class="fn">current_row_num</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Get the current file row number.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/struct.ParquetWriter.html#method.current_written_size" class="anchor">§</a>

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/trait.CurrentFileStatus.html#tymethod.current_written_size" class="fn">current_written_size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Get the current file written size.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/struct.ParquetWriter.html#impl-FileWriter-for-ParquetWriter" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriter.html" class="trait" title="trait iceberg::writer::file_writer::FileWriter">FileWriter</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/struct.ParquetWriter.html" class="struct" title="struct iceberg::writer::file_writer::ParquetWriter">ParquetWriter</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/struct.ParquetWriter.html#method.write" class="anchor">§</a>

#### async fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriter.html#tymethod.write" class="fn">write</a>(&mut self, batch: &<a href="https://docs.rs/arrow-array/55.2.0/x86_64-unknown-linux-gnu/arrow_array/record_batch/struct.RecordBatch.html" class="struct" title="struct arrow_array::record_batch::RecordBatch">RecordBatch</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Write record batch to file.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/struct.ParquetWriter.html#method.close" class="anchor">§</a>

#### async fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriter.html#tymethod.close" class="fn">close</a>(self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFileBuilder.html" class="struct" title="struct iceberg::spec::DataFileBuilder">DataFileBuilder</a>\>\>

Close file writer.

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/struct.ParquetWriter.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/struct.ParquetWriter.html#blanket-implementations" class="anchor">§</a>
