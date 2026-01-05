# Trait IntoWeightedEdge Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/lib.rs.html#614-617" class="src">Source</a>

``` rust
pub trait IntoWeightedEdge<E> {
    type NodeId;

    // Required method
    fn into_weighted_edge(self) -> (Self::NodeId, Self::NodeId, E);
}
```

Expand description

Convert an element like `(i, j)` or `(i, j, w)` into a triple of source, target, edge weight.

For `Graph::from_edges` and `GraphMap::from_edges`.

## Required Associated Types<a href="https://docs.rs/petgraph/latest/petgraph/trait.IntoWeightedEdge.html#required-associated-types" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/trait.IntoWeightedEdge.html#associatedtype.NodeId" class="associatedtype">NodeId</a>

## Required Methods<a href="https://docs.rs/petgraph/latest/petgraph/trait.IntoWeightedEdge.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/trait.IntoWeightedEdge.html#tymethod.into_weighted_edge" class="fn">into_weighted_edge</a>(self) -\> (Self::<a href="https://docs.rs/petgraph/latest/petgraph/trait.IntoWeightedEdge.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::IntoWeightedEdge::NodeId">NodeId</a>, Self::<a href="https://docs.rs/petgraph/latest/petgraph/trait.IntoWeightedEdge.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::IntoWeightedEdge::NodeId">NodeId</a>, E)

## Implementations on Foreign Types<a href="https://docs.rs/petgraph/latest/petgraph/trait.IntoWeightedEdge.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/trait.IntoWeightedEdge.html#impl-IntoWeightedEdge%3CE%3E-for-%26(Ix,+Ix)" class="anchor">§</a>

### impl\<Ix, E\> <a href="https://docs.rs/petgraph/latest/petgraph/trait.IntoWeightedEdge.html" class="trait" title="trait petgraph::IntoWeightedEdge">IntoWeightedEdge</a>\<E\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.tuple.html" class="primitive">(Ix, Ix)</a>

where Ix: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a>, E: <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/trait.IntoWeightedEdge.html#associatedtype.NodeId-1" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/trait.IntoWeightedEdge.html#associatedtype.NodeId" class="associatedtype">NodeId</a> = Ix

<a href="https://docs.rs/petgraph/latest/petgraph/trait.IntoWeightedEdge.html#method.into_weighted_edge" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/trait.IntoWeightedEdge.html#tymethod.into_weighted_edge" class="fn">into_weighted_edge</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.tuple.html" class="primitive">(Ix, Ix, E)</a>

<a href="https://docs.rs/petgraph/latest/petgraph/trait.IntoWeightedEdge.html#impl-IntoWeightedEdge%3CE%3E-for-%26(Ix,+Ix,+E)" class="anchor">§</a>

### impl\<Ix, E\> <a href="https://docs.rs/petgraph/latest/petgraph/trait.IntoWeightedEdge.html" class="trait" title="trait petgraph::IntoWeightedEdge">IntoWeightedEdge</a>\<E\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.tuple.html" class="primitive">(Ix, Ix, E)</a>

where Ix: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a>, E: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/trait.IntoWeightedEdge.html#associatedtype.NodeId-2" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/trait.IntoWeightedEdge.html#associatedtype.NodeId" class="associatedtype">NodeId</a> = Ix

<a href="https://docs.rs/petgraph/latest/petgraph/trait.IntoWeightedEdge.html#method.into_weighted_edge-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/trait.IntoWeightedEdge.html#tymethod.into_weighted_edge" class="fn">into_weighted_edge</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.tuple.html" class="primitive">(Ix, Ix, E)</a>

<a href="https://docs.rs/petgraph/latest/petgraph/trait.IntoWeightedEdge.html#impl-IntoWeightedEdge%3CE%3E-for-(Ix,+Ix,+%26E)" class="anchor">§</a>

### impl\<Ix, E\> <a href="https://docs.rs/petgraph/latest/petgraph/trait.IntoWeightedEdge.html" class="trait" title="trait petgraph::IntoWeightedEdge">IntoWeightedEdge</a>\<E\> for (Ix, Ix, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;E</a>)

where E: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/trait.IntoWeightedEdge.html#associatedtype.NodeId-3" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/trait.IntoWeightedEdge.html#associatedtype.NodeId" class="associatedtype">NodeId</a> = Ix

<a href="https://docs.rs/petgraph/latest/petgraph/trait.IntoWeightedEdge.html#method.into_weighted_edge-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/trait.IntoWeightedEdge.html#tymethod.into_weighted_edge" class="fn">into_weighted_edge</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.tuple.html" class="primitive">(Ix, Ix, E)</a>

<a href="https://docs.rs/petgraph/latest/petgraph/trait.IntoWeightedEdge.html#impl-IntoWeightedEdge%3CE%3E-for-(Ix,+Ix)" class="anchor">§</a>

### impl\<Ix, E\> <a href="https://docs.rs/petgraph/latest/petgraph/trait.IntoWeightedEdge.html" class="trait" title="trait petgraph::IntoWeightedEdge">IntoWeightedEdge</a>\<E\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.tuple.html" class="primitive">(Ix, Ix)</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/trait.IntoWeightedEdge.html#associatedtype.NodeId-4" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/trait.IntoWeightedEdge.html#associatedtype.NodeId" class="associatedtype">NodeId</a> = Ix

<a href="https://docs.rs/petgraph/latest/petgraph/trait.IntoWeightedEdge.html#method.into_weighted_edge-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/trait.IntoWeightedEdge.html#tymethod.into_weighted_edge" class="fn">into_weighted_edge</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.tuple.html" class="primitive">(Ix, Ix, E)</a>

<a href="https://docs.rs/petgraph/latest/petgraph/trait.IntoWeightedEdge.html#impl-IntoWeightedEdge%3CE%3E-for-(Ix,+Ix,+E)" class="anchor">§</a>

### impl\<Ix, E\> <a href="https://docs.rs/petgraph/latest/petgraph/trait.IntoWeightedEdge.html" class="trait" title="trait petgraph::IntoWeightedEdge">IntoWeightedEdge</a>\<E\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.tuple.html" class="primitive">(Ix, Ix, E)</a>

<a href="https://docs.rs/petgraph/latest/petgraph/trait.IntoWeightedEdge.html#associatedtype.NodeId-5" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/trait.IntoWeightedEdge.html#associatedtype.NodeId" class="associatedtype">NodeId</a> = Ix

<a href="https://docs.rs/petgraph/latest/petgraph/trait.IntoWeightedEdge.html#method.into_weighted_edge-4" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/trait.IntoWeightedEdge.html#tymethod.into_weighted_edge" class="fn">into_weighted_edge</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.tuple.html" class="primitive">(Ix, Ix, E)</a>

## Implementors<a href="https://docs.rs/petgraph/latest/petgraph/trait.IntoWeightedEdge.html#implementors" class="anchor">§</a>
