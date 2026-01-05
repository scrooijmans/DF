# Struct SparseMatrixIndexCSX Copy item path

<a href="https://docs.rs/arrow-ipc/56.0.0/x86_64-unknown-linux-gnu/src/arrow_ipc/gen/SparseTensor.rs.html#473" class="src">Source</a>

``` rust
pub struct SparseMatrixIndexCSX<'a> {
    pub _tab: Table<'a>,
}
```

Expand description

Compressed Sparse format, that is matrix-specific.

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseMatrixIndexCSX.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseMatrixIndexCSX.html#structfield._tab" class="anchor field">§</a>`_tab: `<a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/table/struct.Table.html" class="struct" title="struct flatbuffers::table::Table"><code>Table</code></a>`<'a>`

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseMatrixIndexCSX.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseMatrixIndexCSX.html#impl-SparseMatrixIndexCSX%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseMatrixIndexCSX.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseMatrixIndexCSX">SparseMatrixIndexCSX</a>\<'a\>

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseMatrixIndexCSX.html#associatedconstant.VT_COMPRESSEDAXIS" class="constant">VT_COMPRESSEDAXIS</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a> = 4u16

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseMatrixIndexCSX.html#associatedconstant.VT_INDPTRTYPE" class="constant">VT_INDPTRTYPE</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a> = 6u16

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseMatrixIndexCSX.html#associatedconstant.VT_INDPTRBUFFER" class="constant">VT_INDPTRBUFFER</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a> = 8u16

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseMatrixIndexCSX.html#associatedconstant.VT_INDICESTYPE" class="constant">VT_INDICESTYPE</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a> = 10u16

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseMatrixIndexCSX.html#associatedconstant.VT_INDICESBUFFER" class="constant">VT_INDICESBUFFER</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a> = 12u16

#### pub unsafe fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseMatrixIndexCSX.html#method.init_from_table" class="fn">init_from_table</a>(table: <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/table/struct.Table.html" class="struct" title="struct flatbuffers::table::Table">Table</a>\<'a\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseMatrixIndexCSX.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseMatrixIndexCSX">SparseMatrixIndexCSX</a>\<'a\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseMatrixIndexCSX.html#method.create" class="fn">create</a>\<'bldr, 'args, 'mut_bldr, A\>( \_fbb: &'mut_bldr mut <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/builder/struct.FlatBufferBuilder.html" class="struct" title="struct flatbuffers::builder::FlatBufferBuilder">FlatBufferBuilder</a>\<'bldr, A\>, args: &'args <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseMatrixIndexCSXArgs.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseMatrixIndexCSXArgs">SparseMatrixIndexCSXArgs</a>\<'args\>, ) -\> <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/primitives/struct.WIPOffset.html" class="struct" title="struct flatbuffers::primitives::WIPOffset">WIPOffset</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseMatrixIndexCSX.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseMatrixIndexCSX">SparseMatrixIndexCSX</a>\<'bldr\>\>

where 'bldr: 'args, 'args: 'mut_bldr, A: <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/builder/trait.Allocator.html" class="trait" title="trait flatbuffers::builder::Allocator">Allocator</a> + 'bldr,

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseMatrixIndexCSX.html#method.compressedAxis" class="fn">compressedAxis</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseMatrixCompressedAxis.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseMatrixCompressedAxis">SparseMatrixCompressedAxis</a>

Which axis, row or column, is compressed

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseMatrixIndexCSX.html#method.indptrType" class="fn">indptrType</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Int.html" class="struct" title="struct datafusion::common::arrow::ipc::Int">Int</a>\<'a\>

The type of values in indptrBuffer

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseMatrixIndexCSX.html#method.indptrBuffer" class="fn">indptrBuffer</a>(&self) -\> &'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html" class="struct" title="struct datafusion::common::arrow::ipc::Buffer">Buffer</a>

indptrBuffer stores the location and size of indptr array that represents the range of the rows. The i-th row spans from `indptr[i]` to `indptr[i+1]` in the data. The length of this array is 1 + (the number of rows), and the type of index value is long.

For example, let X be the following 6x4 matrix:

``` text
  X := [[0, 1, 2, 0],
        [0, 0, 3, 0],
        [0, 4, 0, 5],
        [0, 0, 0, 0],
        [6, 0, 7, 8],
        [0, 9, 0, 0]].
```

The array of non-zero values in X is:

``` text
  values(X) = [1, 2, 3, 4, 5, 6, 7, 8, 9].
```

And the indptr of X is:

