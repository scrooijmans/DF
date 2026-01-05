# Trait EdgeRef Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/visit/mod.rs.html#223-235" class="src">Source</a>

``` rust
pub trait EdgeRef: Copy {
    type NodeId;
    type EdgeId;
    type Weight;

    // Required methods
    fn source(&self) -> Self::NodeId;
    fn target(&self) -> Self::NodeId;
    fn weight(&self) -> &Self::Weight;
    fn id(&self) -> Self::EdgeId;
}
```

Expand description

An edge reference.

Edge references are used by traits `IntoEdges` and `IntoEdgeReferences`.

## Required Associated Types<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#required-associated-types" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.NodeId" class="associatedtype">NodeId</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.EdgeId" class="associatedtype">EdgeId</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.Weight" class="associatedtype">Weight</a>

## Required Methods<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#tymethod.source" class="fn">source</a>(&self) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::EdgeRef::NodeId">NodeId</a>

The source node of the edge.

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#tymethod.target" class="fn">target</a>(&self) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::EdgeRef::NodeId">NodeId</a>

The target node of the edge.

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#tymethod.weight" class="fn">weight</a>(&self) -\> &Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.Weight" class="associatedtype" title="type petgraph::visit::EdgeRef::Weight">Weight</a>

A reference to the weight of the edge.

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#tymethod.id" class="fn">id</a>(&self) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.EdgeId" class="associatedtype" title="type petgraph::visit::EdgeRef::EdgeId">EdgeId</a>

The edge’s identifier.

## Dyn Compatibility<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementations on Foreign Types<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#impl-EdgeRef-for-(N,+N,+%26E)" class="anchor">§</a>

### impl\<N, E\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html" class="trait" title="trait petgraph::visit::EdgeRef">EdgeRef</a> for (N, N, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;E</a>)

where N: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.NodeId-1" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.NodeId" class="associatedtype">NodeId</a> = N

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.EdgeId-1" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.EdgeId" class="associatedtype">EdgeId</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.tuple.html" class="primitive">(N, N)</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.Weight-1" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.Weight" class="associatedtype">Weight</a> = E

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#method.source" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#tymethod.source" class="fn">source</a>(&self) -\> N

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#method.target" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#tymethod.target" class="fn">target</a>(&self) -\> N

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#method.weight" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#tymethod.weight" class="fn">weight</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;E</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#method.id" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#tymethod.id" class="fn">id</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.tuple.html" class="primitive">(N, N)</a>

## Implementors<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#impl-EdgeRef-for-EdgeReference%3C&#39;_,+E,+Ix%3E" class="anchor">§</a>

### impl\<E, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html" class="trait" title="trait petgraph::visit::EdgeRef">EdgeRef</a> for petgraph::adj::<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeReference.html" class="struct" title="struct petgraph::adj::EdgeReference">EdgeReference</a>\<'\_, E, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.NodeId-2" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.NodeId" class="associatedtype">NodeId</a> = Ix

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.EdgeId-2" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.EdgeId" class="associatedtype">EdgeId</a> = <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeIndex.html" class="struct" title="struct petgraph::adj::EdgeIndex">EdgeIndex</a>\<Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.Weight-2" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.Weight" class="associatedtype">Weight</a> = E

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#impl-EdgeRef-for-EdgeReference%3C&#39;_,+E,+Ty,+Ix%3E" class="anchor">§</a>

### impl\<E, Ty, Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html" class="trait" title="trait petgraph::visit::EdgeRef">EdgeRef</a> for petgraph::csr::<a href="https://docs.rs/petgraph/latest/petgraph/csr/struct.EdgeReference.html" class="struct" title="struct petgraph::csr::EdgeReference">EdgeReference</a>\<'\_, E, Ty, Ix\>

where Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.NodeId-3" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.NodeId" class="associatedtype">NodeId</a> = Ix

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.EdgeId-3" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.EdgeId" class="associatedtype">EdgeId</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.Weight-3" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.Weight" class="associatedtype">Weight</a> = E

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#impl-EdgeRef-for-EdgeReference%3C&#39;_,+E,+Ix%3E-1" class="anchor">§</a>

