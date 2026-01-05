# Trait LocationGenerator Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/writer/file_writer/location_generator.rs.html#27-42" class="src">Source</a>

``` rust
pub trait LocationGenerator:
    Clone
    + Send
    + 'static {
    // Required method
    fn generate_location(
        &self,
        partition_key: Option<&PartitionKey>,
        file_name: &str,
    ) -> String;
}
```

Expand description

`LocationGenerator` used to generate the location of data file.

## Required Methods<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/trait.LocationGenerator.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/trait.LocationGenerator.html#tymethod.generate_location" class="fn">generate_location</a>( &self, partition_key: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionKey.html" class="struct" title="struct iceberg::spec::PartitionKey">PartitionKey</a>\>, file_name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

Generate an absolute path for the given file name that includes the partition path.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/trait.LocationGenerator.html#arguments" class="doc-anchor">§</a>Arguments

- `partition_key` - The partition key of the file. If None, generate a non-partitioned path.
- `file_name` - The name of the file

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/trait.LocationGenerator.html#returns" class="doc-anchor">§</a>Returns

An absolute path that includes the partition path, e.g., “/table/data/id=1/name=alice/part-00000.parquet” or non-partitioned path: “/table/data/part-00000.parquet”

## Dyn Compatibility<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/trait.LocationGenerator.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/trait.LocationGenerator.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/trait.LocationGenerator.html#impl-LocationGenerator-for-DefaultLocationGenerator" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/trait.LocationGenerator.html" class="trait" title="trait iceberg::writer::file_writer::location_generator::LocationGenerator">LocationGenerator</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/struct.DefaultLocationGenerator.html" class="struct" title="struct iceberg::writer::file_writer::location_generator::DefaultLocationGenerator">DefaultLocationGenerator</a>
