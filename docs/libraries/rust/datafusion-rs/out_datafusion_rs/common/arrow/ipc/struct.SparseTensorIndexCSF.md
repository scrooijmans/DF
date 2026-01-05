# Struct SparseTensorIndexCSF Copy item path

<a href="https://docs.rs/arrow-ipc/56.0.0/x86_64-unknown-linux-gnu/src/arrow_ipc/gen/SparseTensor.rs.html#744" class="src">Source</a>

``` rust
pub struct SparseTensorIndexCSF<'a> {
    pub _tab: Table<'a>,
}
```

Expand description

Compressed Sparse Fiber (CSF) sparse tensor index.

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSF.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSF.html#structfield._tab" class="anchor field">§</a>`_tab: `<a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/table/struct.Table.html" class="struct" title="struct flatbuffers::table::Table"><code>Table</code></a>`<'a>`

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSF.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSF.html#impl-SparseTensorIndexCSF%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSF.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseTensorIndexCSF">SparseTensorIndexCSF</a>\<'a\>

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSF.html#associatedconstant.VT_INDPTRTYPE" class="constant">VT_INDPTRTYPE</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a> = 4u16

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSF.html#associatedconstant.VT_INDPTRBUFFERS" class="constant">VT_INDPTRBUFFERS</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a> = 6u16

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSF.html#associatedconstant.VT_INDICESTYPE" class="constant">VT_INDICESTYPE</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a> = 8u16

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSF.html#associatedconstant.VT_INDICESBUFFERS" class="constant">VT_INDICESBUFFERS</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a> = 10u16

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSF.html#associatedconstant.VT_AXISORDER" class="constant">VT_AXISORDER</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a> = 12u16

#### pub unsafe fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSF.html#method.init_from_table" class="fn">init_from_table</a>(table: <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/table/struct.Table.html" class="struct" title="struct flatbuffers::table::Table">Table</a>\<'a\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSF.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseTensorIndexCSF">SparseTensorIndexCSF</a>\<'a\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSF.html#method.create" class="fn">create</a>\<'bldr, 'args, 'mut_bldr, A\>( \_fbb: &'mut_bldr mut <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/builder/struct.FlatBufferBuilder.html" class="struct" title="struct flatbuffers::builder::FlatBufferBuilder">FlatBufferBuilder</a>\<'bldr, A\>, args: &'args <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSFArgs.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseTensorIndexCSFArgs">SparseTensorIndexCSFArgs</a>\<'args\>, ) -\> <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/primitives/struct.WIPOffset.html" class="struct" title="struct flatbuffers::primitives::WIPOffset">WIPOffset</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSF.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseTensorIndexCSF">SparseTensorIndexCSF</a>\<'bldr\>\>

where 'bldr: 'args, 'args: 'mut_bldr, A: <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/builder/trait.Allocator.html" class="trait" title="trait flatbuffers::builder::Allocator">Allocator</a> + 'bldr,

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSF.html#method.indptrType" class="fn">indptrType</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Int.html" class="struct" title="struct datafusion::common::arrow::ipc::Int">Int</a>\<'a\>

