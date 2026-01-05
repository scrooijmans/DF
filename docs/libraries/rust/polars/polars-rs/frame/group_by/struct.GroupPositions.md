# Struct GroupPositions Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/frame/group_by/position.rs.html#599" class="src">Source</a>

``` rust
pub struct GroupPositions { /* private fields */ }
```

## Implementations<a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupPositions.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupPositions.html#impl-GroupPositions" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupPositions.html" class="struct" title="struct polars::prelude::GroupPositions">GroupPositions</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupPositions.html#method.slice" class="fn">slice</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupPositions.html" class="struct" title="struct polars::prelude::GroupPositions">GroupPositions</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupPositions.html#method.sort" class="fn">sort</a>(&mut self)

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupPositions.html#method.unroll" class="fn">unroll</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupPositions.html" class="struct" title="struct polars::prelude::GroupPositions">GroupPositions</a>

## Methods from <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html" class="trait" title="trait core::ops::deref::Deref">Deref</a>\<Target = <a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupsType.html" class="enum" title="enum polars::prelude::GroupsType">GroupsType</a>\><a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupPositions.html#deref-methods-GroupsType" class="anchor">§</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupPositions.html#method.iter" class="fn">iter</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeIter.html" class="struct" title="struct polars::prelude::GroupsTypeIter">GroupsTypeIter</a>\<'\_\> <a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupPositions.html#" class="tooltip" data-notable-ty="GroupsTypeIter&lt;&#39;_&gt;">ⓘ</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupPositions.html#method.par_iter" class="fn">par_iter</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html" class="struct" title="struct polars::prelude::GroupsTypeParIter">GroupsTypeParIter</a>\<'\_\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupPositions.html#method.unwrap_idx" class="fn">unwrap_idx</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html" class="struct" title="struct polars::prelude::GroupsIdx">GroupsIdx</a>

Get a reference to the `GroupsIdx`.

##### <a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupPositions.html#panic" class="doc-anchor">§</a>Panic

panics if the groups are a slice.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupPositions.html#method.unwrap_slice" class="fn">unwrap_slice</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>; <a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">2</a>\]\>

Get a reference to the `GroupsSlice`.

##### <a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupPositions.html#panic-1" class="doc-anchor">§</a>Panic

panics if the groups are an idx.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupPositions.html#method.get" class="fn">get</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupsIndicator.html" class="enum" title="enum polars::prelude::GroupsIndicator">GroupsIndicator</a>\<'\_\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupPositions.html#method.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupPositions.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupPositions.html#method.group_count" class="fn">group_count</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt32Type.html" class="struct" title="struct polars::prelude::UInt32Type">UInt32Type</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupPositions.html#method.as_list_chunked" class="fn">as_list_chunked</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListType.html" class="struct" title="struct polars::prelude::ListType">ListType</a>\>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupPositions.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupPositions.html#impl-AsRef%3CGroupsType%3E-for-GroupPositions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupsType.html" class="enum" title="enum polars::prelude::GroupsType">GroupsType</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupPositions.html" class="struct" title="struct polars::prelude::GroupPositions">GroupPositions</a>

<a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupPositions.html#method.as_ref" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html#tymethod.as_ref" class="fn">as_ref</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupsType.html" class="enum" title="enum polars::prelude::GroupsType">GroupsType</a>

Converts this type into a shared reference of the (usually inferred) input type.

<a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupPositions.html#impl-Clone-for-GroupPositions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupPositions.html" class="struct" title="struct polars::prelude::GroupPositions">GroupPositions</a>

<a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupPositions.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupPositions.html" class="struct" title="struct polars::prelude::GroupPositions">GroupPositions</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupPositions.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupPositions.html#impl-Debug-for-GroupPositions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupPositions.html" class="struct" title="struct polars::prelude::GroupPositions">GroupPositions</a>

<a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupPositions.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupPositions.html#impl-Default-for-GroupPositions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupPositions.html" class="struct" title="struct polars::prelude::GroupPositions">GroupPositions</a>

<a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupPositions.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupPositions.html" class="struct" title="struct polars::prelude::GroupPositions">GroupPositions</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupPositions.html#impl-Deref-for-GroupPositions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html" class="trait" title="trait core::ops::deref::Deref">Deref</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupPositions.html" class="struct" title="struct polars::prelude::GroupPositions">GroupPositions</a>

<a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupPositions.html#associatedtype.Target" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target" class="associatedtype">Target</a> = <a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupsType.html" class="enum" title="enum polars::prelude::GroupsType">GroupsType</a>

The resulting type after dereferencing.

<a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupPositions.html#method.deref" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#tymethod.deref" class="fn">deref</a>(&self) -\> &\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupPositions.html" class="struct" title="struct polars::prelude::GroupPositions">GroupPositions</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html" class="trait" title="trait core::ops::deref::Deref">Deref</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target" class="associatedtype" title="type core::ops::deref::Deref::Target">Target</a>

Dereferences the value.

<a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupPositions.html#impl-PartialEq-for-GroupPositions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupPositions.html" class="struct" title="struct polars::prelude::GroupPositions">GroupPositions</a>

<a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupPositions.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupPositions.html" class="struct" title="struct polars::prelude::GroupPositions">GroupPositions</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupPositions.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupPositions.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupPositions.html#blanket-implementations" class="anchor">§</a>
