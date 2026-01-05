# Enum GroupsType Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/frame/group_by/position.rs.html#253" class="src">Source</a>

``` rust
pub enum GroupsType {
    Idx(GroupsIdx),
    Slice {
        groups: Vec<[u32; 2]>,
        rolling: bool,
    },
}
```

## Variants<a href="https://docs.rs/polars/latest/polars/frame/group_by/enum.GroupsType.html#variants" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/frame/group_by/enum.GroupsType.html#variant.Idx" class="anchor">§</a>

### Idx(<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html" class="struct" title="struct polars::prelude::GroupsIdx">GroupsIdx</a>)

<a href="https://docs.rs/polars/latest/polars/frame/group_by/enum.GroupsType.html#variant.Slice" class="anchor">§</a>

### Slice

Slice is always sorted in ascending order.

#### Fields

<a href="https://docs.rs/polars/latest/polars/frame/group_by/enum.GroupsType.html#variant.Slice.field.groups" class="anchor field">§</a>`groups: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<[`<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive"><code>u32</code></a>`; `<a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive"><code>2</code></a>`]>`

<a href="https://docs.rs/polars/latest/polars/frame/group_by/enum.GroupsType.html#variant.Slice.field.rolling" class="anchor field">§</a>`rolling: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

## Implementations<a href="https://docs.rs/polars/latest/polars/frame/group_by/enum.GroupsType.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/frame/group_by/enum.GroupsType.html#impl-GroupsType" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupsType.html" class="enum" title="enum polars::prelude::GroupsType">GroupsType</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/group_by/enum.GroupsType.html#method.into_idx" class="fn">into_idx</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html" class="struct" title="struct polars::prelude::GroupsIdx">GroupsIdx</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/group_by/enum.GroupsType.html#method.iter" class="fn">iter</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeIter.html" class="struct" title="struct polars::prelude::GroupsTypeIter">GroupsTypeIter</a>\<'\_\> <a href="https://docs.rs/polars/latest/polars/frame/group_by/enum.GroupsType.html#" class="tooltip" data-notable-ty="GroupsTypeIter&lt;&#39;_&gt;">ⓘ</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/group_by/enum.GroupsType.html#method.sort" class="fn">sort</a>(&mut self)

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/group_by/enum.GroupsType.html#method.take_group_firsts" class="fn">take_group_firsts</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>

#### pub unsafe fn <a href="https://docs.rs/polars/latest/polars/frame/group_by/enum.GroupsType.html#method.take_group_lasts" class="fn">take_group_lasts</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>

##### <a href="https://docs.rs/polars/latest/polars/frame/group_by/enum.GroupsType.html#safety" class="doc-anchor">§</a>Safety

This will not do any bounds checks. The caller must ensure all groups have members.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/group_by/enum.GroupsType.html#method.par_iter" class="fn">par_iter</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html" class="struct" title="struct polars::prelude::GroupsTypeParIter">GroupsTypeParIter</a>\<'\_\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/group_by/enum.GroupsType.html#method.unwrap_idx" class="fn">unwrap_idx</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html" class="struct" title="struct polars::prelude::GroupsIdx">GroupsIdx</a>

Get a reference to the `GroupsIdx`.

##### <a href="https://docs.rs/polars/latest/polars/frame/group_by/enum.GroupsType.html#panic" class="doc-anchor">§</a>Panic

panics if the groups are a slice.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/group_by/enum.GroupsType.html#method.unwrap_slice" class="fn">unwrap_slice</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>; <a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">2</a>\]\>

Get a reference to the `GroupsSlice`.

##### <a href="https://docs.rs/polars/latest/polars/frame/group_by/enum.GroupsType.html#panic-1" class="doc-anchor">§</a>Panic

panics if the groups are an idx.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/group_by/enum.GroupsType.html#method.get" class="fn">get</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupsIndicator.html" class="enum" title="enum polars::prelude::GroupsIndicator">GroupsIndicator</a>\<'\_\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/group_by/enum.GroupsType.html#method.idx_mut" class="fn">idx_mut</a>(&mut self) -\> &mut <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html" class="struct" title="struct polars::prelude::GroupsIdx">GroupsIdx</a>

Get a mutable reference to the `GroupsIdx`.

##### <a href="https://docs.rs/polars/latest/polars/frame/group_by/enum.GroupsType.html#panic-2" class="doc-anchor">§</a>Panic

panics if the groups are a slice.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/group_by/enum.GroupsType.html#method.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/group_by/enum.GroupsType.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/group_by/enum.GroupsType.html#method.group_count" class="fn">group_count</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt32Type.html" class="struct" title="struct polars::prelude::UInt32Type">UInt32Type</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/group_by/enum.GroupsType.html#method.as_list_chunked" class="fn">as_list_chunked</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListType.html" class="struct" title="struct polars::prelude::ListType">ListType</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/group_by/enum.GroupsType.html#method.into_sliceable" class="fn">into_sliceable</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupPositions.html" class="struct" title="struct polars::prelude::GroupPositions">GroupPositions</a>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/frame/group_by/enum.GroupsType.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/frame/group_by/enum.GroupsType.html#impl-AsRef%3CGroupsType%3E-for-GroupPositions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupsType.html" class="enum" title="enum polars::prelude::GroupsType">GroupsType</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupPositions.html" class="struct" title="struct polars::prelude::GroupPositions">GroupPositions</a>

<a href="https://docs.rs/polars/latest/polars/frame/group_by/enum.GroupsType.html#method.as_ref" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html#tymethod.as_ref" class="fn">as_ref</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupsType.html" class="enum" title="enum polars::prelude::GroupsType">GroupsType</a>

Converts this type into a shared reference of the (usually inferred) input type.

<a href="https://docs.rs/polars/latest/polars/frame/group_by/enum.GroupsType.html#impl-Clone-for-GroupsType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupsType.html" class="enum" title="enum polars::prelude::GroupsType">GroupsType</a>

<a href="https://docs.rs/polars/latest/polars/frame/group_by/enum.GroupsType.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupsType.html" class="enum" title="enum polars::prelude::GroupsType">GroupsType</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/frame/group_by/enum.GroupsType.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/frame/group_by/enum.GroupsType.html#impl-Debug-for-GroupsType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupsType.html" class="enum" title="enum polars::prelude::GroupsType">GroupsType</a>

<a href="https://docs.rs/polars/latest/polars/frame/group_by/enum.GroupsType.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/frame/group_by/enum.GroupsType.html#impl-Default-for-GroupsType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupsType.html" class="enum" title="enum polars::prelude::GroupsType">GroupsType</a>

<a href="https://docs.rs/polars/latest/polars/frame/group_by/enum.GroupsType.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupsType.html" class="enum" title="enum polars::prelude::GroupsType">GroupsType</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/polars/latest/polars/frame/group_by/enum.GroupsType.html#impl-From%3CGroupsIdx%3E-for-GroupsType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html" class="struct" title="struct polars::prelude::GroupsIdx">GroupsIdx</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupsType.html" class="enum" title="enum polars::prelude::GroupsType">GroupsType</a>

<a href="https://docs.rs/polars/latest/polars/frame/group_by/enum.GroupsType.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(groups: <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html" class="struct" title="struct polars::prelude::GroupsIdx">GroupsIdx</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupsType.html" class="enum" title="enum polars::prelude::GroupsType">GroupsType</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/frame/group_by/enum.GroupsType.html#impl-PartialEq-for-GroupsType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupsType.html" class="enum" title="enum polars::prelude::GroupsType">GroupsType</a>

<a href="https://docs.rs/polars/latest/polars/frame/group_by/enum.GroupsType.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupsType.html" class="enum" title="enum polars::prelude::GroupsType">GroupsType</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/frame/group_by/enum.GroupsType.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/polars/latest/polars/frame/group_by/enum.GroupsType.html#impl-Eq-for-GroupsType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupsType.html" class="enum" title="enum polars::prelude::GroupsType">GroupsType</a>

<a href="https://docs.rs/polars/latest/polars/frame/group_by/enum.GroupsType.html#impl-StructuralPartialEq-for-GroupsType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupsType.html" class="enum" title="enum polars::prelude::GroupsType">GroupsType</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/frame/group_by/enum.GroupsType.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/frame/group_by/enum.GroupsType.html#blanket-implementations" class="anchor">§</a>
