# Struct MaybeReversedEdgeReference Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/visit/undirected_adaptor.rs.html#84-87" class="src">Source</a>

``` rust
pub struct MaybeReversedEdgeReference<R> { /* private fields */ }
```

Expand description

An edge reference which may reverse the edge orientation.

## Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.MaybeReversedEdgeReference.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.MaybeReversedEdgeReference.html#impl-Clone-for-MaybeReversedEdgeReference%3CR%3E" class="anchor">§</a>

### impl\<R: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.MaybeReversedEdgeReference.html" class="struct" title="struct petgraph::visit::MaybeReversedEdgeReference">MaybeReversedEdgeReference</a>\<R\>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.MaybeReversedEdgeReference.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.MaybeReversedEdgeReference.html" class="struct" title="struct petgraph::visit::MaybeReversedEdgeReference">MaybeReversedEdgeReference</a>\<R\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.MaybeReversedEdgeReference.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.MaybeReversedEdgeReference.html#impl-Debug-for-MaybeReversedEdgeReference%3CR%3E" class="anchor">§</a>

### impl\<R: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.MaybeReversedEdgeReference.html" class="struct" title="struct petgraph::visit::MaybeReversedEdgeReference">MaybeReversedEdgeReference</a>\<R\>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.MaybeReversedEdgeReference.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.MaybeReversedEdgeReference.html#impl-EdgeRef-for-MaybeReversedEdgeReference%3CR%3E" class="anchor">§</a>

### impl\<R\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html" class="trait" title="trait petgraph::visit::EdgeRef">EdgeRef</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.MaybeReversedEdgeReference.html" class="struct" title="struct petgraph::visit::MaybeReversedEdgeReference">MaybeReversedEdgeReference</a>\<R\>

where R: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html" class="trait" title="trait petgraph::visit::EdgeRef">EdgeRef</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.MaybeReversedEdgeReference.html#associatedtype.NodeId" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.NodeId" class="associatedtype">NodeId</a> = \<R as <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html" class="trait" title="trait petgraph::visit::EdgeRef">EdgeRef</a>\>::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::EdgeRef::NodeId">NodeId</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.MaybeReversedEdgeReference.html#associatedtype.EdgeId" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.EdgeId" class="associatedtype">EdgeId</a> = \<R as <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html" class="trait" title="trait petgraph::visit::EdgeRef">EdgeRef</a>\>::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.EdgeId" class="associatedtype" title="type petgraph::visit::EdgeRef::EdgeId">EdgeId</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.MaybeReversedEdgeReference.html#associatedtype.Weight" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.Weight" class="associatedtype">Weight</a> = \<R as <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html" class="trait" title="trait petgraph::visit::EdgeRef">EdgeRef</a>\>::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.Weight" class="associatedtype" title="type petgraph::visit::EdgeRef::Weight">Weight</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.MaybeReversedEdgeReference.html#method.source" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#tymethod.source" class="fn">source</a>(&self) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::EdgeRef::NodeId">NodeId</a>

The source node of the edge.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.MaybeReversedEdgeReference.html#method.target" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#tymethod.target" class="fn">target</a>(&self) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::EdgeRef::NodeId">NodeId</a>

The target node of the edge.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.MaybeReversedEdgeReference.html#method.weight" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#tymethod.weight" class="fn">weight</a>(&self) -\> &Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.Weight" class="associatedtype" title="type petgraph::visit::EdgeRef::Weight">Weight</a>

A reference to the weight of the edge.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.MaybeReversedEdgeReference.html#method.id" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#tymethod.id" class="fn">id</a>(&self) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.EdgeId" class="associatedtype" title="type petgraph::visit::EdgeRef::EdgeId">EdgeId</a>

The edge’s identifier.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.MaybeReversedEdgeReference.html#impl-Copy-for-MaybeReversedEdgeReference%3CR%3E" class="anchor">§</a>

### impl\<R: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a>\> <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.MaybeReversedEdgeReference.html" class="struct" title="struct petgraph::visit::MaybeReversedEdgeReference">MaybeReversedEdgeReference</a>\<R\>

## Auto Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.MaybeReversedEdgeReference.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.MaybeReversedEdgeReference.html#blanket-implementations" class="anchor">§</a>
