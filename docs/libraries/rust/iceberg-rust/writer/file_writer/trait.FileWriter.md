# Trait FileWriter Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/writer/file_writer/mod.rs.html#45-50" class="src">Source</a>

``` rust
pub trait FileWriter<O = Vec<DataFileBuilder>>:
    Send
    + CurrentFileStatus
    + 'static {
    // Required methods
    fn write(
        &mut self,
        batch: &RecordBatch,
    ) -> impl Future<Output = Result<()>> + Send;
    fn close(self) -> impl Future<Output = Result<O>> + Send;
}
```

Expand description

File writer focus on writing record batch to different physical file format.(Such as parquet. orc)

## Required Methods<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriter.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriter.html#tymethod.write" class="fn">write</a>( &mut self, batch: &<a href="https://docs.rs/arrow-array/55.2.0/x86_64-unknown-linux-gnu/arrow_array/record_batch/struct.RecordBatch.html" class="struct" title="struct arrow_array::record_batch::RecordBatch">RecordBatch</a>, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>

Write record batch to file.

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriter.html#tymethod.close" class="fn">close</a>(self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<O\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>

Close file writer.

## Dyn Compatibility<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriter.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriter.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriter.html#impl-FileWriter-for-ParquetWriter" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriter.html" class="trait" title="trait iceberg::writer::file_writer::FileWriter">FileWriter</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/struct.ParquetWriter.html" class="struct" title="struct iceberg::writer::file_writer::ParquetWriter">ParquetWriter</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriter.html#impl-FileWriter-for-RollingFileWriter%3CB%3E" class="anchor">§</a>

### impl\<B: <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriterBuilder.html" class="trait" title="trait iceberg::writer::file_writer::FileWriterBuilder">FileWriterBuilder</a>\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriter.html" class="trait" title="trait iceberg::writer::file_writer::FileWriter">FileWriter</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/rolling_writer/struct.RollingFileWriter.html" class="struct" title="struct iceberg::writer::file_writer::rolling_writer::RollingFileWriter">RollingFileWriter</a>\<B\>
