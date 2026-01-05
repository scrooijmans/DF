# Enum Config Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/dot/mod.rs.html#171-184" class="src">Source</a>

``` rust
#[non_exhaustive]pub enum Config {
    NodeIndexLabel,
    EdgeIndexLabel,
    EdgeNoLabel,
    NodeNoLabel,
    GraphContentOnly,
    RankDir(RankDir),
}
```

Expand description

`Dot` configuration.

This enum does not have an exhaustive definition (will be expanded)

## Variants (Non-exhaustive)<a href="https://docs.rs/petgraph/latest/petgraph/dot/enum.Config.html#variants" class="anchor">§</a>

This enum is marked as non-exhaustive

Non-exhaustive enums could have additional variants added in future. Therefore, when matching against variants of non-exhaustive enums, an extra wildcard arm must be added to account for any future variants.

<a href="https://docs.rs/petgraph/latest/petgraph/dot/enum.Config.html#variant.NodeIndexLabel" class="anchor">§</a>

### NodeIndexLabel

Use indices for node labels.

<a href="https://docs.rs/petgraph/latest/petgraph/dot/enum.Config.html#variant.EdgeIndexLabel" class="anchor">§</a>

### EdgeIndexLabel

Use indices for edge labels.

<a href="https://docs.rs/petgraph/latest/petgraph/dot/enum.Config.html#variant.EdgeNoLabel" class="anchor">§</a>

### EdgeNoLabel

Do not generate `label` attributes for edges.

<a href="https://docs.rs/petgraph/latest/petgraph/dot/enum.Config.html#variant.NodeNoLabel" class="anchor">§</a>

### NodeNoLabel

Do not generate `label` attributes for nodes.

<a href="https://docs.rs/petgraph/latest/petgraph/dot/enum.Config.html#variant.GraphContentOnly" class="anchor">§</a>

### GraphContentOnly

Do not print the graph/digraph string.

<a href="https://docs.rs/petgraph/latest/petgraph/dot/enum.Config.html#variant.RankDir" class="anchor">§</a>

### RankDir(<a href="https://docs.rs/petgraph/latest/petgraph/dot/enum.RankDir.html" class="enum" title="enum petgraph::dot::RankDir">RankDir</a>)

Sets direction of graph layout.

## Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/dot/enum.Config.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/dot/enum.Config.html#impl-Debug-for-Config" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/petgraph/latest/petgraph/dot/enum.Config.html" class="enum" title="enum petgraph::dot::Config">Config</a>

<a href="https://docs.rs/petgraph/latest/petgraph/dot/enum.Config.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/petgraph/latest/petgraph/dot/enum.Config.html#impl-PartialEq-for-Config" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/petgraph/latest/petgraph/dot/enum.Config.html" class="enum" title="enum petgraph::dot::Config">Config</a>

<a href="https://docs.rs/petgraph/latest/petgraph/dot/enum.Config.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/petgraph/latest/petgraph/dot/enum.Config.html" class="enum" title="enum petgraph::dot::Config">Config</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/dot/enum.Config.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/petgraph/latest/petgraph/dot/enum.Config.html#impl-Eq-for-Config" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/petgraph/latest/petgraph/dot/enum.Config.html" class="enum" title="enum petgraph::dot::Config">Config</a>

<a href="https://docs.rs/petgraph/latest/petgraph/dot/enum.Config.html#impl-StructuralPartialEq-for-Config" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/petgraph/latest/petgraph/dot/enum.Config.html" class="enum" title="enum petgraph::dot::Config">Config</a>

## Auto Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/dot/enum.Config.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/petgraph/latest/petgraph/dot/enum.Config.html#blanket-implementations" class="anchor">§</a>
