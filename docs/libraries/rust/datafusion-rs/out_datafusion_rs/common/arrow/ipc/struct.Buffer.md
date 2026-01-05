# Struct Buffer Copy item path

<a href="https://docs.rs/arrow-ipc/56.0.0/x86_64-unknown-linux-gnu/src/arrow_ipc/gen/Schema.rs.html#1118" class="src">Source</a>

``` rust
#[repr(transparent)]pub struct Buffer(pub [u8; 16]);
```

Expand description

------------------------------------------------------------------------

A Buffer represents a single contiguous memory segment

## Tuple Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html#structfield.0" class="anchor field">§</a>`0: [`<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive"><code>u8</code></a>`; `<a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive"><code>16</code></a>`]`

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html#impl-Buffer" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html" class="struct" title="struct datafusion::common::arrow::ipc::Buffer">Buffer</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html#method.new" class="fn">new</a>(offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html" class="struct" title="struct datafusion::common::arrow::ipc::Buffer">Buffer</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html#method.offset" class="fn">offset</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

The relative offset into the shared memory page where the bytes for this buffer starts

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html#method.set_offset" class="fn">set_offset</a>(&mut self, x: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>)

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html#method.length" class="fn">length</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

The absolute length (in bytes) of the memory buffer. The memory is found from offset (inclusive) to offset + length (non-inclusive). When building messages using the encapsulated IPC message, padding bytes may be written after a buffer, but such padding bytes do not need to be accounted for in the size here.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html#method.set_length" class="fn">set_length</a>(&mut self, x: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>)

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html#impl-Clone-for-Buffer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html" class="struct" title="struct datafusion::common::arrow::ipc::Buffer">Buffer</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html" class="struct" title="struct datafusion::common::arrow::ipc::Buffer">Buffer</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html#impl-Debug-for-Buffer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html" class="struct" title="struct datafusion::common::arrow::ipc::Buffer">Buffer</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html#impl-Default-for-Buffer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html" class="struct" title="struct datafusion::common::arrow::ipc::Buffer">Buffer</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html" class="struct" title="struct datafusion::common::arrow::ipc::Buffer">Buffer</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html#impl-Follow%3C&#39;a%3E-for-%26Buffer" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/follow/trait.Follow.html" class="trait" title="trait flatbuffers::follow::Follow">Follow</a>\<'a\> for &'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html" class="struct" title="struct datafusion::common::arrow::ipc::Buffer">Buffer</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html#associatedtype.Inner-1" class="anchor">§</a>

#### type <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/follow/trait.Follow.html#associatedtype.Inner" class="associatedtype">Inner</a> = &'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html" class="struct" title="struct datafusion::common::arrow::ipc::Buffer">Buffer</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html#method.follow-1" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/follow/trait.Follow.html#tymethod.follow" class="fn">follow</a>(buf: &'a \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\], loc: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> \<&'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html" class="struct" title="struct datafusion::common::arrow::ipc::Buffer">Buffer</a> as <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/follow/trait.Follow.html" class="trait" title="trait flatbuffers::follow::Follow">Follow</a>\<'a\>\>::<a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/follow/trait.Follow.html#associatedtype.Inner" class="associatedtype" title="type flatbuffers::follow::Follow::Inner">Inner</a>

Safety [Read more](https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/follow/trait.Follow.html#tymethod.follow)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html#impl-Follow%3C&#39;a%3E-for-Buffer" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/follow/trait.Follow.html" class="trait" title="trait flatbuffers::follow::Follow">Follow</a>\<'a\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html" class="struct" title="struct datafusion::common::arrow::ipc::Buffer">Buffer</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html#associatedtype.Inner" class="anchor">§</a>

#### type <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/follow/trait.Follow.html#associatedtype.Inner" class="associatedtype">Inner</a> = &'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html" class="struct" title="struct datafusion::common::arrow::ipc::Buffer">Buffer</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html#method.follow" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/follow/trait.Follow.html#tymethod.follow" class="fn">follow</a>(buf: &'a \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\], loc: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> \<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html" class="struct" title="struct datafusion::common::arrow::ipc::Buffer">Buffer</a> as <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/follow/trait.Follow.html" class="trait" title="trait flatbuffers::follow::Follow">Follow</a>\<'a\>\>::<a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/follow/trait.Follow.html#associatedtype.Inner" class="associatedtype" title="type flatbuffers::follow::Follow::Inner">Inner</a>

Safety [Read more](https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/follow/trait.Follow.html#tymethod.follow)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html#impl-PartialEq-for-Buffer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html" class="struct" title="struct datafusion::common::arrow::ipc::Buffer">Buffer</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html" class="struct" title="struct datafusion::common::arrow::ipc::Buffer">Buffer</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html#impl-Push-for-Buffer" class="anchor">§</a>

### impl\<'b\> <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/push/trait.Push.html" class="trait" title="trait flatbuffers::push::Push">Push</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html" class="struct" title="struct datafusion::common::arrow::ipc::Buffer">Buffer</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html#associatedtype.Output" class="anchor">§</a>

#### type <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/push/trait.Push.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html" class="struct" title="struct datafusion::common::arrow::ipc::Buffer">Buffer</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html#method.push" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/push/trait.Push.html#tymethod.push" class="fn">push</a>(&self, dst: &mut \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\], \_written_len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Safety [Read more](https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/push/trait.Push.html#tymethod.push)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html#method.alignment" class="anchor">§</a>

#### fn <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/push/trait.Push.html#method.alignment" class="fn">alignment</a>() -\> <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/push/struct.PushAlignment.html" class="struct" title="struct flatbuffers::push::PushAlignment">PushAlignment</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html#method.size" class="anchor">§</a>

#### fn <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/push/trait.Push.html#method.size" class="fn">size</a>() -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html#impl-Verifiable-for-Buffer" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/verifier/trait.Verifiable.html" class="trait" title="trait flatbuffers::verifier::Verifiable">Verifiable</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html" class="struct" title="struct datafusion::common::arrow::ipc::Buffer">Buffer</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html#method.run_verifier" class="anchor">§</a>

#### fn <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/verifier/trait.Verifiable.html#tymethod.run_verifier" class="fn">run_verifier</a>( v: &mut <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/verifier/struct.Verifier.html" class="struct" title="struct flatbuffers::verifier::Verifier">Verifier</a>\<'\_, '\_\>, pos: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/verifier/enum.InvalidFlatbuffer.html" class="enum" title="enum flatbuffers::verifier::InvalidFlatbuffer">InvalidFlatbuffer</a>\>

Runs the verifier for this type, assuming its at position `pos` in the verifier’s buffer. Should not need to be called directly.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html#impl-Copy-for-Buffer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html" class="struct" title="struct datafusion::common::arrow::ipc::Buffer">Buffer</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html#impl-SimpleToVerifyInSlice-for-Buffer" class="anchor">§</a>

### impl <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/verifier/trait.SimpleToVerifyInSlice.html" class="trait" title="trait flatbuffers::verifier::SimpleToVerifyInSlice">SimpleToVerifyInSlice</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html" class="struct" title="struct datafusion::common::arrow::ipc::Buffer">Buffer</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html#impl-StructuralPartialEq-for-Buffer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html" class="struct" title="struct datafusion::common::arrow::ipc::Buffer">Buffer</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html#blanket-implementations" class="anchor">§</a>
