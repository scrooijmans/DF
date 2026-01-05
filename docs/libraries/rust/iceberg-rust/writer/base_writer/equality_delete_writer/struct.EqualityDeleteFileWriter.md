# Struct EqualityDeleteFileWriter Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/writer/base_writer/equality_delete_writer.rs.html#129-135" class="src">Source</a>

``` rust
pub struct EqualityDeleteFileWriter<B: FileWriterBuilder> { /* private fields */ }
```

Expand description

Writer used to write equality delete files.

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/base_writer/equality_delete_writer/struct.EqualityDeleteFileWriter.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/base_writer/equality_delete_writer/struct.EqualityDeleteFileWriter.html#impl-Debug-for-EqualityDeleteFileWriter%3CB%3E" class="anchor">§</a>

### impl\<B: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> + <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriterBuilder.html" class="trait" title="trait iceberg::writer::file_writer::FileWriterBuilder">FileWriterBuilder</a>\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/base_writer/equality_delete_writer/struct.EqualityDeleteFileWriter.html" class="struct" title="struct iceberg::writer::base_writer::equality_delete_writer::EqualityDeleteFileWriter">EqualityDeleteFileWriter</a>\<B\>

where B::<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriterBuilder.html#associatedtype.R" class="associatedtype" title="type iceberg::writer::file_writer::FileWriterBuilder::R">R</a>: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>,

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/base_writer/equality_delete_writer/struct.EqualityDeleteFileWriter.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/base_writer/equality_delete_writer/struct.EqualityDeleteFileWriter.html#impl-IcebergWriter-for-EqualityDeleteFileWriter%3CB%3E" class="anchor">§</a>

### impl\<B: <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriterBuilder.html" class="trait" title="trait iceberg::writer::file_writer::FileWriterBuilder">FileWriterBuilder</a>\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/trait.IcebergWriter.html" class="trait" title="trait iceberg::writer::IcebergWriter">IcebergWriter</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/base_writer/equality_delete_writer/struct.EqualityDeleteFileWriter.html" class="struct" title="struct iceberg::writer::base_writer::equality_delete_writer::EqualityDeleteFileWriter">EqualityDeleteFileWriter</a>\<B\>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/base_writer/equality_delete_writer/struct.EqualityDeleteFileWriter.html#method.write" class="anchor">§</a>

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/trait.IcebergWriter.html#tymethod.write" class="fn">write</a>\<'life0, 'async_trait\>( &'life0 mut self, batch: <a href="https://docs.rs/arrow-array/55.2.0/x86_64-unknown-linux-gnu/arrow_array/record_batch/struct.RecordBatch.html" class="struct" title="struct arrow_array::record_batch::RecordBatch">RecordBatch</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait,

Write data to iceberg table.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/base_writer/equality_delete_writer/struct.EqualityDeleteFileWriter.html#method.close" class="anchor">§</a>

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/trait.IcebergWriter.html#tymethod.close" class="fn">close</a>\<'life0, 'async_trait\>( &'life0 mut self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html" class="struct" title="struct iceberg::spec::DataFile">DataFile</a>\>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait,

Close the writer and return the written data files. If close failed, the data written before maybe be lost. User may need to recreate the writer and rewrite the data again. [Read more](https://docs.rs/iceberg/0.7.0/iceberg/writer/trait.IcebergWriter.html#tymethod.close)

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/base_writer/equality_delete_writer/struct.EqualityDeleteFileWriter.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/base_writer/equality_delete_writer/struct.EqualityDeleteFileWriter.html#blanket-implementations" class="anchor">§</a>
