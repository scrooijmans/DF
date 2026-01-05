# Struct Cycle Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/mod.rs.html#518" class="src">Source</a>

``` rust
pub struct Cycle<N>(/* private fields */);
```

Expand description

An algorithm error: a cycle was found in the graph.

## Implementations<a href="https://docs.rs/petgraph/latest/petgraph/algo/struct.Cycle.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/struct.Cycle.html#impl-Cycle%3CN%3E" class="anchor">§</a>

### impl\<N\> <a href="https://docs.rs/petgraph/latest/petgraph/algo/struct.Cycle.html" class="struct" title="struct petgraph::algo::Cycle">Cycle</a>\<N\>

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/struct.Cycle.html#method.node_id" class="fn">node_id</a>(&self) -\> N

where N: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a>,

Return a node id that participates in the cycle

## Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/algo/struct.Cycle.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/struct.Cycle.html#impl-Clone-for-Cycle%3CN%3E" class="anchor">§</a>

### impl\<N: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/petgraph/latest/petgraph/algo/struct.Cycle.html" class="struct" title="struct petgraph::algo::Cycle">Cycle</a>\<N\>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/struct.Cycle.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/petgraph/latest/petgraph/algo/struct.Cycle.html" class="struct" title="struct petgraph::algo::Cycle">Cycle</a>\<N\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/algo/struct.Cycle.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/petgraph/latest/petgraph/algo/struct.Cycle.html#impl-Debug-for-Cycle%3CN%3E" class="anchor">§</a>

### impl\<N: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/petgraph/latest/petgraph/algo/struct.Cycle.html" class="struct" title="struct petgraph::algo::Cycle">Cycle</a>\<N\>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/struct.Cycle.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/petgraph/latest/petgraph/algo/struct.Cycle.html#impl-From%3CCycle%3CN%3E%3E-for-AcyclicEdgeError%3CN%3E" class="anchor">§</a>

### impl\<N\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/petgraph/latest/petgraph/algo/struct.Cycle.html" class="struct" title="struct petgraph::algo::Cycle">Cycle</a>\<N\>\> for <a href="https://docs.rs/petgraph/latest/petgraph/acyclic/enum.AcyclicEdgeError.html" class="enum" title="enum petgraph::acyclic::AcyclicEdgeError">AcyclicEdgeError</a>\<N\>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/struct.Cycle.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(cycle: <a href="https://docs.rs/petgraph/latest/petgraph/algo/struct.Cycle.html" class="struct" title="struct petgraph::algo::Cycle">Cycle</a>\<N\>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/petgraph/latest/petgraph/algo/struct.Cycle.html#impl-PartialEq-for-Cycle%3CN%3E" class="anchor">§</a>

### impl\<N: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>\> <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/petgraph/latest/petgraph/algo/struct.Cycle.html" class="struct" title="struct petgraph::algo::Cycle">Cycle</a>\<N\>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/struct.Cycle.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/petgraph/latest/petgraph/algo/struct.Cycle.html" class="struct" title="struct petgraph::algo::Cycle">Cycle</a>\<N\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/algo/struct.Cycle.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/petgraph/latest/petgraph/algo/struct.Cycle.html#impl-StructuralPartialEq-for-Cycle%3CN%3E" class="anchor">§</a>

### impl\<N\> <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/petgraph/latest/petgraph/algo/struct.Cycle.html" class="struct" title="struct petgraph::algo::Cycle">Cycle</a>\<N\>

## Auto Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/algo/struct.Cycle.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/petgraph/latest/petgraph/algo/struct.Cycle.html#blanket-implementations" class="anchor">§</a>
