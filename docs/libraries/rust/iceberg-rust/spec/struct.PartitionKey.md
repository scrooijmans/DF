# Struct PartitionKey Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/partition.rs.html#182-189" class="src">Source</a>

``` rust
pub struct PartitionKey { /* private fields */ }
```

Expand description

A partition key represents a specific partition in a table, containing the partition spec, schema, and the actual partition values.

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionKey.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionKey.html#impl-PartitionKey" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionKey.html" class="struct" title="struct iceberg::spec::PartitionKey">PartitionKey</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionKey.html#method.new" class="fn">new</a>(spec: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html" class="struct" title="struct iceberg::spec::PartitionSpec">PartitionSpec</a>, schema: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.SchemaRef.html" class="type" title="type iceberg::spec::SchemaRef">SchemaRef</a>, data: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Struct.html" class="struct" title="struct iceberg::spec::Struct">Struct</a>) -\> Self

Creates a new partition key with the given spec, schema, and data.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionKey.html#method.to_path" class="fn">to_path</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

Generates a partition path based on the partition values.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionKey.html#method.is_effectively_none" class="fn">is_effectively_none</a>(partition_key: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionKey.html" class="struct" title="struct iceberg::spec::PartitionKey">PartitionKey</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns `true` if the partition key is absent (`None`) or represents an unpartitioned spec.

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionKey.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionKey.html#impl-Clone-for-PartitionKey" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionKey.html" class="struct" title="struct iceberg::spec::PartitionKey">PartitionKey</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionKey.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionKey.html" class="struct" title="struct iceberg::spec::PartitionKey">PartitionKey</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionKey.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionKey.html#impl-Debug-for-PartitionKey" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionKey.html" class="struct" title="struct iceberg::spec::PartitionKey">PartitionKey</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionKey.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionKey.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionKey.html#blanket-implementations" class="anchor">§</a>
