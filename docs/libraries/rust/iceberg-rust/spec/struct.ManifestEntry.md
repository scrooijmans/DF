# Struct ManifestEntry Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/manifest/entry.rs.html#39-66" class="src">Source</a>

``` rust
pub struct ManifestEntry {
    pub status: ManifestStatus,
    pub snapshot_id: Option<i64>,
    pub sequence_number: Option<i64>,
    pub file_sequence_number: Option<i64>,
    pub data_file: DataFile,
}
```

Expand description

A manifest is an immutable Avro file that lists data files or delete files, along with each file’s partition data tuple, metrics, and tracking information.

## Fields<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestEntry.html#fields" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestEntry.html#structfield.status" class="anchor field">§</a>`status: `<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.ManifestStatus.html" class="enum" title="enum iceberg::spec::ManifestStatus"><code>ManifestStatus</code></a>

field: 0

Used to track additions and deletions.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestEntry.html#structfield.snapshot_id" class="anchor field">§</a>`snapshot_id: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive"><code>i64</code></a>`>`

field id: 1

Snapshot id where the file was added, or deleted if status is 2. Inherited when null.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestEntry.html#structfield.sequence_number" class="anchor field">§</a>`sequence_number: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive"><code>i64</code></a>`>`

field id: 3

Data sequence number of the file. Inherited when null and status is 1 (added).

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestEntry.html#structfield.file_sequence_number" class="anchor field">§</a>`file_sequence_number: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive"><code>i64</code></a>`>`

field id: 4

File sequence number indicating when the file was added. Inherited when null and status is 1 (added).

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestEntry.html#structfield.data_file" class="anchor field">§</a>`data_file: `<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html" class="struct" title="struct iceberg::spec::DataFile"><code>DataFile</code></a>

field id: 2

File path, partition tuple, metrics, …

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestEntry.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestEntry.html#impl-ManifestEntry" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestEntry.html" class="struct" title="struct iceberg::spec::ManifestEntry">ManifestEntry</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestEntry.html#method.builder" class="fn">builder</a>() -\> ManifestEntryBuilder\<(<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>)\>

Create a builder for building `ManifestEntry`. On the builder, call `.status(...)`, `.snapshot_id(...)`(optional), `.sequence_number(...)`(optional), `.file_sequence_number(...)`(optional), `.data_file(...)` to set the values of the fields. Finally, call `.build()` to create the instance of `ManifestEntry`.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestEntry.html#impl-ManifestEntry-1" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestEntry.html" class="struct" title="struct iceberg::spec::ManifestEntry">ManifestEntry</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestEntry.html#method.is_alive" class="fn">is_alive</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Check if this manifest entry is deleted.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestEntry.html#method.status" class="fn">status</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.ManifestStatus.html" class="enum" title="enum iceberg::spec::ManifestStatus">ManifestStatus</a>

Status of this manifest entry

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestEntry.html#method.content_type" class="fn">content_type</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.DataContentType.html" class="enum" title="enum iceberg::spec::DataContentType">DataContentType</a>

Content type of this manifest entry.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestEntry.html#method.file_format" class="fn">file_format</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.DataFileFormat.html" class="enum" title="enum iceberg::spec::DataFileFormat">DataFileFormat</a>

File format of this manifest entry.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestEntry.html#method.file_path" class="fn">file_path</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Data file path of this manifest entry.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestEntry.html#method.record_count" class="fn">record_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

Data file record count of the manifest entry.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestEntry.html#method.snapshot_id" class="fn">snapshot_id</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

Snapshot id

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestEntry.html#method.sequence_number" class="fn">sequence_number</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

Data sequence number.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestEntry.html#method.file_size_in_bytes" class="fn">file_size_in_bytes</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

File size in bytes.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestEntry.html#method.data_file" class="fn">data_file</a>(&self) -\> &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html" class="struct" title="struct iceberg::spec::DataFile">DataFile</a>

get a reference to the actual data file

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestEntry.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestEntry.html#impl-Clone-for-ManifestEntry" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestEntry.html" class="struct" title="struct iceberg::spec::ManifestEntry">ManifestEntry</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestEntry.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestEntry.html" class="struct" title="struct iceberg::spec::ManifestEntry">ManifestEntry</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestEntry.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestEntry.html#impl-Debug-for-ManifestEntry" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestEntry.html" class="struct" title="struct iceberg::spec::ManifestEntry">ManifestEntry</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestEntry.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestEntry.html#impl-PartialEq-for-ManifestEntry" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestEntry.html" class="struct" title="struct iceberg::spec::ManifestEntry">ManifestEntry</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestEntry.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestEntry.html" class="struct" title="struct iceberg::spec::ManifestEntry">ManifestEntry</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestEntry.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestEntry.html#impl-Eq-for-ManifestEntry" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestEntry.html" class="struct" title="struct iceberg::spec::ManifestEntry">ManifestEntry</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestEntry.html#impl-StructuralPartialEq-for-ManifestEntry" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestEntry.html" class="struct" title="struct iceberg::spec::ManifestEntry">ManifestEntry</a>

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestEntry.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestEntry.html#blanket-implementations" class="anchor">§</a>
