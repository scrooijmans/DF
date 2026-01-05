# Trait IcebergWriterBuilder Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/writer/mod.rs.html#239-246" class="src">Source</a>

``` rust
pub trait IcebergWriterBuilder<I = RecordBatch, O = Vec<DataFile>>:
    Send
    + Clone
    + 'static {
    type R: IcebergWriter<I, O>;

    // Required method
    fn build<'async_trait>(
        self,
    ) -> Pin<Box<dyn Future<Output = Result<Self::R>> + Send + 'async_trait>>
       where Self: 'async_trait;
}
```

Expand description

The builder for iceberg writer.

## Required Associated Types<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/trait.IcebergWriterBuilder.html#required-associated-types" class="anchor">§</a>

#### type <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/trait.IcebergWriterBuilder.html#associatedtype.R" class="associatedtype">R</a>: <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/trait.IcebergWriter.html" class="trait" title="trait iceberg::writer::IcebergWriter">IcebergWriter</a>\<I, O\>

The associated writer type.

## Required Methods<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/trait.IcebergWriterBuilder.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/trait.IcebergWriterBuilder.html#tymethod.build" class="fn">build</a>\<'async_trait\>( self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self::<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/trait.IcebergWriterBuilder.html#associatedtype.R" class="associatedtype" title="type iceberg::writer::IcebergWriterBuilder::R">R</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait,

Build the iceberg writer.

## Dyn Compatibility<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/trait.IcebergWriterBuilder.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/trait.IcebergWriterBuilder.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/trait.IcebergWriterBuilder.html#impl-IcebergWriterBuilder-for-DataFileWriterBuilder%3CB%3E" class="anchor">§</a>

### impl\<B: <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriterBuilder.html" class="trait" title="trait iceberg::writer::file_writer::FileWriterBuilder">FileWriterBuilder</a>\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/trait.IcebergWriterBuilder.html" class="trait" title="trait iceberg::writer::IcebergWriterBuilder">IcebergWriterBuilder</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/base_writer/data_file_writer/struct.DataFileWriterBuilder.html" class="struct" title="struct iceberg::writer::base_writer::data_file_writer::DataFileWriterBuilder">DataFileWriterBuilder</a>\<B\>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/trait.IcebergWriterBuilder.html#associatedtype.R-1" class="anchor">§</a>

#### type <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/trait.IcebergWriterBuilder.html#associatedtype.R" class="associatedtype">R</a> = <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/base_writer/data_file_writer/struct.DataFileWriter.html" class="struct" title="struct iceberg::writer::base_writer::data_file_writer::DataFileWriter">DataFileWriter</a>\<B\>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/trait.IcebergWriterBuilder.html#impl-IcebergWriterBuilder-for-EqualityDeleteFileWriterBuilder%3CB%3E" class="anchor">§</a>

### impl\<B: <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriterBuilder.html" class="trait" title="trait iceberg::writer::file_writer::FileWriterBuilder">FileWriterBuilder</a>\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/trait.IcebergWriterBuilder.html" class="trait" title="trait iceberg::writer::IcebergWriterBuilder">IcebergWriterBuilder</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/base_writer/equality_delete_writer/struct.EqualityDeleteFileWriterBuilder.html" class="struct" title="struct iceberg::writer::base_writer::equality_delete_writer::EqualityDeleteFileWriterBuilder">EqualityDeleteFileWriterBuilder</a>\<B\>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/trait.IcebergWriterBuilder.html#associatedtype.R-2" class="anchor">§</a>

#### type <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/trait.IcebergWriterBuilder.html#associatedtype.R" class="associatedtype">R</a> = <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/base_writer/equality_delete_writer/struct.EqualityDeleteFileWriter.html" class="struct" title="struct iceberg::writer::base_writer::equality_delete_writer::EqualityDeleteFileWriter">EqualityDeleteFileWriter</a>\<B\>
