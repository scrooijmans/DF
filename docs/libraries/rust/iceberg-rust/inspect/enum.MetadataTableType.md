# Enum MetadataTableType Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/inspect/metadata_table.rs.html#32-37" class="src">Source</a>

``` rust
pub enum MetadataTableType {
    Snapshots,
    Manifests,
}
```

Expand description

Metadata table type.

## Variants<a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/enum.MetadataTableType.html#variants" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/enum.MetadataTableType.html#variant.Snapshots" class="anchor">§</a>

### Snapshots

[`SnapshotsTable`](https://docs.rs/iceberg/0.7.0/iceberg/inspect/struct.SnapshotsTable.html "struct iceberg::inspect::SnapshotsTable")

<a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/enum.MetadataTableType.html#variant.Manifests" class="anchor">§</a>

### Manifests

[`ManifestsTable`](https://docs.rs/iceberg/0.7.0/iceberg/inspect/struct.ManifestsTable.html "struct iceberg::inspect::ManifestsTable")

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/enum.MetadataTableType.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/enum.MetadataTableType.html#impl-MetadataTableType" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/enum.MetadataTableType.html" class="enum" title="enum iceberg::inspect::MetadataTableType">MetadataTableType</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/enum.MetadataTableType.html#method.as_str" class="fn">as_str</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Returns the string representation of the metadata table type.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/enum.MetadataTableType.html#method.all_types" class="fn">all_types</a>() -\> impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = Self\>

Returns all the metadata table types.

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/enum.MetadataTableType.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/enum.MetadataTableType.html#impl-Clone-for-MetadataTableType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/enum.MetadataTableType.html" class="enum" title="enum iceberg::inspect::MetadataTableType">MetadataTableType</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/enum.MetadataTableType.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/enum.MetadataTableType.html" class="enum" title="enum iceberg::inspect::MetadataTableType">MetadataTableType</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/enum.MetadataTableType.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/enum.MetadataTableType.html#impl-Debug-for-MetadataTableType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/enum.MetadataTableType.html" class="enum" title="enum iceberg::inspect::MetadataTableType">MetadataTableType</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/enum.MetadataTableType.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/enum.MetadataTableType.html#impl-IntoEnumIterator-for-MetadataTableType" class="anchor">§</a>

### impl <a href="https://docs.rs/strum/0.27.2/x86_64-unknown-linux-gnu/strum/trait.IntoEnumIterator.html" class="trait" title="trait strum::IntoEnumIterator">IntoEnumIterator</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/enum.MetadataTableType.html" class="enum" title="enum iceberg::inspect::MetadataTableType">MetadataTableType</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/enum.MetadataTableType.html#associatedtype.Iterator" class="anchor">§</a>

#### type <a href="https://docs.rs/strum/0.27.2/x86_64-unknown-linux-gnu/strum/trait.IntoEnumIterator.html#associatedtype.Iterator" class="associatedtype">Iterator</a> = <a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/struct.MetadataTableTypeIter.html" class="struct" title="struct iceberg::inspect::MetadataTableTypeIter">MetadataTableTypeIter</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/enum.MetadataTableType.html#method.iter" class="anchor">§</a>

#### fn <a href="https://docs.rs/strum/0.27.2/x86_64-unknown-linux-gnu/strum/trait.IntoEnumIterator.html#tymethod.iter" class="fn">iter</a>() -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/struct.MetadataTableTypeIter.html" class="struct" title="struct iceberg::inspect::MetadataTableTypeIter">MetadataTableTypeIter</a> <a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/enum.MetadataTableType.html#" class="tooltip" data-notable-ty="MetadataTableTypeIter">ⓘ</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/enum.MetadataTableType.html#impl-TryFrom%3C%26str%3E-for-MetadataTableType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/enum.MetadataTableType.html" class="enum" title="enum iceberg::inspect::MetadataTableType">MetadataTableType</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/enum.MetadataTableType.html#associatedtype.Error" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

The type returned in the event of a conversion error.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/enum.MetadataTableType.html#method.try_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from" class="fn">try_from</a>(value: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Performs the conversion.

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/enum.MetadataTableType.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/enum.MetadataTableType.html#blanket-implementations" class="anchor">§</a>
