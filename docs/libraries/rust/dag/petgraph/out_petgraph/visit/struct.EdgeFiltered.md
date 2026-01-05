# Struct EdgeFiltered Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/visit/filter.rs.html#381" class="src">Source</a>

``` rust
pub struct EdgeFiltered<G, F>(pub G, pub F);
```

Expand description

An edge-filtering graph adaptor.

The adaptor may filter out edges. The filter implements the trait `FilterEdge`. Closures of type `Fn(G::EdgeRef) -> bool` already implement this trait.

The filter may use edge source, target, id, and weight to select whether to include the edge or not.

## Tuple Fields<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#fields" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#structfield.0" class="anchor field">§</a>`0: G`<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#structfield.1" class="anchor field">§</a>`1: F`

## Implementations<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#impl-EdgeFiltered%3CG,+F%3E" class="anchor">§</a>

### impl\<F, G\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html" class="struct" title="struct petgraph::visit::EdgeFiltered">EdgeFiltered</a>\<G, F\>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgeReferences.html" class="trait" title="trait petgraph::visit::IntoEdgeReferences">IntoEdgeReferences</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(G::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgeReferences.html#associatedtype.EdgeRef" class="associatedtype" title="type petgraph::visit::IntoEdgeReferences::EdgeRef">EdgeRef</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#method.from_fn" class="fn">from_fn</a>(graph: G, filter: F) -\> Self

Create an `EdgeFiltered` adaptor from the closure `filter`.

## Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#impl-Clone-for-EdgeFiltered%3CG,+F%3E" class="anchor">§</a>

### impl\<G: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>, F: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html" class="struct" title="struct petgraph::visit::EdgeFiltered">EdgeFiltered</a>\<G, F\>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html" class="struct" title="struct petgraph::visit::EdgeFiltered">EdgeFiltered</a>\<G, F\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#impl-Data-for-EdgeFiltered%3CG,+F%3E" class="anchor">§</a>

### impl\<G, F\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html" class="trait" title="trait petgraph::visit::Data">Data</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html" class="struct" title="struct petgraph::visit::EdgeFiltered">EdgeFiltered</a>\<G, F\>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html" class="trait" title="trait petgraph::visit::Data">Data</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#associatedtype.NodeWeight" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html#associatedtype.NodeWeight" class="associatedtype">NodeWeight</a> = \<G as <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html" class="trait" title="trait petgraph::visit::Data">Data</a>\>::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html#associatedtype.NodeWeight" class="associatedtype" title="type petgraph::visit::Data::NodeWeight">NodeWeight</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#associatedtype.EdgeWeight" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html#associatedtype.EdgeWeight" class="associatedtype">EdgeWeight</a> = \<G as <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html" class="trait" title="trait petgraph::visit::Data">Data</a>\>::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html#associatedtype.EdgeWeight" class="associatedtype" title="type petgraph::visit::Data::EdgeWeight">EdgeWeight</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#impl-Debug-for-EdgeFiltered%3CG,+F%3E" class="anchor">§</a>

### impl\<G: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>, F: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html" class="struct" title="struct petgraph::visit::EdgeFiltered">EdgeFiltered</a>\<G, F\>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#impl-EdgeIndexable-for-EdgeFiltered%3CG,+F%3E" class="anchor">§</a>

### impl\<G, F\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeIndexable.html" class="trait" title="trait petgraph::visit::EdgeIndexable">EdgeIndexable</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html" class="struct" title="struct petgraph::visit::EdgeFiltered">EdgeFiltered</a>\<G, F\>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeIndexable.html" class="trait" title="trait petgraph::visit::EdgeIndexable">EdgeIndexable</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#method.edge_bound" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeIndexable.html#tymethod.edge_bound" class="fn">edge_bound</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Return an upper bound of the edge indices in the graph (suitable for the size of a bitmap).

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#method.to_index-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeIndexable.html#tymethod.to_index" class="fn">to_index</a>(&self, a: Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.EdgeId" class="associatedtype" title="type petgraph::visit::GraphBase::EdgeId">EdgeId</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Convert `a` to an integer index.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#method.from_index-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeIndexable.html#tymethod.from_index" class="fn">from_index</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.EdgeId" class="associatedtype" title="type petgraph::visit::GraphBase::EdgeId">EdgeId</a>

