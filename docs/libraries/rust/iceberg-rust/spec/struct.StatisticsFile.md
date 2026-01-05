# Struct StatisticsFile Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/statistic_file.rs.html#27-41" class="src">Source</a>

``` rust
pub struct StatisticsFile {
    pub snapshot_id: i64,
    pub statistics_path: String,
    pub file_size_in_bytes: i64,
    pub file_footer_size_in_bytes: i64,
    pub key_metadata: Option<String>,
    pub blob_metadata: Vec<BlobMetadata>,
}
```

Expand description

Represents a statistics file

## Fields<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.StatisticsFile.html#fields" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.StatisticsFile.html#structfield.snapshot_id" class="anchor field">§</a>`snapshot_id: `<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive"><code>i64</code></a>

The snapshot id of the statistics file.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.StatisticsFile.html#structfield.statistics_path" class="anchor field">§</a>`statistics_path: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

Path of the statistics file

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.StatisticsFile.html#structfield.file_size_in_bytes" class="anchor field">§</a>`file_size_in_bytes: `<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive"><code>i64</code></a>

File size in bytes

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.StatisticsFile.html#structfield.file_footer_size_in_bytes" class="anchor field">§</a>`file_footer_size_in_bytes: `<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive"><code>i64</code></a>

File footer size in bytes

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.StatisticsFile.html#structfield.key_metadata" class="anchor field">§</a>`key_metadata: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Base64-encoded implementation-specific key metadata for encryption.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.StatisticsFile.html#structfield.blob_metadata" class="anchor field">§</a>`blob_metadata: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.BlobMetadata.html" class="struct" title="struct iceberg::spec::BlobMetadata"><code>BlobMetadata</code></a>`>`

Blob metadata

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.StatisticsFile.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.StatisticsFile.html#impl-Clone-for-StatisticsFile" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.StatisticsFile.html" class="struct" title="struct iceberg::spec::StatisticsFile">StatisticsFile</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.StatisticsFile.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.StatisticsFile.html" class="struct" title="struct iceberg::spec::StatisticsFile">StatisticsFile</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.StatisticsFile.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.StatisticsFile.html#impl-Debug-for-StatisticsFile" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.StatisticsFile.html" class="struct" title="struct iceberg::spec::StatisticsFile">StatisticsFile</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.StatisticsFile.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.StatisticsFile.html#impl-Deserialize%3C&#39;de%3E-for-StatisticsFile" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.StatisticsFile.html" class="struct" title="struct iceberg::spec::StatisticsFile">StatisticsFile</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.StatisticsFile.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>(\_\_deserializer: \_\_D) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, \_\_D::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.StatisticsFile.html#impl-PartialEq-for-StatisticsFile" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.StatisticsFile.html" class="struct" title="struct iceberg::spec::StatisticsFile">StatisticsFile</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.StatisticsFile.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.StatisticsFile.html" class="struct" title="struct iceberg::spec::StatisticsFile">StatisticsFile</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.StatisticsFile.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.StatisticsFile.html#impl-Serialize-for-StatisticsFile" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.StatisticsFile.html" class="struct" title="struct iceberg::spec::StatisticsFile">StatisticsFile</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.StatisticsFile.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>(&self, \_\_serializer: \_\_S) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\_\_S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \_\_S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.StatisticsFile.html#impl-Eq-for-StatisticsFile" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.StatisticsFile.html" class="struct" title="struct iceberg::spec::StatisticsFile">StatisticsFile</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.StatisticsFile.html#impl-StructuralPartialEq-for-StatisticsFile" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.StatisticsFile.html" class="struct" title="struct iceberg::spec::StatisticsFile">StatisticsFile</a>

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.StatisticsFile.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.StatisticsFile.html#blanket-implementations" class="anchor">§</a>
