# Type Alias NodeIndex Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/adj.rs.html#17" class="src">Source</a>

``` rust
pub type NodeIndex<Ix = DefaultIx> = Ix;
```

Expand description

Adjacency list node index type, a plain integer.

## Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/adj/type.NodeIndex.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/adj/type.NodeIndex.html#impl-NodeRef-for-Ix" class="anchor">§</a>

### impl\<Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeRef.html" class="trait" title="trait petgraph::visit::NodeRef">NodeRef</a> for <a href="https://docs.rs/petgraph/latest/petgraph/adj/type.NodeIndex.html" class="type" title="type petgraph::adj::NodeIndex">NodeIndex</a>\<Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/adj/type.NodeIndex.html#associatedtype.NodeId" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeRef.html#associatedtype.NodeId" class="associatedtype">NodeId</a> = Ix

<a href="https://docs.rs/petgraph/latest/petgraph/adj/type.NodeIndex.html#associatedtype.Weight" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeRef.html#associatedtype.Weight" class="associatedtype">Weight</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>

<a href="https://docs.rs/petgraph/latest/petgraph/adj/type.NodeIndex.html#method.id" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeRef.html#tymethod.id" class="fn">id</a>(&self) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeRef.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::NodeRef::NodeId">NodeId</a>

<a href="https://docs.rs/petgraph/latest/petgraph/adj/type.NodeIndex.html#method.weight" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeRef.html#tymethod.weight" class="fn">weight</a>(&self) -\> &Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeRef.html#associatedtype.Weight" class="associatedtype" title="type petgraph::visit::NodeRef::Weight">Weight</a>
