# Enum RankDir Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/dot/mod.rs.html#155-164" class="src">Source</a>

``` rust
pub enum RankDir {
    TB,
    BT,
    LR,
    RL,
}
```

Expand description

Direction of graph layout.

<https://graphviz.org/docs/attrs/rankdir/>

## Variants<a href="https://docs.rs/petgraph/latest/petgraph/dot/enum.RankDir.html#variants" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/dot/enum.RankDir.html#variant.TB" class="anchor">§</a>

### TB

Top to bottom

<a href="https://docs.rs/petgraph/latest/petgraph/dot/enum.RankDir.html#variant.BT" class="anchor">§</a>

### BT

Bottom to top

<a href="https://docs.rs/petgraph/latest/petgraph/dot/enum.RankDir.html#variant.LR" class="anchor">§</a>

### LR

Left to right

<a href="https://docs.rs/petgraph/latest/petgraph/dot/enum.RankDir.html#variant.RL" class="anchor">§</a>

### RL

Right to left

## Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/dot/enum.RankDir.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/dot/enum.RankDir.html#impl-Clone-for-RankDir" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/petgraph/latest/petgraph/dot/enum.RankDir.html" class="enum" title="enum petgraph::dot::RankDir">RankDir</a>

<a href="https://docs.rs/petgraph/latest/petgraph/dot/enum.RankDir.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/petgraph/latest/petgraph/dot/enum.RankDir.html" class="enum" title="enum petgraph::dot::RankDir">RankDir</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/dot/enum.RankDir.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/petgraph/latest/petgraph/dot/enum.RankDir.html#impl-Debug-for-RankDir" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/petgraph/latest/petgraph/dot/enum.RankDir.html" class="enum" title="enum petgraph::dot::RankDir">RankDir</a>

<a href="https://docs.rs/petgraph/latest/petgraph/dot/enum.RankDir.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/petgraph/latest/petgraph/dot/enum.RankDir.html#impl-PartialEq-for-RankDir" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/petgraph/latest/petgraph/dot/enum.RankDir.html" class="enum" title="enum petgraph::dot::RankDir">RankDir</a>

<a href="https://docs.rs/petgraph/latest/petgraph/dot/enum.RankDir.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/petgraph/latest/petgraph/dot/enum.RankDir.html" class="enum" title="enum petgraph::dot::RankDir">RankDir</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/dot/enum.RankDir.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/petgraph/latest/petgraph/dot/enum.RankDir.html#impl-Copy-for-RankDir" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/petgraph/latest/petgraph/dot/enum.RankDir.html" class="enum" title="enum petgraph::dot::RankDir">RankDir</a>

<a href="https://docs.rs/petgraph/latest/petgraph/dot/enum.RankDir.html#impl-Eq-for-RankDir" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/petgraph/latest/petgraph/dot/enum.RankDir.html" class="enum" title="enum petgraph::dot::RankDir">RankDir</a>

<a href="https://docs.rs/petgraph/latest/petgraph/dot/enum.RankDir.html#impl-StructuralPartialEq-for-RankDir" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/petgraph/latest/petgraph/dot/enum.RankDir.html" class="enum" title="enum petgraph::dot::RankDir">RankDir</a>

## Auto Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/dot/enum.RankDir.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/petgraph/latest/petgraph/dot/enum.RankDir.html#blanket-implementations" class="anchor">§</a>
