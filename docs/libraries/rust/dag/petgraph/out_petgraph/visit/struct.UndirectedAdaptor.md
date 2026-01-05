# Struct UndirectedAdaptor Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/visit/undirected_adaptor.rs.html#10" class="src">Source</a>

``` rust
pub struct UndirectedAdaptor<G>(pub G);
```

Expand description

An edge direction removing graph adaptor.

## Tuple Fields<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html#fields" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html#structfield.0" class="anchor field">§</a>`0: G`

## Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html#impl-Clone-for-UndirectedAdaptor%3CG%3E" class="anchor">§</a>

### impl\<G: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html" class="struct" title="struct petgraph::visit::UndirectedAdaptor">UndirectedAdaptor</a>\<G\>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html" class="struct" title="struct petgraph::visit::UndirectedAdaptor">UndirectedAdaptor</a>\<G\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html#impl-Data-for-UndirectedAdaptor%3CG%3E" class="anchor">§</a>

### impl\<G\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html" class="trait" title="trait petgraph::visit::Data">Data</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html" class="struct" title="struct petgraph::visit::UndirectedAdaptor">UndirectedAdaptor</a>\<G\>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html" class="trait" title="trait petgraph::visit::Data">Data</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html#associatedtype.NodeWeight" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html#associatedtype.NodeWeight" class="associatedtype">NodeWeight</a> = \<G as <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html" class="trait" title="trait petgraph::visit::Data">Data</a>\>::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html#associatedtype.NodeWeight" class="associatedtype" title="type petgraph::visit::Data::NodeWeight">NodeWeight</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html#associatedtype.EdgeWeight" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html#associatedtype.EdgeWeight" class="associatedtype">EdgeWeight</a> = \<G as <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html" class="trait" title="trait petgraph::visit::Data">Data</a>\>::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html#associatedtype.EdgeWeight" class="associatedtype" title="type petgraph::visit::Data::EdgeWeight">EdgeWeight</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html#impl-Debug-for-UndirectedAdaptor%3CG%3E" class="anchor">§</a>

### impl\<G: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html" class="struct" title="struct petgraph::visit::UndirectedAdaptor">UndirectedAdaptor</a>\<G\>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html#impl-GraphBase-for-UndirectedAdaptor%3CG%3E" class="anchor">§</a>

### impl\<G\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html" class="trait" title="trait petgraph::visit::GraphBase">GraphBase</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html" class="struct" title="struct petgraph::visit::UndirectedAdaptor">UndirectedAdaptor</a>\<G\>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html" class="trait" title="trait petgraph::visit::GraphBase">GraphBase</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html#associatedtype.NodeId" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype">NodeId</a> = \<G as <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html" class="trait" title="trait petgraph::visit::GraphBase">GraphBase</a>\>::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>

node identifier

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html#associatedtype.EdgeId" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.EdgeId" class="associatedtype">EdgeId</a> = \<G as <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html" class="trait" title="trait petgraph::visit::GraphBase">GraphBase</a>\>::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.EdgeId" class="associatedtype" title="type petgraph::visit::GraphBase::EdgeId">EdgeId</a>

edge identifier

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html#impl-GraphProp-for-UndirectedAdaptor%3CG%3E" class="anchor">§</a>

### impl\<G\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphProp.html" class="trait" title="trait petgraph::visit::GraphProp">GraphProp</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html" class="struct" title="struct petgraph::visit::UndirectedAdaptor">UndirectedAdaptor</a>\<G\>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html" class="trait" title="trait petgraph::visit::GraphBase">GraphBase</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html#associatedtype.EdgeType" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphProp.html#associatedtype.EdgeType" class="associatedtype">EdgeType</a> = <a href="https://docs.rs/petgraph/latest/petgraph/enum.Undirected.html" class="enum" title="enum petgraph::Undirected">Undirected</a>

The kind of edges in the graph.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html#method.is_directed" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphProp.html#method.is_directed" class="fn">is_directed</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html#impl-IntoEdgeReferences-for-UndirectedAdaptor%3CG%3E" class="anchor">§</a>

