# Enum AcyclicEdgeError Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/acyclic.rs.html#84-91" class="src">Source</a>

``` rust
pub enum AcyclicEdgeError<N> {
    Cycle(Cycle<N>),
    SelfLoop,
    InvalidEdge,
}
```

Expand description

An error that can occur during edge addition for acyclic graphs.

## Variants<a href="https://docs.rs/petgraph/latest/petgraph/acyclic/enum.AcyclicEdgeError.html#variants" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/acyclic/enum.AcyclicEdgeError.html#variant.Cycle" class="anchor">§</a>

### Cycle(<a href="https://docs.rs/petgraph/latest/petgraph/algo/struct.Cycle.html" class="struct" title="struct petgraph::algo::Cycle">Cycle</a>\<N\>)

The edge would create a cycle.

<a href="https://docs.rs/petgraph/latest/petgraph/acyclic/enum.AcyclicEdgeError.html#variant.SelfLoop" class="anchor">§</a>

### SelfLoop

The edge would create a self-loop.

<a href="https://docs.rs/petgraph/latest/petgraph/acyclic/enum.AcyclicEdgeError.html#variant.InvalidEdge" class="anchor">§</a>

### InvalidEdge

Could not successfully add the edge to the underlying graph.

## Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/acyclic/enum.AcyclicEdgeError.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/acyclic/enum.AcyclicEdgeError.html#impl-Clone-for-AcyclicEdgeError%3CN%3E" class="anchor">§</a>

### impl\<N: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/petgraph/latest/petgraph/acyclic/enum.AcyclicEdgeError.html" class="enum" title="enum petgraph::acyclic::AcyclicEdgeError">AcyclicEdgeError</a>\<N\>

<a href="https://docs.rs/petgraph/latest/petgraph/acyclic/enum.AcyclicEdgeError.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/petgraph/latest/petgraph/acyclic/enum.AcyclicEdgeError.html" class="enum" title="enum petgraph::acyclic::AcyclicEdgeError">AcyclicEdgeError</a>\<N\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/acyclic/enum.AcyclicEdgeError.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/petgraph/latest/petgraph/acyclic/enum.AcyclicEdgeError.html#impl-Debug-for-AcyclicEdgeError%3CN%3E" class="anchor">§</a>

### impl\<N: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/petgraph/latest/petgraph/acyclic/enum.AcyclicEdgeError.html" class="enum" title="enum petgraph::acyclic::AcyclicEdgeError">AcyclicEdgeError</a>\<N\>

<a href="https://docs.rs/petgraph/latest/petgraph/acyclic/enum.AcyclicEdgeError.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/petgraph/latest/petgraph/acyclic/enum.AcyclicEdgeError.html#impl-From%3CCycle%3CN%3E%3E-for-AcyclicEdgeError%3CN%3E" class="anchor">§</a>

### impl\<N\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/petgraph/latest/petgraph/algo/struct.Cycle.html" class="struct" title="struct petgraph::algo::Cycle">Cycle</a>\<N\>\> for <a href="https://docs.rs/petgraph/latest/petgraph/acyclic/enum.AcyclicEdgeError.html" class="enum" title="enum petgraph::acyclic::AcyclicEdgeError">AcyclicEdgeError</a>\<N\>

<a href="https://docs.rs/petgraph/latest/petgraph/acyclic/enum.AcyclicEdgeError.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(cycle: <a href="https://docs.rs/petgraph/latest/petgraph/algo/struct.Cycle.html" class="struct" title="struct petgraph::algo::Cycle">Cycle</a>\<N\>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/petgraph/latest/petgraph/acyclic/enum.AcyclicEdgeError.html#impl-PartialEq-for-AcyclicEdgeError%3CN%3E" class="anchor">§</a>

### impl\<N: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>\> <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/petgraph/latest/petgraph/acyclic/enum.AcyclicEdgeError.html" class="enum" title="enum petgraph::acyclic::AcyclicEdgeError">AcyclicEdgeError</a>\<N\>

<a href="https://docs.rs/petgraph/latest/petgraph/acyclic/enum.AcyclicEdgeError.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/petgraph/latest/petgraph/acyclic/enum.AcyclicEdgeError.html" class="enum" title="enum petgraph::acyclic::AcyclicEdgeError">AcyclicEdgeError</a>\<N\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/acyclic/enum.AcyclicEdgeError.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/petgraph/latest/petgraph/acyclic/enum.AcyclicEdgeError.html#impl-StructuralPartialEq-for-AcyclicEdgeError%3CN%3E" class="anchor">§</a>

### impl\<N\> <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/petgraph/latest/petgraph/acyclic/enum.AcyclicEdgeError.html" class="enum" title="enum petgraph::acyclic::AcyclicEdgeError">AcyclicEdgeError</a>\<N\>

## Auto Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/acyclic/enum.AcyclicEdgeError.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/petgraph/latest/petgraph/acyclic/enum.AcyclicEdgeError.html#blanket-implementations" class="anchor">§</a>
