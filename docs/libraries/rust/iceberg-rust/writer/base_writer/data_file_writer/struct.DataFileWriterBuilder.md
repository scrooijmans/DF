# Struct DataFileWriterBuilder Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/writer/base_writer/data_file_writer.rs.html#30-34" class="src">Source</a>

``` rust
pub struct DataFileWriterBuilder<B: FileWriterBuilder> { /* private fields */ }
```

Expand description

Builder for `DataFileWriter`.

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/base_writer/data_file_writer/struct.DataFileWriterBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/base_writer/data_file_writer/struct.DataFileWriterBuilder.html#impl-DataFileWriterBuilder%3CB%3E" class="anchor">§</a>

### impl\<B: <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriterBuilder.html" class="trait" title="trait iceberg::writer::file_writer::FileWriterBuilder">FileWriterBuilder</a>\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/base_writer/data_file_writer/struct.DataFileWriterBuilder.html" class="struct" title="struct iceberg::writer::base_writer::data_file_writer::DataFileWriterBuilder">DataFileWriterBuilder</a>\<B\>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/base_writer/data_file_writer/struct.DataFileWriterBuilder.html#method.new" class="fn">new</a>( inner: B, partition_value: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Struct.html" class="struct" title="struct iceberg::spec::Struct">Struct</a>\>, partition_spec_id: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, ) -\> Self

Create a new `DataFileWriterBuilder` using a `FileWriterBuilder`.

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/base_writer/data_file_writer/struct.DataFileWriterBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/base_writer/data_file_writer/struct.DataFileWriterBuilder.html#impl-Clone-for-DataFileWriterBuilder%3CB%3E" class="anchor">§</a>

### impl\<B: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> + <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriterBuilder.html" class="trait" title="trait iceberg::writer::file_writer::FileWriterBuilder">FileWriterBuilder</a>\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/base_writer/data_file_writer/struct.DataFileWriterBuilder.html" class="struct" title="struct iceberg::writer::base_writer::data_file_writer::DataFileWriterBuilder">DataFileWriterBuilder</a>\<B\>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/base_writer/data_file_writer/struct.DataFileWriterBuilder.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/base_writer/data_file_writer/struct.DataFileWriterBuilder.html" class="struct" title="struct iceberg::writer::base_writer::data_file_writer::DataFileWriterBuilder">DataFileWriterBuilder</a>\<B\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/base_writer/data_file_writer/struct.DataFileWriterBuilder.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/base_writer/data_file_writer/struct.DataFileWriterBuilder.html#impl-Debug-for-DataFileWriterBuilder%3CB%3E" class="anchor">§</a>

### impl\<B: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> + <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriterBuilder.html" class="trait" title="trait iceberg::writer::file_writer::FileWriterBuilder">FileWriterBuilder</a>\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/base_writer/data_file_writer/struct.DataFileWriterBuilder.html" class="struct" title="struct iceberg::writer::base_writer::data_file_writer::DataFileWriterBuilder">DataFileWriterBuilder</a>\<B\>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/base_writer/data_file_writer/struct.DataFileWriterBuilder.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/base_writer/data_file_writer/struct.DataFileWriterBuilder.html#impl-IcebergWriterBuilder-for-DataFileWriterBuilder%3CB%3E" class="anchor">§</a>

### impl\<B: <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriterBuilder.html" class="trait" title="trait iceberg::writer::file_writer::FileWriterBuilder">FileWriterBuilder</a>\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/trait.IcebergWriterBuilder.html" class="trait" title="trait iceberg::writer::IcebergWriterBuilder">IcebergWriterBuilder</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/base_writer/data_file_writer/struct.DataFileWriterBuilder.html" class="struct" title="struct iceberg::writer::base_writer::data_file_writer::DataFileWriterBuilder">DataFileWriterBuilder</a>\<B\>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/base_writer/data_file_writer/struct.DataFileWriterBuilder.html#associatedtype.R" class="anchor">§</a>

#### type <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/trait.IcebergWriterBuilder.html#associatedtype.R" class="associatedtype">R</a> = <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/base_writer/data_file_writer/struct.DataFileWriter.html" class="struct" title="struct iceberg::writer::base_writer::data_file_writer::DataFileWriter">DataFileWriter</a>\<B\>

The associated writer type.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/base_writer/data_file_writer/struct.DataFileWriterBuilder.html#method.build" class="anchor">§</a>

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/trait.IcebergWriterBuilder.html#tymethod.build" class="fn">build</a>\<'async_trait\>( self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self::<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/trait.IcebergWriterBuilder.html#associatedtype.R" class="associatedtype" title="type iceberg::writer::IcebergWriterBuilder::R">R</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait,

Build the iceberg writer.

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/base_writer/data_file_writer/struct.DataFileWriterBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/base_writer/data_file_writer/struct.DataFileWriterBuilder.html#blanket-implementations" class="anchor">§</a>