Convert `i` to an edge index. `i` must be a valid value in the graph.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#impl-GraphBase-for-EdgeFiltered%3CG,+F%3E" class="anchor">§</a>

### impl\<G, F\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html" class="trait" title="trait petgraph::visit::GraphBase">GraphBase</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html" class="struct" title="struct petgraph::visit::EdgeFiltered">EdgeFiltered</a>\<G, F\>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html" class="trait" title="trait petgraph::visit::GraphBase">GraphBase</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#associatedtype.NodeId" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype">NodeId</a> = \<G as <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html" class="trait" title="trait petgraph::visit::GraphBase">GraphBase</a>\>::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>

node identifier

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#associatedtype.EdgeId" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.EdgeId" class="associatedtype">EdgeId</a> = \<G as <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html" class="trait" title="trait petgraph::visit::GraphBase">GraphBase</a>\>::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.EdgeId" class="associatedtype" title="type petgraph::visit::GraphBase::EdgeId">EdgeId</a>

edge identifier

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#impl-GraphProp-for-EdgeFiltered%3CG,+F%3E" class="anchor">§</a>

### impl\<G, F\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphProp.html" class="trait" title="trait petgraph::visit::GraphProp">GraphProp</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html" class="struct" title="struct petgraph::visit::EdgeFiltered">EdgeFiltered</a>\<G, F\>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphProp.html" class="trait" title="trait petgraph::visit::GraphProp">GraphProp</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#associatedtype.EdgeType" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphProp.html#associatedtype.EdgeType" class="associatedtype">EdgeType</a> = \<G as <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphProp.html" class="trait" title="trait petgraph::visit::GraphProp">GraphProp</a>\>::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphProp.html#associatedtype.EdgeType" class="associatedtype" title="type petgraph::visit::GraphProp::EdgeType">EdgeType</a>

The kind of edges in the graph.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#method.is_directed" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphProp.html#method.is_directed" class="fn">is_directed</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#impl-IntoEdgeReferences-for-%26EdgeFiltered%3CG,+F%3E" class="anchor">§</a>

### impl\<'a, G, F\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgeReferences.html" class="trait" title="trait petgraph::visit::IntoEdgeReferences">IntoEdgeReferences</a> for &'a <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html" class="struct" title="struct petgraph::visit::EdgeFiltered">EdgeFiltered</a>\<G, F\>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgeReferences.html" class="trait" title="trait petgraph::visit::IntoEdgeReferences">IntoEdgeReferences</a>, F: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.FilterEdge.html" class="trait" title="trait petgraph::visit::FilterEdge">FilterEdge</a>\<G::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgeReferences.html#associatedtype.EdgeRef" class="associatedtype" title="type petgraph::visit::IntoEdgeReferences::EdgeRef">EdgeRef</a>\>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#associatedtype.EdgeRef" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgeReferences.html#associatedtype.EdgeRef" class="associatedtype">EdgeRef</a> = \<G as <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgeReferences.html" class="trait" title="trait petgraph::visit::IntoEdgeReferences">IntoEdgeReferences</a>\>::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgeReferences.html#associatedtype.EdgeRef" class="associatedtype" title="type petgraph::visit::IntoEdgeReferences::EdgeRef">EdgeRef</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#associatedtype.EdgeReferences" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgeReferences.html#associatedtype.EdgeReferences" class="associatedtype">EdgeReferences</a> = <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFilteredEdges.html" class="struct" title="struct petgraph::visit::EdgeFilteredEdges">EdgeFilteredEdges</a>\<'a, G, \<G as <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgeReferences.html" class="trait" title="trait petgraph::visit::IntoEdgeReferences">IntoEdgeReferences</a>\>::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgeReferences.html#associatedtype.EdgeReferences" class="associatedtype" title="type petgraph::visit::IntoEdgeReferences::EdgeReferences">EdgeReferences</a>, F\>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#method.edge_references" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgeReferences.html#tymethod.edge_references" class="fn">edge_references</a>(self) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgeReferences.html#associatedtype.EdgeReferences" class="associatedtype" title="type petgraph::visit::IntoEdgeReferences::EdgeReferences">EdgeReferences</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#impl-IntoEdges-for-%26EdgeFiltered%3CG,+F%3E" class="anchor">§</a>

