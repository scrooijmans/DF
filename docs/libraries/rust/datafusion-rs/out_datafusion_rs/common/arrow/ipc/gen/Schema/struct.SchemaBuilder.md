# Struct SchemaBuilder Copy item path

<a href="https://docs.rs/arrow-ipc/56.0.0/x86_64-unknown-linux-gnu/src/arrow_ipc/gen/Schema.rs.html#5465" class="src">Source</a>

``` rust
pub struct SchemaBuilder<'a, 'b, A>where
    'a: 'b,
    A: Allocator + 'a,{ /* private fields */ }
```

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Schema/struct.SchemaBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Schema/struct.SchemaBuilder.html#impl-SchemaBuilder%3C&#39;a,+&#39;b,+A%3E" class="anchor">§</a>

### impl\<'a, 'b, A\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SchemaBuilder.html" class="struct" title="struct datafusion::common::arrow::ipc::SchemaBuilder">SchemaBuilder</a>\<'a, 'b, A\>

where 'a: 'b, A: <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/builder/trait.Allocator.html" class="trait" title="trait flatbuffers::builder::Allocator">Allocator</a> + 'a,

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Schema/struct.SchemaBuilder.html#method.add_endianness" class="fn">add_endianness</a>(&mut self, endianness: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Endianness.html" class="struct" title="struct datafusion::common::arrow::ipc::Endianness">Endianness</a>)

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Schema/struct.SchemaBuilder.html#method.add_fields" class="fn">add_fields</a>( &mut self, fields: <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/primitives/struct.WIPOffset.html" class="struct" title="struct flatbuffers::primitives::WIPOffset">WIPOffset</a>\<<a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/vector/struct.Vector.html" class="struct" title="struct flatbuffers::vector::Vector">Vector</a>\<'b, <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/primitives/struct.ForwardsUOffset.html" class="struct" title="struct flatbuffers::primitives::ForwardsUOffset">ForwardsUOffset</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Field.html" class="struct" title="struct datafusion::common::arrow::ipc::Field">Field</a>\<'b\>\>\>\>, )

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Schema/struct.SchemaBuilder.html#method.add_custom_metadata" class="fn">add_custom_metadata</a>( &mut self, custom_metadata: <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/primitives/struct.WIPOffset.html" class="struct" title="struct flatbuffers::primitives::WIPOffset">WIPOffset</a>\<<a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/vector/struct.Vector.html" class="struct" title="struct flatbuffers::vector::Vector">Vector</a>\<'b, <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/primitives/struct.ForwardsUOffset.html" class="struct" title="struct flatbuffers::primitives::ForwardsUOffset">ForwardsUOffset</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.KeyValue.html" class="struct" title="struct datafusion::common::arrow::ipc::KeyValue">KeyValue</a>\<'b\>\>\>\>, )

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Schema/struct.SchemaBuilder.html#method.add_features" class="fn">add_features</a>(&mut self, features: <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/primitives/struct.WIPOffset.html" class="struct" title="struct flatbuffers::primitives::WIPOffset">WIPOffset</a>\<<a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/vector/struct.Vector.html" class="struct" title="struct flatbuffers::vector::Vector">Vector</a>\<'b, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Feature.html" class="struct" title="struct datafusion::common::arrow::ipc::Feature">Feature</a>\>\>)

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Schema/struct.SchemaBuilder.html#method.new" class="fn">new</a>(\_fbb: &'b mut <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/builder/struct.FlatBufferBuilder.html" class="struct" title="struct flatbuffers::builder::FlatBufferBuilder">FlatBufferBuilder</a>\<'a, A\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SchemaBuilder.html" class="struct" title="struct datafusion::common::arrow::ipc::SchemaBuilder">SchemaBuilder</a>\<'a, 'b, A\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Schema/struct.SchemaBuilder.html#method.finish" class="fn">finish</a>(self) -\> <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/primitives/struct.WIPOffset.html" class="struct" title="struct flatbuffers::primitives::WIPOffset">WIPOffset</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::ipc::Schema">Schema</a>\<'a\>\>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Schema/struct.SchemaBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Schema/struct.SchemaBuilder.html#blanket-implementations" class="anchor">§</a>
