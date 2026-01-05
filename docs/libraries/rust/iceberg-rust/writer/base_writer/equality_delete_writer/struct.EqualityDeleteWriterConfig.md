# Struct EqualityDeleteWriterConfig Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/writer/base_writer/equality_delete_writer.rs.html#50-57" class="src">Source</a>

``` rust
pub struct EqualityDeleteWriterConfig { /* private fields */ }
```

Expand description

Config for `EqualityDeleteWriter`.

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/base_writer/equality_delete_writer/struct.EqualityDeleteWriterConfig.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/base_writer/equality_delete_writer/struct.EqualityDeleteWriterConfig.html#impl-EqualityDeleteWriterConfig" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/base_writer/equality_delete_writer/struct.EqualityDeleteWriterConfig.html" class="struct" title="struct iceberg::writer::base_writer::equality_delete_writer::EqualityDeleteWriterConfig">EqualityDeleteWriterConfig</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/base_writer/equality_delete_writer/struct.EqualityDeleteWriterConfig.html#method.new" class="fn">new</a>( equality_ids: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>, original_schema: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.SchemaRef.html" class="type" title="type iceberg::spec::SchemaRef">SchemaRef</a>, partition_value: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Struct.html" class="struct" title="struct iceberg::spec::Struct">Struct</a>\>, partition_spec_id: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, ) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Create a new `DataFileWriterConfig` with equality ids.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/base_writer/equality_delete_writer/struct.EqualityDeleteWriterConfig.html#method.projected_arrow_schema_ref" class="fn">projected_arrow_schema_ref</a>(&self) -\> &<a href="https://docs.rs/arrow-schema/55.2.0/x86_64-unknown-linux-gnu/arrow_schema/schema/type.SchemaRef.html" class="type" title="type arrow_schema::schema::SchemaRef">ArrowSchemaRef</a>

Return projected Schema

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/base_writer/equality_delete_writer/struct.EqualityDeleteWriterConfig.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/base_writer/equality_delete_writer/struct.EqualityDeleteWriterConfig.html#impl-Clone-for-EqualityDeleteWriterConfig" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/base_writer/equality_delete_writer/struct.EqualityDeleteWriterConfig.html" class="struct" title="struct iceberg::writer::base_writer::equality_delete_writer::EqualityDeleteWriterConfig">EqualityDeleteWriterConfig</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/base_writer/equality_delete_writer/struct.EqualityDeleteWriterConfig.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/base_writer/equality_delete_writer/struct.EqualityDeleteWriterConfig.html" class="struct" title="struct iceberg::writer::base_writer::equality_delete_writer::EqualityDeleteWriterConfig">EqualityDeleteWriterConfig</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/base_writer/equality_delete_writer/struct.EqualityDeleteWriterConfig.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/base_writer/equality_delete_writer/struct.EqualityDeleteWriterConfig.html#impl-Debug-for-EqualityDeleteWriterConfig" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/base_writer/equality_delete_writer/struct.EqualityDeleteWriterConfig.html" class="struct" title="struct iceberg::writer::base_writer::equality_delete_writer::EqualityDeleteWriterConfig">EqualityDeleteWriterConfig</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/base_writer/equality_delete_writer/struct.EqualityDeleteWriterConfig.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/base_writer/equality_delete_writer/struct.EqualityDeleteWriterConfig.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/base_writer/equality_delete_writer/struct.EqualityDeleteWriterConfig.html#blanket-implementations" class="anchor">§</a>
