# Struct SparseTensorIndexCSFArgs Copy item path

<a href="https://docs.rs/arrow-ipc/56.0.0/x86_64-unknown-linux-gnu/src/arrow_ipc/gen/SparseTensor.rs.html#962" class="src">Source</a>

``` rust
pub struct SparseTensorIndexCSFArgs<'a> {
    pub indptrType: Option<WIPOffset<Int<'a>>>,
    pub indptrBuffers: Option<WIPOffset<Vector<'a, Buffer>>>,
    pub indicesType: Option<WIPOffset<Int<'a>>>,
    pub indicesBuffers: Option<WIPOffset<Vector<'a, Buffer>>>,
    pub axisOrder: Option<WIPOffset<Vector<'a, i32>>>,
}
```

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSFArgs.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSFArgs.html#structfield.indptrType" class="anchor field">§</a>`indptrType: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/primitives/struct.WIPOffset.html" class="struct" title="struct flatbuffers::primitives::WIPOffset"><code>WIPOffset</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Int.html" class="struct" title="struct datafusion::common::arrow::ipc::Int"><code>Int</code></a>`<'a>>>`<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSFArgs.html#structfield.indptrBuffers" class="anchor field">§</a>`indptrBuffers: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/primitives/struct.WIPOffset.html" class="struct" title="struct flatbuffers::primitives::WIPOffset"><code>WIPOffset</code></a>`<`<a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/vector/struct.Vector.html" class="struct" title="struct flatbuffers::vector::Vector"><code>Vector</code></a>`<'a, `<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html" class="struct" title="struct datafusion::common::arrow::ipc::Buffer"><code>Buffer</code></a>`>>>`<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSFArgs.html#structfield.indicesType" class="anchor field">§</a>`indicesType: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/primitives/struct.WIPOffset.html" class="struct" title="struct flatbuffers::primitives::WIPOffset"><code>WIPOffset</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Int.html" class="struct" title="struct datafusion::common::arrow::ipc::Int"><code>Int</code></a>`<'a>>>`<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSFArgs.html#structfield.indicesBuffers" class="anchor field">§</a>`indicesBuffers: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/primitives/struct.WIPOffset.html" class="struct" title="struct flatbuffers::primitives::WIPOffset"><code>WIPOffset</code></a>`<`<a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/vector/struct.Vector.html" class="struct" title="struct flatbuffers::vector::Vector"><code>Vector</code></a>`<'a, `<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html" class="struct" title="struct datafusion::common::arrow::ipc::Buffer"><code>Buffer</code></a>`>>>`<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSFArgs.html#structfield.axisOrder" class="anchor field">§</a>`axisOrder: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/primitives/struct.WIPOffset.html" class="struct" title="struct flatbuffers::primitives::WIPOffset"><code>WIPOffset</code></a>`<`<a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/vector/struct.Vector.html" class="struct" title="struct flatbuffers::vector::Vector"><code>Vector</code></a>`<'a, `<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive"><code>i32</code></a>`>>>`

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSFArgs.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSFArgs.html#impl-Default-for-SparseTensorIndexCSFArgs%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSFArgs.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseTensorIndexCSFArgs">SparseTensorIndexCSFArgs</a>\<'a\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSFArgs.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSFArgs.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseTensorIndexCSFArgs">SparseTensorIndexCSFArgs</a>\<'a\>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSFArgs.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSFArgs.html#blanket-implementations" class="anchor">§</a>