CSF is a generalization of compressed sparse row (CSR) index. See [smith2017knl](http://shaden.io/pub-files/smith2017knl.pdf)

CSF index recursively compresses each dimension of a tensor into a set of prefix trees. Each path from a root to leaf forms one tensor non-zero index. CSF is implemented with two arrays of buffers and one arrays of integers.

For example, let X be a 2x3x4x5 tensor and let it have the following 8 non-zero values:

``` text
  X[0, 0, 0, 1] := 1
  X[0, 0, 0, 2] := 2
  X[0, 1, 0, 0] := 3
  X[0, 1, 0, 2] := 4
  X[0, 1, 1, 0] := 5
  X[1, 1, 1, 0] := 6
  X[1, 1, 1, 1] := 7
  X[1, 1, 1, 2] := 8
```

As a prefix tree this would be represented as:

``` text
        0          1
       / \         |
      0   1        1
     /   / \       |
    0   0   1      1
   /|  /|   |    /| |
  1 2 0 2   0   0 1 2
```

The type of values in indptrBuffers

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSF.html#method.indptrBuffers" class="fn">indptrBuffers</a>(&self) -\> <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/vector/struct.Vector.html" class="struct" title="struct flatbuffers::vector::Vector">Vector</a>\<'a, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html" class="struct" title="struct datafusion::common::arrow::ipc::Buffer">Buffer</a>\>

indptrBuffers stores the sparsity structure. Each two consecutive dimensions in a tensor correspond to a buffer in indptrBuffers. A pair of consecutive values at `indptrBuffers[dim][i]` and `indptrBuffers[dim][i + 1]` signify a range of nodes in `indicesBuffers[dim + 1]` who are children of `indicesBuffers[dim][i]` node.

For example, the indptrBuffers for the above X is:

``` text
  indptrBuffer(X) = [
                      [0, 2, 3],
                      [0, 1, 3, 4],
                      [0, 2, 4, 5, 8]
                    ].
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSF.html#method.indicesType" class="fn">indicesType</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Int.html" class="struct" title="struct datafusion::common::arrow::ipc::Int">Int</a>\<'a\>

The type of values in indicesBuffers

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSF.html#method.indicesBuffers" class="fn">indicesBuffers</a>(&self) -\> <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/vector/struct.Vector.html" class="struct" title="struct flatbuffers::vector::Vector">Vector</a>\<'a, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html" class="struct" title="struct datafusion::common::arrow::ipc::Buffer">Buffer</a>\>

indicesBuffers stores values of nodes. Each tensor dimension corresponds to a buffer in indicesBuffers. For example, the indicesBuffers for the above X is:

``` text
  indicesBuffer(X) = [
                       [0, 1],
                       [0, 1, 1],
                       [0, 0, 1, 1],
                       [1, 2, 0, 2, 0, 0, 1, 2]
                     ].
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSF.html#method.axisOrder" class="fn">axisOrder</a>(&self) -\> <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/vector/struct.Vector.html" class="struct" title="struct flatbuffers::vector::Vector">Vector</a>\<'a, <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>

axisOrder stores the sequence in which dimensions were traversed to produce the prefix tree. For example, the axisOrder for the above X is:

``` text
  axisOrder(X) = [0, 1, 2, 3].
```

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSF.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSF.html#impl-Clone-for-SparseTensorIndexCSF%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSF.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseTensorIndexCSF">SparseTensorIndexCSF</a>\<'a\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSF.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSF.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseTensorIndexCSF">SparseTensorIndexCSF</a>\<'a\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSF.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSF.html#impl-Debug-for-SparseTensorIndexCSF%3C&#39;_%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSF.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseTensorIndexCSF">SparseTensorIndexCSF</a>\<'\_\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSF.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSF.html#impl-Follow%3C&#39;a%3E-for-SparseTensorIndexCSF%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/follow/trait.Follow.html" class="trait" title="trait flatbuffers::follow::Follow">Follow</a>\<'a\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSF.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseTensorIndexCSF">SparseTensorIndexCSF</a>\<'a\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSF.html#associatedtype.Inner" class="anchor">§</a>

#### type <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/follow/trait.Follow.html#associatedtype.Inner" class="associatedtype">Inner</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSF.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseTensorIndexCSF">SparseTensorIndexCSF</a>\<'a\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSF.html#method.follow" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/follow/trait.Follow.html#tymethod.follow" class="fn">follow</a>( buf: &'a \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\], loc: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> \<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSF.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseTensorIndexCSF">SparseTensorIndexCSF</a>\<'a\> as <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/follow/trait.Follow.html" class="trait" title="trait flatbuffers::follow::Follow">Follow</a>\<'a\>\>::<a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/follow/trait.Follow.html#associatedtype.Inner" class="associatedtype" title="type flatbuffers::follow::Follow::Inner">Inner</a>

Safety [Read more](https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/follow/trait.Follow.html#tymethod.follow)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSF.html#impl-PartialEq-for-SparseTensorIndexCSF%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSF.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseTensorIndexCSF">SparseTensorIndexCSF</a>\<'a\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSF.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSF.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseTensorIndexCSF">SparseTensorIndexCSF</a>\<'a\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSF.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSF.html#impl-Verifiable-for-SparseTensorIndexCSF%3C&#39;_%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/verifier/trait.Verifiable.html" class="trait" title="trait flatbuffers::verifier::Verifiable">Verifiable</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSF.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseTensorIndexCSF">SparseTensorIndexCSF</a>\<'\_\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSF.html#method.run_verifier" class="anchor">§</a>

#### fn <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/verifier/trait.Verifiable.html#tymethod.run_verifier" class="fn">run_verifier</a>( v: &mut <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/verifier/struct.Verifier.html" class="struct" title="struct flatbuffers::verifier::Verifier">Verifier</a>\<'\_, '\_\>, pos: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/verifier/enum.InvalidFlatbuffer.html" class="enum" title="enum flatbuffers::verifier::InvalidFlatbuffer">InvalidFlatbuffer</a>\>

Runs the verifier for this type, assuming its at position `pos` in the verifier’s buffer. Should not need to be called directly.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSF.html#impl-Copy-for-SparseTensorIndexCSF%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSF.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseTensorIndexCSF">SparseTensorIndexCSF</a>\<'a\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSF.html#impl-StructuralPartialEq-for-SparseTensorIndexCSF%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSF.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseTensorIndexCSF">SparseTensorIndexCSF</a>\<'a\>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSF.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSF.html#blanket-implementations" class="anchor">§</a>
