# Struct TableMetadataBuildResult Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/table_metadata_builder.rs.html#62-69" class="src">Source</a>

``` rust
pub struct TableMetadataBuildResult {
    pub metadata: TableMetadata,
    pub changes: Vec<TableUpdate>,
    pub expired_metadata_logs: Vec<MetadataLog>,
}
```

Expand description

Result of modifying or creating a `TableMetadata`.

## Fields<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuildResult.html#fields" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuildResult.html#structfield.metadata" class="anchor field">§</a>`metadata: `<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html" class="struct" title="struct iceberg::spec::TableMetadata"><code>TableMetadata</code></a>

The new `TableMetadata`.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuildResult.html#structfield.changes" class="anchor field">§</a>`changes: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html" class="enum" title="enum iceberg::TableUpdate"><code>TableUpdate</code></a>`>`

The changes that were applied to the metadata.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuildResult.html#structfield.expired_metadata_logs" class="anchor field">§</a>`expired_metadata_logs: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.MetadataLog.html" class="struct" title="struct iceberg::spec::MetadataLog"><code>MetadataLog</code></a>`>`

Expired metadata logs

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuildResult.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuildResult.html#impl-Clone-for-TableMetadataBuildResult" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuildResult.html" class="struct" title="struct iceberg::spec::TableMetadataBuildResult">TableMetadataBuildResult</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuildResult.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuildResult.html" class="struct" title="struct iceberg::spec::TableMetadataBuildResult">TableMetadataBuildResult</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuildResult.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuildResult.html#impl-Debug-for-TableMetadataBuildResult" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuildResult.html" class="struct" title="struct iceberg::spec::TableMetadataBuildResult">TableMetadataBuildResult</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuildResult.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuildResult.html#impl-From%3CTableMetadataBuildResult%3E-for-TableMetadata" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuildResult.html" class="struct" title="struct iceberg::spec::TableMetadataBuildResult">TableMetadataBuildResult</a>\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html" class="struct" title="struct iceberg::spec::TableMetadata">TableMetadata</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuildResult.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(result: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuildResult.html" class="struct" title="struct iceberg::spec::TableMetadataBuildResult">TableMetadataBuildResult</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuildResult.html#impl-PartialEq-for-TableMetadataBuildResult" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuildResult.html" class="struct" title="struct iceberg::spec::TableMetadataBuildResult">TableMetadataBuildResult</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuildResult.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuildResult.html" class="struct" title="struct iceberg::spec::TableMetadataBuildResult">TableMetadataBuildResult</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuildResult.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuildResult.html#impl-StructuralPartialEq-for-TableMetadataBuildResult" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuildResult.html" class="struct" title="struct iceberg::spec::TableMetadataBuildResult">TableMetadataBuildResult</a>

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuildResult.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuildResult.html#blanket-implementations" class="anchor">§</a>
