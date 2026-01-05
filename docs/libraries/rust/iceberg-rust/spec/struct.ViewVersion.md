# Struct ViewVersion Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/view_version.rs.html#46-64" class="src">Source</a>

``` rust
pub struct ViewVersion { /* private fields */ }
```

Expand description

A view versions represents the definition of a view at a specific point in time.

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewVersion.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewVersion.html#impl-ViewVersion" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewVersion.html" class="struct" title="struct iceberg::spec::ViewVersion">ViewVersion</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewVersion.html#method.builder" class="fn">builder</a>() -\> ViewVersionBuilder\<(<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>)\>

Create a builder for building `ViewVersion`. On the builder, call `.version_id(...)`(optional), `.schema_id(...)`, `.timestamp_ms(...)`, `.summary(...)`(optional), `.representations(...)`, `.default_catalog(...)`(optional), `.default_namespace(...)` to set the values of the fields. Finally, call `.build()` to create the instance of `ViewVersion`.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewVersion.html#impl-ViewVersion-1" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewVersion.html" class="struct" title="struct iceberg::spec::ViewVersion">ViewVersion</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewVersion.html#method.version_id" class="fn">version_id</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.ViewVersionId.html" class="type" title="type iceberg::spec::ViewVersionId">ViewVersionId</a>

Get the version id of this view version.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewVersion.html#method.schema_id" class="fn">schema_id</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.SchemaId.html" class="type" title="type iceberg::spec::SchemaId">SchemaId</a>

Get the schema id of this view version.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewVersion.html#method.timestamp" class="fn">timestamp</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/datetime/struct.DateTime.html" class="struct" title="struct chrono::datetime::DateTime">DateTime</a>\<<a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/utc/struct.Utc.html" class="struct" title="struct chrono::offset::utc::Utc">Utc</a>\>\>

Get the timestamp of when the view version was created

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewVersion.html#method.timestamp_ms" class="fn">timestamp_ms</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

Get the timestamp of when the view version was created in milliseconds since epoch

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewVersion.html#method.summary" class="fn">summary</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Get summary of the view version

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewVersion.html#method.representations" class="fn">representations</a>(&self) -\> &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewRepresentations.html" class="struct" title="struct iceberg::spec::ViewRepresentations">ViewRepresentations</a>

Get this views representations

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewVersion.html#method.default_catalog" class="fn">default_catalog</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Get the default catalog for this view version

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewVersion.html#method.default_namespace" class="fn">default_namespace</a>(&self) -\> &<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.NamespaceIdent.html" class="struct" title="struct iceberg::NamespaceIdent">NamespaceIdent</a>

Get the default namespace to use when a reference in the SELECT is a single identifier

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewVersion.html#method.schema" class="fn">schema</a>(&self, view_metadata: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadata.html" class="struct" title="struct iceberg::spec::ViewMetadata">ViewMetadata</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.SchemaRef.html" class="type" title="type iceberg::spec::SchemaRef">SchemaRef</a>\>

Get the schema of this snapshot.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewVersion.html#method.with_version_id" class="fn">with_version_id</a>(self, version_id: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>) -\> Self

Change the version id of this view version.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewVersion.html#method.with_schema_id" class="fn">with_schema_id</a>(self, schema_id: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.SchemaId.html" class="type" title="type iceberg::spec::SchemaId">SchemaId</a>) -\> Self

Change the schema id of this view version.

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewVersion.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewVersion.html#impl-Clone-for-ViewVersion" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewVersion.html" class="struct" title="struct iceberg::spec::ViewVersion">ViewVersion</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewVersion.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewVersion.html" class="struct" title="struct iceberg::spec::ViewVersion">ViewVersion</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewVersion.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewVersion.html#impl-Debug-for-ViewVersion" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewVersion.html" class="struct" title="struct iceberg::spec::ViewVersion">ViewVersion</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewVersion.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewVersion.html#impl-Deserialize%3C&#39;de%3E-for-ViewVersion" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewVersion.html" class="struct" title="struct iceberg::spec::ViewVersion">ViewVersion</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewVersion.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>(\_\_deserializer: \_\_D) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, \_\_D::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewVersion.html#impl-PartialEq-for-ViewVersion" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewVersion.html" class="struct" title="struct iceberg::spec::ViewVersion">ViewVersion</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewVersion.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewVersion.html" class="struct" title="struct iceberg::spec::ViewVersion">ViewVersion</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewVersion.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewVersion.html#impl-Serialize-for-ViewVersion" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewVersion.html" class="struct" title="struct iceberg::spec::ViewVersion">ViewVersion</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewVersion.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>(&self, \_\_serializer: \_\_S) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\_\_S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \_\_S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewVersion.html#impl-Eq-for-ViewVersion" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewVersion.html" class="struct" title="struct iceberg::spec::ViewVersion">ViewVersion</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewVersion.html#impl-StructuralPartialEq-for-ViewVersion" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewVersion.html" class="struct" title="struct iceberg::spec::ViewVersion">ViewVersion</a>

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewVersion.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewVersion.html#blanket-implementations" class="anchor">§</a>
