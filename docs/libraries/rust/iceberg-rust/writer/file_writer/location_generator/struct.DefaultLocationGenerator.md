# Struct DefaultLocationGenerator Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/writer/file_writer/location_generator.rs.html#51-53" class="src">Source</a>

``` rust
pub struct DefaultLocationGenerator { /* private fields */ }
```

Expand description

`DefaultLocationGenerator` used to generate the data dir location of data file. The location is generated based on the table location and the data location in table properties.

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/struct.DefaultLocationGenerator.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/struct.DefaultLocationGenerator.html#impl-DefaultLocationGenerator" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/struct.DefaultLocationGenerator.html" class="struct" title="struct iceberg::writer::file_writer::location_generator::DefaultLocationGenerator">DefaultLocationGenerator</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/struct.DefaultLocationGenerator.html#method.new" class="fn">new</a>(table_metadata: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html" class="struct" title="struct iceberg::spec::TableMetadata">TableMetadata</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Create a new `DefaultLocationGenerator`.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/struct.DefaultLocationGenerator.html#method.with_data_location" class="fn">with_data_location</a>(data_location: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>) -\> Self

Create a new `DefaultLocationGenerator` with a specified data location.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/struct.DefaultLocationGenerator.html#arguments" class="doc-anchor">§</a>Arguments

- `data_location` - The data location to use for generating file locations.

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/struct.DefaultLocationGenerator.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/struct.DefaultLocationGenerator.html#impl-Clone-for-DefaultLocationGenerator" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/struct.DefaultLocationGenerator.html" class="struct" title="struct iceberg::writer::file_writer::location_generator::DefaultLocationGenerator">DefaultLocationGenerator</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/struct.DefaultLocationGenerator.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/struct.DefaultLocationGenerator.html" class="struct" title="struct iceberg::writer::file_writer::location_generator::DefaultLocationGenerator">DefaultLocationGenerator</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/struct.DefaultLocationGenerator.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/struct.DefaultLocationGenerator.html#impl-Debug-for-DefaultLocationGenerator" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/struct.DefaultLocationGenerator.html" class="struct" title="struct iceberg::writer::file_writer::location_generator::DefaultLocationGenerator">DefaultLocationGenerator</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/struct.DefaultLocationGenerator.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/struct.DefaultLocationGenerator.html#impl-LocationGenerator-for-DefaultLocationGenerator" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/trait.LocationGenerator.html" class="trait" title="trait iceberg::writer::file_writer::location_generator::LocationGenerator">LocationGenerator</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/struct.DefaultLocationGenerator.html" class="struct" title="struct iceberg::writer::file_writer::location_generator::DefaultLocationGenerator">DefaultLocationGenerator</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/struct.DefaultLocationGenerator.html#method.generate_location" class="anchor">§</a>

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/trait.LocationGenerator.html#tymethod.generate_location" class="fn">generate_location</a>( &self, partition_key: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionKey.html" class="struct" title="struct iceberg::spec::PartitionKey">PartitionKey</a>\>, file_name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

Generate an absolute path for the given file name that includes the partition path. [Read more](https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/trait.LocationGenerator.html#tymethod.generate_location)

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/struct.DefaultLocationGenerator.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/struct.DefaultLocationGenerator.html#blanket-implementations" class="anchor">§</a>
