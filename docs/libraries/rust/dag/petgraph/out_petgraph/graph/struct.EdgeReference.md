# Struct EdgeReference Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/graph_impl/mod.rs.html#2469-2473" class="src">Source</a>

``` rust
pub struct EdgeReference<'a, E: 'a, Ix = DefaultIx> { /* private fields */ }
```

Expand description

Reference to a `Graph` edge.

## Implementations<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeReference.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeReference.html#impl-EdgeReference%3C&#39;a,+E,+Ix%3E" class="anchor">§</a>

### impl\<'a, Ix, E\> <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeReference.html" class="struct" title="struct petgraph::graph::EdgeReference">EdgeReference</a>\<'a, E, Ix\>

where Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeReference.html#method.weight" class="fn">weight</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a E</a>

Access the edge’s weight.

**NOTE** that this method offers a longer lifetime than the trait (unfortunately they don’t match yet).

## Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeReference.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeReference.html#impl-Clone-for-EdgeReference%3C&#39;_,+E,+Ix%3E" class="anchor">§</a>

### impl\<E, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeReference.html" class="struct" title="struct petgraph::graph::EdgeReference">EdgeReference</a>\<'\_, E, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeReference.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> Self

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeReference.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeReference.html#impl-Debug-for-EdgeReference%3C&#39;a,+E,+Ix%3E" class="anchor">§</a>

### impl\<'a, E: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> + 'a, Ix: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeReference.html" class="struct" title="struct petgraph::graph::EdgeReference">EdgeReference</a>\<'a, E, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeReference.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeReference.html#impl-EdgeRef-for-EdgeReference%3C&#39;_,+E,+Ix%3E" class="anchor">§</a>

### impl\<Ix, E\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html" class="trait" title="trait petgraph::visit::EdgeRef">EdgeRef</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeReference.html" class="struct" title="struct petgraph::graph::EdgeReference">EdgeReference</a>\<'\_, E, Ix\>

where Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeReference.html#associatedtype.NodeId" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.NodeId" class="associatedtype">NodeId</a> = <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeReference.html#associatedtype.EdgeId" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.EdgeId" class="associatedtype">EdgeId</a> = <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeIndex.html" class="struct" title="struct petgraph::graph::EdgeIndex">EdgeIndex</a>\<Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeReference.html#associatedtype.Weight" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.Weight" class="associatedtype">Weight</a> = E

<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeReference.html#method.source" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#tymethod.source" class="fn">source</a>(&self) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::EdgeRef::NodeId">NodeId</a>

The source node of the edge.

<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeReference.html#method.target" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#tymethod.target" class="fn">target</a>(&self) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::EdgeRef::NodeId">NodeId</a>

The target node of the edge.

<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeReference.html#method.weight-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#tymethod.weight" class="fn">weight</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;E</a>

A reference to the weight of the edge.

<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeReference.html#method.id" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#tymethod.id" class="fn">id</a>(&self) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.EdgeId" class="associatedtype" title="type petgraph::visit::EdgeRef::EdgeId">EdgeId</a>

The edge’s identifier.

<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeReference.html#impl-PartialEq-for-EdgeReference%3C&#39;_,+E,+Ix%3E" class="anchor">§</a>

### impl\<E, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeReference.html" class="struct" title="struct petgraph::graph::EdgeReference">EdgeReference</a>\<'\_, E, Ix\>

where E: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeReference.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, rhs: &Self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeReference.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeReference.html#impl-Copy-for-EdgeReference%3C&#39;_,+E,+Ix%3E" class="anchor">§</a>

### impl\<E, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeReference.html" class="struct" title="struct petgraph::graph::EdgeReference">EdgeReference</a>\<'\_, E, Ix\>

## Auto Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeReference.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeReference.html#blanket-implementations" class="anchor">§</a>
