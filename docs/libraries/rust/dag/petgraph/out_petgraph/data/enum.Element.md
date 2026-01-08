# Enum Element Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/data.rs.html#275-284" class="src">Source</a>

``` rust
pub enum Element<N, E> {
    Node {
        weight: N,
    },
    Edge {
        source: usize,
        target: usize,
        weight: E,
    },
}
```

Expand description

A graph element.

A sequence of Elements, for example an iterator, is laid out as follows: Nodes are implicitly given the index of their appearance in the sequence. The edges’ source and target fields refer to these indices.

## Variants<a href="https://docs.rs/petgraph/latest/petgraph/data/enum.Element.html#variants" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/data/enum.Element.html#variant.Node" class="anchor">§</a>

### Node

A graph node.

#### Fields

<a href="https://docs.rs/petgraph/latest/petgraph/data/enum.Element.html#variant.Node.field.weight" class="anchor field">§</a>`weight: N`

<a href="https://docs.rs/petgraph/latest/petgraph/data/enum.Element.html#variant.Edge" class="anchor">§</a>

### Edge

A graph edge.

#### Fields

<a href="https://docs.rs/petgraph/latest/petgraph/data/enum.Element.html#variant.Edge.field.source" class="anchor field">§</a>`source: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>

<a href="https://docs.rs/petgraph/latest/petgraph/data/enum.Element.html#variant.Edge.field.target" class="anchor field">§</a>`target: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>

<a href="https://docs.rs/petgraph/latest/petgraph/data/enum.Element.html#variant.Edge.field.weight" class="anchor field">§</a>`weight: E`

## Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/data/enum.Element.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/data/enum.Element.html#impl-Clone-for-Element%3CN,+E%3E" class="anchor">§</a>

### impl\<N: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>, E: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/petgraph/latest/petgraph/data/enum.Element.html" class="enum" title="enum petgraph::data::Element">Element</a>\<N, E\>

<a href="https://docs.rs/petgraph/latest/petgraph/data/enum.Element.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/petgraph/latest/petgraph/data/enum.Element.html" class="enum" title="enum petgraph::data::Element">Element</a>\<N, E\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/data/enum.Element.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/petgraph/latest/petgraph/data/enum.Element.html#impl-Debug-for-Element%3CN,+E%3E" class="anchor">§</a>

### impl\<N: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>, E: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/petgraph/latest/petgraph/data/enum.Element.html" class="enum" title="enum petgraph::data::Element">Element</a>\<N, E\>

<a href="https://docs.rs/petgraph/latest/petgraph/data/enum.Element.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/petgraph/latest/petgraph/data/enum.Element.html#impl-PartialEq-for-Element%3CN,+E%3E" class="anchor">§</a>

### impl\<N: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>, E: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>\> <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/petgraph/latest/petgraph/data/enum.Element.html" class="enum" title="enum petgraph::data::Element">Element</a>\<N, E\>

<a href="https://docs.rs/petgraph/latest/petgraph/data/enum.Element.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/petgraph/latest/petgraph/data/enum.Element.html" class="enum" title="enum petgraph::data::Element">Element</a>\<N, E\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/data/enum.Element.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/petgraph/latest/petgraph/data/enum.Element.html#impl-Eq-for-Element%3CN,+E%3E" class="anchor">§</a>

### impl\<N: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a>, E: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a>\> <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/petgraph/latest/petgraph/data/enum.Element.html" class="enum" title="enum petgraph::data::Element">Element</a>\<N, E\>

<a href="https://docs.rs/petgraph/latest/petgraph/data/enum.Element.html#impl-StructuralPartialEq-for-Element%3CN,+E%3E" class="anchor">§</a>

### impl\<N, E\> <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/petgraph/latest/petgraph/data/enum.Element.html" class="enum" title="enum petgraph::data::Element">Element</a>\<N, E\>

## Auto Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/data/enum.Element.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/petgraph/latest/petgraph/data/enum.Element.html#blanket-implementations" class="anchor">§</a>
