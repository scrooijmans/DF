# Struct SparseTensorArgs Copy item path

<a href="https://docs.rs/arrow-ipc/56.0.0/x86_64-unknown-linux-gnu/src/arrow_ipc/gen/SparseTensor.rs.html#1797" class="src">Source</a>

``` rust
pub struct SparseTensorArgs<'a> {
    pub type_type: Type,
    pub type_: Option<WIPOffset<UnionWIPOffset>>,
    pub shape: Option<WIPOffset<Vector<'a, ForwardsUOffset<TensorDim<'a>>>>>,
    pub non_zero_length: i64,
    pub sparseIndex_type: SparseTensorIndex,
    pub sparseIndex: Option<WIPOffset<UnionWIPOffset>>,
    pub data: Option<&'a Buffer>,
}
```

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseTensorArgs.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseTensorArgs.html#structfield.type_type" class="anchor field">§</a>`type_type: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type"><code>Type</code></a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseTensorArgs.html#structfield.type_" class="anchor field">§</a>`type_: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/primitives/struct.WIPOffset.html" class="struct" title="struct flatbuffers::primitives::WIPOffset"><code>WIPOffset</code></a>`<`<a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/primitives/struct.UnionWIPOffset.html" class="struct" title="struct flatbuffers::primitives::UnionWIPOffset"><code>UnionWIPOffset</code></a>`>>`<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseTensorArgs.html#structfield.shape" class="anchor field">§</a>`shape: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/primitives/struct.WIPOffset.html" class="struct" title="struct flatbuffers::primitives::WIPOffset"><code>WIPOffset</code></a>`<`<a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/vector/struct.Vector.html" class="struct" title="struct flatbuffers::vector::Vector"><code>Vector</code></a>`<'a, `<a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/primitives/struct.ForwardsUOffset.html" class="struct" title="struct flatbuffers::primitives::ForwardsUOffset"><code>ForwardsUOffset</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.TensorDim.html" class="struct" title="struct datafusion::common::arrow::ipc::TensorDim"><code>TensorDim</code></a>`<'a>>>>>`<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseTensorArgs.html#structfield.non_zero_length" class="anchor field">§</a>`non_zero_length: `<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive"><code>i64</code></a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseTensorArgs.html#structfield.sparseIndex_type" class="anchor field">§</a>`sparseIndex_type: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndex.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseTensorIndex"><code>SparseTensorIndex</code></a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseTensorArgs.html#structfield.sparseIndex" class="anchor field">§</a>`sparseIndex: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/primitives/struct.WIPOffset.html" class="struct" title="struct flatbuffers::primitives::WIPOffset"><code>WIPOffset</code></a>`<`<a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/primitives/struct.UnionWIPOffset.html" class="struct" title="struct flatbuffers::primitives::UnionWIPOffset"><code>UnionWIPOffset</code></a>`>>`<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseTensorArgs.html#structfield.data" class="anchor field">§</a>`data: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<&'a `<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html" class="struct" title="struct datafusion::common::arrow::ipc::Buffer"><code>Buffer</code></a>`>`

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseTensorArgs.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseTensorArgs.html#impl-Default-for-SparseTensorArgs%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorArgs.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseTensorArgs">SparseTensorArgs</a>\<'a\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseTensorArgs.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorArgs.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseTensorArgs">SparseTensorArgs</a>\<'a\>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseTensorArgs.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseTensorArgs.html#blanket-implementations" class="anchor">§</a>