``` text
  indptr(X) = [0, 2, 3, 5, 5, 8, 10].
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseMatrixIndexCSX.html#method.indicesType" class="fn">indicesType</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Int.html" class="struct" title="struct datafusion::common::arrow::ipc::Int">Int</a>\<'a\>

The type of values in indicesBuffer

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseMatrixIndexCSX.html#method.indicesBuffer" class="fn">indicesBuffer</a>(&self) -\> &'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html" class="struct" title="struct datafusion::common::arrow::ipc::Buffer">Buffer</a>

indicesBuffer stores the location and size of the array that contains the column indices of the corresponding non-zero values. The type of index value is long.

For example, the indices of the above X is:

``` text
  indices(X) = [1, 2, 2, 1, 3, 0, 2, 3, 1].
```

Note that the indices are sorted in lexicographical order for each row.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseMatrixIndexCSX.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseMatrixIndexCSX.html#impl-Clone-for-SparseMatrixIndexCSX%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseMatrixIndexCSX.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseMatrixIndexCSX">SparseMatrixIndexCSX</a>\<'a\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseMatrixIndexCSX.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseMatrixIndexCSX.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseMatrixIndexCSX">SparseMatrixIndexCSX</a>\<'a\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseMatrixIndexCSX.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseMatrixIndexCSX.html#impl-Debug-for-SparseMatrixIndexCSX%3C&#39;_%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseMatrixIndexCSX.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseMatrixIndexCSX">SparseMatrixIndexCSX</a>\<'\_\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseMatrixIndexCSX.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseMatrixIndexCSX.html#impl-Follow%3C&#39;a%3E-for-SparseMatrixIndexCSX%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/follow/trait.Follow.html" class="trait" title="trait flatbuffers::follow::Follow">Follow</a>\<'a\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseMatrixIndexCSX.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseMatrixIndexCSX">SparseMatrixIndexCSX</a>\<'a\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseMatrixIndexCSX.html#associatedtype.Inner" class="anchor">§</a>

#### type <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/follow/trait.Follow.html#associatedtype.Inner" class="associatedtype">Inner</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseMatrixIndexCSX.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseMatrixIndexCSX">SparseMatrixIndexCSX</a>\<'a\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseMatrixIndexCSX.html#method.follow" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/follow/trait.Follow.html#tymethod.follow" class="fn">follow</a>( buf: &'a \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\], loc: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> \<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseMatrixIndexCSX.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseMatrixIndexCSX">SparseMatrixIndexCSX</a>\<'a\> as <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/follow/trait.Follow.html" class="trait" title="trait flatbuffers::follow::Follow">Follow</a>\<'a\>\>::<a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/follow/trait.Follow.html#associatedtype.Inner" class="associatedtype" title="type flatbuffers::follow::Follow::Inner">Inner</a>

Safety [Read more](https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/follow/trait.Follow.html#tymethod.follow)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseMatrixIndexCSX.html#impl-PartialEq-for-SparseMatrixIndexCSX%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseMatrixIndexCSX.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseMatrixIndexCSX">SparseMatrixIndexCSX</a>\<'a\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseMatrixIndexCSX.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseMatrixIndexCSX.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseMatrixIndexCSX">SparseMatrixIndexCSX</a>\<'a\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseMatrixIndexCSX.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseMatrixIndexCSX.html#impl-Verifiable-for-SparseMatrixIndexCSX%3C&#39;_%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/verifier/trait.Verifiable.html" class="trait" title="trait flatbuffers::verifier::Verifiable">Verifiable</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseMatrixIndexCSX.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseMatrixIndexCSX">SparseMatrixIndexCSX</a>\<'\_\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseMatrixIndexCSX.html#method.run_verifier" class="anchor">§</a>

#### fn <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/verifier/trait.Verifiable.html#tymethod.run_verifier" class="fn">run_verifier</a>( v: &mut <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/verifier/struct.Verifier.html" class="struct" title="struct flatbuffers::verifier::Verifier">Verifier</a>\<'\_, '\_\>, pos: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/verifier/enum.InvalidFlatbuffer.html" class="enum" title="enum flatbuffers::verifier::InvalidFlatbuffer">InvalidFlatbuffer</a>\>

Runs the verifier for this type, assuming its at position `pos` in the verifier’s buffer. Should not need to be called directly.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseMatrixIndexCSX.html#impl-Copy-for-SparseMatrixIndexCSX%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseMatrixIndexCSX.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseMatrixIndexCSX">SparseMatrixIndexCSX</a>\<'a\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseMatrixIndexCSX.html#impl-StructuralPartialEq-for-SparseMatrixIndexCSX%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseMatrixIndexCSX.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseMatrixIndexCSX">SparseMatrixIndexCSX</a>\<'a\>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseMatrixIndexCSX.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/SparseTensor/struct.SparseMatrixIndexCSX.html#blanket-implementations" class="anchor">§</a>