### impl\<'a, G, F\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdges.html" class="trait" title="trait petgraph::visit::IntoEdges">IntoEdges</a> for &'a <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html" class="struct" title="struct petgraph::visit::EdgeFiltered">EdgeFiltered</a>\<G, F\>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdges.html" class="trait" title="trait petgraph::visit::IntoEdges">IntoEdges</a>, F: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.FilterEdge.html" class="trait" title="trait petgraph::visit::FilterEdge">FilterEdge</a>\<G::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgeReferences.html#associatedtype.EdgeRef" class="associatedtype" title="type petgraph::visit::IntoEdgeReferences::EdgeRef">EdgeRef</a>\>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#associatedtype.Edges" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdges.html#associatedtype.Edges" class="associatedtype">Edges</a> = <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFilteredEdges.html" class="struct" title="struct petgraph::visit::EdgeFilteredEdges">EdgeFilteredEdges</a>\<'a, G, \<G as <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdges.html" class="trait" title="trait petgraph::visit::IntoEdges">IntoEdges</a>\>::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdges.html#associatedtype.Edges" class="associatedtype" title="type petgraph::visit::IntoEdges::Edges">Edges</a>, F\>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#method.edges" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdges.html#tymethod.edges" class="fn">edges</a>(self, n: G::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdges.html#associatedtype.Edges" class="associatedtype" title="type petgraph::visit::IntoEdges::Edges">Edges</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#impl-IntoEdgesDirected-for-%26EdgeFiltered%3CG,+F%3E" class="anchor">§</a>

### impl\<'a, G, F\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgesDirected.html" class="trait" title="trait petgraph::visit::IntoEdgesDirected">IntoEdgesDirected</a> for &'a <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html" class="struct" title="struct petgraph::visit::EdgeFiltered">EdgeFiltered</a>\<G, F\>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgesDirected.html" class="trait" title="trait petgraph::visit::IntoEdgesDirected">IntoEdgesDirected</a>, F: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.FilterEdge.html" class="trait" title="trait petgraph::visit::FilterEdge">FilterEdge</a>\<G::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgeReferences.html#associatedtype.EdgeRef" class="associatedtype" title="type petgraph::visit::IntoEdgeReferences::EdgeRef">EdgeRef</a>\>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#associatedtype.EdgesDirected" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgesDirected.html#associatedtype.EdgesDirected" class="associatedtype">EdgesDirected</a> = <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFilteredEdges.html" class="struct" title="struct petgraph::visit::EdgeFilteredEdges">EdgeFilteredEdges</a>\<'a, G, \<G as <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgesDirected.html" class="trait" title="trait petgraph::visit::IntoEdgesDirected">IntoEdgesDirected</a>\>::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgesDirected.html#associatedtype.EdgesDirected" class="associatedtype" title="type petgraph::visit::IntoEdgesDirected::EdgesDirected">EdgesDirected</a>, F\>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#method.edges_directed" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgesDirected.html#tymethod.edges_directed" class="fn">edges_directed</a>(self, n: G::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>, dir: <a href="https://docs.rs/petgraph/latest/petgraph/enum.Direction.html" class="enum" title="enum petgraph::Direction">Direction</a>) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgesDirected.html#associatedtype.EdgesDirected" class="associatedtype" title="type petgraph::visit::IntoEdgesDirected::EdgesDirected">EdgesDirected</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#impl-IntoNeighbors-for-%26EdgeFiltered%3CG,+F%3E" class="anchor">§</a>

### impl\<'a, G, F\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighbors.html" class="trait" title="trait petgraph::visit::IntoNeighbors">IntoNeighbors</a> for &'a <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html" class="struct" title="struct petgraph::visit::EdgeFiltered">EdgeFiltered</a>\<G, F\>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdges.html" class="trait" title="trait petgraph::visit::IntoEdges">IntoEdges</a>, F: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.FilterEdge.html" class="trait" title="trait petgraph::visit::FilterEdge">FilterEdge</a>\<G::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgeReferences.html#associatedtype.EdgeRef" class="associatedtype" title="type petgraph::visit::IntoEdgeReferences::EdgeRef">EdgeRef</a>\>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#associatedtype.Neighbors" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighbors.html#associatedtype.Neighbors" class="associatedtype">Neighbors</a> = <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFilteredNeighbors.html" class="struct" title="struct petgraph::visit::EdgeFilteredNeighbors">EdgeFilteredNeighbors</a>\<'a, G, F\>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#method.neighbors" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighbors.html#tymethod.neighbors" class="fn">neighbors</a>(self, n: G::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighbors.html#associatedtype.Neighbors" class="associatedtype" title="type petgraph::visit::IntoNeighbors::Neighbors">Neighbors</a>

