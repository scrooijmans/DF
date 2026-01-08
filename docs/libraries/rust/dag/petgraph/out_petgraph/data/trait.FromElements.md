# Trait FromElements Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/data.rs.html#287-312" class="src">Source</a>

``` rust
pub trait FromElements: Create {
    // Provided method
    fn from_elements<I>(iterable: I) -> Self
       where Self: Sized,
             I: IntoIterator<Item = Element<Self::NodeWeight, Self::EdgeWeight>> { ... }
}
```

Expand description

Create a graph from an iterator of elements.

## Provided Methods<a href="https://docs.rs/petgraph/latest/petgraph/data/trait.FromElements.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.FromElements.html#method.from_elements" class="fn">from_elements</a>\<I\>(iterable: I) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://docs.rs/petgraph/latest/petgraph/data/enum.Element.html" class="enum" title="enum petgraph::data::Element">Element</a>\<Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html#associatedtype.NodeWeight" class="associatedtype" title="type petgraph::visit::Data::NodeWeight">NodeWeight</a>, Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html#associatedtype.EdgeWeight" class="associatedtype" title="type petgraph::visit::Data::EdgeWeight">EdgeWeight</a>\>\>,

## Dyn Compatibility<a href="https://docs.rs/petgraph/latest/petgraph/data/trait.FromElements.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/petgraph/latest/petgraph/data/trait.FromElements.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/data/trait.FromElements.html#impl-FromElements-for-Graph%3CN,+E,+Ty,+Ix%3E" class="anchor">§</a>

### impl\<N, E, Ty, Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.FromElements.html" class="trait" title="trait petgraph::data::FromElements">FromElements</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.Graph.html" class="struct" title="struct petgraph::graph::Graph">Graph</a>\<N, E, Ty, Ix\>

where Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/data/trait.FromElements.html#impl-FromElements-for-StableGraph%3CN,+E,+Ty,+Ix%3E" class="anchor">§</a>

### impl\<N, E, Ty, Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.FromElements.html" class="trait" title="trait petgraph::data::FromElements">FromElements</a> for <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html" class="struct" title="struct petgraph::stable_graph::StableGraph">StableGraph</a>\<N, E, Ty, Ix\>

where Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

Available on **crate feature `stable_graph`** only.

<a href="https://docs.rs/petgraph/latest/petgraph/data/trait.FromElements.html#impl-FromElements-for-GraphMap%3CN,+E,+Ty,+S%3E" class="anchor">§</a>

### impl\<N, E, Ty, S\> <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.FromElements.html" class="trait" title="trait petgraph::data::FromElements">FromElements</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html" class="struct" title="struct petgraph::graphmap::GraphMap">GraphMap</a>\<N, E, Ty, S\>

where Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, N: <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/trait.NodeTrait.html" class="trait" title="trait petgraph::graphmap::NodeTrait">NodeTrait</a>, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a> + <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a>,

Available on **crate feature `graphmap`** only.
