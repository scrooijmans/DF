# Struct PartitionSpec Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/partition.rs.html#67-72" class="src">Source</a>

``` rust
pub struct PartitionSpec { /* private fields */ }
```

Expand description

Partition spec that defines how to produce a tuple of partition values from a record.

A [`PartitionSpec`](https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html "struct iceberg::spec::PartitionSpec") is originally obtained by binding an [`UnboundPartitionSpec`](https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpec.html "struct iceberg::spec::UnboundPartitionSpec") to a schema and is only guaranteed to be valid for that schema. The main difference between [`PartitionSpec`](https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html "struct iceberg::spec::PartitionSpec") and [`UnboundPartitionSpec`](https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpec.html "struct iceberg::spec::UnboundPartitionSpec") is that the former has field ids assigned, while field ids are optional for [`UnboundPartitionSpec`](https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpec.html "struct iceberg::spec::UnboundPartitionSpec").

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html#impl-PartitionSpec" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html" class="struct" title="struct iceberg::spec::PartitionSpec">PartitionSpec</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html#method.builder" class="fn">builder</a>(schema: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.SchemaRef.html" class="type" title="type iceberg::spec::SchemaRef">SchemaRef</a>\>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpecBuilder.html" class="struct" title="struct iceberg::spec::PartitionSpecBuilder">PartitionSpecBuilder</a>

Create a new partition spec builder with the given schema.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html#method.fields" class="fn">fields</a>(&self) -\> &\[<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionField.html" class="struct" title="struct iceberg::spec::PartitionField">PartitionField</a>\]

Fields of the partition spec

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html#method.spec_id" class="fn">spec_id</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

Spec id of the partition spec

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html#method.unpartition_spec" class="fn">unpartition_spec</a>() -\> Self

Get a new unpartitioned partition spec

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html#method.is_unpartitioned" class="fn">is_unpartitioned</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns if the partition spec is unpartitioned.

A [`PartitionSpec`](https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html "struct iceberg::spec::PartitionSpec") is unpartitioned if it has no fields or all fields are [`Transform::Void`](https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html#variant.Void "variant iceberg::spec::Transform::Void") transform.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html#method.partition_type" class="fn">partition_type</a>(&self, schema: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html" class="struct" title="struct iceberg::spec::Schema">Schema</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.StructType.html" class="struct" title="struct iceberg::spec::StructType">StructType</a>\>

Returns the partition type of this partition spec.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html#method.into_unbound" class="fn">into_unbound</a>(self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpec.html" class="struct" title="struct iceberg::spec::UnboundPartitionSpec">UnboundPartitionSpec</a>

Convert to unbound partition spec

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html#method.with_spec_id" class="fn">with_spec_id</a>(self, spec_id: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>) -\> Self

Change the spec id of the partition spec

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html#method.has_sequential_ids" class="fn">has_sequential_ids</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Check if this partition spec has sequential partition ids. Sequential ids start from 1000 and increment by 1 for each field. This is required for spec version 1

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html#method.highest_field_id" class="fn">highest_field_id</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>

Get the highest field id in the partition spec.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html#method.is_compatible_with" class="fn">is_compatible_with</a>(&self, other: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html" class="struct" title="struct iceberg::spec::PartitionSpec">PartitionSpec</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Check if this partition spec is compatible with another partition spec.

Returns true if the partition spec is equal to the other spec with partition field ids ignored and spec_id ignored. The following must be identical:

- The number of fields
- Field order
- Field names
- Source column ids
- Transforms

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html#impl-Clone-for-PartitionSpec" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html" class="struct" title="struct iceberg::spec::PartitionSpec">PartitionSpec</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html" class="struct" title="struct iceberg::spec::PartitionSpec">PartitionSpec</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html#impl-Debug-for-PartitionSpec" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html" class="struct" title="struct iceberg::spec::PartitionSpec">PartitionSpec</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html#impl-Deserialize%3C&#39;de%3E-for-PartitionSpec" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html" class="struct" title="struct iceberg::spec::PartitionSpec">PartitionSpec</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>(\_\_deserializer: \_\_D) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, \_\_D::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html#impl-From%3CPartitionSpec%3E-for-UnboundPartitionSpec" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html" class="struct" title="struct iceberg::spec::PartitionSpec">PartitionSpec</a>\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpec.html" class="struct" title="struct iceberg::spec::UnboundPartitionSpec">UnboundPartitionSpec</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(spec: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html" class="struct" title="struct iceberg::spec::PartitionSpec">PartitionSpec</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html#impl-PartialEq-for-PartitionSpec" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html" class="struct" title="struct iceberg::spec::PartitionSpec">PartitionSpec</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html" class="struct" title="struct iceberg::spec::PartitionSpec">PartitionSpec</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html#impl-Serialize-for-PartitionSpec" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html" class="struct" title="struct iceberg::spec::PartitionSpec">PartitionSpec</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>(&self, \_\_serializer: \_\_S) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\_\_S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \_\_S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html#impl-Eq-for-PartitionSpec" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html" class="struct" title="struct iceberg::spec::PartitionSpec">PartitionSpec</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html#impl-StructuralPartialEq-for-PartitionSpec" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html" class="struct" title="struct iceberg::spec::PartitionSpec">PartitionSpec</a>

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html#blanket-implementations" class="anchor">§</a>
