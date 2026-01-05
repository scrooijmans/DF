# Struct Blob Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/puffin/blob.rs.html#29-36" class="src">Source</a>

``` rust
pub struct Blob { /* private fields */ }
```

Expand description

The blob

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.Blob.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.Blob.html#impl-Blob" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.Blob.html" class="struct" title="struct iceberg::puffin::Blob">Blob</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.Blob.html#method.builder" class="fn">builder</a>() -\> BlobBuilder\<(<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>)\>

Create a builder for building `Blob`. On the builder, call `.r#type(...)`, `.fields(...)`, `.snapshot_id(...)`, `.sequence_number(...)`, `.data(...)`, `.properties(...)` to set the values of the fields. Finally, call `.build()` to create the instance of `Blob`.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.Blob.html#impl-Blob-1" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.Blob.html" class="struct" title="struct iceberg::puffin::Blob">Blob</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.Blob.html#method.blob_type" class="fn">blob_type</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

See blob types: https://iceberg.apache.org/puffin-spec/#blob-types

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.Blob.html#method.fields" class="fn">fields</a>(&self) -\> &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\]

List of field IDs the blob was computed for; the order of items is used to compute sketches stored in the blob.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.Blob.html#method.snapshot_id" class="fn">snapshot_id</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

ID of the Iceberg table’s snapshot the blob was computed from

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.Blob.html#method.sequence_number" class="fn">sequence_number</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

Sequence number of the Iceberg table’s snapshot the blob was computed from

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.Blob.html#method.data" class="fn">data</a>(&self) -\> &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\] <a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.Blob.html#" class="tooltip" data-notable-ty="&amp;[u8]">ⓘ</a>

The uncompressed blob data

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.Blob.html#method.properties" class="fn">properties</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Arbitrary meta-information about the blob

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.Blob.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.Blob.html#impl-Clone-for-Blob" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.Blob.html" class="struct" title="struct iceberg::puffin::Blob">Blob</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.Blob.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.Blob.html" class="struct" title="struct iceberg::puffin::Blob">Blob</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.Blob.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.Blob.html#impl-Debug-for-Blob" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.Blob.html" class="struct" title="struct iceberg::puffin::Blob">Blob</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.Blob.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.Blob.html#impl-PartialEq-for-Blob" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.Blob.html" class="struct" title="struct iceberg::puffin::Blob">Blob</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.Blob.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.Blob.html" class="struct" title="struct iceberg::puffin::Blob">Blob</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.Blob.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.Blob.html#impl-StructuralPartialEq-for-Blob" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.Blob.html" class="struct" title="struct iceberg::puffin::Blob">Blob</a>

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.Blob.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.Blob.html#blanket-implementations" class="anchor">§</a>