Return an iterator of the neighbors of node `a`.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#impl-IntoNeighborsDirected-for-%26EdgeFiltered%3CG,+F%3E" class="anchor">§</a>

### impl\<'a, G, F\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighborsDirected.html" class="trait" title="trait petgraph::visit::IntoNeighborsDirected">IntoNeighborsDirected</a> for &'a <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html" class="struct" title="struct petgraph::visit::EdgeFiltered">EdgeFiltered</a>\<G, F\>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgesDirected.html" class="trait" title="trait petgraph::visit::IntoEdgesDirected">IntoEdgesDirected</a>, F: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.FilterEdge.html" class="trait" title="trait petgraph::visit::FilterEdge">FilterEdge</a>\<G::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgeReferences.html#associatedtype.EdgeRef" class="associatedtype" title="type petgraph::visit::IntoEdgeReferences::EdgeRef">EdgeRef</a>\>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#associatedtype.NeighborsDirected" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighborsDirected.html#associatedtype.NeighborsDirected" class="associatedtype">NeighborsDirected</a> = <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFilteredNeighborsDirected.html" class="struct" title="struct petgraph::visit::EdgeFilteredNeighborsDirected">EdgeFilteredNeighborsDirected</a>\<'a, G, F\>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#method.neighbors_directed" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighborsDirected.html#tymethod.neighbors_directed" class="fn">neighbors_directed</a>( self, n: G::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>, dir: <a href="https://docs.rs/petgraph/latest/petgraph/enum.Direction.html" class="enum" title="enum petgraph::Direction">Direction</a>, ) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighborsDirected.html#associatedtype.NeighborsDirected" class="associatedtype" title="type petgraph::visit::IntoNeighborsDirected::NeighborsDirected">NeighborsDirected</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#impl-IntoNodeIdentifiers-for-%26EdgeFiltered%3CG,+F%3E" class="anchor">§</a>

### impl\<'a, G, F\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeIdentifiers.html" class="trait" title="trait petgraph::visit::IntoNodeIdentifiers">IntoNodeIdentifiers</a> for &'a <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html" class="struct" title="struct petgraph::visit::EdgeFiltered">EdgeFiltered</a>\<G, F\>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeIdentifiers.html" class="trait" title="trait petgraph::visit::IntoNodeIdentifiers">IntoNodeIdentifiers</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#associatedtype.NodeIdentifiers" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeIdentifiers.html#associatedtype.NodeIdentifiers" class="associatedtype">NodeIdentifiers</a> = \<G as <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeIdentifiers.html" class="trait" title="trait petgraph::visit::IntoNodeIdentifiers">IntoNodeIdentifiers</a>\>::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeIdentifiers.html#associatedtype.NodeIdentifiers" class="associatedtype" title="type petgraph::visit::IntoNodeIdentifiers::NodeIdentifiers">NodeIdentifiers</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#method.node_identifiers" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeIdentifiers.html#tymethod.node_identifiers" class="fn">node_identifiers</a>(self) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeIdentifiers.html#associatedtype.NodeIdentifiers" class="associatedtype" title="type petgraph::visit::IntoNodeIdentifiers::NodeIdentifiers">NodeIdentifiers</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#impl-IntoNodeReferences-for-%26EdgeFiltered%3CG,+F%3E" class="anchor">§</a>

