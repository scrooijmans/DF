# Struct MetadataLocation Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/catalog/metadata_location.rs.html#27-31" class="src">Source</a>

``` rust
pub struct MetadataLocation { /* private fields */ }
```

Expand description

Helper for parsing a location of the format: `<location>/metadata/<version>-<uuid>.metadata.json`

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.MetadataLocation.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.MetadataLocation.html#impl-MetadataLocation" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.MetadataLocation.html" class="struct" title="struct iceberg::MetadataLocation">MetadataLocation</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.MetadataLocation.html#method.new_with_table_location" class="fn">new_with_table_location</a>(table_location: impl <a href="https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html" class="trait" title="trait alloc::string::ToString">ToString</a>) -\> Self

Creates a completely new metadata location starting at version 0. Only used for creating a new table. For updates, see `with_next_version`.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.MetadataLocation.html#method.with_next_version" class="fn">with_next_version</a>(&self) -\> Self

Creates a new metadata location for an updated metadata file.

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.MetadataLocation.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.MetadataLocation.html#impl-Clone-for-MetadataLocation" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.MetadataLocation.html" class="struct" title="struct iceberg::MetadataLocation">MetadataLocation</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.MetadataLocation.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.MetadataLocation.html" class="struct" title="struct iceberg::MetadataLocation">MetadataLocation</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.MetadataLocation.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.MetadataLocation.html#impl-Debug-for-MetadataLocation" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.MetadataLocation.html" class="struct" title="struct iceberg::MetadataLocation">MetadataLocation</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.MetadataLocation.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.MetadataLocation.html#impl-Display-for-MetadataLocation" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.MetadataLocation.html" class="struct" title="struct iceberg::MetadataLocation">MetadataLocation</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.MetadataLocation.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.MetadataLocation.html#impl-FromStr-for-MetadataLocation" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html" class="trait" title="trait core::str::traits::FromStr">FromStr</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.MetadataLocation.html" class="struct" title="struct iceberg::MetadataLocation">MetadataLocation</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.MetadataLocation.html#associatedtype.Err" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#associatedtype.Err" class="associatedtype">Err</a> = <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html" class="struct" title="struct iceberg::Error">Error</a>

The associated error which can be returned from parsing.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.MetadataLocation.html#method.from_str" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#tymethod.from_str" class="fn">from_str</a>(s: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Parses a string `s` to return a value of this type. [Read more](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#tymethod.from_str)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.MetadataLocation.html#impl-PartialEq-for-MetadataLocation" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.MetadataLocation.html" class="struct" title="struct iceberg::MetadataLocation">MetadataLocation</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.MetadataLocation.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.MetadataLocation.html" class="struct" title="struct iceberg::MetadataLocation">MetadataLocation</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.MetadataLocation.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.MetadataLocation.html#impl-StructuralPartialEq-for-MetadataLocation" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.MetadataLocation.html" class="struct" title="struct iceberg::MetadataLocation">MetadataLocation</a>

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.MetadataLocation.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.MetadataLocation.html#blanket-implementations" class="anchor">§</a>
