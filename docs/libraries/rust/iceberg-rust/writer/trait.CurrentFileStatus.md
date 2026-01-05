# Trait CurrentFileStatus Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/writer/mod.rs.html#262-269" class="src">Source</a>

``` rust
pub trait CurrentFileStatus {
    // Required methods
    fn current_file_path(&self) -> String;
    fn current_row_num(&self) -> usize;
    fn current_written_size(&self) -> usize;
}
```

Expand description

The current file status of the Iceberg writer. This is implemented for writers that write a single file at a time.

## Required Methods<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/trait.CurrentFileStatus.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/trait.CurrentFileStatus.html#tymethod.current_file_path" class="fn">current_file_path</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

Get the current file path.

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/trait.CurrentFileStatus.html#tymethod.current_row_num" class="fn">current_row_num</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Get the current file row number.

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/trait.CurrentFileStatus.html#tymethod.current_written_size" class="fn">current_written_size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Get the current file written size.

## Implementors<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/trait.CurrentFileStatus.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/trait.CurrentFileStatus.html#impl-CurrentFileStatus-for-ParquetWriter" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/trait.CurrentFileStatus.html" class="trait" title="trait iceberg::writer::CurrentFileStatus">CurrentFileStatus</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/struct.ParquetWriter.html" class="struct" title="struct iceberg::writer::file_writer::ParquetWriter">ParquetWriter</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/trait.CurrentFileStatus.html#impl-CurrentFileStatus-for-DataFileWriter%3CB%3E" class="anchor">§</a>

### impl\<B: <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriterBuilder.html" class="trait" title="trait iceberg::writer::file_writer::FileWriterBuilder">FileWriterBuilder</a>\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/trait.CurrentFileStatus.html" class="trait" title="trait iceberg::writer::CurrentFileStatus">CurrentFileStatus</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/base_writer/data_file_writer/struct.DataFileWriter.html" class="struct" title="struct iceberg::writer::base_writer::data_file_writer::DataFileWriter">DataFileWriter</a>\<B\>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/trait.CurrentFileStatus.html#impl-CurrentFileStatus-for-RollingFileWriter%3CB%3E" class="anchor">§</a>

### impl\<B: <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriterBuilder.html" class="trait" title="trait iceberg::writer::file_writer::FileWriterBuilder">FileWriterBuilder</a>\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/trait.CurrentFileStatus.html" class="trait" title="trait iceberg::writer::CurrentFileStatus">CurrentFileStatus</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/rolling_writer/struct.RollingFileWriter.html" class="struct" title="struct iceberg::writer::file_writer::rolling_writer::RollingFileWriter">RollingFileWriter</a>\<B\>
