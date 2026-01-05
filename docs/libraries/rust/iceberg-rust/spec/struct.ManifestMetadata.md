# Struct ManifestMetadata Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/manifest/metadata.rs.html#30-42" class="src">Source</a>

``` rust
pub struct ManifestMetadata {
    pub schema: SchemaRef,
    pub schema_id: SchemaId,
    pub partition_spec: PartitionSpec,
    pub format_version: FormatVersion,
    pub content: ManifestContentType,
}
```

Expand description

Meta data of a manifest that is stored in the key-value metadata of the Avro file

## Fields<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestMetadata.html#fields" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestMetadata.html#structfield.schema" class="anchor field">§</a>`schema: `<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.SchemaRef.html" class="type" title="type iceberg::spec::SchemaRef"><code>SchemaRef</code></a>

The table schema at the time the manifest was written

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestMetadata.html#structfield.schema_id" class="anchor field">§</a>`schema_id: `<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.SchemaId.html" class="type" title="type iceberg::spec::SchemaId"><code>SchemaId</code></a>

ID of the schema used to write the manifest as a string

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestMetadata.html#structfield.partition_spec" class="anchor field">§</a>`partition_spec: `<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html" class="struct" title="struct iceberg::spec::PartitionSpec"><code>PartitionSpec</code></a>

The partition spec used to write the manifest

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestMetadata.html#structfield.format_version" class="anchor field">§</a>`format_version: `<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.FormatVersion.html" class="enum" title="enum iceberg::spec::FormatVersion"><code>FormatVersion</code></a>

Table format version number of the manifest as a string

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestMetadata.html#structfield.content" class="anchor field">§</a>`content: `<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.ManifestContentType.html" class="enum" title="enum iceberg::spec::ManifestContentType"><code>ManifestContentType</code></a>

Type of content files tracked by the manifest: “data” or “deletes”

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestMetadata.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestMetadata.html#impl-ManifestMetadata" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestMetadata.html" class="struct" title="struct iceberg::spec::ManifestMetadata">ManifestMetadata</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestMetadata.html#method.builder" class="fn">builder</a>() -\> ManifestMetadataBuilder\<(<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>)\>

Create a builder for building `ManifestMetadata`. On the builder, call `.schema(...)`, `.schema_id(...)`, `.partition_spec(...)`, `.format_version(...)`, `.content(...)` to set the values of the fields. Finally, call `.build()` to create the instance of `ManifestMetadata`.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestMetadata.html#impl-ManifestMetadata-1" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestMetadata.html" class="struct" title="struct iceberg::spec::ManifestMetadata">ManifestMetadata</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestMetadata.html#method.parse" class="fn">parse</a>(meta: &<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Parse from metadata in avro file.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestMetadata.html#method.schema" class="fn">schema</a>(&self) -\> &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.SchemaRef.html" class="type" title="type iceberg::spec::SchemaRef">SchemaRef</a>

Get the schema of table at the time manifest was written

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestMetadata.html#method.schema_id" class="fn">schema_id</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.SchemaId.html" class="type" title="type iceberg::spec::SchemaId">SchemaId</a>

Get the ID of schema used to write the manifest

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestMetadata.html#method.partition_spec" class="fn">partition_spec</a>(&self) -\> &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html" class="struct" title="struct iceberg::spec::PartitionSpec">PartitionSpec</a>

Get the partition spec used to write manifest

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestMetadata.html#method.format_version" class="fn">format_version</a>(&self) -\> &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.FormatVersion.html" class="enum" title="enum iceberg::spec::FormatVersion">FormatVersion</a>

Get the table format version

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestMetadata.html#method.content" class="fn">content</a>(&self) -\> &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.ManifestContentType.html" class="enum" title="enum iceberg::spec::ManifestContentType">ManifestContentType</a>

Get the type of content files tracked by manifest

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestMetadata.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestMetadata.html#impl-Clone-for-ManifestMetadata" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestMetadata.html" class="struct" title="struct iceberg::spec::ManifestMetadata">ManifestMetadata</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestMetadata.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestMetadata.html" class="struct" title="struct iceberg::spec::ManifestMetadata">ManifestMetadata</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestMetadata.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestMetadata.html#impl-Debug-for-ManifestMetadata" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestMetadata.html" class="struct" title="struct iceberg::spec::ManifestMetadata">ManifestMetadata</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestMetadata.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestMetadata.html#impl-PartialEq-for-ManifestMetadata" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestMetadata.html" class="struct" title="struct iceberg::spec::ManifestMetadata">ManifestMetadata</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestMetadata.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestMetadata.html" class="struct" title="struct iceberg::spec::ManifestMetadata">ManifestMetadata</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestMetadata.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestMetadata.html#impl-Eq-for-ManifestMetadata" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestMetadata.html" class="struct" title="struct iceberg::spec::ManifestMetadata">ManifestMetadata</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestMetadata.html#impl-StructuralPartialEq-for-ManifestMetadata" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestMetadata.html" class="struct" title="struct iceberg::spec::ManifestMetadata">ManifestMetadata</a>

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestMetadata.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestMetadata.html#blanket-implementations" class="anchor">§</a>
