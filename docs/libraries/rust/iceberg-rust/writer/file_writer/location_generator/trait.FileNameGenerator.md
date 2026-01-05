# Trait FileNameGenerator Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/writer/file_writer/location_generator.rs.html#97-100" class="src">Source</a>

``` rust
pub trait FileNameGenerator:
    Clone
    + Send
    + 'static {
    // Required method
    fn generate_file_name(&self) -> String;
}
```

Expand description

`FileNameGeneratorTrait` used to generate file name for data file. The file name can be passed to `LocationGenerator` to generate the location of the file.

## Required Methods<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/trait.FileNameGenerator.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/trait.FileNameGenerator.html#tymethod.generate_file_name" class="fn">generate_file_name</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

Generate a file name.

## Dyn Compatibility<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/trait.FileNameGenerator.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/trait.FileNameGenerator.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/trait.FileNameGenerator.html#impl-FileNameGenerator-for-DefaultFileNameGenerator" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/trait.FileNameGenerator.html" class="trait" title="trait iceberg::writer::file_writer::location_generator::FileNameGenerator">FileNameGenerator</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/struct.DefaultFileNameGenerator.html" class="struct" title="struct iceberg::writer::file_writer::location_generator::DefaultFileNameGenerator">DefaultFileNameGenerator</a>
