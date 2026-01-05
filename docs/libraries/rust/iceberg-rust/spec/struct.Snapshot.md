# Struct Snapshot Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/snapshot.rs.html#89-112" class="src">Source</a>

``` rust
pub struct Snapshot { /* private fields */ }
```

Expand description

A snapshot represents the state of a table at some time and is used to access the complete set of data files in the table.

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Snapshot.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Snapshot.html#impl-Snapshot" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Snapshot.html" class="struct" title="struct iceberg::spec::Snapshot">Snapshot</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Snapshot.html#method.builder" class="fn">builder</a>() -\> SnapshotBuilder\<(<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>)\>

Create a builder for building `Snapshot`. On the builder, call `.snapshot_id(...)`, `.parent_snapshot_id(...)`(optional), `.sequence_number(...)`, `.timestamp_ms(...)`, `.manifest_list(...)`, `.summary(...)`, `.schema_id(...)`(optional) to set the values of the fields. Finally, call `.build()` to create the instance of `Snapshot`.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Snapshot.html#impl-Snapshot-1" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Snapshot.html" class="struct" title="struct iceberg::spec::Snapshot">Snapshot</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Snapshot.html#method.snapshot_id" class="fn">snapshot_id</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

Get the id of the snapshot

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Snapshot.html#method.parent_snapshot_id" class="fn">parent_snapshot_id</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

Get parent snapshot id.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Snapshot.html#method.sequence_number" class="fn">sequence_number</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

Get sequence_number of the snapshot. Is 0 for Iceberg V1 tables.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Snapshot.html#method.manifest_list" class="fn">manifest_list</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Get location of manifest_list file

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Snapshot.html#method.summary" class="fn">summary</a>(&self) -\> &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Summary.html" class="struct" title="struct iceberg::spec::Summary">Summary</a>

Get summary of the snapshot

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Snapshot.html#method.timestamp" class="fn">timestamp</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/datetime/struct.DateTime.html" class="struct" title="struct chrono::datetime::DateTime">DateTime</a>\<<a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/utc/struct.Utc.html" class="struct" title="struct chrono::offset::utc::Utc">Utc</a>\>\>

Get the timestamp of when the snapshot was created

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Snapshot.html#method.timestamp_ms" class="fn">timestamp_ms</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

Get the timestamp of when the snapshot was created in milliseconds

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Snapshot.html#method.schema_id" class="fn">schema_id</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.SchemaId.html" class="type" title="type iceberg::spec::SchemaId">SchemaId</a>\>

Get the schema id of this snapshot.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Snapshot.html#method.schema" class="fn">schema</a>(&self, table_metadata: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html" class="struct" title="struct iceberg::spec::TableMetadata">TableMetadata</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.SchemaRef.html" class="type" title="type iceberg::spec::SchemaRef">SchemaRef</a>\>

Get the schema of this snapshot.

#### pub async fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Snapshot.html#method.load_manifest_list" class="fn">load_manifest_list</a>( &self, file_io: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIO.html" class="struct" title="struct iceberg::io::FileIO">FileIO</a>, table_metadata: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html" class="struct" title="struct iceberg::spec::TableMetadata">TableMetadata</a>, ) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestList.html" class="struct" title="struct iceberg::spec::ManifestList">ManifestList</a>\>

Load manifest list.

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Snapshot.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Snapshot.html#impl-Clone-for-Snapshot" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Snapshot.html" class="struct" title="struct iceberg::spec::Snapshot">Snapshot</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Snapshot.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Snapshot.html" class="struct" title="struct iceberg::spec::Snapshot">Snapshot</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Snapshot.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Snapshot.html#impl-Debug-for-Snapshot" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Snapshot.html" class="struct" title="struct iceberg::spec::Snapshot">Snapshot</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Snapshot.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Snapshot.html#impl-Deserialize%3C&#39;de%3E-for-Snapshot" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Snapshot.html" class="struct" title="struct iceberg::spec::Snapshot">Snapshot</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Snapshot.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>(\_\_deserializer: \_\_D) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, \_\_D::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Snapshot.html#impl-PartialEq-for-Snapshot" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Snapshot.html" class="struct" title="struct iceberg::spec::Snapshot">Snapshot</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Snapshot.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Snapshot.html" class="struct" title="struct iceberg::spec::Snapshot">Snapshot</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Snapshot.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Snapshot.html#impl-Serialize-for-Snapshot" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Snapshot.html" class="struct" title="struct iceberg::spec::Snapshot">Snapshot</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Snapshot.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>(&self, \_\_serializer: \_\_S) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\_\_S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \_\_S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Snapshot.html#impl-Eq-for-Snapshot" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Snapshot.html" class="struct" title="struct iceberg::spec::Snapshot">Snapshot</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Snapshot.html#impl-StructuralPartialEq-for-Snapshot" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Snapshot.html" class="struct" title="struct iceberg::spec::Snapshot">Snapshot</a>

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Snapshot.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Snapshot.html#blanket-implementations" class="anchor">§</a>
