# Struct UnboundPartitionSpecBuilder Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/partition.rs.html#309-312" class="src">Source</a>

``` rust
pub struct UnboundPartitionSpecBuilder { /* private fields */ }
```

Expand description

Create a new UnboundPartitionSpec

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpecBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpecBuilder.html#impl-UnboundPartitionSpecBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpecBuilder.html" class="struct" title="struct iceberg::spec::UnboundPartitionSpecBuilder">UnboundPartitionSpecBuilder</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpecBuilder.html#method.new" class="fn">new</a>() -\> Self

Create a new partition spec builder with the given schema.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpecBuilder.html#method.with_spec_id" class="fn">with_spec_id</a>(self, spec_id: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>) -\> Self

Set the spec id for the partition spec.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpecBuilder.html#method.add_partition_field" class="fn">add_partition_field</a>( self, source_id: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, target_name: impl <a href="https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html" class="trait" title="trait alloc::string::ToString">ToString</a>, transformation: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html" class="enum" title="enum iceberg::spec::Transform">Transform</a>, ) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Add a new partition field to the partition spec from an unbound partition field.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpecBuilder.html#method.add_partition_fields" class="fn">add_partition_fields</a>( self, fields: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionField.html" class="struct" title="struct iceberg::spec::UnboundPartitionField">UnboundPartitionField</a>\>, ) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Add multiple partition fields to the partition spec.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpecBuilder.html#method.build" class="fn">build</a>(self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpec.html" class="struct" title="struct iceberg::spec::UnboundPartitionSpec">UnboundPartitionSpec</a>

Build the unbound partition spec.

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpecBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpecBuilder.html#impl-Debug-for-UnboundPartitionSpecBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpecBuilder.html" class="struct" title="struct iceberg::spec::UnboundPartitionSpecBuilder">UnboundPartitionSpecBuilder</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpecBuilder.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpecBuilder.html#impl-Default-for-UnboundPartitionSpecBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpecBuilder.html" class="struct" title="struct iceberg::spec::UnboundPartitionSpecBuilder">UnboundPartitionSpecBuilder</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpecBuilder.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpecBuilder.html" class="struct" title="struct iceberg::spec::UnboundPartitionSpecBuilder">UnboundPartitionSpecBuilder</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpecBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpecBuilder.html#blanket-implementations" class="anchor">§</a>
