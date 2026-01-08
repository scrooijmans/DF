# Trait Create Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/data.rs.html#82-84" class="src">Source</a>

``` rust
pub trait Create: Build + Default {
    // Required method
    fn with_capacity(nodes: usize, edges: usize) -> Self;
}
```

Expand description

A graph that can be created

## Required Methods<a href="https://docs.rs/petgraph/latest/petgraph/data/trait.Create.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.Create.html#tymethod.with_capacity" class="fn">with_capacity</a>(nodes: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, edges: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

## Dyn Compatibility<a href="https://docs.rs/petgraph/latest/petgraph/data/trait.Create.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/petgraph/latest/petgraph/data/trait.Create.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/data/trait.Create.html#impl-Create-for-Acyclic%3CG%3E" class="anchor">§</a>

### impl\<G: <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.Create.html" class="trait" title="trait petgraph::data::Create">Create</a> + <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html" class="trait" title="trait petgraph::visit::Visitable">Visitable</a> + <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeIndexable.html" class="trait" title="trait petgraph::visit::NodeIndexable">NodeIndexable</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.Create.html" class="trait" title="trait petgraph::data::Create">Create</a> for <a href="https://docs.rs/petgraph/latest/petgraph/acyclic/struct.Acyclic.html" class="struct" title="struct petgraph::acyclic::Acyclic">Acyclic</a>\<G\>

where for\<'a\> <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a G</a>: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighborsDirected.html" class="trait" title="trait petgraph::visit::IntoNeighborsDirected">IntoNeighborsDirected</a> + <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeIdentifiers.html" class="trait" title="trait petgraph::visit::IntoNodeIdentifiers">IntoNodeIdentifiers</a> + <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html" class="trait" title="trait petgraph::visit::Visitable">Visitable</a>\<Map = G::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html#associatedtype.Map" class="associatedtype" title="type petgraph::visit::Visitable::Map">Map</a>\> + <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html" class="trait" title="trait petgraph::visit::GraphBase">GraphBase</a>\<NodeId = G::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>\>, G::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/data/trait.Create.html#impl-Create-for-Graph%3CN,+E,+Ty,+Ix%3E" class="anchor">§</a>

### impl\<N, E, Ty, Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.Create.html" class="trait" title="trait petgraph::data::Create">Create</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.Graph.html" class="struct" title="struct petgraph::graph::Graph">Graph</a>\<N, E, Ty, Ix\>

where Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/data/trait.Create.html#impl-Create-for-StableGraph%3CN,+E,+Ty,+Ix%3E" class="anchor">§</a>

### impl\<N, E, Ty, Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.Create.html" class="trait" title="trait petgraph::data::Create">Create</a> for <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html" class="struct" title="struct petgraph::stable_graph::StableGraph">StableGraph</a>\<N, E, Ty, Ix\>

where Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

Available on **crate feature `stable_graph`** only.

<a href="https://docs.rs/petgraph/latest/petgraph/data/trait.Create.html#impl-Create-for-GraphMap%3CN,+E,+Ty,+S%3E" class="anchor">§</a>

### impl\<N, E, Ty, S\> <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.Create.html" class="trait" title="trait petgraph::data::Create">Create</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html" class="struct" title="struct petgraph::graphmap::GraphMap">GraphMap</a>\<N, E, Ty, S\>

where Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, N: <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/trait.NodeTrait.html" class="trait" title="trait petgraph::graphmap::NodeTrait">NodeTrait</a>, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a> + <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a>,

Available on **crate feature `graphmap`** only.