### impl\<G\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgeReferences.html" class="trait" title="trait petgraph::visit::IntoEdgeReferences">IntoEdgeReferences</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html" class="struct" title="struct petgraph::visit::UndirectedAdaptor">UndirectedAdaptor</a>\<G\>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgeReferences.html" class="trait" title="trait petgraph::visit::IntoEdgeReferences">IntoEdgeReferences</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html#associatedtype.EdgeRef" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgeReferences.html#associatedtype.EdgeRef" class="associatedtype">EdgeRef</a> = <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.MaybeReversedEdgeReference.html" class="struct" title="struct petgraph::visit::MaybeReversedEdgeReference">MaybeReversedEdgeReference</a>\<\<G as <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgeReferences.html" class="trait" title="trait petgraph::visit::IntoEdgeReferences">IntoEdgeReferences</a>\>::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgeReferences.html#associatedtype.EdgeRef" class="associatedtype" title="type petgraph::visit::IntoEdgeReferences::EdgeRef">EdgeRef</a>\>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html#associatedtype.EdgeReferences" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgeReferences.html#associatedtype.EdgeReferences" class="associatedtype">EdgeReferences</a> = <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.MaybeReversedEdgeReferences.html" class="struct" title="struct petgraph::visit::MaybeReversedEdgeReferences">MaybeReversedEdgeReferences</a>\<\<G as <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgeReferences.html" class="trait" title="trait petgraph::visit::IntoEdgeReferences">IntoEdgeReferences</a>\>::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgeReferences.html#associatedtype.EdgeReferences" class="associatedtype" title="type petgraph::visit::IntoEdgeReferences::EdgeReferences">EdgeReferences</a>\>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html#method.edge_references" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgeReferences.html#tymethod.edge_references" class="fn">edge_references</a>(self) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgeReferences.html#associatedtype.EdgeReferences" class="associatedtype" title="type petgraph::visit::IntoEdgeReferences::EdgeReferences">EdgeReferences</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html#impl-IntoEdges-for-UndirectedAdaptor%3CG%3E" class="anchor">§</a>

### impl\<G\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdges.html" class="trait" title="trait petgraph::visit::IntoEdges">IntoEdges</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html" class="struct" title="struct petgraph::visit::UndirectedAdaptor">UndirectedAdaptor</a>\<G\>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgesDirected.html" class="trait" title="trait petgraph::visit::IntoEdgesDirected">IntoEdgesDirected</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html#associatedtype.Edges" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdges.html#associatedtype.Edges" class="associatedtype">Edges</a> = <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/chain/struct.Chain.html" class="struct" title="struct core::iter::adapters::chain::Chain">Chain</a>\<<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.MaybeReversedEdges.html" class="struct" title="struct petgraph::visit::MaybeReversedEdges">MaybeReversedEdges</a>\<\<G as <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgesDirected.html" class="trait" title="trait petgraph::visit::IntoEdgesDirected">IntoEdgesDirected</a>\>::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgesDirected.html#associatedtype.EdgesDirected" class="associatedtype" title="type petgraph::visit::IntoEdgesDirected::EdgesDirected">EdgesDirected</a>\>, <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.MaybeReversedEdges.html" class="struct" title="struct petgraph::visit::MaybeReversedEdges">MaybeReversedEdges</a>\<\<G as <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgesDirected.html" class="trait" title="trait petgraph::visit::IntoEdgesDirected">IntoEdgesDirected</a>\>::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgesDirected.html#associatedtype.EdgesDirected" class="associatedtype" title="type petgraph::visit::IntoEdgesDirected::EdgesDirected">EdgesDirected</a>\>\>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html#method.edges" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdges.html#tymethod.edges" class="fn">edges</a>(self, a: Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdges.html#associatedtype.Edges" class="associatedtype" title="type petgraph::visit::IntoEdges::Edges">Edges</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html#impl-IntoNeighbors-for-UndirectedAdaptor%3CG%3E" class="anchor">§</a>

