# Struct Manifest Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/manifest/mod.rs.html#41-44" class="src">Source</a>

``` rust
pub struct Manifest { /* private fields */ }
```

Expand description

A manifest contains metadata and a list of entries.

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Manifest.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Manifest.html#impl-Manifest" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Manifest.html" class="struct" title="struct iceberg::spec::Manifest">Manifest</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Manifest.html#method.parse_avro" class="fn">parse_avro</a>(bs: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Parse manifest from bytes of avro file.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Manifest.html#method.entries" class="fn">entries</a>(&self) -\> &\[<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.ManifestEntryRef.html" class="type" title="type iceberg::spec::ManifestEntryRef">ManifestEntryRef</a>\] <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Manifest.html#" class="tooltip" data-notable-ty="&amp;[ManifestEntryRef]">ⓘ</a>

Entries slice.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Manifest.html#method.metadata" class="fn">metadata</a>(&self) -\> &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestMetadata.html" class="struct" title="struct iceberg::spec::ManifestMetadata">ManifestMetadata</a>

Get metadata.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Manifest.html#method.into_parts" class="fn">into_parts</a>(self) -\> (<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.ManifestEntryRef.html" class="type" title="type iceberg::spec::ManifestEntryRef">ManifestEntryRef</a>\>, <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestMetadata.html" class="struct" title="struct iceberg::spec::ManifestMetadata">ManifestMetadata</a>)

Consume this Manifest, returning its constituent parts

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Manifest.html#method.new" class="fn">new</a>(metadata: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestMetadata.html" class="struct" title="struct iceberg::spec::ManifestMetadata">ManifestMetadata</a>, entries: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestEntry.html" class="struct" title="struct iceberg::spec::ManifestEntry">ManifestEntry</a>\>) -\> Self

Constructor from [`ManifestMetadata`](https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestMetadata.html "struct iceberg::spec::ManifestMetadata") and [`ManifestEntry`](https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestEntry.html "struct iceberg::spec::ManifestEntry")s.

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Manifest.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Manifest.html#impl-Clone-for-Manifest" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Manifest.html" class="struct" title="struct iceberg::spec::Manifest">Manifest</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Manifest.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Manifest.html" class="struct" title="struct iceberg::spec::Manifest">Manifest</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Manifest.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Manifest.html#impl-Debug-for-Manifest" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Manifest.html" class="struct" title="struct iceberg::spec::Manifest">Manifest</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Manifest.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Manifest.html#impl-PartialEq-for-Manifest" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Manifest.html" class="struct" title="struct iceberg::spec::Manifest">Manifest</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Manifest.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Manifest.html" class="struct" title="struct iceberg::spec::Manifest">Manifest</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Manifest.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Manifest.html#impl-Eq-for-Manifest" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Manifest.html" class="struct" title="struct iceberg::spec::Manifest">Manifest</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Manifest.html#impl-StructuralPartialEq-for-Manifest" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Manifest.html" class="struct" title="struct iceberg::spec::Manifest">Manifest</a>

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Manifest.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Manifest.html#blanket-implementations" class="anchor">§</a>
