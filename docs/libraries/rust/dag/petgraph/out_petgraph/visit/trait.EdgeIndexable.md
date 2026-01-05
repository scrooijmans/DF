# Trait EdgeIndexable Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/visit/mod.rs.html#358-373" class="src">Source</a>

``` rust
pub trait EdgeIndexable: GraphBase {
    // Required methods
    fn edge_bound(&self) -> usize;
    fn to_index(&self, a: Self::EdgeId) -> usize;
    fn from_index(&self, i: usize) -> Self::EdgeId;
}
```

Expand description

The graph’s `NodeId`s map to indices

## Required Methods<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeIndexable.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeIndexable.html#tymethod.edge_bound" class="fn">edge_bound</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Return an upper bound of the edge indices in the graph (suitable for the size of a bitmap).

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeIndexable.html#tymethod.to_index" class="fn">to_index</a>(&self, a: Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.EdgeId" class="associatedtype" title="type petgraph::visit::GraphBase::EdgeId">EdgeId</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Convert `a` to an integer index.

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeIndexable.html#tymethod.from_index" class="fn">from_index</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.EdgeId" class="associatedtype" title="type petgraph::visit::GraphBase::EdgeId">EdgeId</a>

Convert `i` to an edge index. `i` must be a valid value in the graph.

## Implementations on Foreign Types<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeIndexable.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeIndexable.html#impl-EdgeIndexable-for-%26G" class="anchor">§</a>

### impl\<'a, G\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeIndexable.html" class="trait" title="trait petgraph::visit::EdgeIndexable">EdgeIndexable</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a G</a>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeIndexable.html" class="trait" title="trait petgraph::visit::EdgeIndexable">EdgeIndexable</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeIndexable.html#method.edge_bound" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeIndexable.html#tymethod.edge_bound" class="fn">edge_bound</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeIndexable.html#method.to_index" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeIndexable.html#tymethod.to_index" class="fn">to_index</a>(&self, a: Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.EdgeId" class="associatedtype" title="type petgraph::visit::GraphBase::EdgeId">EdgeId</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeIndexable.html#method.from_index" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeIndexable.html#tymethod.from_index" class="fn">from_index</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.EdgeId" class="associatedtype" title="type petgraph::visit::GraphBase::EdgeId">EdgeId</a>

## Implementors<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeIndexable.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeIndexable.html#impl-EdgeIndexable-for-Frozen%3C&#39;a,+G%3E" class="anchor">§</a>

### impl\<'a, G\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeIndexable.html" class="trait" title="trait petgraph::visit::EdgeIndexable">EdgeIndexable</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.Frozen.html" class="struct" title="struct petgraph::graph::Frozen">Frozen</a>\<'a, G\>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeIndexable.html" class="trait" title="trait petgraph::visit::EdgeIndexable">EdgeIndexable</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeIndexable.html#impl-EdgeIndexable-for-Reversed%3CG%3E" class="anchor">§</a>

### impl\<G\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeIndexable.html" class="trait" title="trait petgraph::visit::EdgeIndexable">EdgeIndexable</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Reversed.html" class="struct" title="struct petgraph::visit::Reversed">Reversed</a>\<G\>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeIndexable.html" class="trait" title="trait petgraph::visit::EdgeIndexable">EdgeIndexable</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeIndexable.html#impl-EdgeIndexable-for-EdgeFiltered%3CG,+F%3E" class="anchor">§</a>

### impl\<G, F\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeIndexable.html" class="trait" title="trait petgraph::visit::EdgeIndexable">EdgeIndexable</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html" class="struct" title="struct petgraph::visit::EdgeFiltered">EdgeFiltered</a>\<G, F\>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeIndexable.html" class="trait" title="trait petgraph::visit::EdgeIndexable">EdgeIndexable</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeIndexable.html#impl-EdgeIndexable-for-NodeFiltered%3CG,+F%3E" class="anchor">§</a>

### impl\<G, F\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeIndexable.html" class="trait" title="trait petgraph::visit::EdgeIndexable">EdgeIndexable</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.NodeFiltered.html" class="struct" title="struct petgraph::visit::NodeFiltered">NodeFiltered</a>\<G, F\>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeIndexable.html" class="trait" title="trait petgraph::visit::EdgeIndexable">EdgeIndexable</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeIndexable.html#impl-EdgeIndexable-for-Acyclic%3CG%3E" class="anchor">§</a>

### impl\<G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html" class="trait" title="trait petgraph::visit::Visitable">Visitable</a> + <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeIndexable.html" class="trait" title="trait petgraph::visit::EdgeIndexable">EdgeIndexable</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeIndexable.html" class="trait" title="trait petgraph::visit::EdgeIndexable">EdgeIndexable</a> for <a href="https://docs.rs/petgraph/latest/petgraph/acyclic/struct.Acyclic.html" class="struct" title="struct petgraph::acyclic::Acyclic">Acyclic</a>\<G\>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeIndexable.html#impl-EdgeIndexable-for-Graph%3CN,+E,+Ty,+Ix%3E" class="anchor">§</a>

### impl\<N, E, Ty, Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeIndexable.html" class="trait" title="trait petgraph::visit::EdgeIndexable">EdgeIndexable</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.Graph.html" class="struct" title="struct petgraph::graph::Graph">Graph</a>\<N, E, Ty, Ix\>

where Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeIndexable.html#impl-EdgeIndexable-for-StableGraph%3CN,+E,+Ty,+Ix%3E" class="anchor">§</a>

### impl\<N, E, Ty, Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeIndexable.html" class="trait" title="trait petgraph::visit::EdgeIndexable">EdgeIndexable</a> for <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html" class="struct" title="struct petgraph::stable_graph::StableGraph">StableGraph</a>\<N, E, Ty, Ix\>

where Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeIndexable.html#impl-EdgeIndexable-for-GraphMap%3CN,+E,+Ty,+S%3E" class="anchor">§</a>

### impl\<N, E, Ty, S\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeIndexable.html" class="trait" title="trait petgraph::visit::EdgeIndexable">EdgeIndexable</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html" class="struct" title="struct petgraph::graphmap::GraphMap">GraphMap</a>\<N, E, Ty, S\>

where N: <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/trait.NodeTrait.html" class="trait" title="trait petgraph::graphmap::NodeTrait">NodeTrait</a>, Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a>,