### impl\<G\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighbors.html" class="trait" title="trait petgraph::visit::IntoNeighbors">IntoNeighbors</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html" class="struct" title="struct petgraph::visit::UndirectedAdaptor">UndirectedAdaptor</a>\<G\>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighborsDirected.html" class="trait" title="trait petgraph::visit::IntoNeighborsDirected">IntoNeighborsDirected</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html#associatedtype.Neighbors" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighbors.html#associatedtype.Neighbors" class="associatedtype">Neighbors</a> = <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/chain/struct.Chain.html" class="struct" title="struct core::iter::adapters::chain::Chain">Chain</a>\<\<G as <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighborsDirected.html" class="trait" title="trait petgraph::visit::IntoNeighborsDirected">IntoNeighborsDirected</a>\>::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighborsDirected.html#associatedtype.NeighborsDirected" class="associatedtype" title="type petgraph::visit::IntoNeighborsDirected::NeighborsDirected">NeighborsDirected</a>, \<G as <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighborsDirected.html" class="trait" title="trait petgraph::visit::IntoNeighborsDirected">IntoNeighborsDirected</a>\>::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighborsDirected.html#associatedtype.NeighborsDirected" class="associatedtype" title="type petgraph::visit::IntoNeighborsDirected::NeighborsDirected">NeighborsDirected</a>\>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html#method.neighbors" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighbors.html#tymethod.neighbors" class="fn">neighbors</a>(self, n: G::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighbors.html#associatedtype.Neighbors" class="associatedtype" title="type petgraph::visit::IntoNeighbors::Neighbors">Neighbors</a>

Return an iterator of the neighbors of node `a`.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html#impl-IntoNodeIdentifiers-for-UndirectedAdaptor%3CG%3E" class="anchor">§</a>

### impl\<G\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeIdentifiers.html" class="trait" title="trait petgraph::visit::IntoNodeIdentifiers">IntoNodeIdentifiers</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html" class="struct" title="struct petgraph::visit::UndirectedAdaptor">UndirectedAdaptor</a>\<G\>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeIdentifiers.html" class="trait" title="trait petgraph::visit::IntoNodeIdentifiers">IntoNodeIdentifiers</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html#associatedtype.NodeIdentifiers" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeIdentifiers.html#associatedtype.NodeIdentifiers" class="associatedtype">NodeIdentifiers</a> = \<G as <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeIdentifiers.html" class="trait" title="trait petgraph::visit::IntoNodeIdentifiers">IntoNodeIdentifiers</a>\>::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeIdentifiers.html#associatedtype.NodeIdentifiers" class="associatedtype" title="type petgraph::visit::IntoNodeIdentifiers::NodeIdentifiers">NodeIdentifiers</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html#method.node_identifiers" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeIdentifiers.html#tymethod.node_identifiers" class="fn">node_identifiers</a>(self) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeIdentifiers.html#associatedtype.NodeIdentifiers" class="associatedtype" title="type petgraph::visit::IntoNodeIdentifiers::NodeIdentifiers">NodeIdentifiers</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html#impl-IntoNodeReferences-for-UndirectedAdaptor%3CG%3E" class="anchor">§</a>

### impl\<G\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeReferences.html" class="trait" title="trait petgraph::visit::IntoNodeReferences">IntoNodeReferences</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html" class="struct" title="struct petgraph::visit::UndirectedAdaptor">UndirectedAdaptor</a>\<G\>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeReferences.html" class="trait" title="trait petgraph::visit::IntoNodeReferences">IntoNodeReferences</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html#associatedtype.NodeRef" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeReferences.html#associatedtype.NodeRef" class="associatedtype">NodeRef</a> = \<G as <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeReferences.html" class="trait" title="trait petgraph::visit::IntoNodeReferences">IntoNodeReferences</a>\>::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeReferences.html#associatedtype.NodeRef" class="associatedtype" title="type petgraph::visit::IntoNodeReferences::NodeRef">NodeRef</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html#associatedtype.NodeReferences" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeReferences.html#associatedtype.NodeReferences" class="associatedtype">NodeReferences</a> = \<G as <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeReferences.html" class="trait" title="trait petgraph::visit::IntoNodeReferences">IntoNodeReferences</a>\>::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeReferences.html#associatedtype.NodeReferences" class="associatedtype" title="type petgraph::visit::IntoNodeReferences::NodeReferences">NodeReferences</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html#method.node_references" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeReferences.html#tymethod.node_references" class="fn">node_references</a>(self) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeReferences.html#associatedtype.NodeReferences" class="associatedtype" title="type petgraph::visit::IntoNodeReferences::NodeReferences">NodeReferences</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html#impl-NodeCount-for-UndirectedAdaptor%3CG%3E" class="anchor">§</a>

