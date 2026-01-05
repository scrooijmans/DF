# Struct PartitionSpecBuilder Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/partition.rs.html#378-383" class="src">Source</a>

``` rust
pub struct PartitionSpecBuilder { /* private fields */ }
```

Expand description

Create valid partition specs for a given schema.

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpecBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpecBuilder.html#impl-PartitionSpecBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpecBuilder.html" class="struct" title="struct iceberg::spec::PartitionSpecBuilder">PartitionSpecBuilder</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpecBuilder.html#method.new" class="fn">new</a>(schema: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.SchemaRef.html" class="type" title="type iceberg::spec::SchemaRef">SchemaRef</a>\>) -\> Self

Create a new partition spec builder with the given schema.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpecBuilder.html#method.new_from_unbound" class="fn">new_from_unbound</a>( unbound: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpec.html" class="struct" title="struct iceberg::spec::UnboundPartitionSpec">UnboundPartitionSpec</a>, schema: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.SchemaRef.html" class="type" title="type iceberg::spec::SchemaRef">SchemaRef</a>\>, ) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Create a new partition spec builder from an existing unbound partition spec.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpecBuilder.html#method.with_last_assigned_field_id" class="fn">with_last_assigned_field_id</a>(self, last_assigned_field_id: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>) -\> Self

Set the last assigned field id for the partition spec.

Set this field when a new partition spec is created for an existing TableMetaData. As `field_id` must be unique in V2 metadata, this should be set to the highest field id used previously.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpecBuilder.html#method.with_spec_id" class="fn">with_spec_id</a>(self, spec_id: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>) -\> Self

Set the spec id for the partition spec.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpecBuilder.html#method.add_partition_field" class="fn">add_partition_field</a>( self, source_name: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>, target_name: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, transform: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html" class="enum" title="enum iceberg::spec::Transform">Transform</a>, ) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Add a new partition field to the partition spec.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpecBuilder.html#method.add_unbound_field" class="fn">add_unbound_field</a>(self, field: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionField.html" class="struct" title="struct iceberg::spec::UnboundPartitionField">UnboundPartitionField</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Add a new partition field to the partition spec.

If partition field id is set, it is used as the field id. Otherwise, a new `field_id` is assigned.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpecBuilder.html#method.add_unbound_fields" class="fn">add_unbound_fields</a>( self, fields: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionField.html" class="struct" title="struct iceberg::spec::UnboundPartitionField">UnboundPartitionField</a>\>, ) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Wrapper around `with_unbound_fields` to add multiple partition fields.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpecBuilder.html#method.build" class="fn">build</a>(self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html" class="struct" title="struct iceberg::spec::PartitionSpec">PartitionSpec</a>\>

Build a bound partition spec with the given schema.

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpecBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpecBuilder.html#impl-Debug-for-PartitionSpecBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpecBuilder.html" class="struct" title="struct iceberg::spec::PartitionSpecBuilder">PartitionSpecBuilder</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpecBuilder.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpecBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpecBuilder.html#blanket-implementations" class="anchor">§</a>
