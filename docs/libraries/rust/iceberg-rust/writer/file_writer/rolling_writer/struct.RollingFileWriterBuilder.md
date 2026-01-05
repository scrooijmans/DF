# Struct RollingFileWriterBuilder Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/writer/file_writer/rolling_writer.rs.html#28-31" class="src">Source</a>

``` rust
pub struct RollingFileWriterBuilder<B: FileWriterBuilder> { /* private fields */ }
```

Expand description

Builder for creating a `RollingFileWriter` that rolls over to a new file when the data size exceeds a target threshold.

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/rolling_writer/struct.RollingFileWriterBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/rolling_writer/struct.RollingFileWriterBuilder.html#impl-RollingFileWriterBuilder%3CB%3E" class="anchor">§</a>

### impl\<B: <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriterBuilder.html" class="trait" title="trait iceberg::writer::file_writer::FileWriterBuilder">FileWriterBuilder</a>\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/rolling_writer/struct.RollingFileWriterBuilder.html" class="struct" title="struct iceberg::writer::file_writer::rolling_writer::RollingFileWriterBuilder">RollingFileWriterBuilder</a>\<B\>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/rolling_writer/struct.RollingFileWriterBuilder.html#method.new" class="fn">new</a>(inner_builder: B, target_file_size: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

Creates a new `RollingFileWriterBuilder` with the specified inner builder and target size.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/rolling_writer/struct.RollingFileWriterBuilder.html#arguments" class="doc-anchor">§</a>Arguments

- `inner_builder` - The builder for the underlying file writer
- `target_file_size` - The target size in bytes before rolling over to a new file

NOTE: The `target_file_size` does not exactly reflect the final size on physical storage. This is because the input size is based on the Arrow in-memory format and cannot precisely control rollover behavior. The actual file size on disk is expected to be slightly larger than `target_file_size`.

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/rolling_writer/struct.RollingFileWriterBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/rolling_writer/struct.RollingFileWriterBuilder.html#impl-Clone-for-RollingFileWriterBuilder%3CB%3E" class="anchor">§</a>

### impl\<B: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> + <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriterBuilder.html" class="trait" title="trait iceberg::writer::file_writer::FileWriterBuilder">FileWriterBuilder</a>\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/rolling_writer/struct.RollingFileWriterBuilder.html" class="struct" title="struct iceberg::writer::file_writer::rolling_writer::RollingFileWriterBuilder">RollingFileWriterBuilder</a>\<B\>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/rolling_writer/struct.RollingFileWriterBuilder.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/rolling_writer/struct.RollingFileWriterBuilder.html" class="struct" title="struct iceberg::writer::file_writer::rolling_writer::RollingFileWriterBuilder">RollingFileWriterBuilder</a>\<B\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/rolling_writer/struct.RollingFileWriterBuilder.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/rolling_writer/struct.RollingFileWriterBuilder.html#impl-FileWriterBuilder-for-RollingFileWriterBuilder%3CB%3E" class="anchor">§</a>

### impl\<B: <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriterBuilder.html" class="trait" title="trait iceberg::writer::file_writer::FileWriterBuilder">FileWriterBuilder</a>\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriterBuilder.html" class="trait" title="trait iceberg::writer::file_writer::FileWriterBuilder">FileWriterBuilder</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/rolling_writer/struct.RollingFileWriterBuilder.html" class="struct" title="struct iceberg::writer::file_writer::rolling_writer::RollingFileWriterBuilder">RollingFileWriterBuilder</a>\<B\>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/rolling_writer/struct.RollingFileWriterBuilder.html#associatedtype.R" class="anchor">§</a>

#### type <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriterBuilder.html#associatedtype.R" class="associatedtype">R</a> = <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/rolling_writer/struct.RollingFileWriter.html" class="struct" title="struct iceberg::writer::file_writer::rolling_writer::RollingFileWriter">RollingFileWriter</a>\<B\>

The associated file writer type.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/rolling_writer/struct.RollingFileWriterBuilder.html#method.build" class="anchor">§</a>

#### async fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriterBuilder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self::<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriterBuilder.html#associatedtype.R" class="associatedtype" title="type iceberg::writer::file_writer::FileWriterBuilder::R">R</a>\>

Build file writer.

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/rolling_writer/struct.RollingFileWriterBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/rolling_writer/struct.RollingFileWriterBuilder.html#blanket-implementations" class="anchor">§</a>
