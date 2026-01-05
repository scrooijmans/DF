# Struct SparseTensorIndexCOO Copy item path

<a href="https://docs.rs/arrow-ipc/56.0.0/x86_64-unknown-linux-gnu/src/arrow_ipc/gen/SparseTensor.rs.html#261" class="src">Source</a>

``` rust
pub struct SparseTensorIndexCOO<'a> {
    pub _tab: Table<'a>,
}
```

Expand description

------------------------------------------------------------------------

EXPERIMENTAL: Data structures for sparse tensors Coordinate (COO) format of sparse tensor index.

COO’s index list are represented as a NxM matrix, where N is the number of non-zero values, and M is the number of dimensions of a sparse tensor.

indicesBuffer stores the location and size of the data of this indices matrix. The value type and the stride of the indices matrix is specified in indicesType and indicesStrides fields.

For example, let X be a 2x3x4x5 tensor, and it has the following 6 non-zero values:

``` text
  X[0, 1, 2, 0] := 1
  X[1, 1, 2, 3] := 2
  X[0, 2, 1, 0] := 3
  X[0, 1, 3, 0] := 4
  X[0, 1, 2, 1] := 5
  X[1, 2, 0, 4] := 6
```

In COO format, the index matrix of X is the following 4x6 matrix:

``` text
  [[0, 0, 0, 0, 1, 1],
   [1, 1, 1, 2, 1, 2],
   [2, 2, 3, 1, 2, 0],
   [0, 1, 0, 0, 3, 4]]
```

When isCanonical is true, the indices is sorted in lexicographical order (row-major order), and it does not have duplicated entries. Otherwise, the indices may not be sorted, or may have duplicated entries.

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCOO.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCOO.html#structfield._tab" class="anchor field">§</a>`_tab: `<a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/table/struct.Table.html" class="struct" title="struct flatbuffers::table::Table"><code>Table</code></a>`<'a>`

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCOO.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCOO.html#impl-SparseTensorIndexCOO%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCOO.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseTensorIndexCOO">SparseTensorIndexCOO</a>\<'a\>

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCOO.html#associatedconstant.VT_INDICESTYPE" class="constant">VT_INDICESTYPE</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a> = 4u16

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCOO.html#associatedconstant.VT_INDICESSTRIDES" class="constant">VT_INDICESSTRIDES</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a> = 6u16

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCOO.html#associatedconstant.VT_INDICESBUFFER" class="constant">VT_INDICESBUFFER</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a> = 8u16

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCOO.html#associatedconstant.VT_ISCANONICAL" class="constant">VT_ISCANONICAL</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a> = 10u16

#### pub unsafe fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCOO.html#method.init_from_table" class="fn">init_from_table</a>(table: <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/table/struct.Table.html" class="struct" title="struct flatbuffers::table::Table">Table</a>\<'a\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCOO.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseTensorIndexCOO">SparseTensorIndexCOO</a>\<'a\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCOO.html#method.create" class="fn">create</a>\<'bldr, 'args, 'mut_bldr, A\>( \_fbb: &'mut_bldr mut <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/builder/struct.FlatBufferBuilder.html" class="struct" title="struct flatbuffers::builder::FlatBufferBuilder">FlatBufferBuilder</a>\<'bldr, A\>, args: &'args <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCOOArgs.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseTensorIndexCOOArgs">SparseTensorIndexCOOArgs</a>\<'args\>, ) -\> <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/primitives/struct.WIPOffset.html" class="struct" title="struct flatbuffers::primitives::WIPOffset">WIPOffset</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCOO.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseTensorIndexCOO">SparseTensorIndexCOO</a>\<'bldr\>\>

where 'bldr: 'args, 'args: 'mut_bldr, A: <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/builder/trait.Allocator.html" class="trait" title="trait flatbuffers::builder::Allocator">Allocator</a> + 'bldr,

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCOO.html#method.indicesType" class="fn">indicesType</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Int.html" class="struct" title="struct datafusion::common::arrow::ipc::Int">Int</a>\<'a\>

