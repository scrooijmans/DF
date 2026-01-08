# Trait DataMapMut Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/data.rs.html#39-47" class="src">Source</a>

``` rust
pub trait DataMapMut: DataMap {
    // Required methods
    fn node_weight_mut(
        &mut self,
        id: Self::NodeId,
    ) -> Option<&mut Self::NodeWeight>;
    fn edge_weight_mut(
        &mut self,
        id: Self::EdgeId,
    ) -> Option<&mut Self::EdgeWeight>;
}
```

Expand description

Access node and edge weights mutably.

## Required Methods<a href="https://docs.rs/petgraph/latest/petgraph/data/trait.DataMapMut.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.DataMapMut.html#tymethod.node_weight_mut" class="fn">node_weight_mut</a>(&mut self, id: Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&mut Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html#associatedtype.NodeWeight" class="associatedtype" title="type petgraph::visit::Data::NodeWeight">NodeWeight</a>\>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.DataMapMut.html#tymethod.edge_weight_mut" class="fn">edge_weight_mut</a>(&mut self, id: Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.EdgeId" class="associatedtype" title="type petgraph::visit::GraphBase::EdgeId">EdgeId</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&mut Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html#associatedtype.EdgeWeight" class="associatedtype" title="type petgraph::visit::Data::EdgeWeight">EdgeWeight</a>\>

## Implementations on Foreign Types<a href="https://docs.rs/petgraph/latest/petgraph/data/trait.DataMapMut.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/data/trait.DataMapMut.html#impl-DataMapMut-for-%26mut+G" class="anchor">§</a>

### impl\<'a, G\> <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.DataMapMut.html" class="trait" title="trait petgraph::data::DataMapMut">DataMapMut</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a mut G</a>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.DataMapMut.html" class="trait" title="trait petgraph::data::DataMapMut">DataMapMut</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/data/trait.DataMapMut.html#method.node_weight_mut" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.DataMapMut.html#tymethod.node_weight_mut" class="fn">node_weight_mut</a>(&mut self, id: Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&mut Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html#associatedtype.NodeWeight" class="associatedtype" title="type petgraph::visit::Data::NodeWeight">NodeWeight</a>\>

<a href="https://docs.rs/petgraph/latest/petgraph/data/trait.DataMapMut.html#method.edge_weight_mut" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.DataMapMut.html#tymethod.edge_weight_mut" class="fn">edge_weight_mut</a>(&mut self, id: Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.EdgeId" class="associatedtype" title="type petgraph::visit::GraphBase::EdgeId">EdgeId</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&mut Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html#associatedtype.EdgeWeight" class="associatedtype" title="type petgraph::visit::Data::EdgeWeight">EdgeWeight</a>\>

## Implementors<a href="https://docs.rs/petgraph/latest/petgraph/data/trait.DataMapMut.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/data/trait.DataMapMut.html#impl-DataMapMut-for-Frozen%3C&#39;a,+G%3E" class="anchor">§</a>

### impl\<'a, G\> <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.DataMapMut.html" class="trait" title="trait petgraph::data::DataMapMut">DataMapMut</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.Frozen.html" class="struct" title="struct petgraph::graph::Frozen">Frozen</a>\<'a, G\>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.DataMapMut.html" class="trait" title="trait petgraph::data::DataMapMut">DataMapMut</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/data/trait.DataMapMut.html#impl-DataMapMut-for-List%3CE,+Ix%3E" class="anchor">§</a>

### impl\<E, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.DataMapMut.html" class="trait" title="trait petgraph::data::DataMapMut">DataMapMut</a> for <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html" class="struct" title="struct petgraph::adj::List">List</a>\<E, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/data/trait.DataMapMut.html#impl-DataMapMut-for-Reversed%3CG%3E" class="anchor">§</a>

### impl\<G\> <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.DataMapMut.html" class="trait" title="trait petgraph::data::DataMapMut">DataMapMut</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Reversed.html" class="struct" title="struct petgraph::visit::Reversed">Reversed</a>\<G\>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.DataMapMut.html" class="trait" title="trait petgraph::data::DataMapMut">DataMapMut</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/data/trait.DataMapMut.html#impl-DataMapMut-for-Acyclic%3CG%3E" class="anchor">§</a>

### impl\<G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html" class="trait" title="trait petgraph::visit::Visitable">Visitable</a> + <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.DataMapMut.html" class="trait" title="trait petgraph::data::DataMapMut">DataMapMut</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.DataMapMut.html" class="trait" title="trait petgraph::data::DataMapMut">DataMapMut</a> for <a href="https://docs.rs/petgraph/latest/petgraph/acyclic/struct.Acyclic.html" class="struct" title="struct petgraph::acyclic::Acyclic">Acyclic</a>\<G\>

<a href="https://docs.rs/petgraph/latest/petgraph/data/trait.DataMapMut.html#impl-DataMapMut-for-Graph%3CN,+E,+Ty,+Ix%3E" class="anchor">§</a>

### impl\<N, E, Ty, Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.DataMapMut.html" class="trait" title="trait petgraph::data::DataMapMut">DataMapMut</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.Graph.html" class="struct" title="struct petgraph::graph::Graph">Graph</a>\<N, E, Ty, Ix\>

where Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/data/trait.DataMapMut.html#impl-DataMapMut-for-StableGraph%3CN,+E,+Ty,+Ix%3E" class="anchor">§</a>

### impl\<N, E, Ty, Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.DataMapMut.html" class="trait" title="trait petgraph::data::DataMapMut">DataMapMut</a> for <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html" class="struct" title="struct petgraph::stable_graph::StableGraph">StableGraph</a>\<N, E, Ty, Ix\>

where Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

Available on **crate feature `stable_graph`** only.
