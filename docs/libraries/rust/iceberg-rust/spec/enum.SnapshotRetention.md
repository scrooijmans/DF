# Enum SnapshotRetention Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/snapshot.rs.html#353-379" class="src">Source</a>

``` rust
pub enum SnapshotRetention {
    Branch {
        min_snapshots_to_keep: Option<i32>,
        max_snapshot_age_ms: Option<i64>,
        max_ref_age_ms: Option<i64>,
    },
    Tag {
        max_ref_age_ms: Option<i64>,
    },
}
```

Expand description

The snapshot expiration procedure removes snapshots from table metadata and applies the table’s retention policy.

## Variants<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SnapshotRetention.html#variants" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SnapshotRetention.html#variant.Branch" class="anchor">§</a>

### Branch

Branches are mutable named references that can be updated by committing a new snapshot as the branch’s referenced snapshot using the Commit Conflict Resolution and Retry procedures.

#### Fields

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SnapshotRetention.html#variant.Branch.field.min_snapshots_to_keep" class="anchor field">§</a>`min_snapshots_to_keep: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive"><code>i32</code></a>`>`

A positive number for the minimum number of snapshots to keep in a branch while expiring snapshots. Defaults to table property history.expire.min-snapshots-to-keep.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SnapshotRetention.html#variant.Branch.field.max_snapshot_age_ms" class="anchor field">§</a>`max_snapshot_age_ms: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive"><code>i64</code></a>`>`

A positive number for the max age of snapshots to keep when expiring, including the latest snapshot. Defaults to table property history.expire.max-snapshot-age-ms.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SnapshotRetention.html#variant.Branch.field.max_ref_age_ms" class="anchor field">§</a>`max_ref_age_ms: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive"><code>i64</code></a>`>`

For snapshot references except the main branch, a positive number for the max age of the snapshot reference to keep while expiring snapshots. Defaults to table property history.expire.max-ref-age-ms. The main branch never expires.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SnapshotRetention.html#variant.Tag" class="anchor">§</a>

### Tag

Tags are labels for individual snapshots.

#### Fields

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SnapshotRetention.html#variant.Tag.field.max_ref_age_ms" class="anchor field">§</a>`max_ref_age_ms: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive"><code>i64</code></a>`>`

For snapshot references except the main branch, a positive number for the max age of the snapshot reference to keep while expiring snapshots. Defaults to table property history.expire.max-ref-age-ms. The main branch never expires.

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SnapshotRetention.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SnapshotRetention.html#impl-SnapshotRetention" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SnapshotRetention.html" class="enum" title="enum iceberg::spec::SnapshotRetention">SnapshotRetention</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SnapshotRetention.html#method.branch" class="fn">branch</a>( min_snapshots_to_keep: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>, max_snapshot_age_ms: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>, max_ref_age_ms: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>, ) -\> Self

Create a new branch retention policy

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SnapshotRetention.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SnapshotRetention.html#impl-Clone-for-SnapshotRetention" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SnapshotRetention.html" class="enum" title="enum iceberg::spec::SnapshotRetention">SnapshotRetention</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SnapshotRetention.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SnapshotRetention.html" class="enum" title="enum iceberg::spec::SnapshotRetention">SnapshotRetention</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SnapshotRetention.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SnapshotRetention.html#impl-Debug-for-SnapshotRetention" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SnapshotRetention.html" class="enum" title="enum iceberg::spec::SnapshotRetention">SnapshotRetention</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SnapshotRetention.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SnapshotRetention.html#impl-Deserialize%3C&#39;de%3E-for-SnapshotRetention" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SnapshotRetention.html" class="enum" title="enum iceberg::spec::SnapshotRetention">SnapshotRetention</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SnapshotRetention.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>(\_\_deserializer: \_\_D) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, \_\_D::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SnapshotRetention.html#impl-PartialEq-for-SnapshotRetention" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SnapshotRetention.html" class="enum" title="enum iceberg::spec::SnapshotRetention">SnapshotRetention</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SnapshotRetention.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SnapshotRetention.html" class="enum" title="enum iceberg::spec::SnapshotRetention">SnapshotRetention</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SnapshotRetention.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SnapshotRetention.html#impl-Serialize-for-SnapshotRetention" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SnapshotRetention.html" class="enum" title="enum iceberg::spec::SnapshotRetention">SnapshotRetention</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SnapshotRetention.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>(&self, \_\_serializer: \_\_S) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\_\_S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \_\_S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SnapshotRetention.html#impl-Eq-for-SnapshotRetention" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SnapshotRetention.html" class="enum" title="enum iceberg::spec::SnapshotRetention">SnapshotRetention</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SnapshotRetention.html#impl-StructuralPartialEq-for-SnapshotRetention" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SnapshotRetention.html" class="enum" title="enum iceberg::spec::SnapshotRetention">SnapshotRetention</a>

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SnapshotRetention.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SnapshotRetention.html#blanket-implementations" class="anchor">§</a>
