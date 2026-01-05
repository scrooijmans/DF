# Struct BlobMetadata Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/puffin/metadata.rs.html#35-48" class="src">Source</a>

``` rust
pub struct BlobMetadata { /* private fields */ }
```

Expand description

Metadata about a blob. For more information, see: https://iceberg.apache.org/puffin-spec/#blobmetadata

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.BlobMetadata.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.BlobMetadata.html#impl-BlobMetadata" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.BlobMetadata.html" class="struct" title="struct iceberg::puffin::BlobMetadata">BlobMetadata</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.BlobMetadata.html#method.blob_type" class="fn">blob_type</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

See blob types: https://iceberg.apache.org/puffin-spec/#blob-types

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.BlobMetadata.html#method.fields" class="fn">fields</a>(&self) -\> &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\]

List of field IDs the blob was computed for; the order of items is used to compute sketches stored in the blob.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.BlobMetadata.html#method.snapshot_id" class="fn">snapshot_id</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

ID of the Iceberg table’s snapshot the blob was computed from

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.BlobMetadata.html#method.sequence_number" class="fn">sequence_number</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

Sequence number of the Iceberg table’s snapshot the blob was computed from

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.BlobMetadata.html#method.offset" class="fn">offset</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

The offset in the file where the blob contents start

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.BlobMetadata.html#method.length" class="fn">length</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

The length of the blob stored in the file (after compression, if compressed)

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.BlobMetadata.html#method.compression_codec" class="fn">compression_codec</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/enum.CompressionCodec.html" class="enum" title="enum iceberg::puffin::CompressionCodec">CompressionCodec</a>

The compression codec used to compress the data

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.BlobMetadata.html#method.properties" class="fn">properties</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Arbitrary meta-information about the blob

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.BlobMetadata.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.BlobMetadata.html#impl-Clone-for-BlobMetadata" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.BlobMetadata.html" class="struct" title="struct iceberg::puffin::BlobMetadata">BlobMetadata</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.BlobMetadata.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.BlobMetadata.html" class="struct" title="struct iceberg::puffin::BlobMetadata">BlobMetadata</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.BlobMetadata.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.BlobMetadata.html#impl-Debug-for-BlobMetadata" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.BlobMetadata.html" class="struct" title="struct iceberg::puffin::BlobMetadata">BlobMetadata</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.BlobMetadata.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.BlobMetadata.html#impl-Deserialize%3C&#39;de%3E-for-BlobMetadata" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.BlobMetadata.html" class="struct" title="struct iceberg::puffin::BlobMetadata">BlobMetadata</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.BlobMetadata.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>(\_\_deserializer: \_\_D) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, \_\_D::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.BlobMetadata.html#impl-PartialEq-for-BlobMetadata" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.BlobMetadata.html" class="struct" title="struct iceberg::puffin::BlobMetadata">BlobMetadata</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.BlobMetadata.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.BlobMetadata.html" class="struct" title="struct iceberg::puffin::BlobMetadata">BlobMetadata</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.BlobMetadata.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.BlobMetadata.html#impl-Serialize-for-BlobMetadata" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.BlobMetadata.html" class="struct" title="struct iceberg::puffin::BlobMetadata">BlobMetadata</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.BlobMetadata.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>(&self, \_\_serializer: \_\_S) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\_\_S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \_\_S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.BlobMetadata.html#impl-Eq-for-BlobMetadata" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.BlobMetadata.html" class="struct" title="struct iceberg::puffin::BlobMetadata">BlobMetadata</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.BlobMetadata.html#impl-StructuralPartialEq-for-BlobMetadata" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.BlobMetadata.html" class="struct" title="struct iceberg::puffin::BlobMetadata">BlobMetadata</a>

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.BlobMetadata.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.BlobMetadata.html#blanket-implementations" class="anchor">§</a>