The type of values in indicesBuffer

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCOO.html#method.indicesStrides" class="fn">indicesStrides</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/vector/struct.Vector.html" class="struct" title="struct flatbuffers::vector::Vector">Vector</a>\<'a, <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>\>

Non-negative byte offsets to advance one value cell along each dimension If omitted, default to row-major order (C-like).

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCOO.html#method.indicesBuffer" class="fn">indicesBuffer</a>(&self) -\> &'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html" class="struct" title="struct datafusion::common::arrow::ipc::Buffer">Buffer</a>

The location and size of the indices matrix’s data

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCOO.html#method.isCanonical" class="fn">isCanonical</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

This flag is true if and only if the indices matrix is sorted in row-major order, and does not have duplicated entries. This sort order is the same as of Tensorflow’s SparseTensor, but it is inverse order of SciPy’s canonical coo_matrix (SciPy employs column-major order for its coo_matrix).

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCOO.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCOO.html#impl-Clone-for-SparseTensorIndexCOO%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCOO.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseTensorIndexCOO">SparseTensorIndexCOO</a>\<'a\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCOO.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCOO.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseTensorIndexCOO">SparseTensorIndexCOO</a>\<'a\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCOO.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCOO.html#impl-Debug-for-SparseTensorIndexCOO%3C&#39;_%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCOO.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseTensorIndexCOO">SparseTensorIndexCOO</a>\<'\_\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCOO.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCOO.html#impl-Follow%3C&#39;a%3E-for-SparseTensorIndexCOO%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/follow/trait.Follow.html" class="trait" title="trait flatbuffers::follow::Follow">Follow</a>\<'a\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCOO.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseTensorIndexCOO">SparseTensorIndexCOO</a>\<'a\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCOO.html#associatedtype.Inner" class="anchor">§</a>

#### type <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/follow/trait.Follow.html#associatedtype.Inner" class="associatedtype">Inner</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCOO.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseTensorIndexCOO">SparseTensorIndexCOO</a>\<'a\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCOO.html#method.follow" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/follow/trait.Follow.html#tymethod.follow" class="fn">follow</a>( buf: &'a \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\], loc: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> \<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCOO.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseTensorIndexCOO">SparseTensorIndexCOO</a>\<'a\> as <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/follow/trait.Follow.html" class="trait" title="trait flatbuffers::follow::Follow">Follow</a>\<'a\>\>::<a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/follow/trait.Follow.html#associatedtype.Inner" class="associatedtype" title="type flatbuffers::follow::Follow::Inner">Inner</a>

Safety [Read more](https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/follow/trait.Follow.html#tymethod.follow)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCOO.html#impl-PartialEq-for-SparseTensorIndexCOO%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCOO.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseTensorIndexCOO">SparseTensorIndexCOO</a>\<'a\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCOO.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCOO.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseTensorIndexCOO">SparseTensorIndexCOO</a>\<'a\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCOO.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCOO.html#impl-Verifiable-for-SparseTensorIndexCOO%3C&#39;_%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/verifier/trait.Verifiable.html" class="trait" title="trait flatbuffers::verifier::Verifiable">Verifiable</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCOO.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseTensorIndexCOO">SparseTensorIndexCOO</a>\<'\_\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCOO.html#method.run_verifier" class="anchor">§</a>

#### fn <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/verifier/trait.Verifiable.html#tymethod.run_verifier" class="fn">run_verifier</a>( v: &mut <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/verifier/struct.Verifier.html" class="struct" title="struct flatbuffers::verifier::Verifier">Verifier</a>\<'\_, '\_\>, pos: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/verifier/enum.InvalidFlatbuffer.html" class="enum" title="enum flatbuffers::verifier::InvalidFlatbuffer">InvalidFlatbuffer</a>\>

Runs the verifier for this type, assuming its at position `pos` in the verifier’s buffer. Should not need to be called directly.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCOO.html#impl-Copy-for-SparseTensorIndexCOO%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCOO.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseTensorIndexCOO">SparseTensorIndexCOO</a>\<'a\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCOO.html#impl-StructuralPartialEq-for-SparseTensorIndexCOO%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCOO.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseTensorIndexCOO">SparseTensorIndexCOO</a>\<'a\>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCOO.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCOO.html#blanket-implementations" class="anchor">§</a>
