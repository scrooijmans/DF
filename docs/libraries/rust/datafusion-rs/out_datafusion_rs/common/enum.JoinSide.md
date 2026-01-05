# Enum JoinSide Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/join_type.rs.html#177" class="src">Source</a>

``` rust
pub enum JoinSide {
    Left,
    Right,
    None,
}
```

Expand description

Join side. Stores the referred table side during calculations

## Variants<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.JoinSide.html#variants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.JoinSide.html#variant.Left" class="anchor">§</a>

### Left

Left side of the join

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.JoinSide.html#variant.Right" class="anchor">§</a>

### Right

Right side of the join

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.JoinSide.html#variant.None" class="anchor">§</a>

### None

Neither side of the join, used for Mark joins where the mark column does not belong to either side of the join

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.JoinSide.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.JoinSide.html#impl-JoinSide" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.JoinSide.html" class="enum" title="enum datafusion::common::JoinSide">JoinSide</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.JoinSide.html#method.negate" class="fn">negate</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.JoinSide.html" class="enum" title="enum datafusion::common::JoinSide">JoinSide</a>

Inverse the join side

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.JoinSide.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.JoinSide.html#impl-Clone-for-JoinSide" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.JoinSide.html" class="enum" title="enum datafusion::common::JoinSide">JoinSide</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.JoinSide.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.JoinSide.html" class="enum" title="enum datafusion::common::JoinSide">JoinSide</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.JoinSide.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.JoinSide.html#impl-Debug-for-JoinSide" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.JoinSide.html" class="enum" title="enum datafusion::common::JoinSide">JoinSide</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.JoinSide.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.JoinSide.html#impl-Display-for-JoinSide" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.JoinSide.html" class="enum" title="enum datafusion::common::JoinSide">JoinSide</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.JoinSide.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.JoinSide.html#impl-PartialEq-for-JoinSide" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.JoinSide.html" class="enum" title="enum datafusion::common::JoinSide">JoinSide</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.JoinSide.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.JoinSide.html" class="enum" title="enum datafusion::common::JoinSide">JoinSide</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.JoinSide.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.JoinSide.html#impl-Copy-for-JoinSide" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.JoinSide.html" class="enum" title="enum datafusion::common::JoinSide">JoinSide</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.JoinSide.html#impl-StructuralPartialEq-for-JoinSide" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.JoinSide.html" class="enum" title="enum datafusion::common::JoinSide">JoinSide</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.JoinSide.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.JoinSide.html#blanket-implementations" class="anchor">§</a>
