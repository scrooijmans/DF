# Enum TableRequirement Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/catalog/mod.rs.html#367-424" class="src">Source</a>

``` rust
pub enum TableRequirement {
    NotExist,
    UuidMatch {
        uuid: Uuid,
    },
    RefSnapshotIdMatch {
        ref: String,
        snapshot_id: Option<i64>,
    },
    LastAssignedFieldIdMatch {
        last_assigned_field_id: i32,
    },
    CurrentSchemaIdMatch {
        current_schema_id: SchemaId,
    },
    LastAssignedPartitionIdMatch {
        last_assigned_partition_id: i32,
    },
    DefaultSpecIdMatch {
        default_spec_id: i32,
    },
    DefaultSortOrderIdMatch {
        default_sort_order_id: i64,
    },
}
```

Expand description

TableRequirement represents a requirement for a table in the catalog.

## Variants<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableRequirement.html#variants" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableRequirement.html#variant.NotExist" class="anchor">§</a>

### NotExist

The table must not already exist; used for create transactions

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableRequirement.html#variant.UuidMatch" class="anchor">§</a>

### UuidMatch

The table UUID must match the requirement.

#### Fields

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableRequirement.html#variant.UuidMatch.field.uuid" class="anchor field">§</a>`uuid: `<a href="https://docs.rs/uuid/1.18.1/x86_64-unknown-linux-gnu/uuid/struct.Uuid.html" class="struct" title="struct uuid::Uuid"><code>Uuid</code></a>

Uuid of original table.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableRequirement.html#variant.RefSnapshotIdMatch" class="anchor">§</a>

### RefSnapshotIdMatch

The table branch or tag identified by the requirement’s `reference` must reference the requirement’s `snapshot-id`.

#### Fields

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableRequirement.html#variant.RefSnapshotIdMatch.field.ref" class="anchor field">§</a>`ref: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

The reference of the table to assert.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableRequirement.html#variant.RefSnapshotIdMatch.field.snapshot_id" class="anchor field">§</a>`snapshot_id: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive"><code>i64</code></a>`>`

The snapshot id of the table to assert. If the id is `None`, the ref must not already exist.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableRequirement.html#variant.LastAssignedFieldIdMatch" class="anchor">§</a>

### LastAssignedFieldIdMatch

The table’s last assigned column id must match the requirement.

#### Fields

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableRequirement.html#variant.LastAssignedFieldIdMatch.field.last_assigned_field_id" class="anchor field">§</a>`last_assigned_field_id: `<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive"><code>i32</code></a>

The last assigned field id of the table to assert.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableRequirement.html#variant.CurrentSchemaIdMatch" class="anchor">§</a>

### CurrentSchemaIdMatch

The table’s current schema id must match the requirement.

#### Fields

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableRequirement.html#variant.CurrentSchemaIdMatch.field.current_schema_id" class="anchor field">§</a>`current_schema_id: `<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.SchemaId.html" class="type" title="type iceberg::spec::SchemaId"><code>SchemaId</code></a>

Current schema id of the table to assert.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableRequirement.html#variant.LastAssignedPartitionIdMatch" class="anchor">§</a>

### LastAssignedPartitionIdMatch

The table’s last assigned partition id must match the requirement.

#### Fields

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableRequirement.html#variant.LastAssignedPartitionIdMatch.field.last_assigned_partition_id" class="anchor field">§</a>`last_assigned_partition_id: `<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive"><code>i32</code></a>

Last assigned partition id of the table to assert.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableRequirement.html#variant.DefaultSpecIdMatch" class="anchor">§</a>

### DefaultSpecIdMatch

The table’s default spec id must match the requirement.

#### Fields

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableRequirement.html#variant.DefaultSpecIdMatch.field.default_spec_id" class="anchor field">§</a>`default_spec_id: `<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive"><code>i32</code></a>

Default spec id of the table to assert.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableRequirement.html#variant.DefaultSortOrderIdMatch" class="anchor">§</a>

### DefaultSortOrderIdMatch

The table’s default sort order id must match the requirement.

#### Fields

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableRequirement.html#variant.DefaultSortOrderIdMatch.field.default_sort_order_id" class="anchor field">§</a>`default_sort_order_id: `<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive"><code>i64</code></a>

Default sort order id of the table to assert.

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableRequirement.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableRequirement.html#impl-TableRequirement" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableRequirement.html" class="enum" title="enum iceberg::TableRequirement">TableRequirement</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableRequirement.html#method.check" class="fn">check</a>(&self, metadata: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html" class="struct" title="struct iceberg::spec::TableMetadata">TableMetadata</a>\>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Check that the requirement is met by the table metadata. If the requirement is not met, an appropriate error is returned.

Provide metadata as `None` if the table does not exist.

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableRequirement.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableRequirement.html#impl-Clone-for-TableRequirement" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableRequirement.html" class="enum" title="enum iceberg::TableRequirement">TableRequirement</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableRequirement.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableRequirement.html" class="enum" title="enum iceberg::TableRequirement">TableRequirement</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableRequirement.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableRequirement.html#impl-Debug-for-TableRequirement" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableRequirement.html" class="enum" title="enum iceberg::TableRequirement">TableRequirement</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableRequirement.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableRequirement.html#impl-Deserialize%3C&#39;de%3E-for-TableRequirement" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableRequirement.html" class="enum" title="enum iceberg::TableRequirement">TableRequirement</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableRequirement.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>(\_\_deserializer: \_\_D) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, \_\_D::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableRequirement.html#impl-PartialEq-for-TableRequirement" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableRequirement.html" class="enum" title="enum iceberg::TableRequirement">TableRequirement</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableRequirement.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableRequirement.html" class="enum" title="enum iceberg::TableRequirement">TableRequirement</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableRequirement.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableRequirement.html#impl-Serialize-for-TableRequirement" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableRequirement.html" class="enum" title="enum iceberg::TableRequirement">TableRequirement</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableRequirement.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>(&self, \_\_serializer: \_\_S) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\_\_S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \_\_S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableRequirement.html#impl-StructuralPartialEq-for-TableRequirement" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableRequirement.html" class="enum" title="enum iceberg::TableRequirement">TableRequirement</a>

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableRequirement.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableRequirement.html#blanket-implementations" class="anchor">§</a>