### impl\<'a, G, F\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeReferences.html" class="trait" title="trait petgraph::visit::IntoNodeReferences">IntoNodeReferences</a> for &'a <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html" class="struct" title="struct petgraph::visit::EdgeFiltered">EdgeFiltered</a>\<G, F\>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeReferences.html" class="trait" title="trait petgraph::visit::IntoNodeReferences">IntoNodeReferences</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#associatedtype.NodeRef" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeReferences.html#associatedtype.NodeRef" class="associatedtype">NodeRef</a> = \<G as <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeReferences.html" class="trait" title="trait petgraph::visit::IntoNodeReferences">IntoNodeReferences</a>\>::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeReferences.html#associatedtype.NodeRef" class="associatedtype" title="type petgraph::visit::IntoNodeReferences::NodeRef">NodeRef</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#associatedtype.NodeReferences" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeReferences.html#associatedtype.NodeReferences" class="associatedtype">NodeReferences</a> = \<G as <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeReferences.html" class="trait" title="trait petgraph::visit::IntoNodeReferences">IntoNodeReferences</a>\>::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeReferences.html#associatedtype.NodeReferences" class="associatedtype" title="type petgraph::visit::IntoNodeReferences::NodeReferences">NodeReferences</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#method.node_references" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeReferences.html#tymethod.node_references" class="fn">node_references</a>(self) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeReferences.html#associatedtype.NodeReferences" class="associatedtype" title="type petgraph::visit::IntoNodeReferences::NodeReferences">NodeReferences</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#impl-NodeCount-for-EdgeFiltered%3CG,+F%3E" class="anchor">§</a>

### impl\<G, F\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeCount.html" class="trait" title="trait petgraph::visit::NodeCount">NodeCount</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html" class="struct" title="struct petgraph::visit::EdgeFiltered">EdgeFiltered</a>\<G, F\>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeCount.html" class="trait" title="trait petgraph::visit::NodeCount">NodeCount</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#method.node_count" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeCount.html#tymethod.node_count" class="fn">node_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#impl-NodeIndexable-for-EdgeFiltered%3CG,+F%3E" class="anchor">§</a>

### impl\<G, F\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeIndexable.html" class="trait" title="trait petgraph::visit::NodeIndexable">NodeIndexable</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html" class="struct" title="struct petgraph::visit::EdgeFiltered">EdgeFiltered</a>\<G, F\>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeIndexable.html" class="trait" title="trait petgraph::visit::NodeIndexable">NodeIndexable</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#method.node_bound" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeIndexable.html#tymethod.node_bound" class="fn">node_bound</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Return an upper bound of the node indices in the graph (suitable for the size of a bitmap).

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#method.to_index" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeIndexable.html#tymethod.to_index" class="fn">to_index</a>(&self, a: Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Convert `a` to an integer index.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#method.from_index" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeIndexable.html#tymethod.from_index" class="fn">from_index</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>

Convert `i` to a node index. `i` must be a valid value in the graph.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#impl-Visitable-for-EdgeFiltered%3CG,+F%3E" class="anchor">§</a>

### impl\<G, F\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html" class="trait" title="trait petgraph::visit::Visitable">Visitable</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html" class="struct" title="struct petgraph::visit::EdgeFiltered">EdgeFiltered</a>\<G, F\>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html" class="trait" title="trait petgraph::visit::Visitable">Visitable</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#associatedtype.Map" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html#associatedtype.Map" class="associatedtype">Map</a> = \<G as <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html" class="trait" title="trait petgraph::visit::Visitable">Visitable</a>\>::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html#associatedtype.Map" class="associatedtype" title="type petgraph::visit::Visitable::Map">Map</a>

The associated map type

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#method.visit_map" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html#tymethod.visit_map" class="fn">visit_map</a>(&self) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html#associatedtype.Map" class="associatedtype" title="type petgraph::visit::Visitable::Map">Map</a>

Create a new visitor map

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#method.reset_map" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html#tymethod.reset_map" class="fn">reset_map</a>(&self, map: &mut Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html#associatedtype.Map" class="associatedtype" title="type petgraph::visit::Visitable::Map">Map</a>)

Reset the visitor map (and resize to new size of graph if needed)

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#impl-Copy-for-EdgeFiltered%3CG,+F%3E" class="anchor">§</a>

### impl\<G: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a>, F: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a>\> <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html" class="struct" title="struct petgraph::visit::EdgeFiltered">EdgeFiltered</a>\<G, F\>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#impl-NodeCompactIndexable-for-EdgeFiltered%3CG,+F%3E" class="anchor">§</a>

### impl\<G, F\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeCompactIndexable.html" class="trait" title="trait petgraph::visit::NodeCompactIndexable">NodeCompactIndexable</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html" class="struct" title="struct petgraph::visit::EdgeFiltered">EdgeFiltered</a>\<G, F\>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeCompactIndexable.html" class="trait" title="trait petgraph::visit::NodeCompactIndexable">NodeCompactIndexable</a>,

## Auto Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.EdgeFiltered.html#blanket-implementations" class="anchor">§</a>
