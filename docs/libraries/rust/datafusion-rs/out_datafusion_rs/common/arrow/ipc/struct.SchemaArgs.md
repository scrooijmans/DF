# Struct SchemaArgs Copy item path

<a href="https://docs.rs/arrow-ipc/56.0.0/x86_64-unknown-linux-gnu/src/arrow_ipc/gen/Schema.rs.html#5443" class="src">Source</a>

``` rust
pub struct SchemaArgs<'a> {
    pub endianness: Endianness,
    pub fields: Option<WIPOffset<Vector<'a, ForwardsUOffset<Field<'a>>>>>,
    pub custom_metadata: Option<WIPOffset<Vector<'a, ForwardsUOffset<KeyValue<'a>>>>>,
    pub features: Option<WIPOffset<Vector<'a, Feature>>>,
}
```

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SchemaArgs.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SchemaArgs.html#structfield.endianness" class="anchor field">§</a>`endianness: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Endianness.html" class="struct" title="struct datafusion::common::arrow::ipc::Endianness"><code>Endianness</code></a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SchemaArgs.html#structfield.fields" class="anchor field">§</a>`fields: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/primitives/struct.WIPOffset.html" class="struct" title="struct flatbuffers::primitives::WIPOffset"><code>WIPOffset</code></a>`<`<a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/vector/struct.Vector.html" class="struct" title="struct flatbuffers::vector::Vector"><code>Vector</code></a>`<'a, `<a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/primitives/struct.ForwardsUOffset.html" class="struct" title="struct flatbuffers::primitives::ForwardsUOffset"><code>ForwardsUOffset</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Field.html" class="struct" title="struct datafusion::common::arrow::ipc::Field"><code>Field</code></a>`<'a>>>>>`<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SchemaArgs.html#structfield.custom_metadata" class="anchor field">§</a>`custom_metadata: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/primitives/struct.WIPOffset.html" class="struct" title="struct flatbuffers::primitives::WIPOffset"><code>WIPOffset</code></a>`<`<a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/vector/struct.Vector.html" class="struct" title="struct flatbuffers::vector::Vector"><code>Vector</code></a>`<'a, `<a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/primitives/struct.ForwardsUOffset.html" class="struct" title="struct flatbuffers::primitives::ForwardsUOffset"><code>ForwardsUOffset</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.KeyValue.html" class="struct" title="struct datafusion::common::arrow::ipc::KeyValue"><code>KeyValue</code></a>`<'a>>>>>`<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SchemaArgs.html#structfield.features" class="anchor field">§</a>`features: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/primitives/struct.WIPOffset.html" class="struct" title="struct flatbuffers::primitives::WIPOffset"><code>WIPOffset</code></a>`<`<a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/vector/struct.Vector.html" class="struct" title="struct flatbuffers::vector::Vector"><code>Vector</code></a>`<'a, `<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Feature.html" class="struct" title="struct datafusion::common::arrow::ipc::Feature"><code>Feature</code></a>`>>>`

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SchemaArgs.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SchemaArgs.html#impl-Default-for-SchemaArgs%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SchemaArgs.html" class="struct" title="struct datafusion::common::arrow::ipc::SchemaArgs">SchemaArgs</a>\<'a\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SchemaArgs.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SchemaArgs.html" class="struct" title="struct datafusion::common::arrow::ipc::SchemaArgs">SchemaArgs</a>\<'a\>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SchemaArgs.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SchemaArgs.html#blanket-implementations" class="anchor">§</a>