### impl\<G\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeCount.html" class="trait" title="trait petgraph::visit::NodeCount">NodeCount</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html" class="struct" title="struct petgraph::visit::UndirectedAdaptor">UndirectedAdaptor</a>\<G\>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeCount.html" class="trait" title="trait petgraph::visit::NodeCount">NodeCount</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html#method.node_count" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeCount.html#tymethod.node_count" class="fn">node_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html#impl-NodeIndexable-for-UndirectedAdaptor%3CG%3E" class="anchor">§</a>

### impl\<G\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeIndexable.html" class="trait" title="trait petgraph::visit::NodeIndexable">NodeIndexable</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html" class="struct" title="struct petgraph::visit::UndirectedAdaptor">UndirectedAdaptor</a>\<G\>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeIndexable.html" class="trait" title="trait petgraph::visit::NodeIndexable">NodeIndexable</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html#method.node_bound" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeIndexable.html#tymethod.node_bound" class="fn">node_bound</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Return an upper bound of the node indices in the graph (suitable for the size of a bitmap).

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html#method.to_index" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeIndexable.html#tymethod.to_index" class="fn">to_index</a>(&self, a: Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Convert `a` to an integer index.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html#method.from_index" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeIndexable.html#tymethod.from_index" class="fn">from_index</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>

Convert `i` to a node index. `i` must be a valid value in the graph.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html#impl-Visitable-for-UndirectedAdaptor%3CG%3E" class="anchor">§</a>

### impl\<G\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html" class="trait" title="trait petgraph::visit::Visitable">Visitable</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html" class="struct" title="struct petgraph::visit::UndirectedAdaptor">UndirectedAdaptor</a>\<G\>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html" class="trait" title="trait petgraph::visit::Visitable">Visitable</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html#associatedtype.Map" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html#associatedtype.Map" class="associatedtype">Map</a> = \<G as <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html" class="trait" title="trait petgraph::visit::Visitable">Visitable</a>\>::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html#associatedtype.Map" class="associatedtype" title="type petgraph::visit::Visitable::Map">Map</a>

The associated map type

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html#method.visit_map" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html#tymethod.visit_map" class="fn">visit_map</a>(&self) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html#associatedtype.Map" class="associatedtype" title="type petgraph::visit::Visitable::Map">Map</a>

Create a new visitor map

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html#method.reset_map" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html#tymethod.reset_map" class="fn">reset_map</a>(&self, map: &mut Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html#associatedtype.Map" class="associatedtype" title="type petgraph::visit::Visitable::Map">Map</a>)

Reset the visitor map (and resize to new size of graph if needed)

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html#impl-Copy-for-UndirectedAdaptor%3CG%3E" class="anchor">§</a>

### impl\<G: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a>\> <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html" class="struct" title="struct petgraph::visit::UndirectedAdaptor">UndirectedAdaptor</a>\<G\>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html#impl-GraphRef-for-UndirectedAdaptor%3CG%3E" class="anchor">§</a>

### impl\<G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphRef.html" class="trait" title="trait petgraph::visit::GraphRef">GraphRef</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphRef.html" class="trait" title="trait petgraph::visit::GraphRef">GraphRef</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html" class="struct" title="struct petgraph::visit::UndirectedAdaptor">UndirectedAdaptor</a>\<G\>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html#impl-NodeCompactIndexable-for-UndirectedAdaptor%3CG%3E" class="anchor">§</a>

### impl\<G\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeCompactIndexable.html" class="trait" title="trait petgraph::visit::NodeCompactIndexable">NodeCompactIndexable</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html" class="struct" title="struct petgraph::visit::UndirectedAdaptor">UndirectedAdaptor</a>\<G\>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeCompactIndexable.html" class="trait" title="trait petgraph::visit::NodeCompactIndexable">NodeCompactIndexable</a>,

## Auto Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html#blanket-implementations" class="anchor">§</a>
