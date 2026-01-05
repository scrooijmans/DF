# Trait IcebergWriter Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/writer/mod.rs.html#250-258" class="src">Source</a>

``` rust
pub trait IcebergWriter<I = RecordBatch, O = Vec<DataFile>>: Send + 'static {
    // Required methods
    fn write<'life0, 'async_trait>(
        &'life0 mut self,
        input: I,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'async_trait>>
       where Self: 'async_trait,
             'life0: 'async_trait;
    fn close<'life0, 'async_trait>(
        &'life0 mut self,
    ) -> Pin<Box<dyn Future<Output = Result<O>> + Send + 'async_trait>>
       where Self: 'async_trait,
             'life0: 'async_trait;
}
```

Expand description

The iceberg writer used to write data to iceberg table.

## Required Methods<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/trait.IcebergWriter.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/trait.IcebergWriter.html#tymethod.write" class="fn">write</a>\<'life0, 'async_trait\>( &'life0 mut self, input: I, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait,

Write data to iceberg table.

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/trait.IcebergWriter.html#tymethod.close" class="fn">close</a>\<'life0, 'async_trait\>( &'life0 mut self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<O\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait,

Close the writer and return the written data files. If close failed, the data written before maybe be lost. User may need to recreate the writer and rewrite the data again.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/trait.IcebergWriter.html#note" class="doc-anchor">§</a>NOTE

After close, regardless of success or failure, the writer should never be used again, otherwise the writer will panic.

## Implementors<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/trait.IcebergWriter.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/trait.IcebergWriter.html#impl-IcebergWriter-for-DataFileWriter%3CB%3E" class="anchor">§</a>

### impl\<B: <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriterBuilder.html" class="trait" title="trait iceberg::writer::file_writer::FileWriterBuilder">FileWriterBuilder</a>\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/trait.IcebergWriter.html" class="trait" title="trait iceberg::writer::IcebergWriter">IcebergWriter</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/base_writer/data_file_writer/struct.DataFileWriter.html" class="struct" title="struct iceberg::writer::base_writer::data_file_writer::DataFileWriter">DataFileWriter</a>\<B\>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/trait.IcebergWriter.html#impl-IcebergWriter-for-EqualityDeleteFileWriter%3CB%3E" class="anchor">§</a>

### impl\<B: <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriterBuilder.html" class="trait" title="trait iceberg::writer::file_writer::FileWriterBuilder">FileWriterBuilder</a>\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/trait.IcebergWriter.html" class="trait" title="trait iceberg::writer::IcebergWriter">IcebergWriter</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/base_writer/equality_delete_writer/struct.EqualityDeleteFileWriter.html" class="struct" title="struct iceberg::writer::base_writer::equality_delete_writer::EqualityDeleteFileWriter">EqualityDeleteFileWriter</a>\<B\>
