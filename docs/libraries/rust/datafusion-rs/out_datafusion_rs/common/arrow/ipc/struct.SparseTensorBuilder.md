# Struct SparseTensorBuilder Copy item path

<a href="https://docs.rs/arrow-ipc/56.0.0/x86_64-unknown-linux-gnu/src/arrow_ipc/gen/SparseTensor.rs.html#1825" class="src">Source</a>

``` rust
pub struct SparseTensorBuilder<'a, 'b, A>where
    'a: 'b,
    A: Allocator + 'a,{ /* private fields */ }
```

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorBuilder.html#impl-SparseTensorBuilder%3C&#39;a,+&#39;b,+A%3E" class="anchor">§</a>

### impl\<'a, 'b, A\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorBuilder.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseTensorBuilder">SparseTensorBuilder</a>\<'a, 'b, A\>

where 'a: 'b, A: <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/builder/trait.Allocator.html" class="trait" title="trait flatbuffers::builder::Allocator">Allocator</a> + 'a,

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorBuilder.html#method.add_type_type" class="fn">add_type_type</a>(&mut self, type_type: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a>)

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorBuilder.html#method.add_type_" class="fn">add_type_</a>(&mut self, type\_: <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/primitives/struct.WIPOffset.html" class="struct" title="struct flatbuffers::primitives::WIPOffset">WIPOffset</a>\<<a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/primitives/struct.UnionWIPOffset.html" class="struct" title="struct flatbuffers::primitives::UnionWIPOffset">UnionWIPOffset</a>\>)

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorBuilder.html#method.add_shape" class="fn">add_shape</a>( &mut self, shape: <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/primitives/struct.WIPOffset.html" class="struct" title="struct flatbuffers::primitives::WIPOffset">WIPOffset</a>\<<a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/vector/struct.Vector.html" class="struct" title="struct flatbuffers::vector::Vector">Vector</a>\<'b, <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/primitives/struct.ForwardsUOffset.html" class="struct" title="struct flatbuffers::primitives::ForwardsUOffset">ForwardsUOffset</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.TensorDim.html" class="struct" title="struct datafusion::common::arrow::ipc::TensorDim">TensorDim</a>\<'b\>\>\>\>, )

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorBuilder.html#method.add_non_zero_length" class="fn">add_non_zero_length</a>(&mut self, non_zero_length: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>)

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorBuilder.html#method.add_sparseIndex_type" class="fn">add_sparseIndex_type</a>(&mut self, sparseIndex_type: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndex.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseTensorIndex">SparseTensorIndex</a>)

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorBuilder.html#method.add_sparseIndex" class="fn">add_sparseIndex</a>(&mut self, sparseIndex: <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/primitives/struct.WIPOffset.html" class="struct" title="struct flatbuffers::primitives::WIPOffset">WIPOffset</a>\<<a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/primitives/struct.UnionWIPOffset.html" class="struct" title="struct flatbuffers::primitives::UnionWIPOffset">UnionWIPOffset</a>\>)

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorBuilder.html#method.add_data" class="fn">add_data</a>(&mut self, data: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html" class="struct" title="struct datafusion::common::arrow::ipc::Buffer">Buffer</a>)

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorBuilder.html#method.new" class="fn">new</a>( \_fbb: &'b mut <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/builder/struct.FlatBufferBuilder.html" class="struct" title="struct flatbuffers::builder::FlatBufferBuilder">FlatBufferBuilder</a>\<'a, A\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorBuilder.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseTensorBuilder">SparseTensorBuilder</a>\<'a, 'b, A\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorBuilder.html#method.finish" class="fn">finish</a>(self) -\> <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/primitives/struct.WIPOffset.html" class="struct" title="struct flatbuffers::primitives::WIPOffset">WIPOffset</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensor.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseTensor">SparseTensor</a>\<'a\>\>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorBuilder.html#blanket-implementations" class="anchor">§</a>
