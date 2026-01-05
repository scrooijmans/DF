# Struct ManifestFile Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/manifest_list.rs.html#509-583" class="src">Source</a>

``` rust
pub struct ManifestFile {Show 15 fields
    pub manifest_path: String,
    pub manifest_length: i64,
    pub partition_spec_id: i32,
    pub content: ManifestContentType,
    pub sequence_number: i64,
    pub min_sequence_number: i64,
    pub added_snapshot_id: i64,
    pub added_files_count: Option<u32>,
    pub existing_files_count: Option<u32>,
    pub deleted_files_count: Option<u32>,
    pub added_rows_count: Option<u64>,
    pub existing_rows_count: Option<u64>,
    pub deleted_rows_count: Option<u64>,
    pub partitions: Option<Vec<FieldSummary>>,
    pub key_metadata: Option<Vec<u8>>,
}
```

Expand description

Entry in a manifest list.

## Fields<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestFile.html#fields" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestFile.html#structfield.manifest_path" class="anchor field">§</a>`manifest_path: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

field: 500

Location of the manifest file

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestFile.html#structfield.manifest_length" class="anchor field">§</a>`manifest_length: `<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive"><code>i64</code></a>

field: 501

Length of the manifest file in bytes

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestFile.html#structfield.partition_spec_id" class="anchor field">§</a>`partition_spec_id: `<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive"><code>i32</code></a>

field: 502

ID of a partition spec used to write the manifest; must be listed in table metadata partition-specs

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestFile.html#structfield.content" class="anchor field">§</a>`content: `<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.ManifestContentType.html" class="enum" title="enum iceberg::spec::ManifestContentType"><code>ManifestContentType</code></a>

field: 517

The type of files tracked by the manifest, either data or delete files; 0 for all v1 manifests

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestFile.html#structfield.sequence_number" class="anchor field">§</a>`sequence_number: `<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive"><code>i64</code></a>

field: 515

The sequence number when the manifest was added to the table; use 0 when reading v1 manifest lists

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestFile.html#structfield.min_sequence_number" class="anchor field">§</a>`min_sequence_number: `<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive"><code>i64</code></a>

field: 516

The minimum data sequence number of all live data or delete files in the manifest; use 0 when reading v1 manifest lists

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestFile.html#structfield.added_snapshot_id" class="anchor field">§</a>`added_snapshot_id: `<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive"><code>i64</code></a>

field: 503

ID of the snapshot where the manifest file was added

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestFile.html#structfield.added_files_count" class="anchor field">§</a>`added_files_count: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive"><code>u32</code></a>`>`

field: 504

Number of entries in the manifest that have status ADDED, when null this is assumed to be non-zero

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestFile.html#structfield.existing_files_count" class="anchor field">§</a>`existing_files_count: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive"><code>u32</code></a>`>`

field: 505

Number of entries in the manifest that have status EXISTING (0), when null this is assumed to be non-zero

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestFile.html#structfield.deleted_files_count" class="anchor field">§</a>`deleted_files_count: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive"><code>u32</code></a>`>`

field: 506

Number of entries in the manifest that have status DELETED (2), when null this is assumed to be non-zero

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestFile.html#structfield.added_rows_count" class="anchor field">§</a>`added_rows_count: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive"><code>u64</code></a>`>`

field: 512

Number of rows in all of files in the manifest that have status ADDED, when null this is assumed to be non-zero

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestFile.html#structfield.existing_rows_count" class="anchor field">§</a>`existing_rows_count: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive"><code>u64</code></a>`>`

field: 513

Number of rows in all of files in the manifest that have status EXISTING, when null this is assumed to be non-zero

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestFile.html#structfield.deleted_rows_count" class="anchor field">§</a>`deleted_rows_count: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive"><code>u64</code></a>`>`

field: 514

Number of rows in all of files in the manifest that have status DELETED, when null this is assumed to be non-zero

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestFile.html#structfield.partitions" class="anchor field">§</a>`partitions: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.FieldSummary.html" class="struct" title="struct iceberg::spec::FieldSummary"><code>FieldSummary</code></a>`>>`

field: 507 element_field: 508

A list of field summaries for each partition field in the spec. Each field in the list corresponds to a field in the manifest file’s partition spec.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestFile.html#structfield.key_metadata" class="anchor field">§</a>`key_metadata: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive"><code>u8</code></a>`>>`

field: 519

Implementation-specific key metadata for encryption

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestFile.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestFile.html#impl-ManifestFile" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestFile.html" class="struct" title="struct iceberg::spec::ManifestFile">ManifestFile</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestFile.html#method.has_added_files" class="fn">has_added_files</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Checks if the manifest file has any added files.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestFile.html#method.has_deleted_files" class="fn">has_deleted_files</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Checks whether this manifest contains entries with DELETED status.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestFile.html#method.has_existing_files" class="fn">has_existing_files</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Checks if the manifest file has any existed files.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestFile.html#impl-ManifestFile-1" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestFile.html" class="struct" title="struct iceberg::spec::ManifestFile">ManifestFile</a>

#### pub async fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestFile.html#method.load_manifest" class="fn">load_manifest</a>(&self, file_io: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIO.html" class="struct" title="struct iceberg::io::FileIO">FileIO</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Manifest.html" class="struct" title="struct iceberg::spec::Manifest">Manifest</a>\>

Load [`Manifest`](https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Manifest.html "struct iceberg::spec::Manifest").

This method will also initialize inherited values of \[`ManifestEntry`\], such as `sequence_number`.

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestFile.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestFile.html#impl-Clone-for-ManifestFile" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestFile.html" class="struct" title="struct iceberg::spec::ManifestFile">ManifestFile</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestFile.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestFile.html" class="struct" title="struct iceberg::spec::ManifestFile">ManifestFile</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestFile.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestFile.html#impl-Debug-for-ManifestFile" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestFile.html" class="struct" title="struct iceberg::spec::ManifestFile">ManifestFile</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestFile.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestFile.html#impl-Hash-for-ManifestFile" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestFile.html" class="struct" title="struct iceberg::spec::ManifestFile">ManifestFile</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestFile.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestFile.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestFile.html#impl-PartialEq-for-ManifestFile" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestFile.html" class="struct" title="struct iceberg::spec::ManifestFile">ManifestFile</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestFile.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestFile.html" class="struct" title="struct iceberg::spec::ManifestFile">ManifestFile</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestFile.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestFile.html#impl-Eq-for-ManifestFile" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestFile.html" class="struct" title="struct iceberg::spec::ManifestFile">ManifestFile</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestFile.html#impl-StructuralPartialEq-for-ManifestFile" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestFile.html" class="struct" title="struct iceberg::spec::ManifestFile">ManifestFile</a>

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestFile.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestFile.html#blanket-implementations" class="anchor">§</a>