### impl\<Ix, E\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html" class="trait" title="trait petgraph::visit::EdgeRef">EdgeRef</a> for petgraph::graph::<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeReference.html" class="struct" title="struct petgraph::graph::EdgeReference">EdgeReference</a>\<'\_, E, Ix\>

where Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.NodeId-4" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.NodeId" class="associatedtype">NodeId</a> = <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.EdgeId-4" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.EdgeId" class="associatedtype">EdgeId</a> = <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeIndex.html" class="struct" title="struct petgraph::graph::EdgeIndex">EdgeIndex</a>\<Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.Weight-4" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.Weight" class="associatedtype">Weight</a> = E

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#impl-EdgeRef-for-EdgeReference%3C&#39;_,+E,+Ix%3E-2" class="anchor">§</a>

### impl\<Ix, E\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html" class="trait" title="trait petgraph::visit::EdgeRef">EdgeRef</a> for petgraph::stable_graph::<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.EdgeReference.html" class="struct" title="struct petgraph::stable_graph::EdgeReference">EdgeReference</a>\<'\_, E, Ix\>

where Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.NodeId-5" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.NodeId" class="associatedtype">NodeId</a> = <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.EdgeId-5" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.EdgeId" class="associatedtype">EdgeId</a> = <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeIndex.html" class="struct" title="struct petgraph::graph::EdgeIndex">EdgeIndex</a>\<Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.Weight-5" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.Weight" class="associatedtype">Weight</a> = E

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#impl-EdgeRef-for-MaybeReversedEdgeReference%3CR%3E" class="anchor">§</a>

### impl\<R\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html" class="trait" title="trait petgraph::visit::EdgeRef">EdgeRef</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.MaybeReversedEdgeReference.html" class="struct" title="struct petgraph::visit::MaybeReversedEdgeReference">MaybeReversedEdgeReference</a>\<R\>

where R: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html" class="trait" title="trait petgraph::visit::EdgeRef">EdgeRef</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.NodeId-6" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.NodeId" class="associatedtype">NodeId</a> = \<R as <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html" class="trait" title="trait petgraph::visit::EdgeRef">EdgeRef</a>\>::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::EdgeRef::NodeId">NodeId</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.EdgeId-6" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.EdgeId" class="associatedtype">EdgeId</a> = \<R as <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html" class="trait" title="trait petgraph::visit::EdgeRef">EdgeRef</a>\>::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.EdgeId" class="associatedtype" title="type petgraph::visit::EdgeRef::EdgeId">EdgeId</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.Weight-6" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.Weight" class="associatedtype">Weight</a> = \<R as <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html" class="trait" title="trait petgraph::visit::EdgeRef">EdgeRef</a>\>::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.Weight" class="associatedtype" title="type petgraph::visit::EdgeRef::Weight">Weight</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#impl-EdgeRef-for-ReversedEdgeReference%3CR%3E" class="anchor">§</a>

### impl\<R\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html" class="trait" title="trait petgraph::visit::EdgeRef">EdgeRef</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdgeReference.html" class="struct" title="struct petgraph::visit::ReversedEdgeReference">ReversedEdgeReference</a>\<R\>

where R: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html" class="trait" title="trait petgraph::visit::EdgeRef">EdgeRef</a>,

An edge reference

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.NodeId-7" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.NodeId" class="associatedtype">NodeId</a> = \<R as <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html" class="trait" title="trait petgraph::visit::EdgeRef">EdgeRef</a>\>::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::EdgeRef::NodeId">NodeId</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.EdgeId-7" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.EdgeId" class="associatedtype">EdgeId</a> = \<R as <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html" class="trait" title="trait petgraph::visit::EdgeRef">EdgeRef</a>\>::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.EdgeId" class="associatedtype" title="type petgraph::visit::EdgeRef::EdgeId">EdgeId</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.Weight-7" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.Weight" class="associatedtype">Weight</a> = \<R as <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html" class="trait" title="trait petgraph::visit::EdgeRef">EdgeRef</a>\>::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.Weight" class="associatedtype" title="type petgraph::visit::EdgeRef::Weight">Weight</a>
