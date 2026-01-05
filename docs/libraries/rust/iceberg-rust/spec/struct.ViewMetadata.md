# Struct ViewMetadata Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/view_metadata.rs.html#59-79" class="src">Source</a>

``` rust
pub struct ViewMetadata { /* private fields */ }
```

Expand description

Fields for the version 1 of the view metadata.

We assume that this data structure is always valid, so we will panic when invalid error happens. We check the validity of this data structure when constructing.

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadata.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadata.html#impl-ViewMetadata" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadata.html" class="struct" title="struct iceberg::spec::ViewMetadata">ViewMetadata</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadata.html#method.into_builder" class="fn">into_builder</a>(self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadataBuilder.html" class="struct" title="struct iceberg::spec::ViewMetadataBuilder">ViewMetadataBuilder</a>

Convert this View Metadata into a builder for modification.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadata.html#method.format_version" class="fn">format_version</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.ViewFormatVersion.html" class="enum" title="enum iceberg::spec::ViewFormatVersion">ViewFormatVersion</a>

Returns format version of this metadata.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadata.html#method.uuid" class="fn">uuid</a>(&self) -\> <a href="https://docs.rs/uuid/1.18.1/x86_64-unknown-linux-gnu/uuid/struct.Uuid.html" class="struct" title="struct uuid::Uuid">Uuid</a>

Returns uuid of current view.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadata.html#method.location" class="fn">location</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Returns view location.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadata.html#method.current_version_id" class="fn">current_version_id</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.ViewVersionId.html" class="type" title="type iceberg::spec::ViewVersionId">ViewVersionId</a>

Returns the current version id.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadata.html#method.versions" class="fn">versions</a>(&self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/exact_size/trait.ExactSizeIterator.html" class="trait" title="trait core::iter::traits::exact_size::ExactSizeIterator">ExactSizeIterator</a>\<Item = &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.ViewVersionRef.html" class="type" title="type iceberg::spec::ViewVersionRef">ViewVersionRef</a>\>

Returns all view versions.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadata.html#method.version_by_id" class="fn">version_by_id</a>( &self, version_id: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.ViewVersionId.html" class="type" title="type iceberg::spec::ViewVersionId">ViewVersionId</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.ViewVersionRef.html" class="type" title="type iceberg::spec::ViewVersionRef">ViewVersionRef</a>\>

Lookup a view version by id.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadata.html#method.current_version" class="fn">current_version</a>(&self) -\> &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.ViewVersionRef.html" class="type" title="type iceberg::spec::ViewVersionRef">ViewVersionRef</a>

Returns the current view version.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadata.html#method.schemas_iter" class="fn">schemas_iter</a>(&self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/exact_size/trait.ExactSizeIterator.html" class="trait" title="trait core::iter::traits::exact_size::ExactSizeIterator">ExactSizeIterator</a>\<Item = &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.SchemaRef.html" class="type" title="type iceberg::spec::SchemaRef">SchemaRef</a>\>

Returns schemas

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadata.html#method.schema_by_id" class="fn">schema_by_id</a>(&self, schema_id: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.SchemaId.html" class="type" title="type iceberg::spec::SchemaId">SchemaId</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.SchemaRef.html" class="type" title="type iceberg::spec::SchemaRef">SchemaRef</a>\>

Lookup schema by id.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadata.html#method.current_schema" class="fn">current_schema</a>(&self) -\> &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.SchemaRef.html" class="type" title="type iceberg::spec::SchemaRef">SchemaRef</a>

Get current schema

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadata.html#method.properties" class="fn">properties</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Returns properties of the view.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadata.html#method.history" class="fn">history</a>(&self) -\> &\[<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewVersionLog.html" class="struct" title="struct iceberg::spec::ViewVersionLog">ViewVersionLog</a>\]

Returns view history.

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadata.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadata.html#impl-Clone-for-ViewMetadata" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadata.html" class="struct" title="struct iceberg::spec::ViewMetadata">ViewMetadata</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadata.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadata.html" class="struct" title="struct iceberg::spec::ViewMetadata">ViewMetadata</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadata.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadata.html#impl-Debug-for-ViewMetadata" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadata.html" class="struct" title="struct iceberg::spec::ViewMetadata">ViewMetadata</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadata.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadata.html#impl-Deserialize%3C&#39;de%3E-for-ViewMetadata" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadata.html" class="struct" title="struct iceberg::spec::ViewMetadata">ViewMetadata</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadata.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>(\_\_deserializer: \_\_D) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, \_\_D::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadata.html#impl-PartialEq-for-ViewMetadata" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadata.html" class="struct" title="struct iceberg::spec::ViewMetadata">ViewMetadata</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadata.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadata.html" class="struct" title="struct iceberg::spec::ViewMetadata">ViewMetadata</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadata.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadata.html#impl-Serialize-for-ViewMetadata" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadata.html" class="struct" title="struct iceberg::spec::ViewMetadata">ViewMetadata</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadata.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<S\>(&self, serializer: S) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadata.html#impl-Eq-for-ViewMetadata" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadata.html" class="struct" title="struct iceberg::spec::ViewMetadata">ViewMetadata</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadata.html#impl-StructuralPartialEq-for-ViewMetadata" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadata.html" class="struct" title="struct iceberg::spec::ViewMetadata">ViewMetadata</a>

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadata.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadata.html#blanket-implementations" class="anchor">§</a>
