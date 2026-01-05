# Struct ObjectWriterBuilder Copy item path

<a href="https://docs.rs/datafusion-datasource/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_datasource/write/mod.rs.html#125" class="src">Source</a>

``` rust
pub struct ObjectWriterBuilder { /* private fields */ }
```

Expand description

A builder for an [`AsyncWrite`](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/io/async_write/trait.AsyncWrite.html "trait tokio::io::async_write::AsyncWrite") that writes to an object store location.

This can be used to specify file compression on the writer. The writer will have a default buffer size unless altered. The specific default size is chosen by [`BufWriter::new`](https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/buffered/struct.BufWriter.html#method.new "associated function object_store::buffered::BufWriter::new").

We drop the `AbortableWrite` struct and the writer will not try to cleanup on failure. Users can configure automatic cleanup with their cloud provider.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/write/struct.ObjectWriterBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/write/struct.ObjectWriterBuilder.html#impl-ObjectWriterBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/write/struct.ObjectWriterBuilder.html" class="struct" title="struct datafusion::datasource::file_format::write::ObjectWriterBuilder">ObjectWriterBuilder</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/write/struct.ObjectWriterBuilder.html#method.new" class="fn">new</a>( file_compression_type: <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/file_compression_type/struct.FileCompressionType.html" class="struct" title="struct datafusion::datasource::file_format::file_compression_type::FileCompressionType">FileCompressionType</a>, location: &<a href="https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, object_store: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/write/struct.ObjectWriterBuilder.html" class="struct" title="struct datafusion::datasource::file_format::write::ObjectWriterBuilder">ObjectWriterBuilder</a>

Create a new [`ObjectWriterBuilder`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/write/struct.ObjectWriterBuilder.html "struct datafusion::datasource::file_format::write::ObjectWriterBuilder") for the specified path and compression type.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/write/struct.ObjectWriterBuilder.html#method.set_buffer_size" class="fn">set_buffer_size</a>(&mut self, buffer_size: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>)

Set buffer size in bytes for object writer.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/write/struct.ObjectWriterBuilder.html#example" class="doc-anchor">§</a>Example

``` rust
let mut builder = ObjectWriterBuilder::new(compression_type, &location, object_store);
builder.set_buffer_size(Some(20 * 1024 * 1024)); //20 MiB
assert_eq!(builder.get_buffer_size(), Some(20 * 1024 * 1024), "Internal error: Builder buffer size doesn't match");
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/write/struct.ObjectWriterBuilder.html#method.with_buffer_size" class="fn">with_buffer_size</a>(self, buffer_size: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/write/struct.ObjectWriterBuilder.html" class="struct" title="struct datafusion::datasource::file_format::write::ObjectWriterBuilder">ObjectWriterBuilder</a>

Set buffer size in bytes for object writer, returning the builder.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/write/struct.ObjectWriterBuilder.html#example-1" class="doc-anchor">§</a>Example

``` rust
let builder = ObjectWriterBuilder::new(compression_type, &location, object_store)
    .with_buffer_size(Some(20 * 1024 * 1024)); //20 MiB
assert_eq!(builder.get_buffer_size(), Some(20 * 1024 * 1024), "Internal error: Builder buffer size doesn't match");
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/write/struct.ObjectWriterBuilder.html#method.get_buffer_size" class="fn">get_buffer_size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

Currently specified buffer size in bytes.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/write/struct.ObjectWriterBuilder.html#method.build" class="fn">build</a>( self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/io/async_write/trait.AsyncWrite.html" class="trait" title="trait tokio::io::async_write::AsyncWrite">AsyncWrite</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html" class="trait" title="trait core::marker::Unpin">Unpin</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Return a writer object that writes to the object store location.

If a buffer size has not been set, the default buffer buffer size will be used.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/write/struct.ObjectWriterBuilder.html#errors" class="doc-anchor">§</a>Errors

If there is an error applying the compression type.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/write/struct.ObjectWriterBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/write/struct.ObjectWriterBuilder.html#impl-Debug-for-ObjectWriterBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/write/struct.ObjectWriterBuilder.html" class="struct" title="struct datafusion::datasource::file_format::write::ObjectWriterBuilder">ObjectWriterBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/write/struct.ObjectWriterBuilder.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/write/struct.ObjectWriterBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/write/struct.ObjectWriterBuilder.html#blanket-implementations" class="anchor">§</a>
