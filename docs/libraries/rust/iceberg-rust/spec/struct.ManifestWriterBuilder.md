# Struct ManifestWriterBuilder Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/manifest/writer.rs.html#40-46" class="src">Source</a>

``` rust
pub struct ManifestWriterBuilder { /* private fields */ }
```

Expand description

The builder used to create a [`ManifestWriter`](https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestWriter.html "struct iceberg::spec::ManifestWriter").

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestWriterBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestWriterBuilder.html#impl-ManifestWriterBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestWriterBuilder.html" class="struct" title="struct iceberg::spec::ManifestWriterBuilder">ManifestWriterBuilder</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestWriterBuilder.html#method.new" class="fn">new</a>( output: <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.OutputFile.html" class="struct" title="struct iceberg::io::OutputFile">OutputFile</a>, snapshot_id: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>, key_metadata: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\>, schema: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.SchemaRef.html" class="type" title="type iceberg::spec::SchemaRef">SchemaRef</a>, partition_spec: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html" class="struct" title="struct iceberg::spec::PartitionSpec">PartitionSpec</a>, ) -\> Self

Create a new builder.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestWriterBuilder.html#method.build_v1" class="fn">build_v1</a>(self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestWriter.html" class="struct" title="struct iceberg::spec::ManifestWriter">ManifestWriter</a>

Build a [`ManifestWriter`](https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestWriter.html "struct iceberg::spec::ManifestWriter") for format version 1.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestWriterBuilder.html#method.build_v2_data" class="fn">build_v2_data</a>(self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestWriter.html" class="struct" title="struct iceberg::spec::ManifestWriter">ManifestWriter</a>

Build a [`ManifestWriter`](https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestWriter.html "struct iceberg::spec::ManifestWriter") for format version 2, data content.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestWriterBuilder.html#method.build_v2_deletes" class="fn">build_v2_deletes</a>(self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestWriter.html" class="struct" title="struct iceberg::spec::ManifestWriter">ManifestWriter</a>

Build a [`ManifestWriter`](https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestWriter.html "struct iceberg::spec::ManifestWriter") for format version 2, deletes content.

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestWriterBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestWriterBuilder.html#blanket-implementations" class="anchor">§</a>
