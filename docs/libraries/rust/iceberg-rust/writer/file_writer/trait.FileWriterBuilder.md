# Trait FileWriterBuilder Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/writer/file_writer/mod.rs.html#37-42" class="src">Source</a>

``` rust
pub trait FileWriterBuilder<O = Vec<DataFileBuilder>>:
    Send
    + Clone
    + 'static {
    type R: FileWriter<O>;

    // Required method
    fn build(self) -> impl Future<Output = Result<Self::R>> + Send;
}
```

Expand description

File writer builder trait.

## Required Associated Types<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriterBuilder.html#required-associated-types" class="anchor">§</a>

#### type <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriterBuilder.html#associatedtype.R" class="associatedtype">R</a>: <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriter.html" class="trait" title="trait iceberg::writer::file_writer::FileWriter">FileWriter</a>\<O\>

The associated file writer type.

## Required Methods<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriterBuilder.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriterBuilder.html#tymethod.build" class="fn">build</a>(self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self::<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriterBuilder.html#associatedtype.R" class="associatedtype" title="type iceberg::writer::file_writer::FileWriterBuilder::R">R</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>

Build file writer.

## Dyn Compatibility<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriterBuilder.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriterBuilder.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriterBuilder.html#impl-FileWriterBuilder-for-RollingFileWriterBuilder%3CB%3E" class="anchor">§</a>

### impl\<B: <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriterBuilder.html" class="trait" title="trait iceberg::writer::file_writer::FileWriterBuilder">FileWriterBuilder</a>\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriterBuilder.html" class="trait" title="trait iceberg::writer::file_writer::FileWriterBuilder">FileWriterBuilder</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/rolling_writer/struct.RollingFileWriterBuilder.html" class="struct" title="struct iceberg::writer::file_writer::rolling_writer::RollingFileWriterBuilder">RollingFileWriterBuilder</a>\<B\>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriterBuilder.html#associatedtype.R-1" class="anchor">§</a>

#### type <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriterBuilder.html#associatedtype.R" class="associatedtype">R</a> = <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/rolling_writer/struct.RollingFileWriter.html" class="struct" title="struct iceberg::writer::file_writer::rolling_writer::RollingFileWriter">RollingFileWriter</a>\<B\>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriterBuilder.html#impl-FileWriterBuilder-for-ParquetWriterBuilder%3CT,+F%3E" class="anchor">§</a>

### impl\<T: <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/trait.LocationGenerator.html" class="trait" title="trait iceberg::writer::file_writer::location_generator::LocationGenerator">LocationGenerator</a>, F: <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/trait.FileNameGenerator.html" class="trait" title="trait iceberg::writer::file_writer::location_generator::FileNameGenerator">FileNameGenerator</a>\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriterBuilder.html" class="trait" title="trait iceberg::writer::file_writer::FileWriterBuilder">FileWriterBuilder</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/struct.ParquetWriterBuilder.html" class="struct" title="struct iceberg::writer::file_writer::ParquetWriterBuilder">ParquetWriterBuilder</a>\<T, F\>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriterBuilder.html#associatedtype.R-2" class="anchor">§</a>

#### type <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriterBuilder.html#associatedtype.R" class="associatedtype">R</a> = <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/struct.ParquetWriter.html" class="struct" title="struct iceberg::writer::file_writer::ParquetWriter">ParquetWriter</a>
