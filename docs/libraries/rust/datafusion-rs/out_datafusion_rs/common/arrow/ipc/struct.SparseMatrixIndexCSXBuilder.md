# Struct SparseMatrixIndexCSXBuilder Copy item path

<a href="https://docs.rs/arrow-ipc/56.0.0/x86_64-unknown-linux-gnu/src/arrow_ipc/gen/SparseTensor.rs.html#667" class="src">Source</a>

``` rust
pub struct SparseMatrixIndexCSXBuilder<'a, 'b, A>where
    'a: 'b,
    A: Allocator + 'a,{ /* private fields */ }
```

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseMatrixIndexCSXBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseMatrixIndexCSXBuilder.html#impl-SparseMatrixIndexCSXBuilder%3C&#39;a,+&#39;b,+A%3E" class="anchor">§</a>

### impl\<'a, 'b, A\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseMatrixIndexCSXBuilder.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseMatrixIndexCSXBuilder">SparseMatrixIndexCSXBuilder</a>\<'a, 'b, A\>

where 'a: 'b, A: <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/builder/trait.Allocator.html" class="trait" title="trait flatbuffers::builder::Allocator">Allocator</a> + 'a,

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseMatrixIndexCSXBuilder.html#method.add_compressedAxis" class="fn">add_compressedAxis</a>(&mut self, compressedAxis: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseMatrixCompressedAxis.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseMatrixCompressedAxis">SparseMatrixCompressedAxis</a>)

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseMatrixIndexCSXBuilder.html#method.add_indptrType" class="fn">add_indptrType</a>(&mut self, indptrType: <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/primitives/struct.WIPOffset.html" class="struct" title="struct flatbuffers::primitives::WIPOffset">WIPOffset</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Int.html" class="struct" title="struct datafusion::common::arrow::ipc::Int">Int</a>\<'b\>\>)

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseMatrixIndexCSXBuilder.html#method.add_indptrBuffer" class="fn">add_indptrBuffer</a>(&mut self, indptrBuffer: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html" class="struct" title="struct datafusion::common::arrow::ipc::Buffer">Buffer</a>)

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseMatrixIndexCSXBuilder.html#method.add_indicesType" class="fn">add_indicesType</a>(&mut self, indicesType: <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/primitives/struct.WIPOffset.html" class="struct" title="struct flatbuffers::primitives::WIPOffset">WIPOffset</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Int.html" class="struct" title="struct datafusion::common::arrow::ipc::Int">Int</a>\<'b\>\>)

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseMatrixIndexCSXBuilder.html#method.add_indicesBuffer" class="fn">add_indicesBuffer</a>(&mut self, indicesBuffer: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html" class="struct" title="struct datafusion::common::arrow::ipc::Buffer">Buffer</a>)

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseMatrixIndexCSXBuilder.html#method.new" class="fn">new</a>( \_fbb: &'b mut <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/builder/struct.FlatBufferBuilder.html" class="struct" title="struct flatbuffers::builder::FlatBufferBuilder">FlatBufferBuilder</a>\<'a, A\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseMatrixIndexCSXBuilder.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseMatrixIndexCSXBuilder">SparseMatrixIndexCSXBuilder</a>\<'a, 'b, A\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseMatrixIndexCSXBuilder.html#method.finish" class="fn">finish</a>(self) -\> <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/primitives/struct.WIPOffset.html" class="struct" title="struct flatbuffers::primitives::WIPOffset">WIPOffset</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseMatrixIndexCSX.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseMatrixIndexCSX">SparseMatrixIndexCSX</a>\<'a\>\>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseMatrixIndexCSXBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseMatrixIndexCSXBuilder.html#blanket-implementations" class="anchor">§</a>
