# Struct SparseTensorIndexCSFBuilder Copy item path

<a href="https://docs.rs/arrow-ipc/56.0.0/x86_64-unknown-linux-gnu/src/arrow_ipc/gen/SparseTensor.rs.html#982" class="src">Source</a>

``` rust
pub struct SparseTensorIndexCSFBuilder<'a, 'b, A>where
    'a: 'b,
    A: Allocator + 'a,{ /* private fields */ }
```

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseTensorIndexCSFBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseTensorIndexCSFBuilder.html#impl-SparseTensorIndexCSFBuilder%3C&#39;a,+&#39;b,+A%3E" class="anchor">§</a>

### impl\<'a, 'b, A\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSFBuilder.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseTensorIndexCSFBuilder">SparseTensorIndexCSFBuilder</a>\<'a, 'b, A\>

where 'a: 'b, A: <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/builder/trait.Allocator.html" class="trait" title="trait flatbuffers::builder::Allocator">Allocator</a> + 'a,

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseTensorIndexCSFBuilder.html#method.add_indptrType" class="fn">add_indptrType</a>(&mut self, indptrType: <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/primitives/struct.WIPOffset.html" class="struct" title="struct flatbuffers::primitives::WIPOffset">WIPOffset</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Int.html" class="struct" title="struct datafusion::common::arrow::ipc::Int">Int</a>\<'b\>\>)

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseTensorIndexCSFBuilder.html#method.add_indptrBuffers" class="fn">add_indptrBuffers</a>( &mut self, indptrBuffers: <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/primitives/struct.WIPOffset.html" class="struct" title="struct flatbuffers::primitives::WIPOffset">WIPOffset</a>\<<a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/vector/struct.Vector.html" class="struct" title="struct flatbuffers::vector::Vector">Vector</a>\<'b, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html" class="struct" title="struct datafusion::common::arrow::ipc::Buffer">Buffer</a>\>\>, )

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseTensorIndexCSFBuilder.html#method.add_indicesType" class="fn">add_indicesType</a>(&mut self, indicesType: <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/primitives/struct.WIPOffset.html" class="struct" title="struct flatbuffers::primitives::WIPOffset">WIPOffset</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Int.html" class="struct" title="struct datafusion::common::arrow::ipc::Int">Int</a>\<'b\>\>)

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseTensorIndexCSFBuilder.html#method.add_indicesBuffers" class="fn">add_indicesBuffers</a>( &mut self, indicesBuffers: <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/primitives/struct.WIPOffset.html" class="struct" title="struct flatbuffers::primitives::WIPOffset">WIPOffset</a>\<<a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/vector/struct.Vector.html" class="struct" title="struct flatbuffers::vector::Vector">Vector</a>\<'b, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html" class="struct" title="struct datafusion::common::arrow::ipc::Buffer">Buffer</a>\>\>, )

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseTensorIndexCSFBuilder.html#method.add_axisOrder" class="fn">add_axisOrder</a>(&mut self, axisOrder: <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/primitives/struct.WIPOffset.html" class="struct" title="struct flatbuffers::primitives::WIPOffset">WIPOffset</a>\<<a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/vector/struct.Vector.html" class="struct" title="struct flatbuffers::vector::Vector">Vector</a>\<'b, <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>\>)

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseTensorIndexCSFBuilder.html#method.new" class="fn">new</a>( \_fbb: &'b mut <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/builder/struct.FlatBufferBuilder.html" class="struct" title="struct flatbuffers::builder::FlatBufferBuilder">FlatBufferBuilder</a>\<'a, A\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSFBuilder.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseTensorIndexCSFBuilder">SparseTensorIndexCSFBuilder</a>\<'a, 'b, A\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseTensorIndexCSFBuilder.html#method.finish" class="fn">finish</a>(self) -\> <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/primitives/struct.WIPOffset.html" class="struct" title="struct flatbuffers::primitives::WIPOffset">WIPOffset</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSF.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseTensorIndexCSF">SparseTensorIndexCSF</a>\<'a\>\>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseTensorIndexCSFBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseTensorIndexCSFBuilder.html#blanket-implementations" class="anchor">§</a>
