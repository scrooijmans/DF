# Struct ParquetWriterBuilder Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/writer/file_writer/parquet_writer.rs.html#55-64" class="src">Source</a>

``` rust
pub struct ParquetWriterBuilder<T: LocationGenerator, F: FileNameGenerator> { /* private fields */ }
```

Expand description

ParquetWriterBuilder is used to builder a [`ParquetWriter`](https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/struct.ParquetWriter.html "struct iceberg::writer::file_writer::ParquetWriter")

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/struct.ParquetWriterBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/struct.ParquetWriterBuilder.html#impl-ParquetWriterBuilder%3CT,+F%3E" class="anchor">§</a>

### impl\<T: <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/trait.LocationGenerator.html" class="trait" title="trait iceberg::writer::file_writer::location_generator::LocationGenerator">LocationGenerator</a>, F: <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/trait.FileNameGenerator.html" class="trait" title="trait iceberg::writer::file_writer::location_generator::FileNameGenerator">FileNameGenerator</a>\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/struct.ParquetWriterBuilder.html" class="struct" title="struct iceberg::writer::file_writer::ParquetWriterBuilder">ParquetWriterBuilder</a>\<T, F\>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/struct.ParquetWriterBuilder.html#method.new" class="fn">new</a>( props: <a href="https://docs.rs/parquet/55.2.0/x86_64-unknown-linux-gnu/parquet/file/properties/struct.WriterProperties.html" class="struct" title="struct parquet::file::properties::WriterProperties">WriterProperties</a>, schema: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.SchemaRef.html" class="type" title="type iceberg::spec::SchemaRef">SchemaRef</a>, partition_key: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionKey.html" class="struct" title="struct iceberg::spec::PartitionKey">PartitionKey</a>\>, file_io: <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIO.html" class="struct" title="struct iceberg::io::FileIO">FileIO</a>, location_generator: T, file_name_generator: F, ) -\> Self

Create a new `ParquetWriterBuilder` To construct the write result, the schema should contain the `PARQUET_FIELD_ID_META_KEY` metadata for each field.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/struct.ParquetWriterBuilder.html#method.new_with_match_mode" class="fn">new_with_match_mode</a>( props: <a href="https://docs.rs/parquet/55.2.0/x86_64-unknown-linux-gnu/parquet/file/properties/struct.WriterProperties.html" class="struct" title="struct parquet::file::properties::WriterProperties">WriterProperties</a>, schema: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.SchemaRef.html" class="type" title="type iceberg::spec::SchemaRef">SchemaRef</a>, partition_key: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionKey.html" class="struct" title="struct iceberg::spec::PartitionKey">PartitionKey</a>\>, match_mode: <a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/enum.FieldMatchMode.html" class="enum" title="enum iceberg::arrow::FieldMatchMode">FieldMatchMode</a>, file_io: <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIO.html" class="struct" title="struct iceberg::io::FileIO">FileIO</a>, location_generator: T, file_name_generator: F, ) -\> Self

Create a new `ParquetWriterBuilder` with custom match mode

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/struct.ParquetWriterBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/struct.ParquetWriterBuilder.html#impl-Clone-for-ParquetWriterBuilder%3CT,+F%3E" class="anchor">§</a>

### impl\<T: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> + <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/trait.LocationGenerator.html" class="trait" title="trait iceberg::writer::file_writer::location_generator::LocationGenerator">LocationGenerator</a>, F: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> + <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/trait.FileNameGenerator.html" class="trait" title="trait iceberg::writer::file_writer::location_generator::FileNameGenerator">FileNameGenerator</a>\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/struct.ParquetWriterBuilder.html" class="struct" title="struct iceberg::writer::file_writer::ParquetWriterBuilder">ParquetWriterBuilder</a>\<T, F\>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/struct.ParquetWriterBuilder.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/struct.ParquetWriterBuilder.html" class="struct" title="struct iceberg::writer::file_writer::ParquetWriterBuilder">ParquetWriterBuilder</a>\<T, F\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/struct.ParquetWriterBuilder.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/struct.ParquetWriterBuilder.html#impl-Debug-for-ParquetWriterBuilder%3CT,+F%3E" class="anchor">§</a>

### impl\<T: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> + <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/trait.LocationGenerator.html" class="trait" title="trait iceberg::writer::file_writer::location_generator::LocationGenerator">LocationGenerator</a>, F: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> + <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/trait.FileNameGenerator.html" class="trait" title="trait iceberg::writer::file_writer::location_generator::FileNameGenerator">FileNameGenerator</a>\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/struct.ParquetWriterBuilder.html" class="struct" title="struct iceberg::writer::file_writer::ParquetWriterBuilder">ParquetWriterBuilder</a>\<T, F\>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/struct.ParquetWriterBuilder.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/struct.ParquetWriterBuilder.html#impl-FileWriterBuilder-for-ParquetWriterBuilder%3CT,+F%3E" class="anchor">§</a>

### impl\<T: <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/trait.LocationGenerator.html" class="trait" title="trait iceberg::writer::file_writer::location_generator::LocationGenerator">LocationGenerator</a>, F: <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/trait.FileNameGenerator.html" class="trait" title="trait iceberg::writer::file_writer::location_generator::FileNameGenerator">FileNameGenerator</a>\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriterBuilder.html" class="trait" title="trait iceberg::writer::file_writer::FileWriterBuilder">FileWriterBuilder</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/struct.ParquetWriterBuilder.html" class="struct" title="struct iceberg::writer::file_writer::ParquetWriterBuilder">ParquetWriterBuilder</a>\<T, F\>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/struct.ParquetWriterBuilder.html#associatedtype.R" class="anchor">§</a>

#### type <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriterBuilder.html#associatedtype.R" class="associatedtype">R</a> = <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/struct.ParquetWriter.html" class="struct" title="struct iceberg::writer::file_writer::ParquetWriter">ParquetWriter</a>

The associated file writer type.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/struct.ParquetWriterBuilder.html#method.build" class="anchor">§</a>

#### async fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriterBuilder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self::<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriterBuilder.html#associatedtype.R" class="associatedtype" title="type iceberg::writer::file_writer::FileWriterBuilder::R">R</a>\>

Build file writer.

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/struct.ParquetWriterBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/struct.ParquetWriterBuilder.html#blanket-implementations" class="anchor">§</a>
